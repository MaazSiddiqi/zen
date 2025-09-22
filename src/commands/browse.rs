use crate::commands::handle_list_command;
use crate::commands::handle_run_command;
use crate::config::ZenConfig;
use crate::utils::{fzf_is_available, fzf_select, print_fzf_install_message, print_no_aliases_message};

fn print_fallback_message() {
    println!();
    println!("Falling back to list view:");
}

pub fn handle_browse_command(_args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    let config = ZenConfig::load()?;

    if config.is_empty() {
        print_no_aliases_message();
        return Ok(());
    }

    if !fzf_is_available() {
        print_fzf_install_message();
        print_fallback_message();
        return handle_list_command(&[]);
    }

    let entries: Vec<String> = config
        .available_aliases()
        .map(|(alias, command)| format!("{}\t{}", alias, command))
        .collect();

    match fzf_select(&entries)? {
        Some(selection) => {
            let alias = selection.split('\t').next().unwrap_or(&selection);
            handle_run_command(&[alias.to_string()])?;
        }
        None => {} // User cancelled - exit silently
    }

    Ok(())
}
