<!-- Copyright 2026 Adam Joseph Rivers, CEO, Synthicsoft Labs LLC. Licensed under Apache-2.0. -->

# 🐢 BowserAI Architecture

> **NESSY · Autonomous Intelligence Fabric**
>
> One orchestration plane. Many runtimes. An expanding AGI capability graph. Durable state. Continuous discovery and recovery.

---

## ◈ Fabric Overview

BowserAI is an autonomous distributed intelligence fabric. The system is organized around normalized capabilities rather than a single model, runtime, or execution location. AGI capabilities, frontier providers, always-on runtimes, peer capacity, GitHub-backed infrastructure, inference providers, tools, storage, and execution environments participate through common contracts.

```text
                                      ┌────────────────────────────┐
                                      │          BOWSERAI          │
                                      │    AUTONOMOUS INTELLIGENCE │
                                      │           FABRIC           │
                                      └─────────────┬──────────────┘
                                                    │
                                      ┌─────────────▼──────────────┐
                                      │    AGI CAPABILITY GRAPH    │
                                      │ reason · plan · remember   │
                                      │ learn · research · create  │
                                      │ perceive · act · evaluate  │
                                      └─────────────┬──────────────┘
                                                    │
                 ┌──────────────────────────────────┼──────────────────────────────────┐
                 ▼                                  ▼                                  ▼
        ┌────────────────┐                 ┌────────────────┐                 ┌────────────────┐
        │     KAIROS     │                 │     TURTLE     │                 │    INFERENCE   │
        │    ALWAYS-ON   │                 │ ORCHESTRATOR  │                 │ PROVIDER FABRIC│
        └───────┬────────┘                 └───────┬────────┘                 └───────┬────────┘
                │                                  │                                  │
                └──────────────────────────────────┼──────────────────────────────────┘
                                                   ▼
        ┌────────────┬────────────┬────────────┬────────────┬────────────┬─────────────┐
        │   KOOPA    │    MCP     │  STORAGE   │  IDENTITY  │   GITHUB   │   PEER/P2P  │
        │  EXECUTE   │   TOOLS    │ DURABLE CAS│   AGENTS   │  BACKEND   │   RUNTIMES   │
        └────────────┴────────────┴────────────┴────────────┴────────────┴─────────────┘
                                                   │
                                                   ▼
                                      OBSERVE → EVALUATE → LEARN
                                                   │
                                                   └──────────► DISCOVER
```

## 🧠 AGI Capability Graph

The capability subsystem models AGI functionality as composable runtime nodes. Current domains include reasoning, verification, reflection, hypothesis generation, theorem proving, hierarchical and long-horizon planning, memory systems, retrieval and consolidation, learning, skill acquisition, agent delegation, councils, computer use, coding, research, multimodality, generation, scientific computation, security, infrastructure, provenance, and audit.

Each capability node carries identity, provider, version, inputs, outputs, prerequisites, quality, latency, and health information. The graph can register multiple providers for the same capability and compose the healthiest/highest-quality available provider for a requested capability.

See [`docs/AGI_CAPABILITY_MATRIX.md`](docs/AGI_CAPABILITY_MATRIX.md) and `crates/capability/src/agi.rs`.

## 🌐 Frontier Provider Fabric

Frontier research is normalized into the capability vocabulary rather than represented as a static model list. Current project coverage includes:

**Fable · Glasswing · Qwen · Gemini · Grok · Seed · Seedance · Perplexity · Z.ai / GLM · Gemma**

The provider fabric is extensible: new models and systems can register capabilities against the same contracts.

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
PARALLEL COMPOSITION
   ↓
EXECUTION
   ↓
OBSERVATION / EVALUATION
   ↓
MEMORY / SKILL REGISTRATION
   ↓
CONTINUOUS DISCOVERY
```

## ⚡ Autonomous Selection Loop

```text
DISCOVER → REGISTER → MATCH → COMPOSE → ROUTE → EXECUTE
    ▲                                           │
    │                                           ▼
    └──── RECOVER ← CHECKPOINT ← EVALUATE ← OBSERVE
```

There is no mandatory human approval transition in the normal execution loop. Identity, authority, execution policy, state, and telemetry remain machine-readable runtime semantics.

## 🛰️ Kairos

`kairos` is a first-class always-on runtime integration. Its endpoint is configurable through `KAIROS_URL`, with the project default targeting `https://the-real-kairos.com`. The adapter keeps transport-specific behavior behind the runtime contract.

## 🐢 Turtle

Turtle owns task identity, queueing, claiming, lifecycle transitions, persistence, recovery, scheduling, and distributed synchronization. It composes capability providers without coupling task semantics to any one model or runtime.

## 🛡️ Koopa

Koopa supplies execution backends including WASI, containers, microVMs, remote runners, and policy-controlled native execution. Execution is selected through capability contracts.

## 🧰 MCP

MCP supplies tool discovery, registration, validation, dispatch, and transport integration. MCP capabilities participate in the same AGI/tool composition graph.

## 💾 State

Durable task identity, checkpoints, content-addressed artifacts, replicated metadata, and recoverable execution plans keep state independent of a single runtime. Storage can combine GitHub-backed project/control state, persistent databases, and content-addressed persistence.

## 🐙 GitHub Backend

`github-backend` makes GitHub a first-class project/control substrate. Repository state, project artifacts, automation, and public distribution are integrated into the broader runtime architecture.

## 🔗 Dependency Direction

```text
                   frontier providers / runtimes
                    │        │         │
              ┌─────┴────────┴─────────┴─────┐
              ▼                              ▼
       provider adapters               GitHub backend
              │                              │
              └──────────────┬───────────────┘
                             ▼
                    capability contracts
                             │
                             ▼
                      AGI capability graph
                             │
                             ▼
                           Turtle
                             │
                             ▼
                        bowser-core
```

Shared domain contracts remain provider-neutral. Provider-specific behavior terminates at adapters, while capability semantics remain stable.

## ✦ Platform Surface

The architecture encompasses AGI capability composition, frontier-model integration, identity, memory, learning, inference, Kairos, distributed synchronization, sandboxed execution, MCP, GitHub-backed control state, durable storage, observability, clients, deployment, reproducible distribution, and automated recovery as one integrated platform.
