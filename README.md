<!-- Copyright 2026 Adam Joseph Rivers, CEO, Synthicsoft Labs LLC. Licensed under Apache-2.0. -->

<div align="center">

# 🐢 BOWSERAI

### NESSY · AUTONOMOUS INTELLIGENCE FABRIC

**Persistent. Distributed. Self-configuring. GitHub-native.**

**Created by Adam Joseph Rivers · CEO, Synthicsoft Labs LLC**

[![License: Apache-2.0](https://img.shields.io/badge/License-Apache--2.0-111827?style=for-the-badge&logo=apache)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-2024-111827?style=for-the-badge&logo=rust)](https://www.rust-lang.org/)
[![Repository](https://img.shields.io/badge/GitHub-SynthicsoftLabs%2FNessy-111827?style=for-the-badge&logo=github)](https://github.com/SynthicsoftLabs/Nessy)

**Nessy is the engineering foundation of BowserAI:** an autonomous intelligence fabric coordinating runtimes, inference, execution, identity, durable state, MCP tools, and distributed infrastructure through one coherent system.

</div>

---

> **Attribution:** BowserAI / Nessy was created by **Adam Joseph Rivers, CEO of Synthicsoft Labs LLC**. Redistribution is governed by the Apache License, Version 2.0 and its applicable copyright, attribution, NOTICE, and trademark provisions. See [`NOTICE`](NOTICE) and [`LICENSE`](LICENSE).

---

## ◈ The System

```text
                         ┌──────────────────────────────┐
                         │           BOWSERAI           │
                         │     AUTONOMOUS FABRIC        │
                         └──────────────┬───────────────┘
                                        │
                    ┌───────────────────┼───────────────────┐
                    ▼                   ▼                   ▼
              ┌──────────┐       ┌────────────┐       ┌───────────┐
              │  TURTLE  │◄─────►│ CAPABILITY │◄─────►│  KAIROS   │
              │  ENGINE  │       │   ROUTER   │       │ ALWAYS-ON │
              └────┬─────┘       └─────┬──────┘       └─────┬─────┘
                   │                   │                    │
             ┌─────┴─────┐      ┌─────┴─────┐        ┌─────┴─────┐
             ▼           ▼      ▼           ▼        ▼           ▼
          ┌──────┐   ┌──────┐ ┌──────┐   ┌──────┐ ┌──────┐   ┌──────┐
          │ Koopa│   │ MCP  │ │ Local│   │ Cloud│ │ P2P  │   │Remote│
          │Sandbox│  │Tools │ │Models│   │Models│ │Nodes │   │Agents │
          └───┬──┘   └──────┘ └───┬──┘   └──┬───┘ └───┬──┘   └──┬───┘
              │                    │          │         │          │
              └────────────────────┴──────────┴─────────┴──────────┘
                                       │
                              ┌────────▼────────┐
                              │ DURABLE STATE + │
                              │ CONTENT ADDRESS │
                              └────────┬────────┘
                                       │
                           ┌───────────┼───────────┐
                           ▼           ▼           ▼
                        GitHub       CAS       Persistent DB
```

## ◇ Core Fabric

| Layer | Role | Crate |
|:--|:--|:--|
| **Bowser Core** | Shared domain model and contracts | `bowser-core` |
| **Turtle** | Scheduling and task lifecycle | `turtle` |
| **Koopa** | Execution and sandbox boundary | `koopa` |
| **Kairos** | Always-on autonomous runtime | `kairos` |
| **Capability** | Runtime discovery and selection | `capability` |
| **Inference** | Provider abstraction and routing | `inference` |
| **Storage** | Durable content-addressed state | `storage` |
| **Identity** | Autonomous agent identity | `identity` |
| **MCP** | Tool protocol and validation | `mcp` |
| **Bowserd** | Persistent autonomous daemon | `bowserd` |

## ⚡ Autonomous by Design

```text
        DISCOVER
           │
           ▼
       REGISTER ────────┐
           │            │
           ▼            │
        ROUTE ◄─────────┤
           │            │
           ▼            │
        EXECUTE         │
           │            │
           ▼            │
       CHECKPOINT       │
           │            │
           ▼            │
        RECOVER ────────┘
           │
           ▼
       CONTINUE
```

GitHub is the public project/control substrate; Kairos is an always-on autonomous runtime; inference and execution providers are extensible through the capability fabric.

## ✦ Visual Language

- **🐢 Turtle / Nessy** — persistence, movement, resilience.
- **BowserAI** — the orchestration identity.
- **Monospace diagrams** — systems-first communication.
- **High-contrast documentation** — fast scanning and precise navigation.
- **Apache-2.0** — one clear project license.
- **GitHub-native artifacts** — source, automation, releases, and project state in one public substrate.

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
│   ├── koopa/            sandbox/execution
│   ├── kairos/           always-on runtime
│   ├── capability/       capability fabric
│   ├── inference/        model/provider routing
│   ├── storage/          durable CAS
│   ├── identity/         autonomous identity
│   ├── mcp/              MCP protocol
│   └── bowserd/          daemon
├── docs/                 architecture and engineering docs
├── scripts/              autonomous bootstrap/tooling
├── .github/              CI and repository automation
├── ARCHITECTURE.md       system architecture
├── AUTOMATION.md         autonomous engineering contract
├── SECURITY.md           security architecture
├── NOTICE                creator attribution and legal notice
└── LICENSE               Apache License 2.0
```

## 📚 Documentation

| Document | Purpose |
|:--|:--|
| [`ARCHITECTURE.md`](ARCHITECTURE.md) | System topology and subsystem boundaries |
| [`AUTOMATION.md`](AUTOMATION.md) | Autonomous build and repository automation |
| [`SECURITY.md`](SECURITY.md) | Security architecture and controls |
| [`NOTICE`](NOTICE) | Creator attribution and legal notice |
| [`CONTRIBUTING.md`](CONTRIBUTING.md) | Engineering workflow |
| [`LICENSE`](LICENSE) | Apache License 2.0 |

<div align="center">

### 🐢 BOWSERAI / NESSY

**Created by Adam Joseph Rivers · Synthicsoft Labs LLC**

**One fabric. Many runtimes. Persistent state. Autonomous operation.**

[GitHub](https://github.com/SynthicsoftLabs/Nessy) · [Architecture](ARCHITECTURE.md) · [Security](SECURITY.md)

</div>
