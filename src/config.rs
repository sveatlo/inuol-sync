use figment::{
    providers::{Env, Format, Toml},
    Figment,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    pub server: ServerConfig,
    pub invoice_ninja: InvoiceNinjaConfig,
    pub uol: UolConfig,
    pub logging: LoggingConfig,
    pub email: EmailConfig,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct InvoiceNinjaConfig {
    pub base_url: String,
    pub api_token: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UolConfig {
    pub base_url: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct LoggingConfig {
    #[serde(default = "default_log_level")]
    pub level: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct EmailConfig {
    pub smtp_host: String,
    pub smtp_port: u16,
    pub smtp_username: String,
    pub smtp_password: String,
    pub from: String,
    pub to: String,
}

impl Config {
    #[expect(clippy::result_large_err)]
    pub fn load() -> Result<Self, figment::Error> {
        Figment::new()
            .merge(Toml::file("config.toml"))
            .merge(Env::prefixed("APP_").split("__"))
            .extract()
    }
}

fn default_log_level() -> String {
    "INFO".to_owned()
}
