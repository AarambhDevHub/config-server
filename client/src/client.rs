use crate::{config::ConfigSource, error::ConfigError};
use reqwest::Client;
use serde_json::Value;
use std::collections::HashMap;

pub struct ConfigClient {
    client: Client,
    server_url: String,
    application: String,
    profile: String,
    label: String,
}

impl ConfigClient {
    pub fn new(server_url: String, application: String, profile: String, label: String) -> Self {
        Self {
            client: Client::new(),
            server_url,
            application,
            profile,
            label,
        }
    }

    pub async fn fetch_config(&self) -> Result<ConfigSource, ConfigError> {
        let url = format!(
            "{}/{}/{}/{}",
            self.server_url.trim_end_matches('/'),
            self.application,
            self.profile,
            self.label
        );

        tracing::debug!("Fetching config from: {}", url);

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(ConfigError::Request)?;

        if !response.status().is_success() {
            return Err(ConfigError::Http(response.status().as_u16()));
        }

        let config_response: serde_json::Value =
            response.json().await.map_err(ConfigError::Request)?;

        Ok(ConfigSource::from_response(config_response))
    }

    pub async fn encrypt_value(&self, value: &str) -> Result<String, ConfigError> {
        let url = format!("{}/encrypt", self.server_url.trim_end_matches('/'));

        let request_body = serde_json::json!({ "value": value });

        let response = self
            .client
            .post(&url)
            .json(&request_body)
            .send()
            .await
            .map_err(ConfigError::Request)?;

        if !response.status().is_success() {
            return Err(ConfigError::Http(response.status().as_u16()));
        }

        let encrypt_response: Value = response.json().await.map_err(ConfigError::Request)?;

        Ok(encrypt_response["encrypted"]
            .as_str()
            .unwrap_or("")
            .to_string())
    }

    pub async fn decrypt_value(&self, encrypted: &str) -> Result<String, ConfigError> {
        let url = format!("{}/decrypt", self.server_url.trim_end_matches('/'));

        let request_body = serde_json::json!({ "encrypted": encrypted });

        let response = self
            .client
            .post(&url)
            .json(&request_body)
            .send()
            .await
            .map_err(ConfigError::Request)?;

        if !response.status().is_success() {
            return Err(ConfigError::Http(response.status().as_u16()));
        }

        let decrypt_response: Value = response.json().await.map_err(ConfigError::Request)?;

        Ok(decrypt_response["decrypted"]
            .as_str()
            .unwrap_or("")
            .to_string())
    }

    pub async fn refresh_server_config(&self) -> Result<(), ConfigError> {
        let url = format!("{}/refresh", self.server_url.trim_end_matches('/'));

        let response = self
            .client
            .post(&url)
            .send()
            .await
            .map_err(ConfigError::Request)?;

        if !response.status().is_success() {
            return Err(ConfigError::Http(response.status().as_u16()));
        }

        Ok(())
    }
}
