// Copyright 2026 Synthicsoft Labs LLC
// Licensed under the Apache License, Version 2.0 and the MIT License.

use bowser_core::{validate_tool, ToolDescriptor};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum JsonRpcId {
    Number(i64),
    String(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcRequest {
    pub jsonrpc: String,
    pub id: Option<JsonRpcId>,
    pub method: String,
    #[serde(default)]
    pub params: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcError {
    pub code: i64,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcResponse {
    pub jsonrpc: String,
    pub id: Option<JsonRpcId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<JsonRpcError>,
}

#[derive(Debug, thiserror::Error)]
pub enum McpError {
    #[error("invalid JSON-RPC version")]
    InvalidVersion,
    #[error("invalid method")]
    InvalidMethod,
    #[error("tool validation failed: {0}")]
    InvalidTool(String),
    #[error("tool not found")]
    ToolNotFound,
}

impl JsonRpcRequest {
    pub fn validate(&self) -> Result<(), McpError> {
        if self.jsonrpc != "2.0" { return Err(McpError::InvalidVersion); }
        if self.method.is_empty() || self.method.len() > 256 { return Err(McpError::InvalidMethod); }
        Ok(())
    }
}

#[derive(Clone, Default)]
pub struct ToolRegistry {
    tools: Arc<RwLock<HashMap<String, ToolDescriptor>>>,
}

impl ToolRegistry {
    pub async fn register(&self, tool: ToolDescriptor) -> Result<(), McpError> {
        validate_tool(&tool).map_err(|e| McpError::InvalidTool(e.to_string()))?;
        self.tools.write().await.insert(tool.name.clone(), tool);
        Ok(())
    }

    pub async fn get(&self, name: &str) -> Option<ToolDescriptor> {
        self.tools.read().await.get(name).cloned()
    }

    pub async fn content_digest(&self, name: &str) -> Result<String, McpError> {
        let tool = self.get(name).await.ok_or(McpError::ToolNotFound)?;
        let encoded = serde_json::to_vec(&tool).map_err(|_| McpError::InvalidTool("serialization failed".into()))?;
        let digest = Sha256::digest(encoded);
        Ok(format!("sha256:{}", hex_encode(&digest)))
    }
}

fn hex_encode(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut out = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        out.push(HEX[(byte >> 4) as usize] as char);
        out.push(HEX[(byte & 0x0f) as usize] as char);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use bowser_core::ToolAnnotation;

    #[test]
    fn rpc_validation_requires_v2() {
        let req = JsonRpcRequest { jsonrpc: "1.0".into(), id: None, method: "ping".into(), params: Value::Null };
        assert!(matches!(req.validate(), Err(McpError::InvalidVersion)));
    }

    #[tokio::test]
    async fn registry_returns_stable_content_digest() {
        let registry = ToolRegistry::default();
        registry.register(ToolDescriptor {
            name: "ping".into(),
            description: "Health check".into(),
            annotation: ToolAnnotation { read_only_hint: true, destructive_hint: false },
        }).await.unwrap();
        let digest = registry.content_digest("ping").await.unwrap();
        assert!(digest.starts_with("sha256:"));
        assert_eq!(digest, registry.content_digest("ping").await.unwrap());
    }
}
