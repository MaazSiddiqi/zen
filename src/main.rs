use serde::{Deserialize, Serialize};
use std::process::Command;
use std::{collections::HashMap, env};
use toml;

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
        if !std::fs::metadata(ZEN_CONFIG_IDENTIFIER).is_ok() {
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
    has_loaded: bool,
}

impl Zen {
    fn new() -> Zen {
        Zen {
            config: ZenConfig::new(),
            has_loaded: false,
        }
    }
    fn load(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.config = ZenConfig::load()?;
        self.has_loaded = true;
        Ok(())
    }

    fn register_alias(
        &mut self,
        alias: String,
        command: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if !self.has_loaded {
            panic!("zen has not been loaded");
        }

        self.config.add_command(alias.clone(), command.clone());
        self.config.save()?;

        Ok(())
    }

    fn execute_alias(
        self,
        alias: String,
        args: Vec<&str>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        if !self.has_loaded {
            panic!("zen has not been loaded");
        }

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
}

fn main() {
    let args: Vec<String> = env::args().collect();

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
            let alias = args[1..idx].join(" ");
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
            let alias = &args[1];
            let alias_args: Vec<&str> = args.iter().skip(2).map(|x| x.as_str()).collect();

            match zen.execute_alias(alias.clone(), alias_args) {
                Ok(found) => {
                    if !found {
                        println!("No command registered for alias '{}'", alias);
                        println!("Register a command with:");
                        println!("  zz {} --register <command> [args]", alias);
                        return;
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
