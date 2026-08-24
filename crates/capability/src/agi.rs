// Copyright 2026 Synthicsoft Labs LLC
// Licensed under the Apache License, Version 2.0.

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum AgiCapability {
    Perception,
    WorldModel,
    WorkingMemory,
    EpisodicMemory,
    SemanticMemory,
    ProceduralMemory,
    Retrieval,
    MemoryConsolidation,
    Reasoning,
    Verification,
    Reflection,
    HypothesisGeneration,
    Planning,
    Replanning,
    Scheduling,
    ToolUse,
    Delegation,
    ParallelAgents,
    AgentCouncil,
    ComputerUse,
    BrowserUse,
    TerminalUse,
    IdeUse,
    Coding,
    RepositoryEngineering,
    Debugging,
    Testing,
    WebResearch,
    EvidenceSynthesis,
    ExperimentPlanning,
    SymbolicMath,
    Simulation,
    DataAnalysis,
    Vision,
    Audio,
    Speech,
    Video,
    ImageGeneration,
    VideoGeneration,
    AudioGeneration,
    RealtimeInteraction,
    FunctionCalling,
    StructuredOutput,
    Mcp,
    Connectors,
    OnlineLearning,
    SkillAcquisition,
    SelfEvaluation,
    ContinuousImprovement,
    Provenance,
    Audit,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityNode {
    pub id: String,
    pub capability: AgiCapability,
    pub provider: String,
    pub version: String,
    pub inputs: Vec<String>,
    pub outputs: Vec<String>,
    pub prerequisites: HashSet<AgiCapability>,
    pub quality: f64,
    pub latency_ms: u64,
    pub healthy: bool,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CapabilityGraph {
    pub nodes: HashMap<String, CapabilityNode>,
}

impl CapabilityGraph {
    pub fn register(&mut self, node: CapabilityNode) {
        self.nodes.insert(node.id.clone(), node);
    }

    pub fn providers_for(&self, capability: &AgiCapability) -> Vec<&CapabilityNode> {
        self.nodes
            .values()
            .filter(|node| node.healthy && node.capability == *capability)
            .collect()
    }

    pub fn compose(&self, requested: &[AgiCapability]) -> Vec<&CapabilityNode> {
        let mut selected = Vec::new();
        for capability in requested {
            if let Some(node) = self
                .providers_for(capability)
                .into_iter()
                .max_by(|a, b| a.quality.total_cmp(&b.quality))
            {
                selected.push(node);
            }
        }
        selected
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn graph_selects_best_healthy_provider() {
        let mut graph = CapabilityGraph::default();
        for (id, provider, quality) in [("local", "local", 0.70), ("frontier", "frontier", 0.99)] {
            graph.register(CapabilityNode {
                id: id.into(),
                capability: AgiCapability::Reasoning,
                provider: provider.into(),
                version: "1".into(),
                inputs: vec!["prompt".into()],
                outputs: vec!["answer".into()],
                prerequisites: HashSet::new(),
                quality,
                latency_ms: 10,
                healthy: true,
            });
        }

        let selected = graph.compose(&[AgiCapability::Reasoning]);
        assert_eq!(selected.len(), 1);
        assert_eq!(selected[0].provider, "frontier");
    }
}
