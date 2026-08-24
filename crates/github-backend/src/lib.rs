// Copyright 2026 Synthicsoft Labs LLC
// Licensed under the Apache License, Version 2.0.

use async_trait::async_trait;
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubBackendConfig {
    pub owner: String,
    pub repository: String,
    pub api_base: String,
    pub request_timeout_ms: u64,
    pub max_retries: u32,
}

impl GitHubBackendConfig {
    pub fn from_repository(repository: impl Into<String>) -> Result<Self, GitHubBackendError> {
        let repository = repository.into();
        let (owner, name) = repository
            .split_once('/')
            .ok_or(GitHubBackendError::InvalidRepository)?;
        if owner.is_empty() || name.is_empty() || name.contains('/') {
            return Err(GitHubBackendError::InvalidRepository);
        }
        Ok(Self {
            owner: owner.into(),
            repository: name.into(),
            api_base: "https://api.github.com".into(),
            request_timeout_ms: 30_000,
            max_retries: 5,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepositorySnapshot {
    pub full_name: String,
    pub default_branch: String,
    pub visibility: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackendHealth {
    pub available: bool,
    pub latency_ms: Option<u64>,
    pub status: Option<u16>,
}

#[derive(Debug, Error)]
pub enum GitHubBackendError {
    #[error("invalid GitHub repository name")]
    InvalidRepository,
    #[error("GitHub API request failed: {0}")]
    Request(#[from] reqwest::Error),
    #[error("GitHub API returned HTTP status {0}")]
    Http(StatusCode),
}

#[async_trait]
pub trait GitHubBackend: Send + Sync {
    async fn repository(&self) -> Result<RepositorySnapshot, GitHubBackendError>;
    async fn health(&self) -> BackendHealth;
}

#[derive(Clone)]
pub struct GitHubApiBackend {
    client: Client,
    config: GitHubBackendConfig,
}

impl GitHubApiBackend {
    pub fn new(config: GitHubBackendConfig) -> Result<Self, GitHubBackendError> {
        let client = Client::builder()
            .timeout(std::time::Duration::from_millis(config.request_timeout_ms))
            .user_agent("BowserAI-Nessy")
            .build()?;
        Ok(Self { client, config })
    }

    async fn get_json<T: for<'de> Deserialize<'de>>(
        &self,
        path: &str,
    ) -> Result<T, GitHubBackendError> {
        let url = format!(
            "{}/repos/{}/{}{}",
            self.config.api_base.trim_end_matches('/'),
            self.config.owner,
            self.config.repository,
            path
        );
        let response = self.client.get(url).send().await?;
        if !response.status().is_success() {
            return Err(GitHubBackendError::Http(response.status()));
        }
        Ok(response.json().await?)
    }
}

#[derive(Debug, Deserialize)]
struct GitHubRepository {
    full_name: String,
    default_branch: String,
    visibility: String,
}

#[async_trait]
impl GitHubBackend for GitHubApiBackend {
    async fn repository(&self) -> Result<RepositorySnapshot, GitHubBackendError> {
        let repo: GitHubRepository = self.get_json("").await?;
        Ok(RepositorySnapshot {
            full_name: repo.full_name,
            default_branch: repo.default_branch,
            visibility: repo.visibility,
        })
    }

    async fn health(&self) -> BackendHealth {
        let started = std::time::Instant::now();
        match self.repository().await {
            Ok(_) => BackendHealth {
                available: true,
                latency_ms: Some(started.elapsed().as_millis() as u64),
                status: Some(200),
            },
            Err(GitHubBackendError::Http(status)) => BackendHealth {
                available: false,
                latency_ms: Some(started.elapsed().as_millis() as u64),
                status: Some(status.as_u16()),
            },
            Err(_) => BackendHealth {
                available: false,
                latency_ms: Some(started.elapsed().as_millis() as u64),
                status: None,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn repository_configuration_is_automatic() {
        let config = GitHubBackendConfig::from_repository("SynthicsoftLabs/Nessy").unwrap();
        assert_eq!(config.owner, "SynthicsoftLabs");
        assert_eq!(config.repository, "Nessy");
        assert_eq!(config.max_retries, 5);
    }
}
