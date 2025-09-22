use crate::config::ZenConfig;
use std::env;
use std::process::Command;

const ZEN_ENV_USE_INTERACTIVE: &str = "ZEN_USE_INTERACTIVE";

pub struct Zen {
    config: ZenConfig,
}

impl Zen {
    pub fn new() -> Self {
        Self {
            config: ZenConfig::new(),
        }
    }

    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let config = ZenConfig::load()?;
        Ok(Self { config })
    }

    pub fn register_alias(
        &mut self,
        alias: String,
        command: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.config.add_command(alias, command);
        self.config.save()?;
        Ok(())
    }

    pub fn execute_alias(
        &self,
        alias: &str,
        args: &[&str],
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let command = match self.config.get_command(alias) {
            Some(command) => {
                if args.is_empty() {
                    command.clone()
                } else {
                    format!("{} {}", command, args.join(" "))
                }
            }
            None => return Ok(false),
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

    pub fn available_aliases(&self) -> impl Iterator<Item = (&String, &String)> {
        self.config.available_aliases()
    }
}
