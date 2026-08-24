<!-- Copyright 2026 Adam Joseph Rivers, CEO, Synthicsoft Labs LLC. Licensed under Apache-2.0. -->

# 🧠 Nessy AGI & Frontier Capability Matrix

## ◈ Purpose

Nessy converts frontier AI and AGI research into executable, composable platform capabilities. The capability graph is the common language between models, runtimes, tools, memory, research systems, execution environments, and autonomous workflows.

## Capability Domains

| Domain | Capability surface |
|:--|:--|
| **Reasoning** | structured reasoning, verification, reflection, hypothesis generation, theorem proving |
| **Planning** | decomposition, hierarchical planning, long-horizon planning, replanning, scheduling, dependency graphs |
| **Memory** | working, episodic, semantic, procedural, persistent memory, retrieval, consolidation |
| **Learning** | online adaptation, preference learning, self-evaluation, skill acquisition, curriculum generation, continual improvement |
| **Agents** | tool use, delegation, parallel agents, teams, councils, role specialization, asynchronous tasks |
| **Computer Use** | browser, GUI, terminal, filesystem, IDE, remote desktop |
| **Coding** | generation, repository understanding, refactoring, debugging, testing, benchmarking, worktree orchestration |
| **Research** | search, fetch, synthesis, literature analysis, experiment planning, citation/evidence graphs |
| **Multimodal** | vision, OCR, image, audio, speech, video, cross-modal reasoning |
| **Generation** | text, code, image, video, audio, document generation and editing |
| **Interaction** | realtime voice, streaming, structured output, function calling, MCP, connectors, notifications |
| **Scientific** | symbolic math, numerical computation, simulation, data analysis, model discovery, experiments |
| **Security** | code audit, dependency analysis, threat modeling, secret detection, policy evaluation, telemetry |
| **Infrastructure** | distributed scheduling, peer discovery, failover, checkpointing, replication, content addressing, workload migration |
| **Governance** | provenance, attribution, licensing metadata, audit trails, reproducible builds, artifact signing |

## 🌐 Frontier Coverage

Nessy maintains normalized capability coverage for the named frontier systems and expands the registry as additional systems are researched:

| System | Capability areas represented |
|:--|:--|
| **Fable** | long-horizon reasoning, research, coding, agent workflows |
| **Glasswing** | multimodal and interactive agent workflows |
| **Qwen** | reasoning, multimodality, coding, tools, structured interaction, agents |
| **Gemini** | multimodal reasoning, long context, search, code execution, agentic coding, structured interaction |
| **Grok** | reasoning, search, coding, multimodal interaction, tool workflows |
| **Seed** | long-horizon agents, research, coding, computer interaction |
| **Seedance** | multimodal video generation, editing, transformation |
| **Perplexity** | web research, persistent context, connectors, computer agents, asynchronous workflows |
| **Z.ai / GLM** | reasoning, coding, tools, multimodality, agent workflows |
| **Gemma** | reasoning, multimodality, long context, function calling, deployable inference |

## 🔭 Continuous Frontier Expansion

The registry is intentionally broader than the initial named set. Research additions are normalized into the same contract vocabulary across:

**OpenAI · Anthropic · Meta · Mistral · DeepSeek · xAI · Google · Alibaba · ByteDance · Moonshot AI · Z.ai · Perplexity · AI21 · Cohere · NVIDIA · Microsoft · Amazon · IBM · Hugging Face · Stability AI · Runway · Luma · Black Forest Labs · ElevenLabs · Open-source and research models**

The provider registry is designed to add additional model families, agent frameworks, modalities, tools, memory systems, inference techniques, and research capabilities without changing the core orchestration model.

## ⚙️ Capability Contract

Each registered capability exposes:

```text
identity
version
provider
inputs
outputs
requirements
quality
latency
health
authorization
telemetry
recovery
composition rules
evaluation suite
provenance
```

## 🧩 Composition

```text
                 TASK / GOAL
                     │
                     ▼
              REQUIREMENTS GRAPH
                     │
                     ▼
             CAPABILITY MATCHING
                     │
          ┌──────────┼──────────┐
          ▼          ▼          ▼
       PROVIDER    PROVIDER    PROVIDER
          │          │          │
          └──────────┼──────────┘
                     ▼
              COMPOSITE PLAN
                     │
                     ▼
            PARALLEL / SERIAL RUN
                     │
                     ▼
             OBSERVE / EVALUATE
                     │
                     ▼
             MEMORY / SKILL STORE
                     │
                     └──────────► NEXT TASK
```

## 🧠 AGI Runtime Graph

```text
                    ┌───────────────┐
                    │ WORLD / INPUT │
                    └───────┬───────┘
                            ▼
                    ┌───────────────┐
                    │ PERCEPTION    │
                    └───────┬───────┘
                            ▼
              ┌─────────────┴─────────────┐
              ▼                           ▼
        MEMORY GRAPH                 KNOWLEDGE GRAPH
              │                           │
              └─────────────┬─────────────┘
                            ▼
                    ┌───────────────┐
                    │ REASON / PLAN │
                    └───────┬───────┘
                            ▼
                    ┌───────────────┐
                    │ ACT / DELEGATE│
                    └───────┬───────┘
                            ▼
                    ┌───────────────┐
                    │ OBSERVE RESULT│
                    └───────┬───────┘
                            ▼
                    ┌───────────────┐
                    │ EVALUATE      │
                    └───────┬───────┘
                            ▼
                    ┌───────────────┐
                    │ LEARN / STORE │
                    └───────┬───────┘
                            └──────────────► CONTINUOUS DISCOVERY
```

## 🔬 Research-to-Runtime Pipeline

```text
RESEARCH
   ↓
EXTRACT FEATURE / FUNCTION
   ↓
DEFINE NORMALIZED CAPABILITY
   ↓
IMPLEMENT ADAPTER
   ↓
ADD EVALUATION
   ↓
ADD TELEMETRY
   ↓
ADD PERSISTENCE / RECOVERY
   ↓
REGISTER IN CAPABILITY GRAPH
   ↓
COMPOSE WITH EXISTING CAPABILITIES
   ↓
DEPLOY THROUGH AUTONOMOUS RUNTIME
```

## ✦ Source of Truth

The executable capability graph lives in `crates/capability/src/agi.rs`. This document describes its capability vocabulary and frontier coverage. New research should update both the registry and this matrix so documentation and implementation remain synchronized.
