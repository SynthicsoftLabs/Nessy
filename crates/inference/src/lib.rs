// Copyright 2026 Synthicsoft Labs LLC
// Licensed under the Apache License, Version 2.0.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InferenceRequest {
    pub model: Option<String>,
    pub input: serde_json::Value,
    pub stream: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InferenceResponse {
    pub provider: String,
    pub model: Option<String>,
    pub output: serde_json::Value,
}

#[derive(Debug, thiserror::Error)]
pub enum InferenceError {
    #[error("provider unavailable: {0}")]
    Unavailable(String),
    #[error("provider rejected request: {0}")]
    Rejected(String),
    #[error("provider error: {0}")]
    Provider(String),
}

#[async_trait]
pub trait InferenceProvider: Send + Sync {
    fn name(&self) -> &str;
    async fn health(&self) -> bool;
    async fn infer(&self, request: InferenceRequest) -> Result<InferenceResponse, InferenceError>;
}

#[derive(Default, Clone)]
pub struct InferenceRouter {
    providers: Arc<RwLock<Vec<Arc<dyn InferenceProvider>>>>,
}

impl InferenceRouter {
    pub async fn register(&self, provider: Arc<dyn InferenceProvider>) {
        self.providers.write().await.push(provider);
    }

    pub async fn providers(&self) -> Vec<String> {
        self.providers
            .read()
            .await
            .iter()
            .map(|p| p.name().to_owned())
            .collect()
    }

    pub async fn infer(
        &self,
        request: InferenceRequest,
    ) -> Result<InferenceResponse, InferenceError> {
        let providers = self.providers.read().await.clone();
        let mut last_error = None;
        for provider in providers {
            if !provider.health().await {
                continue;
            }
            match provider.infer(request.clone()).await {
                Ok(response) => return Ok(response),
                Err(error) => last_error = Some(error),
            }
        }
        Err(last_error
            .unwrap_or_else(|| InferenceError::Unavailable("no healthy providers".into())))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Provider {
        name: &'static str,
        succeeds: bool,
    }

    #[async_trait]
    impl InferenceProvider for Provider {
        fn name(&self) -> &str {
            self.name
        }

        async fn health(&self) -> bool {
            true
        }

        async fn infer(
            &self,
            request: InferenceRequest,
        ) -> Result<InferenceResponse, InferenceError> {
            if !self.succeeds {
                return Err(InferenceError::Provider("failed".into()));
            }
            Ok(InferenceResponse {
                provider: self.name.into(),
                model: request.model,
                output: request.input,
            })
        }
    }

    #[tokio::test]
    async fn router_fails_over_to_next_provider() {
        let router = InferenceRouter::default();
        router
            .register(Arc::new(Provider {
                name: "first",
                succeeds: false,
            }))
            .await;
        router
            .register(Arc::new(Provider {
                name: "second",
                succeeds: true,
            }))
            .await;
        let response = router
            .infer(InferenceRequest {
                model: Some("test".into()),
                input: serde_json::json!({"x":1}),
                stream: false,
            })
            .await
            .unwrap();
        assert_eq!(response.provider, "second");
    }
}
