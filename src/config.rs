use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use tracing::{debug, info};

use crate::error::{SimError, SimResult};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Config {
    /// Default app URL (so you don't type it every time)
    pub app_url: Option<String>,

    /// Default phone number
    pub phone: Option<String>,

    /// Max steps before assuming infinite loop
    pub max_steps: Option<u32>,

    /// Request timeout in seconds
    pub timeout_secs: Option<u64>,

    /// Directory to save session logs
    pub log_dir: Option<String>,
}

impl Config {
    /// Returns the path to the config file: ~/.config/ussd-sim/config.toml
    pub fn config_path() -> PathBuf {
        let base = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."));
        base.join("ussd-sim").join("config.toml")
    }

    /// Loads config from disk, returns default if file doesn't exist
    pub fn load() -> SimResult<Self> {
        let path = Self::config_path();
        debug!("Looking for config at: {}", path.display());

        if !path.exists() {
            info!("No config file found, using defaults");
            return Ok(Self::default());
        }

        let contents = std::fs::read_to_string(&path)
            .map_err(|e| SimError::ConfigError(e.to_string()))?;

        let config: Config = toml::from_str(&contents)
            .map_err(|e| SimError::ConfigError(e.to_string()))?;

        info!("Loaded config from: {}", path.display());
        debug!("Config: {:?}", config);

        Ok(config)
    }

    /// Saves config to disk
    pub fn save(&self) -> SimResult<()> {
        let path = Self::config_path();

        // Create directory if it doesn't exist
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| SimError::ConfigError(e.to_string()))?;
        }

        let contents = toml::to_string_pretty(self)
            .map_err(|e| SimError::ConfigError(e.to_string()))?;

        std::fs::write(&path, contents)
            .map_err(|e| SimError::ConfigError(e.to_string()))?;

        info!("Config saved to: {}", path.display());
        Ok(())
    }

    /// Prints current config to terminal
    pub fn print(&self) {
        let path = Self::config_path();
        println!("\n📄 Config file: {}", path.display());
        println!("{}", "─".repeat(41));
        println!("  app_url     : {}", self.app_url.as_deref().unwrap_or("not set"));
        println!("  phone       : {}", self.phone.as_deref().unwrap_or("not set"));
        println!("  max_steps   : {}", self.max_steps.unwrap_or(20));
        println!("  timeout_secs: {}", self.timeout_secs.unwrap_or(30));
        println!("  log_dir     : {}", self.log_dir.as_deref().unwrap_or("not set"));
        println!("{}", "─".repeat(41));
    }
}