use crate::commands::handle_browse_command;
use crate::core::Zen;

fn print_registration_success(alias: &str, command: &str) {
    println!("Successfully registered alias");
    println!("  {}: {}", alias, command);
}

fn print_alias_not_found(alias: &str) {
    println!("No command registered for alias '{}'", alias);
    println!();
    println!("Register a command to this alias with:");
    println!("  zz {} --register <command> [args]", alias);
}

pub fn handle_run_command(args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    if args.is_empty() {
        return handle_browse_command(&[]);
    }

    let mut zen = Zen::load()?;

    let register_idx = args.iter().position(|arg| arg == "--register");

    match register_idx {
        Some(idx) => {
            let alias = args[..idx].join(" ");
            let command = args[idx + 1..].join(" ");

            zen.register_alias(alias.clone(), command.clone())?;
            print_registration_success(&alias, &command);
        }
        None => {
            let alias = &args[0];
            let alias_args: Vec<&str> = args.iter().skip(1).map(|x| x.as_str()).collect();

            match zen.execute_alias(alias, &alias_args)? {
                true => {} // Command executed successfully
                false => {
                    print_alias_not_found(alias);
                }
            }
        }
    }

    Ok(())
}
