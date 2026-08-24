// Copyright 2026 Synthicsoft Labs LLC
// Licensed under the Apache License, Version 2.0.

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum Capability {
    Inference,
    ToolCalling,
    Streaming,
    StructuredOutput,
    Multimodal,
    CodeExecution,
    Wasi,
    Container,
    MicroVm,
    Network,
    PersistentState,
    ContentAddressedStorage,
    PeerMesh,
    LocalRuntime,
    RemoteRuntime,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeDescriptor {
    pub id: Uuid,
    pub name: String,
    pub capabilities: BTreeSet<Capability>,
    pub endpoint: Option<String>,
    pub priority: i32,
    pub healthy: bool,
}

impl RuntimeDescriptor {
    pub fn supports_all<I>(&self, required: I) -> bool
    where
        I: IntoIterator<Item = Capability>,
    {
        required.into_iter().all(|cap| self.capabilities.contains(&cap))
    }

    pub fn fingerprint(&self) -> String {
        let encoded = serde_json::to_vec(self).expect("runtime descriptor serialization");
        let digest = Sha256::digest(encoded);
        digest.iter().map(|b| format!("{b:02x}")).collect()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionPlan {
    pub task_id: Uuid,
    pub candidates: Vec<RuntimeDescriptor>,
    pub required: BTreeSet<Capability>,
}

#[derive(Debug, thiserror::Error)]
pub enum RoutingError {
    #[error("no runtime satisfies the requested capabilities")]
    NoCapableRuntime,
}

impl ExecutionPlan {
    pub fn select(&self) -> Result<RuntimeDescriptor, RoutingError> {
        self.candidates
            .iter()
            .filter(|runtime| runtime.healthy && runtime.supports_all(self.required.iter().cloned()))
            .max_by_key(|runtime| runtime.priority)
            .cloned()
            .ok_or(RoutingError::NoCapableRuntime)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn runtime(name: &str, priority: i32, capabilities: &[Capability]) -> RuntimeDescriptor {
        RuntimeDescriptor {
            id: Uuid::new_v4(),
            name: name.into(),
            capabilities: capabilities.iter().cloned().collect(),
            endpoint: None,
            priority,
            healthy: true,
        }
    }

    #[test]
    fn selects_highest_priority_capable_runtime() {
        let plan = ExecutionPlan {
            task_id: Uuid::new_v4(),
            required: [Capability::Inference].into_iter().collect(),
            candidates: vec![
                runtime("local", 10, &[Capability::Inference]),
                runtime("kairos", 100, &[Capability::Inference, Capability::PeerMesh]),
            ],
        };
        assert_eq!(plan.select().unwrap().name, "kairos");
    }

    #[test]
    fn unhealthy_runtime_is_not_selected() {
        let mut candidate = runtime("offline", 1000, &[Capability::Inference]);
        candidate.healthy = false;
        let plan = ExecutionPlan {
            task_id: Uuid::new_v4(),
            required: [Capability::Inference].into_iter().collect(),
            candidates: vec![candidate],
        };
        assert!(matches!(plan.select(), Err(RoutingError::NoCapableRuntime)));
    }
}
