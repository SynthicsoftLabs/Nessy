// Copyright 2026 Synthicsoft Labs LLC
// Licensed under the Apache License, Version 2.0.

use serde::{Deserialize, Serialize};

/// Provider-neutral capabilities exposed by frontier and local runtimes.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum FrontierCapability {
    Reasoning,
    Thinking,
    LongContext,
    LongHorizon,
    Planning,
    PersistentMemory,
    SelfImprovingMemory,
    WebSearch,
    WebFetch,
    CodeExecution,
    AgenticCoding,
    FunctionCalling,
    StructuredOutput,
    Mcp,
    GuiComputerUse,
    Multimodal,
    Vision,
    Audio,
    RealtimeAudio,
    VideoUnderstanding,
    ImageGeneration,
    ImageEditing,
    VideoGeneration,
    VideoEditing,
    VideoExtension,
    AudioVideoGeneration,
    Connectors,
    Scheduling,
    AsyncExecution,
    SandboxedExecution,
    ModelCouncil,
    ScientificResearch,
    SecurityAnalysis,
    SpeculativeDecoding,
    MixtureOfExperts,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FrontierProfile {
    pub provider: String,
    pub model: String,
    pub capabilities: Vec<FrontierCapability>,
    pub context_tokens: Option<u64>,
    pub source: String,
}

impl FrontierProfile {
    pub fn supports(&self, capability: FrontierCapability) -> bool {
        self.capabilities.contains(&capability)
    }

    pub fn coverage(&self, requested: &[FrontierCapability]) -> usize {
        requested.iter().filter(|cap| self.supports(**cap)).count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn profile_scores_requested_capabilities() {
        let profile = FrontierProfile {
            provider: "example".into(),
            model: "frontier".into(),
            capabilities: vec![
                FrontierCapability::Reasoning,
                FrontierCapability::LongContext,
                FrontierCapability::Mcp,
            ],
            context_tokens: Some(1_000_000),
            source: "public documentation".into(),
        };

        assert!(profile.supports(FrontierCapability::Reasoning));
        assert_eq!(profile.coverage(&[
            FrontierCapability::Reasoning,
            FrontierCapability::Mcp,
            FrontierCapability::VideoGeneration,
        ]), 2);
    }
}
