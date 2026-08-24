<!-- Copyright 2026 Adam Joseph Rivers, CEO, Synthicsoft Labs LLC. Licensed under Apache-2.0. -->

<div align="center">

# 🐢 BOWSERAI

### NESSY · AUTONOMOUS INTELLIGENCE FABRIC

**Persistent · Distributed · Self-configuring · GitHub-native · Frontier-capable**

**Created by Adam Joseph Rivers · CEO, Synthicsoft Labs LLC**

[![License: Apache-2.0](https://img.shields.io/badge/License-Apache--2.0-111827?style=for-the-badge&logo=apache)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-2024-111827?style=for-the-badge&logo=rust)](https://www.rust-lang.org/)
[![Repository](https://img.shields.io/badge/GitHub-SynthicsoftLabs%2FNessy-111827?style=for-the-badge&logo=github)](https://github.com/SynthicsoftLabs/Nessy)

**Nessy is the engineering foundation of BowserAI:** an autonomous intelligence fabric coordinating AGI capabilities, frontier model providers, runtimes, inference, execution, identity, durable state, MCP tools, research, learning, and distributed infrastructure through one coherent system.

### 💬 [**CHAT WITH NESSY**](https://html-preview.github.io/?url=https://github.com/SynthicsoftLabs/Nessy/blob/main/docs/chat/index.html)

**Open the live BowserAI interface directly from this repository.**

</div>

---

> **Attribution:** BowserAI / Nessy was created by **Adam Joseph Rivers, CEO of Synthicsoft Labs LLC**. Redistribution is governed by the Apache License, Version 2.0 and its applicable copyright, attribution, NOTICE, and trademark provisions. See [`NOTICE`](NOTICE) and [`LICENSE`](LICENSE).

---

## ◈ Live Intelligence Interface

The repository ships its browser-native Nessy interface at [`docs/chat/index.html`](docs/chat/index.html). The README chat button uses a repository HTML renderer so the interface is accessible without depending on an unpublished GitHub Pages site.

```text
GitHub Repository
       │
       ▼
   README.md
       │
       ▼
  CHAT WITH NESSY
       │
       ▼
Repository HTML Renderer
       │
       ▼
  BowserAI Chat UI
       │
       ▼
  Kairos / Chat API
       │
       ▼
 Nessy Intelligence Fabric
```

The chat surface is dependency-free: a single browser application with session continuity, capability declarations, responsive layout, keyboard-friendly composition, and endpoint configuration through `?endpoint=`. The application source is versioned directly in this repository.

## ◈ System Fabric

```text
                              ┌─────────────────────────┐
                              │        BOWSERAI         │
                              │   AUTONOMOUS INTELLIGENCE│
                              │          FABRIC          │
                              └────────────┬────────────┘
                                           │
                              ┌────────────▼────────────┐
                              │   AGI CAPABILITY GRAPH  │
                              │ reason · plan · remember│
                              │ learn · research · act  │
                              └────────────┬────────────┘
                                           │
              ┌────────────────────────────┼────────────────────────────┐
              ▼                            ▼                            ▼
        ┌────────────┐              ┌────────────┐              ┌────────────┐
        │   KAIROS   │              │   TURTLE   │              │ INFERENCE  │
        │  ALWAYS-ON │              │ORCHESTRATOR│              │ PROVIDERS  │
        └─────┬──────┘              └──────┬─────┘              └─────┬──────┘
              │                            │                            │
              └────────────────────────────┼────────────────────────────┘
                                           ▼
        ┌────────────┬────────────┬────────────┬────────────┬────────────┐
        │   KOOPA    │    MCP     │  STORAGE   │  IDENTITY  │   GITHUB   │
        │  EXECUTE   │   TOOLS    │ DURABLE CAS│  AGENTS    │  BACKEND   │
        └────────────┴────────────┴────────────┴────────────┴────────────┘
                                           │
                                           ▼
                               OBSERVE → EVALUATE → LEARN
                                           │
                                           └──────► DISCOVER
```

## 🧠 AGI Capability Fabric

Nessy now models AGI capabilities as executable, composable runtime contracts rather than a static model list. The capability graph spans:

| Domain | Fabric |
|:--|:--|
| **Reasoning** | structured reasoning, verification, reflection, hypotheses, theorem proving |
| **Planning** | decomposition, hierarchy, long-horizon plans, replanning, scheduling |
| **Memory** | working, episodic, semantic, procedural, persistent memory, retrieval, consolidation |
| **Learning** | online adaptation, preference learning, self-evaluation, skill acquisition, continual improvement |
| **Agents** | tools, delegation, parallel agents, councils, specialization, asynchronous tasks |
| **Computer use** | browser, GUI, terminal, filesystem, IDE, remote desktop |
| **Coding** | generation, repository understanding, refactoring, debugging, testing, benchmarking, worktrees |
| **Research** | search, fetch, synthesis, literature analysis, experiments, evidence aggregation |
| **Multimodal** | vision, OCR, audio, speech, video, cross-modal reasoning |
| **Generation** | text, code, image, video, audio, document generation/editing |
| **Scientific** | symbolic math, numerical computation, simulation, data analysis, discovery |
| **Security** | code audit, dependency analysis, threat modeling, secret detection, telemetry |
| **Infrastructure** | scheduling, peer discovery, failover, checkpointing, replication, workload migration |
| **Governance** | provenance, attribution, licensing metadata, audit trails, reproducible artifacts |

See [`docs/AGI_CAPABILITY_MATRIX.md`](docs/AGI_CAPABILITY_MATRIX.md) and `crates/capability/src/agi.rs`.

## 🌐 Frontier Model Fabric

Nessy normalizes frontier capabilities across providers into the same capability graph. Current research coverage includes:

**Fable · Glasswing · Qwen · Gemini · Grok · Seed · Seedance · Perplexity · Z.ai / GLM · Gemma**

The provider layer is extensible so additional models and systems become capability providers without redesigning the orchestration layer.

```text
FRONTIER RESEARCH
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
OBSERVE / EVALUATE
       ↓
MEMORY / SKILL REGISTRATION
       ↓
CONTINUOUS DISCOVERY
```

## ⚡ Autonomous Runtime

```text
DISCOVER → REGISTER → MATCH → COMPOSE → ROUTE → EXECUTE
    ▲                                           │
    │                                           ▼
    └──── RECOVER ← CHECKPOINT ← EVALUATE ← OBSERVE
```

GitHub is the public project/control substrate. Kairos provides always-on runtime integration. Turtle coordinates execution. Koopa supplies execution backends. MCP exposes tools. Storage preserves durable state. Identity establishes agent identity. Inference connects model providers.

## ◇ Core Fabric

| Layer | Role | Crate |
|:--|:--|:--|
| **Bowser Core** | Shared domain contracts | `bowser-core` |
| **Turtle** | Scheduling, state, recovery | `turtle` |
| **Koopa** | Execution backends | `koopa` |
| **Kairos** | Always-on runtime | `kairos` |
| **Capability** | Runtime + AGI capability graph | `capability` |
| **Inference** | Provider routing | `inference` |
| **Storage** | Durable content-addressed state | `storage` |
| **Identity** | Autonomous agent identity | `identity` |
| **MCP** | Tool protocol | `mcp` |
| **GitHub Backend** | GitHub-backed project/control state | `github-backend` |
| **Bowserd** | Persistent daemon | `bowserd` |

## 🚀 Build

```bash
cargo check --workspace
cargo test --workspace
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

Autonomous bootstrap is provided by `scripts/bootstrap.sh`.

## 🧭 Repository Map

```text
.
├── crates/
│   ├── bowser-core/      domain contracts
│   ├── turtle/           orchestration
│   ├── koopa/            execution
│   ├── kairos/           always-on runtime
│   ├── capability/       AGI + capability fabric
│   ├── inference/        provider routing
│   ├── storage/          durable CAS
│   ├── identity/         agent identity
│   ├── mcp/              MCP protocol
│   ├── github-backend/   GitHub backend
│   └── bowserd/          daemon
├── docs/
│   ├── chat/             repository-native chat UI
│   ├── AGI_CAPABILITY_MATRIX.md
│   └── ...
├── scripts/              autonomous tooling
├── .github/              repository automation
├── ARCHITECTURE.md       system topology
├── AUTOMATION.md         autonomous engineering contract
├── SECURITY.md           security architecture
├── NOTICE                creator attribution
└── LICENSE               Apache License 2.0
```

## 📚 Documentation

| Document | Purpose |
|:--|:--|
| [`ARCHITECTURE.md`](ARCHITECTURE.md) | Full system topology and runtime fabric |
| [`docs/chat/index.html`](docs/chat/index.html) | Browser chat interface |
| [`docs/AGI_CAPABILITY_MATRIX.md`](docs/AGI_CAPABILITY_MATRIX.md) | AGI domains and frontier-provider capability map |
| [`AUTOMATION.md`](AUTOMATION.md) | Autonomous engineering and repository automation |
| [`SECURITY.md`](SECURITY.md) | Security architecture |
| [`NOTICE`](NOTICE) | Creator attribution and legal notice |
| [`CONTRIBUTING.md`](CONTRIBUTING.md) | Engineering workflow |
| [`LICENSE`](LICENSE) | Apache License 2.0 |

<div align="center">

### 🐢 BOWSERAI / NESSY

**Created by Adam Joseph Rivers · CEO, Synthicsoft Labs LLC**

**AGI capability fabric · Frontier model fabric · Autonomous runtime**

[GitHub](https://github.com/SynthicsoftLabs/Nessy) · [Architecture](ARCHITECTURE.md) · [AGI Matrix](docs/AGI_CAPABILITY_MATRIX.md) · [Chat UI](docs/chat/index.html) · [Security](SECURITY.md)

</div>
