use std::fs;

use anyhow::{Context, Result};

use crate::config::{Config, paths::config_file};

impl Config {
    pub fn load() -> Result<Self> {
        let path = config_file()?;

        // Create parent directory if it doesn't exist.
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).context("Failed to create configuration directory")?;
        }

        // First launch: create config.toml.
        if !path.exists() {
            let default = Config::default();

            let toml = toml::to_string_pretty(&default)
                .context("Failed to serialize default configuration")?;

            fs::write(&path, toml).context("Failed to create config.toml")?;

            return Ok(default);
        }

        let contents = fs::read_to_string(&path).context("Failed to read config.toml")?;

        let config: Config = toml::from_str(&contents).context("Invalid configuration file")?;

        config.validate()?;

        Ok(config)
    }

    pub fn save(&self) -> Result<()> {
        let path = config_file()?;

        let toml = toml::to_string_pretty(self).context("Failed to serialize configuration")?;

        fs::write(path, toml).context("Failed to save configuration")?;

        Ok(())
    }

    fn validate(&self) -> Result<()> {
        if self.volume > 100 {
            anyhow::bail!("Volume must be between 0 and 100.");
        }

        if self.visualizer_fps == 0 {
            anyhow::bail!("Visualizer FPS must be greater than zero.");
        }

        Ok(())
    }
}
