use super::file::FileRepository;
use crate::models::*;
use anyhow::Result;
use git2::{Cred, FetchOptions, RemoteCallbacks, Repository};
use std::path::Path;

pub struct GitRepository {
    repo_path: String,
    git_uri: String,
    username: Option<String>,
    password: Option<String>,
    file_repo: FileRepository,
}

impl GitRepository {
    pub fn new(git_uri: &str, username: Option<&str>, password: Option<&str>) -> Result<Self> {
        let repo_path = "./git-config-repo";

        // Clone or open repository
        let _repo = if Path::new(repo_path).exists() {
            Repository::open(repo_path)?
        } else {
            let mut callbacks = RemoteCallbacks::new();

            if let (Some(user), Some(pass)) = (username, password) {
                callbacks.credentials(|_url, _username_from_url, _allowed_types| {
                    Cred::userpass_plaintext(user, pass)
                });
            }

            let mut fetch_options = FetchOptions::new();
            fetch_options.remote_callbacks(callbacks);

            let mut builder = git2::build::RepoBuilder::new();
            builder.fetch_options(fetch_options);
            builder.clone(git_uri, Path::new(repo_path))?
        };

        let file_repo = FileRepository::new(repo_path)?;

        Ok(Self {
            repo_path: repo_path.to_string(),
            git_uri: git_uri.to_string(),
            username: username.map(|s| s.to_string()),
            password: password.map(|s| s.to_string()),
            file_repo,
        })
    }

    pub async fn get_config(
        &self,
        application: &str,
        profile: &str,
        label: &str,
    ) -> Result<ConfigResponse> {
        // Ensure we're on the correct branch/label
        self.checkout_label(label)?;
        self.file_repo.get_config(application, profile, label)
    }

    pub async fn pull(&self) -> Result<()> {
        let repo = Repository::open(&self.repo_path)?;
        let mut remote = repo.find_remote("origin")?;

        let mut callbacks = RemoteCallbacks::new();
        if let (Some(user), Some(pass)) = (&self.username, &self.password) {
            callbacks.credentials(|_url, _username_from_url, _allowed_types| {
                Cred::userpass_plaintext(user, pass)
            });
        }

        let mut fetch_options = FetchOptions::new();
        fetch_options.remote_callbacks(callbacks);

        remote.fetch(
            &["refs/heads/*:refs/remotes/origin/*"],
            Some(&mut fetch_options),
            None,
        )?;

        Ok(())
    }

    fn checkout_label(&self, label: &str) -> Result<()> {
        let repo = Repository::open(&self.repo_path)?;

        // Try to find the branch
        let branch_name = format!("refs/remotes/origin/{}", label);
        if let Ok(reference) = repo.find_reference(&branch_name) {
            let commit = reference.peel_to_commit()?;
            repo.set_head_detached(commit.id())?;

            let mut checkout_builder = git2::build::CheckoutBuilder::new();
            checkout_builder.force();
            repo.checkout_head(Some(&mut checkout_builder))?;
        }

        Ok(())
    }
}
