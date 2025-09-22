use crate::config::ZenConfig;

fn print_usage() {
    println!("Usage: zen add <alias> <command>");
}

fn print_success(alias: &str, command: &str) {
    println!("Successfully registered alias");
    println!("  {}: {}", alias, command);
}

pub fn handle_add_command(args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    if args.len() < 2 {
        print_usage();
        return Ok(());
    }

    let alias = &args[0];
    let command = args[1..].join(" ");

    let mut config = ZenConfig::load()?;
    config.add_command(alias.clone(), command.clone());
    config.save()?;

    print_success(alias, &command);

    Ok(())
}
