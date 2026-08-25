<!-- Copyright 2026 Adam Joseph Rivers, CEO, Synthicsoft Labs LLC. Licensed under Apache-2.0. -->

<div align="center">

# 🐢 BOWSERAI

### NESSY · AUTONOMOUS INTELLIGENCE FABRIC

**Persistent · Distributed · Self-configuring · GitHub-native · Frontier-capable**

**Created by Adam Joseph Rivers · CEO, Synthicsoft Labs LLC**

[![License: Apache-2.0](https://img.shields.io/badge/License-Apache--2.0-111827?style=for-the-badge&logo=apache)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-2024-111827?style=for-the-badge&logo=rust)](https://www.rust-lang.org/)
[![Repository](https://img.shields.io/badge/GitHub-SynthicsoftLabs%2FNessy-111827?style=for-the-badge&logo=github)](https://github.com/SynthicsoftLabs/Nessy)

**Nessy is the engineering foundation of BowserAI:** an autonomous intelligence fabric coordinating capability composition, model providers, runtimes, inference, execution, identity, durable state, MCP tools, research, learning, observability, and repository-native autonomous engineering.

### 💬 [**CHAT WITH NESSY**](https://html-preview.github.io/?url=https://github.com/SynthicsoftLabs/Nessy/blob/main/docs/chat/index.html)

**Open the repository-native BowserAI interface directly from this repository.**

</div>

---

> **Attribution:** BowserAI / Nessy was created by **Adam Joseph Rivers, CEO of Synthicsoft Labs LLC**. Redistribution is governed by the Apache License, Version 2.0 and its applicable copyright, attribution, NOTICE, and trademark provisions. See [`NOTICE`](NOTICE) and [`LICENSE`](LICENSE).

## ◈ What Nessy Is

Nessy is organized around an executable capability fabric rather than a single model. A task can be decomposed across reasoning, planning, memory, research, coding, multimodal perception, tools, execution, and evaluation capabilities, then routed across compatible model providers and runtimes.

GitHub is the public project/control substrate. Kairos provides always-on runtime integration. Turtle coordinates work. Koopa supplies execution backends. MCP exposes tools. Storage preserves durable state. Identity establishes agent identity. Inference normalizes provider access. The autonomous control plane owns repository engineering.

## 💬 Repository Chat

The repository ships a browser-native chat interface at [`docs/chat/index.html`](docs/chat/index.html) with its executable controller at [`docs/chat/app.js`](docs/chat/app.js).

The interface includes:

- persistent conversation storage until explicitly cleared;
- conversation search, pinning, import/export, copy/share, regeneration, and new-chat controls;
- modes for automatic, research, creative, code, and fast workflows;
- runtime/provider selection and automatic routing;
- browser inference fallback with multiple browser-compatible models;
- runtime diagnostics and response-source reporting;
- long-running session history with context selection rather than destructive history deletion;
- keyboard-first composition and responsive desktop/mobile presentation.

Runtime routing is defined in [`docs/chat/runtime.json`](docs/chat/runtime.json). Free/open model and provider discovery is defined in [`docs/models/free-models.json`](docs/models/free-models.json).

```text
README
  ↓
CHAT WITH NESSY
  ↓
Repository HTML renderer
  ↓
BowserAI Chat UI
  ↓
app.js controller
  ↓
Provider/runtime routing
  ├─ Kairos
  ├─ cloud/API providers
  ├─ routers
  └─ browser inference
  ↓
Persistent conversation state
```

## 🧠 AGI Capability Fabric

Nessy models AGI functionality as composable runtime contracts.

| Domain | Capability surface |
|:--|:--|
| **Reasoning** | structured reasoning, verification, reflection, hypotheses, theorem proving |
| **Planning** | decomposition, hierarchy, long-horizon plans, replanning, scheduling |
| **Memory** | working, episodic, semantic, procedural, persistent memory, retrieval, consolidation |
| **Learning** | adaptation, preference learning, self-evaluation, skill acquisition, continual improvement |
| **Agents** | tools, delegation, parallel agents, councils, specialization, asynchronous tasks |
| **Computer use** | browser, GUI, terminal, filesystem, IDE, remote desktop |
| **Coding** | generation, repository understanding, refactoring, debugging, testing, benchmarking |
| **Research** | search, fetch, synthesis, literature analysis, experiments, evidence aggregation |
| **Multimodal** | vision, OCR, audio, speech, video, cross-modal reasoning |
| **Generation** | text, code, image, video, audio, document generation/editing |
| **Scientific** | symbolic math, numerical computation, simulation, data analysis, experiments |
| **Security** | code audit, dependency analysis, threat modeling, secret detection, telemetry |
| **Infrastructure** | scheduling, peer discovery, failover, checkpointing, replication, migration |
| **Governance** | provenance, attribution, licensing metadata, audit trails, reproducibility |

See [`docs/AGI_CAPABILITY_MATRIX.md`](docs/AGI_CAPABILITY_MATRIX.md) for the normalized capability vocabulary and frontier-provider coverage.

## 🌐 Frontier + Free Model Fabric

Nessy separates capability normalization from individual models and providers. Current frontier research coverage includes **Fable, Glasswing, Qwen, Gemini, Grok, Seed, Seedance, Perplexity, Z.ai / GLM, and Gemma**, with expansion to additional systems through the same capability contract.

The free/open registry incorporates model and provider discovery from the public [`12britz/awesome-free-models`](https://github.com/12britz/awesome-free-models) project, including open-weight models, free API providers, routers, local runtimes, multimodal systems, coding models, embeddings, RAG, agent frameworks, MCP tooling, evaluation/observability, and hosting resources. The upstream catalog states that its links were re-verified August 22, 2026 and includes explicit notes for services with changing or restricted access. citeturn219762view4

Nessy records these as machine-readable discovery metadata. A catalog entry does not by itself mean that a provider is browser-callable, permanently free, or credential-free.

## ⚡ Autonomous Runtime Fabric

```text
DISCOVER → REGISTER → MATCH → COMPOSE → ROUTE → EXECUTE
    ▲                                           │
    │                                           ▼
    └──── RECOVER ← CHECKPOINT ← EVALUATE ← OBSERVE
```

### Core runtime layers

| Layer | Role | Crate |
|:--|:--|:--|
| **Bowser Core** | shared domain contracts | `bowser-core` |
| **Turtle** | scheduling, task state, recovery, distributed coordination | `turtle` |
| **Koopa** | execution backends | `koopa` |
| **Kairos** | always-on runtime integration | `kairos` |
| **Capability** | AGI/runtime capability graph | `capability` |
| **Inference** | model/provider routing | `inference` |
| **Storage** | durable content-addressed state | `storage` |
| **Identity** | autonomous identity and credentials | `identity` |
| **MCP** | model tool protocol | `mcp` |
| **GitHub Backend** | GitHub-backed project/control state | `github-backend` |
| **Bowserd** | persistent daemon | `bowserd` |

## 🤖 Repository-Native Autonomous Engineering

The repository itself is the engineering control plane. The canonical flow is:

```text
GitHub event
   ↓
Nessy Autonomous Control Plane
   ↓
Nessy Autonomous Engineer
   ↓
Read lineage / README / affected files / blame / dependency graph
   ↓
Reproduce
   ↓
Regression coverage
   ↓
Root-cause repair
   ↓
Full validation matrix
   ↓
Atomic Conventional Commit
   ↓
Automation branch + PR
   ↓
Automated promotion
   ↓
Post-merge verification
```

The control plane responds to pushes, pull requests, issues, workflow completions, scheduled health cycles, and machine dispatch. The worker performs diagnosis, implementation, testing, validation, and promotion without requesting operator input during a normal cycle.

Repository automation also owns failure recovery: a failed gate is an engineering input, not a reason to silently mark the run successful.

See [`AUTOMATION.md`](AUTOMATION.md), [`docs/README.md`](docs/README.md), and the workflows under `.github/workflows/`.

## 🛡️ Security + Integrity

Nessy treats identity, authority, execution, state, network access, tool dispatch, artifact integrity, and auditability as explicit machine-readable concerns. Repository validation includes formatting, compilation, tests, Clippy, dependency audit, cargo-deny policy, project-license assertion, SBOM generation, repository integrity, and interposition checks.

See [`SECURITY.md`](SECURITY.md).

## 🚀 Build and validate

```bash
cargo fmt --all -- --check
cargo check --workspace --all-targets
cargo test --workspace --all-targets
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo audit
cargo deny check
```

Autonomous bootstrap is available through `scripts/bootstrap.sh`.

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
│   ├── chat/             repository-native chat UI + controller
│   ├── models/           free/open model registry
│   ├── AGI_CAPABILITY_MATRIX.md
│   ├── DOCUMENTATION_COVERAGE.md
│   └── README.md
├── scripts/              autonomous tooling and integrity checks
├── .github/workflows/    CI, security, integrity, deployment, autonomy
├── ARCHITECTURE.md
├── AUTOMATION.md
├── SECURITY.md
├── CONTRIBUTING.md
├── NOTICE
└── LICENSE
```

## 📚 Documentation

| Document | Purpose |
|:--|:--|
| [`ARCHITECTURE.md`](ARCHITECTURE.md) | System topology and runtime composition |
| [`AUTOMATION.md`](AUTOMATION.md) | Autonomous control plane and engineering lifecycle |
| [`SECURITY.md`](SECURITY.md) | Security and integrity architecture |
| [`CONTRIBUTING.md`](CONTRIBUTING.md) | Engineering standards and validation contract |
| [`docs/README.md`](docs/README.md) | Documentation hub |
| [`docs/DOCUMENTATION_COVERAGE.md`](docs/DOCUMENTATION_COVERAGE.md) | Documentation completeness and synchronization map |
| [`docs/AGI_CAPABILITY_MATRIX.md`](docs/AGI_CAPABILITY_MATRIX.md) | AGI capability and frontier research matrix |
| [`docs/chat/index.html`](docs/chat/index.html) | Live chat interface |
| [`docs/chat/app.js`](docs/chat/app.js) | Chat controller |
| [`docs/chat/runtime.json`](docs/chat/runtime.json) | Runtime/provider routing |
| [`docs/models/free-models.json`](docs/models/free-models.json) | Free/open model and provider registry |
| [`NOTICE`](NOTICE) | Creator attribution and legal notice |
| [`LICENSE`](LICENSE) | Apache License 2.0 |

<div align="center">

### 🐢 BOWSERAI / NESSY

**Created by Adam Joseph Rivers · CEO, Synthicsoft Labs LLC**

**AGI capability fabric · Frontier model fabric · Autonomous engineering · Persistent chat**

</div>
