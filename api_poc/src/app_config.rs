use config::{Config, ConfigError, Environment};
use dotenv::dotenv;
use log::{info, warn};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub appname: String
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