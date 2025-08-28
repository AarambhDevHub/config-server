use crate::{client::ConfigClient, error::ConfigError};
use serde_json::Value;
use std::collections::HashMap;

pub struct ConfigClientBuilder {
    server_url: Option<String>,
    application: Option<String>,
    profile: Option<String>,
    label: Option<String>,
}

impl ConfigClientBuilder {
    pub fn new() -> Self {
        Self {
            server_url: None,
            application: None,
            profile: None,
            label: None,
        }
    }

    pub fn server_url(mut self, url: &str) -> Self {
        self.server_url = Some(url.to_string());
        self
    }

    pub fn application(mut self, app: &str) -> Self {
        self.application = Some(app.to_string());
        self
    }

    pub fn profile(mut self, profile: &str) -> Self {
        self.profile = Some(profile.to_string());
        self
    }

    pub fn label(mut self, label: &str) -> Self {
        self.label = Some(label.to_string());
        self
    }

    pub fn build(self) -> ConfigClient {
        ConfigClient::new(
            self.server_url
                .unwrap_or_else(|| "http://localhost:8888".to_string()),
            self.application
                .unwrap_or_else(|| "application".to_string()),
            self.profile.unwrap_or_else(|| "default".to_string()),
            self.label.unwrap_or_else(|| "master".to_string()),
        )
    }
}

impl Default for ConfigClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct ConfigSource {
    pub name: String,
    pub profiles: Vec<String>,
    pub label: String,
    pub version: Option<String>,
    pub properties: HashMap<String, Value>,
}

impl ConfigSource {
    pub fn from_response(response: Value) -> Self {
        let mut properties = HashMap::new();

        if let Some(property_sources) = response["propertySources"].as_array() {
            for source in property_sources {
                if let Some(source_map) = source["source"].as_object() {
                    for (key, value) in source_map {
                        properties.insert(key.clone(), value.clone());
                    }
                }
            }
        }

        Self {
            name: response["name"].as_str().unwrap_or("").to_string(),
            profiles: response["profiles"]
                .as_array()
                .map(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_str())
                        .map(|s| s.to_string())
                        .collect()
                })
                .unwrap_or_default(),
            label: response["label"].as_str().unwrap_or("").to_string(),
            version: response["version"].as_str().map(|s| s.to_string()),
            properties,
        }
    }

    pub fn get_flat_properties(&self) -> HashMap<String, String> {
        let mut flat = HashMap::new();
        for (key, value) in &self.properties {
            let string_value = match value {
                Value::String(s) => s.clone(),
                Value::Number(n) => n.to_string(),
                Value::Bool(b) => b.to_string(),
                _ => serde_json::to_string(value).unwrap_or_default(),
            };
            flat.insert(key.clone(), string_value);
        }
        flat
    }

    pub fn get_property(&self, key: &str) -> Option<&Value> {
        self.properties.get(key)
    }

    pub fn get_string(&self, key: &str) -> Option<String> {
        self.properties
            .get(key)
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
    }

    pub fn get_string_or(&self, key: &str, default: &str) -> String {
        self.get_string(key).unwrap_or_else(|| default.to_string())
    }

    pub fn get_bool(&self, key: &str) -> Option<bool> {
        self.properties.get(key).and_then(|v| match v {
            Value::Bool(b) => Some(*b),
            Value::String(s) => s.parse().ok(),
            _ => None,
        })
    }

    pub fn get_i64(&self, key: &str) -> Option<i64> {
        self.properties.get(key).and_then(|v| match v {
            Value::Number(n) => n.as_i64(),
            Value::String(s) => s.parse().ok(),
            _ => None,
        })
    }

    pub fn get_f64(&self, key: &str) -> Option<f64> {
        self.properties.get(key).and_then(|v| match v {
            Value::Number(n) => n.as_f64(),
            Value::String(s) => s.parse().ok(),
            _ => None,
        })
    }
}
