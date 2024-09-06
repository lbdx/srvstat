#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub server_port: Option<&'static str>,
    pub server_url: Option<&'static str>,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Config> {
        let server_port = option_env!("SERVER_PORT");
        let server_url = option_env!("SERVER_URL");

        Ok(Config {
            server_port,
            server_url,
        })
    }
}