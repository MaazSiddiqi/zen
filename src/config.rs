use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const ZEN_CONFIG_IDENTIFIER: &str = "zen-config.toml";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ZenConfig {
    pub commands: HashMap<String, String>,
}

impl ZenConfig {
    pub fn new() -> Self {
        Self {
            commands: HashMap::new(),
        }
    }

    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        if std::fs::metadata(ZEN_CONFIG_IDENTIFIER).is_err() {
            return Ok(Self::new());
        }

        let file_contents = std::fs::read_to_string(ZEN_CONFIG_IDENTIFIER)?;
        let config = toml::from_str(&file_contents)?;
        Ok(config)
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let file_contents = toml::to_string(self)?;
        std::fs::write(ZEN_CONFIG_IDENTIFIER, file_contents)?;
        Ok(())
    }

    pub fn add_command(&mut self, alias: String, command: String) {
        self.commands.insert(alias, command);
    }

    pub fn get_command(&self, alias: &str) -> Option<&String> {
        self.commands.get(alias)
    }

    pub fn remove_command(&mut self, alias: &str) -> bool {
        self.commands.remove(alias).is_some()
    }

    pub fn is_empty(&self) -> bool {
        self.commands.is_empty()
    }

    pub fn available_aliases(&self) -> impl Iterator<Item = (&String, &String)> {
        self.commands.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_config() {
        let config = ZenConfig::new();
        assert!(config.commands.is_empty());
        assert!(config.is_empty());
    }

    #[test]
    fn test_add_command() {
        let mut config = ZenConfig::new();
        config.add_command("dev".to_string(), "npm run dev".to_string());

        assert_eq!(config.get_command("dev"), Some(&"npm run dev".to_string()));
        assert!(!config.is_empty());
    }

    #[test]
    fn test_remove_command() {
        let mut config = ZenConfig::new();
        config.add_command("dev".to_string(), "npm run dev".to_string());

        assert!(config.remove_command("dev"));
        assert!(config.is_empty());
        assert!(!config.remove_command("nonexistent"));
    }

    #[test]
    fn test_available_aliases() {
        let mut config = ZenConfig::new();
        config.add_command("dev".to_string(), "npm run dev".to_string());
        config.add_command("test".to_string(), "npm test".to_string());

        let aliases: Vec<_> = config.available_aliases().collect();
        assert_eq!(aliases.len(), 2);

        // Check that both aliases are present (order may vary due to HashMap)
        let has_dev = aliases
            .iter()
            .any(|(alias, cmd)| *alias == "dev" && *cmd == "npm run dev");
        let has_test = aliases
            .iter()
            .any(|(alias, cmd)| *alias == "test" && *cmd == "npm test");

        assert!(has_dev);
        assert!(has_test);
    }
}
