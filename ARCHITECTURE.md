<!-- Copyright 2026 Adam Joseph Rivers, CEO, Synthicsoft Labs LLC. Licensed under Apache-2.0. -->

# 🐢 BowserAI Architecture

> **NESSY · Autonomous Intelligence Fabric**
>
> One capability graph. Many runtimes. Durable state. Provider redundancy. GitHub-backed control state. Repository-native autonomous engineering.

## 1. Fabric Overview

BowserAI is organized around normalized capabilities instead of dependence on a single model or provider. Models, runtimes, tools, memory, execution environments, GitHub state, and autonomous engineering workflows communicate through explicit contracts.

```text
                            BOWSERAI / NESSY
                                  │
                           CAPABILITY GRAPH
                                  │
       ┌──────────────────────────┼──────────────────────────┐
       ▼                          ▼                          ▼
    KAIROS                     TURTLE                    INFERENCE
  always-on runtime        orchestration/state       provider/model fabric
       │                          │                          │
       └──────────────────────────┼──────────────────────────┘
                                  ▼
       ┌────────────┬────────────┬────────────┬────────────┬────────────┐
       │   KOOPA    │    MCP     │  STORAGE   │  IDENTITY  │  GITHUB    │
       │ execution  │   tools    │ durable CAS│  agents    │  backend   │
       └────────────┴────────────┴────────────┴────────────┴────────────┘
                                  │
                                  ▼
                       OBSERVE → EVALUATE → LEARN
                                  │
                                  └────────► DISCOVER
```

## 2. Capability Graph

Capabilities are executable runtime contracts. Domains include reasoning, verification, reflection, planning, memory, learning, agents, computer use, coding, research, multimodality, generation, scientific computation, security, infrastructure, governance, and interaction.

A capability registration includes identity, version, provider, input/output contract, prerequisites, quality, latency, health, authorization, telemetry, recovery, provenance, and evaluation metadata.

Multiple implementations can satisfy the same capability. The routing layer selects and composes compatible providers and can recover to alternate implementations when a provider becomes unavailable or unhealthy.

See [`docs/AGI_CAPABILITY_MATRIX.md`](docs/AGI_CAPABILITY_MATRIX.md).

## 3. Frontier + Free Model Fabric

Frontier research is normalized into the capability vocabulary. Current named coverage includes Fable, Glasswing, Qwen, Gemini, Grok, Seed, Seedance, Perplexity, Z.ai / GLM, and Gemma, alongside broader provider families.

The repository also contains `docs/models/free-models.json`, a machine-readable catalog derived from the public `12britz/awesome-free-models` resource. It separates discovery metadata from runtime availability so the router can reason about access, credentials, capability, modality, context, and licensing without treating every entry as a guaranteed browser endpoint.

## 4. Runtime Topology

```text
DISCOVER → REGISTER → MATCH → COMPOSE → ROUTE → EXECUTE
    ▲                                           │
    │                                           ▼
    └──── RECOVER ← CHECKPOINT ← EVALUATE ← OBSERVE
```

### Kairos

`kairos` is the always-on runtime integration. Its transport is encapsulated behind the runtime contract and may use the configured `KAIROS_URL` / public Kairos service.

### Turtle

Turtle owns task identity, queueing, scheduling, claiming, lifecycle state, distributed synchronization, checkpoints, and recovery.

### Koopa

Koopa provides execution backends including WASI, containers, microVMs, remote runners, and policy-controlled native execution.

### MCP

MCP supplies tool discovery, registration, validation, and dispatch. Tool capabilities participate in the common capability graph.

### Storage

Durable task state, checkpoints, artifacts, project data, content-addressed objects, and persistent conversation state survive individual runtime failure.

### GitHub Backend

`github-backend` treats GitHub as a first-class public project/control substrate for repository state, automation, artifacts, issue/PR workflows, and distribution.

## 5. Repository-Native Chat

The browser chat is an executable product surface, not documentation decoration.

```text
README → HTML renderer → chat/index.html → chat/app.js
                                           │
                            ┌──────────────┼──────────────┐
                            ▼              ▼              ▼
                         Kairos        cloud/API      browser inference
                                           │              │
                                           └──────┬───────┘
                                                  ▼
                                         persistent session state
```

The chat persists conversations until explicitly cleared, supports search/import/export/pinning/share/copy/regeneration, and routes across multiple inference surfaces. The model-request context layer can use stored history without deleting the underlying transcript.

## 6. Autonomous Engineering Control Plane

The repository has one authoritative autonomous engineering architecture:

```text
EVENT
  │
  ▼
NESSY AUTONOMOUS CONTROL PLANE
  │
  ▼
NESSY AUTONOMOUS ENGINEER
  │
  ├─ establish Git/DAG/repository context
  ├─ inspect README / architecture / blame / dependencies
  ├─ reproduce observed failure
  ├─ add regression coverage
  ├─ implement root-cause change
  ├─ run full validation matrix
  ├─ create atomic Conventional Commit
  ├─ create/update automation branch + PR
  ├─ monitor validation
  ├─ promote verified change
  └─ verify resulting main SHA
```

The control plane is event-driven across pushes, pull requests, issues, workflow completions, recurring schedules, and machine dispatch. The worker receives an explicit objective and target reference and performs the engineering cycle without operator prompts during normal operation.

Legacy competing autonomous controllers have been retired so the repository does not have multiple independent promotion loops.

## 7. State and Recovery

State is separated into:

- working/session context;
- durable task/project state;
- content-addressed artifacts;
- repository history;
- capability/provider metadata;
- audit and telemetry records.

A failed runtime can be replaced without rewriting task identity. Checkpoints and content addressing allow recovery, verification, replication, and provider migration.

## 8. Security and Integrity

Security semantics are explicit at identity, authority, tool, execution, network, secret, storage, and artifact layers. Repository integrity checks verify checkout identity, unresolved merge markers, dependency metadata, licensing, audit policy, SBOM generation, and interposition state.

See [`SECURITY.md`](SECURITY.md).

## 9. Dependency Direction

```text
models / providers / runtimes
            │
            ▼
      provider adapters
            │
            ▼
    capability contracts
            │
            ▼
      capability graph
            │
            ▼
          Turtle
            │
            ▼
       bowser-core
```

Provider-specific behavior terminates at adapters. Shared contracts remain provider-neutral.

## 10. Documentation and Source of Truth

The executable implementation is authoritative. The documentation hub at [`docs/README.md`](docs/README.md) indexes the canonical surfaces, while [`docs/DOCUMENTATION_COVERAGE.md`](docs/DOCUMENTATION_COVERAGE.md) records what must remain synchronized as the platform evolves.
