pub mod client;
pub mod config;
pub mod error;

pub use client::ConfigClient;
pub use config::{ConfigClientBuilder, ConfigSource};
pub use error::ConfigError;

use once_cell::sync::Lazy;
use std::{collections::HashMap, env};
use tokio::sync::RwLock;

static GLOBAL_CONFIG: Lazy<RwLock<HashMap<String, String>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));

/// Initialize the config client and load configuration
pub async fn init_config(
    server_url: &str,
    application: &str,
    profile: &str,
    label: Option<&str>,
) -> Result<(), ConfigError> {
    let client = ConfigClientBuilder::new()
        .server_url(server_url)
        .application(application)
        .profile(profile)
        .label(label.unwrap_or("master"))
        .build();

    let config = client.fetch_config().await?;

    // Load into global configuration
    let mut global_config = GLOBAL_CONFIG.write().await;
    for (key, value) in config.get_flat_properties() {
        // std::env::set_var is safe - no unsafe block needed
        unsafe {
            env::set_var(&key, &value);
        }
        global_config.insert(key, value);
    }

    tracing::info!("Configuration loaded successfully from {}", server_url);
    Ok(())
}

/// Get a configuration value
pub async fn get_config(key: &str) -> Option<String> {
    let config = GLOBAL_CONFIG.read().await;
    config.get(key).cloned()
}

/// Get a configuration value with default
pub async fn get_config_or(key: &str, default: &str) -> String {
    get_config(key).await.unwrap_or_else(|| default.to_string())
}

/// Refresh configuration from server
pub async fn refresh_config() -> Result<(), ConfigError> {
    // This would need to store the original client parameters
    // For simplicity, this is a placeholder
    Ok(())
}

/// Get all configuration as a HashMap
pub async fn get_all_config() -> HashMap<String, String> {
    let config = GLOBAL_CONFIG.read().await;
    config.clone()
}

/// Print all configuration to stdout
pub async fn print_all_config() -> Result<(), ConfigError> {
    let config = GLOBAL_CONFIG.read().await;
    println!("\n📋 All loaded configurations:");
    println!("{:-<50}", "");

    if config.is_empty() {
        println!("No configurations loaded.");
        return Ok(());
    }

    let mut configs: Vec<_> = config.iter().collect();
    configs.sort_by_key(|(k, _)| *k);

    for (key, value) in configs {
        // Mask sensitive values (passwords, secrets, etc.)
        let masked_value = if key.to_lowercase().contains("password")
            || key.to_lowercase().contains("secret")
            || key.to_lowercase().contains("key")
            || value.starts_with("{cipher}")
        {
            "***MASKED***".to_string()
        } else {
            value.clone()
        };
        println!("{:<30} = {}", key, masked_value);
    }
    println!("{:-<50}", "");
    println!("Total configurations: {}\n", config.len());
    Ok(())
}
