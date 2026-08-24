// Copyright 2026 Synthicsoft Labs LLC
// Licensed under the Apache License, Version 2.0 and the MIT License.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionRequest {
    pub program: String,
    pub args: Vec<String>,
    pub timeout_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionOutput {
    pub exit_code: i32,
    pub stdout: String,
    pub stderr: String,
}

#[derive(Debug, thiserror::Error)]
pub enum SandboxError {
    #[error("execution timed out")]
    Timeout,
    #[error("sandbox backend rejected request: {0}")]
    Rejected(String),
    #[error("sandbox backend failed: {0}")]
    Backend(String),
}

#[async_trait]
pub trait Sandbox: Send + Sync {
    async fn execute(&self, request: ExecutionRequest) -> Result<ExecutionOutput, SandboxError>;
}

#[derive(Debug, Default, Clone, Copy)]
pub struct DenyAllSandbox;

#[async_trait]
impl Sandbox for DenyAllSandbox {
    async fn execute(&self, _request: ExecutionRequest) -> Result<ExecutionOutput, SandboxError> {
        Err(SandboxError::Rejected("no execution backend is enabled".into()))
    }
}

pub fn bounded_timeout(ms: u64) -> Duration {
    Duration::from_millis(ms.clamp(1, 300_000))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn deny_all_never_executes_host_code() {
        let sandbox = DenyAllSandbox;
        let result = sandbox.execute(ExecutionRequest {
            program: "echo".into(),
            args: vec!["hello".into()],
            timeout_ms: 1000,
        }).await;
        assert!(matches!(result, Err(SandboxError::Rejected(_))));
    }

    #[test]
    fn timeout_is_bounded() {
        assert_eq!(bounded_timeout(0), Duration::from_millis(1));
        assert_eq!(bounded_timeout(u64::MAX), Duration::from_millis(300_000));
    }
}
