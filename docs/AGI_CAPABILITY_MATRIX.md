<!-- Copyright 2026 Synthicsoft Labs LLC; Licensed under Apache-2.0. -->

# 🧠 Nessy AGI Capability Matrix

## Purpose

This matrix turns frontier-model and AGI research into executable platform capabilities. Every capability has a normalized contract, provider adapters, evaluation hooks, and a path into the autonomous runtime.

## Capability Domains

| Domain | Capabilities |
|---|---|
| Reasoning | chain-of-thought execution, structured reasoning, verification, reflection, hypothesis generation, theorem proving |
| Planning | decomposition, hierarchical planning, long-horizon planning, replanning, scheduling, dependency graphs |
| Memory | working memory, episodic memory, semantic memory, procedural memory, persistent memory, retrieval, consolidation |
| Learning | online adaptation, preference learning, self-evaluation, skill acquisition, curriculum generation, continual improvement |
| Agents | tool use, delegation, parallel agents, agent teams, councils, role specialization, asynchronous tasks |
| Computer Use | browser control, GUI interaction, terminal control, filesystem operations, IDE workflows, remote desktop |
| Coding | code generation, repository understanding, refactoring, debugging, testing, benchmarking, worktree orchestration |
| Research | web search, web fetch, source synthesis, literature analysis, experiment planning, citation graphs, evidence aggregation |
| Multimodal | vision, OCR, image understanding, audio understanding, speech, video understanding, cross-modal reasoning |
| Generation | text, code, image generation/editing, video generation/editing, audio generation/editing, document generation |
| Interaction | realtime voice, streaming, structured output, function calling, MCP, connectors, notifications |
| Scientific | symbolic math, numerical computation, simulation, data analysis, model discovery, experiment execution |
| Security | code audit, dependency analysis, threat modeling, secret detection, policy evaluation, runtime telemetry |
| Infrastructure | distributed scheduling, peer discovery, failover, checkpointing, replication, content addressing, workload migration |
| Governance | provenance, attribution, licensing metadata, audit trails, reproducible builds, artifact signing |

## Frontier Provider Coverage

Nessy maps provider features into the same capability vocabulary so the router can compose capabilities rather than hard-code a single model.

- **Fable** — long-horizon reasoning, research, coding, agent workflows.
- **Glasswing** — multimodal and interactive agent workflows.
- **Qwen** — reasoning, multimodality, coding, tool use, structured interaction, agent workflows.
- **Gemini** — multimodal reasoning, long context, search, code execution, agentic coding, structured interaction.
- **Grok** — reasoning, search, coding, multimodal interaction, tool-oriented workflows.
- **Seed** — long-horizon agent execution, research, coding, computer interaction.
- **Seedance** — multimodal video generation, editing and transformation.
- **Perplexity** — web research, persistent context, connectors, computer-oriented agents, asynchronous workflows.
- **Z.ai / GLM** — reasoning, coding, tool use, multimodality and agent workflows.
- **Gemma** — reasoning, multimodality, long context, function calling and deployable local inference.

## Runtime Composition

```text
RESEARCH
   ↓
CAPABILITY EXTRACTION
   ↓
NORMALIZED CONTRACT
   ↓
PROVIDER REGISTRY
   ↓
CAPABILITY GRAPH
   ↓
TASK DECOMPOSITION
   ↓
PARALLEL EXECUTION
   ↓
OBSERVATION / EVALUATION
   ↓
MEMORY CONSOLIDATION
   ↓
SKILL REGISTRATION
   ↓
CONTINUOUS DISCOVERY
```

## Capability Contract

Every capability implementation should expose:

```text
identity
version
provider
inputs
outputs
requirements
cost model
latency model
health
authorization
telemetry
recovery
composition rules
evaluation suite
provenance
```

## AGI Runtime Graph

```text
                    ┌───────────────┐
                    │ WORLD / INPUT │
                    └───────┬───────┘
                            ↓
                    ┌───────────────┐
                    │ PERCEPTION    │
                    └───────┬───────┘
                            ↓
              ┌─────────────┴─────────────┐
              ↓                           ↓
         MEMORY GRAPH                 KNOWLEDGE
              │                           │
              └─────────────┬─────────────┘
                            ↓
                    ┌───────────────┐
                    │ REASON / PLAN │
                    └───────┬───────┘
                            ↓
                    ┌───────────────┐
                    │ ACT / DELEGATE│
                    └───────┬───────┘
                            ↓
                    ┌───────────────┐
                    │ OBSERVE RESULT│
                    └───────┬───────┘
                            ↓
                    ┌───────────────┐
                    │ EVALUATE      │
                    └───────┬───────┘
                            ↓
                    ┌───────────────┐
                    │ LEARN / STORE │
                    └───────┬───────┘
                            └──────────────► NEXT CYCLE
```

## Implementation Rule

A newly discovered frontier capability becomes a first-class Nessy capability when its contract, adapter, evaluation, telemetry, persistence behavior, and composition semantics are implemented. The capability registry is therefore an expanding runtime graph rather than a static model list.
