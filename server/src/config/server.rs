use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub server_port: u16,
    pub health_port: u16,
    pub metrics_port: u16,
    pub config_path: String,
    pub git_uri: Option<String>,
    pub git_username: Option<String>,
    pub git_password: Option<String>,
    pub encrypt_key: String,
    pub default_label: String,
    pub search_locations: Vec<String>,
}

impl ServerConfig {
    pub fn from_env() -> anyhow::Result<Self> {
        Ok(Self {
            server_port: env::var("SERVER_PORT")
                .unwrap_or_else(|_| "8888".to_string())
                .parse()?,
            health_port: env::var("HEALTH_PORT")
                .unwrap_or_else(|_| "8889".to_string())
                .parse()?,
            metrics_port: env::var("METRICS_PORT")
                .unwrap_or_else(|_| "8890".to_string())
                .parse()?,
            config_path: env::var("CONFIG_PATH")
                .unwrap_or_else(|_| "./configs".to_string()),
            git_uri: env::var("GIT_URI").ok(),
            git_username: env::var("GIT_USERNAME").ok(),
            git_password: env::var("GIT_PASSWORD").ok(),
            encrypt_key: env::var("ENCRYPT_KEY")
                .unwrap_or_else(|_| "default-secret-key-32-characters".to_string()),
            default_label: env::var("DEFAULT_LABEL")
                .unwrap_or_else(|_| "master".to_string()),
            search_locations: env::var("SEARCH_LOCATIONS")
                .unwrap_or_else(|_| "classpath:/,classpath:/config/".to_string())
                .split(',')
                .map(|s| s.trim().to_string())
                .collect(),
        })
    }
}
