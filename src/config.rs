use anyhow::Context;
use std::env::var;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub broker_url: String,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Config> {
        let broker_url = var("BROKER_URL")
            .context("Environment variable BROKER_URL is not set or is invalid")?;
        Ok(Config { broker_url })
    }
}
