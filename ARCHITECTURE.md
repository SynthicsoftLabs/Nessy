<!--
Copyright 2026 Synthicsoft Labs LLC
Licensed under the Apache License, Version 2.0.
-->

# 🐢 BowserAI Architecture

> **NESSY · Autonomous Intelligence Fabric**
>
> One orchestration plane. Many runtimes. Durable state. Continuous recovery.

---

## ◈ Fabric Overview

BowserAI is an autonomous, distributed execution fabric. Local execution is one capability—not the system boundary. Always-on runtimes, peer runtimes, GitHub-backed infrastructure, inference providers, and execution environments participate through common capability contracts.

```text
                                      ┌─────────────────────┐
                                      │      BOWSERAI       │
                                      │  AUTONOMOUS FABRIC  │
                                      └──────────┬──────────┘
                                                 │
                                      ┌──────────▼──────────┐
                                      │  CAPABILITY ROUTER  │
                                      └──────────┬──────────┘
                                                 │
                  ┌──────────────────────────────┼──────────────────────────────┐
                  │                              │                              │
                  ▼                              ▼                              ▼
          ┌────────────────┐            ┌────────────────┐            ┌────────────────┐
          │     KAIROS     │            │   PEER RUNTIMES │            │    INFERENCE   │
          │    ALWAYS-ON   │            │    LIBP2P MESH  │            │ PROVIDER POOL  │
          └───────┬────────┘            └───────┬────────┘            └───────┬────────┘
                  └─────────────────────────────┼──────────────────────────────┘
                                                ▼
                                  ┌────────────────────────┐
                                  │    TURTLE ORCHESTRATOR  │
                                  │ queues · state · CRDT   │
                                  │ recovery · scheduling   │
                                  └────────────┬───────────┘
                                               │
                         ┌─────────────────────┼─────────────────────┐
                         ▼                     ▼                     ▼
                  ┌─────────────┐      ┌─────────────┐       ┌─────────────┐
                  │    KOOPA    │      │   STORAGE   │       │     MCP     │
                  │  EXECUTION  │      │ DURABLE CAS │       │    TOOLS    │
                  └──────┬──────┘      └──────┬──────┘       └──────┬──────┘
                         │                    │                     │
                  WASI · containers     GitHub · SQLite        HTTP · stdio
                  microVM · remote      Postgres · IPFS       WebSocket · P2P
```

## ◇ Runtime Fabric

The `capability` layer describes what a runtime can do. Selection can combine capability compatibility, health, priority, identity, and availability. Tasks retain identity independently of their selected execution provider, allowing the orchestration layer to continue through provider changes.

| Fabric | Responsibility |
|:--|:--|
| **Kairos** | Always-on autonomous runtime integration |
| **Peer mesh** | Distributed runtime capacity and synchronization |
| **Inference** | Model/provider discovery and routing |
| **GitHub backend** | Public project/control substrate and repository state |
| **Turtle** | Task lifecycle, queues, scheduling, recovery |
| **Koopa** | Sandboxed execution backends |
| **Storage** | Durable content-addressed objects and state |
| **MCP** | Tool discovery, validation, and transport |

## ⚡ Autonomous Selection Loop

```text
          DISCOVER
             │
             ▼
          REGISTER
             │
             ▼
       MATCH CAPABILITIES
             │
             ▼
          SCORE / ROUTE
             │
             ▼
           EXECUTE
             │
             ▼
         CHECKPOINT
             │
       ┌─────┴─────┐
       │           │
    HEALTHY     FAILURE
       │           │
       ▼           ▼
    CONTINUE    RECOVER
                   │
                   ▼
              RE-ROUTE
                   │
                   └──────────────► EXECUTE
```

## 🛰️ Kairos

`kairos` is a first-class autonomous runtime integration. Its endpoint is configurable through `KAIROS_URL`, with the project default targeting `https://the-real-kairos.com`. The adapter boundary keeps transport-specific details isolated from core orchestration contracts.

## 🧠 Inference

`inference` defines a provider-neutral interface. Providers are dynamically registered and health-aware. Local models, Kairos-backed inference, OpenAI-compatible gateways, and additional providers belong behind the same routing surface.

## 🐢 Turtle

Turtle owns orchestration semantics: task identity, queueing, claiming, lifecycle transitions, persistence, recovery, and distributed synchronization. Providers and transports remain replaceable.

## 🛡️ Koopa

Koopa is the execution boundary. The architecture accommodates WASI, containers, microVMs, remote execution, and policy-controlled native runners. Backend selection is capability-driven.

## 💾 State & Redundancy

Durable task identity, checkpoints, content-addressed artifacts, replicated metadata, and recoverable execution plans keep state independent of any single runtime. GitHub, databases, and content-addressed storage can participate as persistence layers without changing task semantics.

## 🔗 Dependency Direction

```text
                   provider-specific systems
                            │
                 ┌──────────┼──────────┐
                 ▼          ▼          ▼
              Kairos     GitHub     Inference
                 │          │          │
                 └──────────┼──────────┘
                            ▼
                    capability contracts
                            │
                            ▼
                         Turtle
                            │
                            ▼
                       bowser-core
```

Shared domain contracts remain provider-neutral. Provider-specific behavior terminates at adapter boundaries rather than leaking into the core model.

## ✦ Platform Surface

The architecture encompasses identity, storage, inference, Kairos integration, distributed synchronization, sandbox backends, MCP transports, observability, clients, deployment, reproducible distribution, and automated recovery as one platform rather than isolated optional projects.
