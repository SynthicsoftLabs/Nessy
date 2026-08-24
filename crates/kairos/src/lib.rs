// Copyright 2026 Synthicsoft Labs LLC
// Licensed under the Apache License, Version 2.0.

use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use thiserror::Error;

pub const DEFAULT_KAIROS_URL: &str = "https://the-real-kairos.com";

#[derive(Debug, Clone)]
pub struct KairosClient {
    base_url: String,
    client: Client,
}

#[derive(Debug, Clone, Serialize)]
pub struct TaskRequest {
    pub task_id: String,
    pub session_id: String,
    pub input: serde_json::Value,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TaskResponse {
    pub task_id: Option<String>,
    pub status: Option<String>,
    #[serde(default)]
    pub output: serde_json::Value,
}

#[derive(Debug, Error)]
pub enum KairosError {
    #[error("invalid Kairos URL")]
    InvalidUrl,
    #[error("Kairos request failed: {0}")]
    Request(#[from] reqwest::Error),
    #[error("Kairos returned HTTP {0}")]
    Http(reqwest::StatusCode),
}

impl KairosClient {
    pub fn new(base_url: impl Into<String>) -> Result<Self, KairosError> {
        let base_url = base_url.into().trim_end_matches('/').to_owned();
        if !(base_url.starts_with("https://") || base_url.starts_with("http://")) {
            return Err(KairosError::InvalidUrl);
        }
        let client = Client::builder()
            .connect_timeout(Duration::from_secs(10))
            .timeout(Duration::from_secs(120))
            .build()?;
        Ok(Self { base_url, client })
    }

    pub fn from_environment() -> Result<Self, KairosError> {
        let url = std::env::var("KAIROS_URL").unwrap_or_else(|_| DEFAULT_KAIROS_URL.to_owned());
        Self::new(url)
    }

    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    pub async fn submit(&self, request: &TaskRequest) -> Result<TaskResponse, KairosError> {
        let response = self
            .client
            .post(format!("{}/v1/tasks", self.base_url))
            .json(request)
            .send()
            .await?;
        let status = response.status();
        if !status.is_success() {
            return Err(KairosError::Http(status));
        }
        Ok(response.json().await?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_to_real_kairos() {
        let client = KairosClient::from_environment().unwrap();
        assert!(client.base_url() == DEFAULT_KAIROS_URL || client.base_url().starts_with("http"));
    }

    #[test]
    fn rejects_non_http_urls() {
        assert!(matches!(
            KairosClient::new("file:///tmp"),
            Err(KairosError::InvalidUrl)
        ));
    }
}
