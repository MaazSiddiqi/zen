use std::io::Write;
use std::process::{Command, Stdio};

pub fn fzf_is_available() -> bool {
    Command::new("fzf")
        .arg("--version")
        .output()
        .is_ok_and(|output| output.status.success())
}

pub fn fzf_select(entries: &[String]) -> Result<Option<String>, Box<dyn std::error::Error>> {
    let mut cmd = Command::new("fzf")
        .args(["--delimiter=\t", "--with-nth=1", "--preview=echo {2}"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    if let Some(mut stdin) = cmd.stdin.take() {
        stdin.write_all(entries.join("\n").as_bytes())?;
    }

    let output = cmd.wait_with_output()?;
    if output.status.success() {
        let selection = String::from_utf8_lossy(&output.stdout).trim().to_string();
        return Ok((!selection.is_empty()).then_some(selection));
    }
    Ok(None)
}

pub fn print_no_aliases_message() {
    println!("No aliases registered.");
    println!();
    println!("Register an alias with:");
    println!("  zen add <alias> <command>");
}

pub fn print_fzf_install_message() {
    println!("fzf is not installed or not in PATH.");
    println!("Install fzf to use the browse feature:");
    println!("  brew install fzf                    # macOS");
    println!("  sudo apt install fzf                # Ubuntu/Debian");
    println!("  https://github.com/junegunn/fzf     # Other systems");
}
