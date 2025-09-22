use crate::config::ZenConfig;
use crate::utils::print_no_aliases_message;

fn print_aliases_header() {
    println!("Available aliases:");
}

fn print_alias_entry(alias: &str, command: &str) {
    println!("  {}: {}", alias, command);
}

pub fn handle_list_command(_args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    let config = ZenConfig::load()?;

    if config.is_empty() {
        print_no_aliases_message();
        return Ok(());
    }

    print_aliases_header();
    for (alias, command) in config.available_aliases() {
        print_alias_entry(alias, command);
    }

    Ok(())
}
