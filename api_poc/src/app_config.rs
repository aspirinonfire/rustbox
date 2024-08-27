use config::{Config, ConfigError, Environment};
use dotenvy::dotenv;
use log::{info, warn};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct AppConfig {
    pub mongo_connection_string: String,
    pub appname: String,
    pub host_ip: String,
    pub port: u16,
    pub jwt_signing_key: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            mongo_connection_string: String::from("mongo://localhost:27017?directConnection=true&authSource=admin"),
            appname: String::from("game api"),
            host_ip: String::from("0.0.0.0"),
            port: 8000,
            jwt_signing_key: String::new(),
        }
    }
}

impl AppConfig {
    pub fn build_config() -> Result<Self, ConfigError> {
        // if .env file is available, parse it, and load parsed values as env vars
        let dot_env_res = dotenv().ok();
        match dot_env_res {
            Some(_) => info!("using .env file for configuration."),
            None => warn!(".env file not present!"),
        }

        // Add in settings from the environment (with a prefix of APP)
        // E.g., `APP_DATABASE_URL` would set the `database_url`
        let config = Config::builder()
            .add_source(Environment::with_prefix("APP").try_parsing(true))
            .build()?;

        config.try_deserialize()
    }
}
