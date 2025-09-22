use serde::{Deserialize, Serialize};
use std::process::Command;
use std::{collections::HashMap, env};

const ZEN_CONFIG_IDENTIFIER: &str = "zen-config.toml";
const ZEN_ENV_USE_INTERACTIVE: &str = "ZEN_USE_INTERACTIVE";

#[derive(Serialize, Deserialize)]
struct ZenConfig {
    commands: HashMap<String, String>,
}

impl ZenConfig {
    fn new() -> ZenConfig {
        ZenConfig {
            commands: HashMap::new(),
        }
    }
    fn load() -> Result<ZenConfig, Box<dyn std::error::Error>> {
        if std::fs::metadata(ZEN_CONFIG_IDENTIFIER).is_err() {
            return Ok(ZenConfig::new());
        }

        let file_contents = std::fs::read_to_string(ZEN_CONFIG_IDENTIFIER)?;
        let config = toml::from_str(file_contents.as_str())?;
        Ok(config)
    }
    fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let file_contents = toml::to_string(self)?;
        std::fs::write(ZEN_CONFIG_IDENTIFIER, file_contents)?;
        Ok(())
    }
    fn add_command(&mut self, alias: String, command: String) {
        self.commands.insert(alias, command);
    }
    fn get_command(&self, alias: &str) -> Option<&String> {
        self.commands.get(alias)
    }
}

struct Zen {
    config: ZenConfig,
}

impl Zen {
    fn new() -> Zen {
        Zen {
            config: ZenConfig::new(),
        }
    }
    fn load(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.config = ZenConfig::load()?;
        Ok(())
    }

    fn register_alias(
        &mut self,
        alias: String,
        command: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.config.add_command(alias.clone(), command.clone());
        self.config.save()?;

        Ok(())
    }

    fn execute_alias(
        &self,
        alias: String,
        args: Vec<&str>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let command = match self.config.get_command(alias.as_str()) {
            Some(command) => {
                if args.is_empty() {
                    command.clone()
                } else {
                    format!("{} {}", command, args.join(" "))
                }
            }
            None => {
                return Ok(false);
            }
        };

        let shell = env::var("SHELL").unwrap_or("/bin/sh".to_string());
        let use_interactive = env::var(ZEN_ENV_USE_INTERACTIVE).unwrap_or("false".to_string());

        let mut program = Command::new(shell);

        if use_interactive.to_lowercase() == "true" {
            program.arg("-i");
        }

        program.arg("-c").arg(command);
        program.status()?;

        Ok(true)
    }

    fn discard_alias(&mut self, alias: String) -> Result<bool, Box<dyn std::error::Error>> {
        if !self.config.commands.contains_key(&alias) {
            return Ok(false);
        }

        self.config.commands.remove(&alias);
        self.config.save()?;
        Ok(true)
    }
}

fn handle_list_command(_args: &[String]) {
    let mut zen = Zen::new();

    match zen.load() {
        Ok(_) => {}
        Err(err) => {
            println!("Something went wrong while initializing zen: {}", err);
            std::process::exit(1);
        }
    }

    if zen.config.commands.is_empty() {
        println!("No aliases registered.");
        println!();
        println!("Register an alias with:");
        println!("  zen add <alias> <command>");
        return;
    }

    println!("Available aliases:");
    for (alias, command) in &zen.config.commands {
        println!("  {}: {}", alias, command);
    }
}

fn handle_add_command(args: &[String]) {
    if args.len() < 2 {
        println!("Usage: zen add <alias> <command>");
        return;
    }

    let alias = &args[0];
    let command = args[1..].join(" ");

    let mut zen = Zen::new();
    match zen.load() {
        Ok(_) => {}
        Err(err) => {
            println!("Something went wrong while initializing zen: {}", err);
            std::process::exit(1);
        }
    }

    match zen.register_alias(alias.clone(), command.clone()) {
        Ok(_) => {
            println!("Successfully registered alias");
            println!("  {}: {}", alias, command);
        }
        Err(err) => {
            println!("Could not register alias: {}", err);
            std::process::exit(1);
        }
    }
}

fn handle_remove_command(args: &[String]) {
    if args.is_empty() {
        println!("Usage: zen remove <alias>");
        return;
    }

    let mut zen = Zen::new();

    match zen.load() {
        Ok(_) => {}
        Err(err) => {
            println!("Something went wrong while initializing zen: {}", err);
            std::process::exit(1);
        }
    }

    let alias = &args[0];

    match zen.discard_alias(alias.clone()) {
        Ok(existed) => {
            if !existed {
                println!("Alias was not found in the registry");
            } else {
                println!("Successfully discarded alias {}", alias);
            }
        }
        Err(err) => {
            println!("Something went wrong while discarding: {}", err)
        }
    }
}

fn handle_run_command(args: &[String]) {
    if args.is_empty() {
        return handle_browse_command(&[]);
    }

    let mut zen = Zen::new();

    match zen.load() {
        Ok(_) => {}
        Err(err) => {
            println!("Something went wrong while initializing zen: {}", err);
            std::process::exit(1);
        }
    }

    let register_idx = args.iter().position(|arg| arg == "--register");

    match register_idx {
        Some(idx) => {
            let alias = args[..idx].join(" ");
            let command = args[idx + 1..].join(" ");

            match zen.register_alias(alias.clone(), command.clone()) {
                Ok(_) => {
                    println!("Successfully registered alias");
                    println!("  {}: {}", alias, command);
                }
                Err(err) => {
                    println!("Could not register alias: {}", err);
                    std::process::exit(1);
                }
            }
        }
        None => {
            let alias = &args[0];
            let alias_args: Vec<&str> = args.iter().skip(1).map(|x| x.as_str()).collect();

            match zen.execute_alias(alias.clone(), alias_args) {
                Ok(found) => {
                    if !found {
                        println!("No command registered for alias '{}'", alias);
                        println!();
                        println!("Register a command to this alias with:");
                        println!("  zz {} --register <command> [args]", alias);
                    }
                }
                Err(err) => {
                    println!("Something went wrong while executing the command: {}", err);
                    std::process::exit(1);
                }
            }
        }
    }
}

fn check_fzf_available() -> bool {
    Command::new("fzf")
        .arg("--version")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

fn handle_browse_command(_args: &[String]) {
    let mut zen = Zen::new();

    match zen.load() {
        Ok(_) => {}
        Err(err) => {
            println!("Something went wrong while initializing zen: {}", err);
            std::process::exit(1);
        }
    }

    if zen.config.commands.is_empty() {
        println!("No aliases registered.");
        println!();
        println!("Register an alias with:");
        println!("  zen add <alias> <command>");
        return;
    }

    if !check_fzf_available() {
        println!("fzf is not installed or not in PATH.");
        println!("Install fzf to use the browse feature:");
        println!("  brew install fzf                    # macOS");
        println!("  sudo apt install fzf                # Ubuntu/Debian");
        println!("  https://github.com/junegunn/fzf     # Other systems");
        println!();
        println!("Falling back to list view:");
        return handle_list_command(&[]);
    }

    let fzf_entries: Vec<String> = zen
        .config
        .commands
        .iter()
        .map(|(alias, command)| format!("{}\t{}", alias, command))
        .collect();

    let mut browse = match Command::new("fzf")
        .arg("--delimiter=\t")
        .arg("--with-nth=1")
        .arg("--preview=echo {2}")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
    {
        Ok(child) => child,
        Err(err) => {
            println!("Something went wrong while browsing: {}", err);
            return handle_list_command(&[]);
        }
    };

    if let Some(stdin) = browse.stdin.take() {
        use std::io::Write;
        let mut stdin = stdin;
        let input = fzf_entries.join("\n");
        let _ = stdin.write_all(input.as_bytes());
    }

    match browse.wait_with_output() {
        Ok(output) => {
            if output.status.success() {
                let selection = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if !selection.is_empty() {
                    let alias = selection.split('\t').next().unwrap_or(selection.as_str());

                    handle_run_command(&[alias.to_string()]);
                }
            }
            // User cancelled (Ctrl+C) - just exit silently
        }
        Err(err) => {
            println!("Error running fzf: {}", err);
        }
    }
}

fn print_usage() {
    println!("zen - A simple command launcher and alias manager");
    println!();
    println!("USAGE:");
    println!("    zen <COMMAND> [OPTIONS]");
    println!();
    println!("COMMANDS:");
    println!("    add <alias> <command>    Register a new command alias");
    println!("    run <alias> [args]       Execute a registered alias");
    println!("    list                     Show all registered aliases");
    println!("    remove <alias>           Delete an alias");
    println!("    browse                   Interactive alias selection (requires fzf)");
    println!();
    println!("EXAMPLES:");
    println!("    zen add dev \"npm run dev\"");
    println!("    zen run dev --port 3000");
    println!("    zen list");
    println!("    zen remove dev");
    println!();
    println!("QUICK USAGE:");
    println!("    zz <alias> [args]              Same as: zen run <alias> [args]");
    println!("    zz <alias> --register <cmd>    Same as: zen add <alias> <cmd>");
    println!("    zz                             Interactive browse (fzf required)");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        return;
    }

    let subcommand = &args[1];

    match subcommand.as_str() {
        "run" => handle_run_command(&args[2..]),
        "list" => handle_list_command(&args[2..]),
        "add" => handle_add_command(&args[2..]),
        "remove" => handle_remove_command(&args[2..]),
        "browse" => handle_browse_command(&args[2..]),
        "--help" | "-h" | "help" => print_usage(),
        _ => {
            println!("Unknown command: {}", subcommand);
            print_usage();
        }
    }
}
