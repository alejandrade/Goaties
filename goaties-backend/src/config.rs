use anyhow::Result;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub server: ServerConfig,
    pub app: AppConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub rest_port: u16,
    pub rest_path: String,
    pub ws_port: u16,
    pub ws_path: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub db_path: String,
}

impl Config {
    pub fn load() -> Result<Self> {
        let settings = config::Config::builder()
            .add_source(config::File::with_name("config"))
            .build()?;

        let config: Config = settings.try_deserialize()?;
        Ok(config)
    }
}