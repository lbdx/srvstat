use anyhow::Context;
use std::env::var;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub broker_url: String,
    pub fan_speed_command: Option<String>,
    pub fan_speed_file: Option<String>,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Config> {
        let broker_url = var("BROKER_URL")
            .context("Environment variable BROKER_URL is not set or is invalid")?;
        let fan_speed_command = std::env::var("SRVSTAT_FAN_SPEED_COMMAND").ok();
        let fan_speed_file = std::env::var("SRVSTAT_FAN_SPEED_FILE").ok();
        Ok(Config {
            broker_url,
            fan_speed_command,
            fan_speed_file,
        })
    }
}
