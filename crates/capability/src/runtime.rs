// Copyright 2026 Adam Joseph Rivers, CEO, Synthicsoft Labs LLC
// Licensed under the Apache License, Version 2.0.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum MemoryTier {
    Working,
    Episodic,
    Semantic,
    Procedural,
    Durable,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryRecord {
    pub id: Uuid,
    pub tier: MemoryTier,
    pub key: String,
    pub value: String,
    pub priority: u8,
    pub provenance: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct MemoryHierarchy {
    pub records: BTreeMap<String, MemoryRecord>,
}

impl MemoryHierarchy {
    pub fn put(&mut self, record: MemoryRecord) {
        self.records.insert(record.key.clone(), record);
    }

    pub fn get(&self, key: &str) -> Option<&MemoryRecord> {
        self.records.get(key)
    }

    pub fn by_tier(&self, tier: MemoryTier) -> Vec<&MemoryRecord> {
        self.records
            .values()
            .filter(|record| record.tier == tier)
            .collect()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillStep {
    pub capability: String,
    pub input: serde_json::Value,
    pub output_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillChain {
    pub id: String,
    pub version: String,
    pub steps: Vec<SkillStep>,
}

impl SkillChain {
    pub fn validate(&self) -> Result<(), &'static str> {
        if self.id.trim().is_empty() || self.version.trim().is_empty() {
            return Err("skill identity is required");
        }
        if self.steps.is_empty() {
            return Err("skill chain requires at least one step");
        }
        if self
            .steps
            .iter()
            .any(|step| step.capability.trim().is_empty())
        {
            return Err("every skill step requires a capability");
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParallelCall {
    pub id: Uuid,
    pub capability: String,
    pub input: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParallelBatch {
    pub calls: Vec<ParallelCall>,
    pub concurrency: usize,
}

impl ParallelBatch {
    pub fn validate(&self) -> Result<(), &'static str> {
        if self.calls.is_empty() {
            return Err("parallel batch requires at least one call");
        }
        if self.concurrency == 0 {
            return Err("parallel concurrency must be positive");
        }
        if self
            .calls
            .iter()
            .any(|call| call.capability.trim().is_empty())
        {
            return Err("every parallel call requires a capability");
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderRoute {
    pub provider: String,
    pub capability: String,
    pub priority: i32,
    pub healthy: bool,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ProviderRegistry {
    pub routes: Vec<ProviderRoute>,
}

impl ProviderRegistry {
    pub fn best(&self, capability: &str) -> Option<&ProviderRoute> {
        self.routes
            .iter()
            .filter(|route| route.healthy && route.capability == capability)
            .max_by_key(|route| route.priority)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn memory_hierarchy_indexes_by_tier() {
        let mut memory = MemoryHierarchy::default();
        memory.put(MemoryRecord {
            id: Uuid::new_v4(),
            tier: MemoryTier::Semantic,
            key: "architecture".into(),
            value: "capability graph".into(),
            priority: 5,
            provenance: Some("nessy".into()),
        });
        assert_eq!(memory.by_tier(MemoryTier::Semantic).len(), 1);
    }

    #[test]
    fn skill_chain_and_parallel_batch_validate() {
        let skill = SkillChain {
            id: "research".into(),
            version: "1".into(),
            steps: vec![SkillStep {
                capability: "web_research".into(),
                input: serde_json::json!({}),
                output_key: "sources".into(),
            }],
        };
        assert!(skill.validate().is_ok());

        let batch = ParallelBatch {
            calls: vec![ParallelCall {
                id: Uuid::new_v4(),
                capability: "reasoning".into(),
                input: serde_json::json!({}),
            }],
            concurrency: 4,
        };
        assert!(batch.validate().is_ok());
    }

    #[test]
    fn provider_registry_selects_best_healthy_route() {
        let registry = ProviderRegistry {
            routes: vec![
                ProviderRoute {
                    provider: "a".into(),
                    capability: "reasoning".into(),
                    priority: 10,
                    healthy: true,
                },
                ProviderRoute {
                    provider: "b".into(),
                    capability: "reasoning".into(),
                    priority: 20,
                    healthy: true,
                },
                ProviderRoute {
                    provider: "c".into(),
                    capability: "reasoning".into(),
                    priority: 100,
                    healthy: false,
                },
            ],
        };
        assert_eq!(registry.best("reasoning").unwrap().provider, "b");
    }
}
