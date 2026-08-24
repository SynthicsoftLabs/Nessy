// Copyright 2026 Synthicsoft Labs LLC
// Licensed under the Apache License, Version 2.0.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SessionId(Uuid);

impl SessionId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for SessionId {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TaskId(Uuid);

impl TaskId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for TaskId {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TaskState {
    Queued,
    Running,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: TaskId,
    pub session_id: SessionId,
    pub state: TaskState,
    pub input: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolAnnotation {
    #[serde(default)]
    pub read_only_hint: bool,
    #[serde(default)]
    pub destructive_hint: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolDescriptor {
    pub name: String,
    pub description: String,
    pub annotation: ToolAnnotation,
}

#[derive(Debug, thiserror::Error)]
pub enum CoreError {
    #[error("invalid identifier: {0}")]
    InvalidIdentifier(String),
    #[error("invalid tool descriptor: {0}")]
    InvalidTool(String),
}

pub fn validate_tool(tool: &ToolDescriptor) -> Result<(), CoreError> {
    if tool.name.trim().is_empty() {
        return Err(CoreError::InvalidTool("tool name is empty".into()));
    }
    if tool.name.len() > 128 {
        return Err(CoreError::InvalidTool("tool name exceeds 128 bytes".into()));
    }
    if tool.description.len() > 16_384 {
        return Err(CoreError::InvalidTool(
            "tool description exceeds 16 KiB".into(),
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generated_ids_are_unique() {
        assert_ne!(SessionId::new(), SessionId::new());
        assert_ne!(TaskId::new(), TaskId::new());
    }

    #[test]
    fn tool_validation_rejects_empty_names() {
        let tool = ToolDescriptor {
            name: " ".into(),
            description: String::new(),
            annotation: ToolAnnotation {
                read_only_hint: true,
                destructive_hint: false,
            },
        };
        assert!(validate_tool(&tool).is_err());
    }
}
