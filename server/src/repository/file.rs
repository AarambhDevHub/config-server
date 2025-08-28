use crate::models::*;
use anyhow::Result;
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub struct FileRepository {
    base_path: String,
}

impl FileRepository {
    pub fn new(base_path: &str) -> Result<Self> {
        fs::create_dir_all(base_path)?;
        Ok(Self {
            base_path: base_path.to_string(),
        })
    }

    pub fn get_config(
        &self,
        application: &str,
        profile: &str,
        label: &str,
    ) -> Result<ConfigResponse> {
        let mut property_sources = Vec::new();
        let profiles: Vec<String> = profile.split(',').map(|s| s.trim().to_string()).collect();

        // Load application.yml first (lowest priority)
        if let Ok(source) = self.load_property_source("application", None) {
            property_sources.push(source);
        }

        // Load profile-specific files
        for prof in &profiles {
            if let Ok(source) = self.load_property_source("application", Some(prof)) {
                property_sources.push(source);
            }
        }

        // Load application-specific files
        if application != "application" {
            if let Ok(source) = self.load_property_source(application, None) {
                property_sources.push(source);
            }

            for prof in &profiles {
                if let Ok(source) = self.load_property_source(application, Some(prof)) {
                    property_sources.push(source);
                }
            }
        }

        Ok(ConfigResponse {
            name: application.to_string(),
            profiles,
            label: label.to_string(),
            version: None,
            property_sources,
        })
    }

    fn load_property_source(
        &self,
        application: &str,
        profile: Option<&str>,
    ) -> Result<PropertySource> {
        let filename = match profile {
            Some(p) => format!("{}-{}", application, p),
            None => application.to_string(),
        };

        // Try different file extensions
        for ext in &["yml", "yaml", "properties", "json"] {
            let file_path = Path::new(&self.base_path).join(format!("{}.{}", filename, ext));

            if file_path.exists() {
                let content = fs::read_to_string(&file_path)?;
                let source = match *ext {
                    "yml" | "yaml" => self.parse_yaml(&content)?,
                    "json" => self.parse_json(&content)?,
                    "properties" => self.parse_properties(&content)?,
                    _ => HashMap::new(),
                };

                return Ok(PropertySource {
                    name: file_path.to_string_lossy().to_string(),
                    source,
                });
            }
        }

        Err(anyhow::anyhow!(
            "No configuration file found for {}",
            filename
        ))
    }

    fn parse_yaml(&self, content: &str) -> Result<HashMap<String, Value>> {
        let yaml_value: serde_yaml::Value = serde_yaml::from_str(content)?;
        let json_value = serde_json::to_value(yaml_value)?;
        self.flatten_json(json_value, String::new())
    }

    fn parse_json(&self, content: &str) -> Result<HashMap<String, Value>> {
        let json_value: Value = serde_json::from_str(content)?;
        self.flatten_json(json_value, String::new())
    }

    fn parse_properties(&self, content: &str) -> Result<HashMap<String, Value>> {
        let mut map = HashMap::new();
        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            if let Some((key, value)) = line.split_once('=') {
                map.insert(
                    key.trim().to_string(),
                    Value::String(value.trim().to_string()),
                );
            }
        }
        Ok(map)
    }

    fn flatten_json(&self, value: Value, prefix: String) -> Result<HashMap<String, Value>> {
        let mut map = HashMap::new();

        match value {
            Value::Object(obj) => {
                for (key, val) in obj {
                    let new_key = if prefix.is_empty() {
                        key
                    } else {
                        format!("{}.{}", prefix, key)
                    };

                    match val {
                        Value::Object(_) | Value::Array(_) => {
                            let nested = self.flatten_json(val, new_key)?;
                            map.extend(nested);
                        }
                        _ => {
                            map.insert(new_key, val);
                        }
                    }
                }
            }
            Value::Array(arr) => {
                for (i, val) in arr.into_iter().enumerate() {
                    let new_key = format!("{}[{}]", prefix, i);
                    let nested = self.flatten_json(val, new_key)?;
                    map.extend(nested);
                }
            }
            _ => {
                map.insert(prefix, value);
            }
        }

        Ok(map)
    }
}
