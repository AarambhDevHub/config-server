use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigResponse {
    pub name: String,
    pub profiles: Vec<String>,
    pub label: String,
    pub version: Option<String>,
    #[serde(rename = "propertySources")]
    pub property_sources: Vec<PropertySource>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropertySource {
    pub name: String,
    pub source: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EncryptRequest {
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EncryptResponse {
    pub encrypted: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecryptRequest {
    pub encrypted: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecryptResponse {
    pub decrypted: String,
}
