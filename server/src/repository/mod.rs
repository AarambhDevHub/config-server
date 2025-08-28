pub mod file;
pub mod git;

use crate::{config::ServerConfig, models::*};
use anyhow::Result;
use dashmap::DashMap;
use std::sync::Arc;

pub struct ConfigRepository {
    config: ServerConfig,
    cache: Arc<DashMap<String, ConfigResponse>>,
    file_repo: file::FileRepository,
    git_repo: Option<git::GitRepository>,
}

impl ConfigRepository {
    pub async fn new(config: ServerConfig) -> Result<Self> {
        let file_repo = file::FileRepository::new(&config.config_path)?;
        let git_repo = if let Some(git_uri) = &config.git_uri {
            Some(git::GitRepository::new(
                git_uri,
                config.git_username.as_deref(),
                config.git_password.as_deref(),
            )?)
        } else {
            None
        };

        Ok(Self {
            config,
            cache: Arc::new(DashMap::new()),
            file_repo,
            git_repo,
        })
    }

    pub async fn get_config(
        &self,
        application: &str,
        profile: &str,
        label: &str,
    ) -> Result<ConfigResponse> {
        let cache_key = format!("{}:{}:{}", application, profile, label);

        if let Some(cached) = self.cache.get(&cache_key) {
            return Ok(cached.clone());
        }

        // Try Git repository first, then file repository
        let config = if let Some(git_repo) = &self.git_repo {
            git_repo.get_config(application, profile, label).await
                .or_else(|_| self.file_repo.get_config(application, profile, label))
        } else {
            self.file_repo.get_config(application, profile, label)
        }?;

        self.cache.insert(cache_key, config.clone());
        Ok(config)
    }

    pub async fn refresh(&self) -> Result<()> {
        self.cache.clear();
        if let Some(git_repo) = &self.git_repo {
            git_repo.pull().await?;
        }
        Ok(())
    }
}
