<!-- Copyright 2026 Adam Joseph Rivers, CEO, Synthicsoft Labs LLC. Licensed under Apache-2.0. -->

# Nessy Documentation

Nessy documentation is organized around one implementation: the BowserAI autonomous intelligence fabric.

## Core references

| Document | Purpose |
|:--|:--|
| [`../README.md`](../README.md) | Product overview, capabilities, runtime fabric, chat, model redundancy, and repository map |
| [`../ARCHITECTURE.md`](../ARCHITECTURE.md) | System topology, dependency direction, runtime composition, state, providers, and execution surfaces |
| [`../AUTOMATION.md`](../AUTOMATION.md) | Autonomous control-plane and engineering lifecycle |
| [`../SECURITY.md`](../SECURITY.md) | Identity, execution, network, MCP, state, secrets, supply chain, integrity, and recovery |
| [`../CONTRIBUTING.md`](../CONTRIBUTING.md) | Repository engineering standards and automated change lifecycle |
| [`AGI_CAPABILITY_MATRIX.md`](AGI_CAPABILITY_MATRIX.md) | Capability vocabulary, frontier coverage, and research-to-runtime methodology |
| [`DOCUMENTATION_COVERAGE.md`](DOCUMENTATION_COVERAGE.md) | Documentation completeness map and synchronization rule |
| [`chat/index.html`](chat/index.html) | Repository-native chat surface |
| [`chat/app.js`](chat/app.js) | Chat controller and runtime/session behavior |
| [`chat/runtime.json`](chat/runtime.json) | Machine-readable runtime/provider routing registry |
| [`models/free-models.json`](models/free-models.json) | Machine-readable free/open model and provider discovery catalog |

## System lifecycle

```text
DISCOVER → REGISTER → NORMALIZE → MATCH → COMPOSE → ROUTE → EXECUTE
    ▲                                                       │
    │                                                       ▼
    └──────── RECOVER ← CHECKPOINT ← EVALUATE ← OBSERVE ←──┘
```

## Autonomous engineering lifecycle

```text
EVENT → CONTROL PLANE → ENGINEER → REPRODUCE → TEST → FIX → VALIDATE
                                                     ↓
                           COMMIT → PR → PROMOTE → VERIFY MAIN
```

The repository is designed so routine engineering operation proceeds from repository state and machine-readable evidence rather than an operator-driven setup sequence.

## Documentation synchronization

Executable behavior is authoritative. Documentation is updated with architecture, interface, runtime, capability, provider, data-format, and operational-lifecycle changes. Machine-readable registries are source data for routing and discovery; they do not assert that every provider is browser-callable, permanently free, or anonymously accessible.

## Legal and attribution

BowserAI / Nessy was created by **Adam Joseph Rivers, CEO of Synthicsoft Labs LLC**. The project is licensed under the Apache License, Version 2.0. See [`../NOTICE`](../NOTICE) and [`../LICENSE`](../LICENSE).
