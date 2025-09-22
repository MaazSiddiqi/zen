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
        let command_template = match self.config.get_command(alias) {
            Some(command) => command,
            None => return Ok(false),
        };

        let command = self.substitute_parameters(command_template, args);

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

    fn substitute_parameters(&self, template: &str, args: &[&str]) -> String {
        let mut result = template.to_string();

        // Replace {} placeholders with arguments
        for arg in args {
            if let Some(pos) = result.find("{}") {
                result.replace_range(pos..pos + 2, arg);
            } else {
                // If no more placeholders, append remaining args
                result.push(' ');
                result.push_str(arg);
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parameter_substitution() {
        let zen = Zen::new();

        // Test single parameter
        let result = zen.substitute_parameters("test {} --verbose", &["mytest"]);
        assert_eq!(result, "test mytest --verbose");

        // Test multiple parameters
        let result = zen.substitute_parameters("cp {} {}", &["file1.txt", "file2.txt"]);
        assert_eq!(result, "cp file1.txt file2.txt");

        // Test extra args without placeholders
        let result = zen.substitute_parameters("make build", &["--release", "--target"]);
        assert_eq!(result, "make build --release --target");

        // Test no args with placeholders (should leave placeholders)
        let result = zen.substitute_parameters("test {} --verbose", &[]);
        assert_eq!(result, "test {} --verbose");

        // Test mixed: some placeholders filled, extra args appended
        let result = zen.substitute_parameters("run {} tests", &["unit", "--verbose", "--parallel"]);
        assert_eq!(result, "run unit tests --verbose --parallel");
    }
}
