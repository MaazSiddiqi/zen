use std::env;
use zen::commands::{
    handle_add_command, handle_browse_command, handle_list_command, handle_remove_command,
    handle_run_command,
};

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

    let result = match subcommand.as_str() {
        "run" => handle_run_command(&args[2..]),
        "list" => handle_list_command(&args[2..]),
        "add" => handle_add_command(&args[2..]),
        "remove" => handle_remove_command(&args[2..]),
        "browse" => handle_browse_command(&args[2..]),
        "--help" | "-h" | "help" => {
            print_usage();
            Ok(())
        }
        _ => {
            println!("Unknown command: {}", subcommand);
            print_usage();
            Ok(())
        }
    };

    if let Err(err) = result {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
}
