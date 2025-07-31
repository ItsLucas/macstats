//! Configuration management for macstats

use crate::influx::InfluxConfig;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, path::PathBuf};

/// Main configuration structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// InfluxDB configuration
    pub influx: InfluxConfig,
    /// System hostname/identifier
    pub hostname: Option<String>,
    /// Collection interval in seconds
    pub interval: Option<u64>,
    /// Metrics to collect
    pub metrics: MetricsConfig,
}

/// Metrics collection configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsConfig {
    /// Collect CPU temperatures
    pub cpu_temp: bool,
    /// Collect GPU temperatures  
    pub gpu_temp: bool,
    /// Collect system temperatures
    pub system_temp: bool,
    /// Collect power metrics
    pub power: bool,
    /// Collect fan speeds
    pub fans: bool,
}

impl Default for Config {
    fn default() -> Self {
        let hostname = hostname::get()
            .ok()
            .and_then(|h| h.into_string().ok())
            .unwrap_or_else(|| "unknown".to_string());

        Self {
            influx: InfluxConfig::default(),
            hostname: Some(hostname),
            interval: Some(30),
            metrics: MetricsConfig::default(),
        }
    }
}

impl Default for MetricsConfig {
    fn default() -> Self {
        Self {
            cpu_temp: true,
            gpu_temp: true,
            system_temp: true,
            power: true,
            fans: true,
        }
    }
}

impl Config {
    /// Get configuration file path
    pub fn config_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
        let config_dir = dirs::config_dir()
            .ok_or("Cannot determine config directory")?;
        
        let app_config_dir = config_dir.join("macstats");
        fs::create_dir_all(&app_config_dir)?;
        
        Ok(app_config_dir.join("config.toml"))
    }

    /// Load configuration from file
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let config_path = Self::config_path()?;
        println!("Loading config from: {:?}", config_path);
        if config_path.exists() {
            let content = fs::read_to_string(&config_path)?;
            let config: Config = toml::from_str(&content)?;
            Ok(config)
        } else {
            // Create default config
            let config = Config::default();
            config.save()?;
            Ok(config)
        }
    }

    /// Save configuration to file
    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config_path = Self::config_path()?;
        let content = toml::to_string_pretty(self)?;
        fs::write(&config_path, content)?;
        Ok(())
    }

    /// Create example configuration
    pub fn create_example() -> Self {
        let mut influx = InfluxConfig::default();
        influx.url = "http://localhost:8086".to_string();
        influx.database = "macstats".to_string();
        influx.username = Some("your_username".to_string());
        influx.password = Some("your_password".to_string());
        
        // Example for InfluxDB v2
        influx.org = Some("your_org".to_string());
        influx.token = Some("your_token_here".to_string());
        influx.bucket = Some("macstats".to_string());
        
        // Add example tags
        let mut tags = HashMap::new();
        tags.insert("host".to_string(), "your_hostname".to_string());
        tags.insert("location".to_string(), "office".to_string());
        influx.tags = Some(tags);

        Self {
            influx,
            hostname: Some("your_mac".to_string()),  
            interval: Some(30),
            metrics: MetricsConfig {
                cpu_temp: true,
                gpu_temp: true,
                system_temp: true,
                power: true,
                fans: true,
            },
        }
    }
}