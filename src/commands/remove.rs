use crate::config::ZenConfig;

fn print_usage() {
    println!("Usage: zen remove <alias>");
}

fn print_success(alias: &str) {
    println!("Successfully discarded alias {}", alias);
}

fn print_not_found() {
    println!("Alias was not found in the registry");
}

pub fn handle_remove_command(args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    if args.is_empty() {
        print_usage();
        return Ok(());
    }

    let alias = &args[0];
    let mut config = ZenConfig::load()?;

    if config.remove_command(alias) {
        config.save()?;
        print_success(alias);
    } else {
        print_not_found();
    }

    Ok(())
}
