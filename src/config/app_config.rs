use std::env;
use std::path::Path;
use std::time::Duration;

use config::File;
use config::{Config, ConfigError};
use serde::Deserialize;

use crate::tools::json::deserialize_duration;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub server: Server,
    pub db: Database,
    pub redis: Redis,
}

impl AppConfig {
    pub fn new() -> Self {
        Self::load().expect("failed to load config file")
    }

    fn load() -> Result<AppConfig, ConfigError> {
        let path = Path::new("src/config/env");
        let profile = env::var("RUST_ENV").unwrap_or_else(|_| String::from("local"));

        let result = Config::builder()
            .add_source(File::from(path.join("defaults.yml")))
            .add_source(File::from(path.join(format!("{}.yml", profile))))
            .build()?;

        let conf = result.try_deserialize()?;

        Ok(conf)
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Deserialize)]
pub struct Server {
    pub port: u16,
}

#[derive(Debug, Deserialize)]
pub struct Database {
    pub host: String,
    pub port: u16,
    pub max_connections: usize,

    #[serde(deserialize_with = "deserialize_duration")]
    pub max_idle: Duration,
}

#[derive(Debug, Deserialize)]
pub struct Redis {
    pub host: String,
    pub port: u16,

    #[serde(deserialize_with = "deserialize_duration")]
    pub ttl: Duration,
}
