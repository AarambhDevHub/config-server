use std::fmt;

#[derive(Debug)]
pub enum ConfigError {
    Request(reqwest::Error),
    Http(u16),
    Parse(String),
    NotFound(String),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::Request(err) => write!(f, "Request error: {}", err),
            ConfigError::Http(status) => write!(f, "HTTP error: {}", status),
            ConfigError::Parse(msg) => write!(f, "Parse error: {}", msg),
            ConfigError::NotFound(resource) => write!(f, "Not found: {}", resource),
        }
    }
}

impl std::error::Error for ConfigError {}
