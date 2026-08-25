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
| [`chat/index.html`](chat/index.html) | Repository-native chat surface |
| [`chat/app.js`](chat/app.js) | Chat controller and runtime/session behavior |
| [`chat/runtime.json`](chat/runtime.json) | Machine-readable runtime/provider routing registry |
| [`models/free-models.json`](models/free-models.json) | Machine-readable free/open model and provider discovery catalog |

## System lifecycle

```text
DISCOVER
   ↓
REGISTER
   ↓
NORMALIZE
   ↓
MATCH
   ↓
COMPOSE
   ↓
ROUTE
   ↓
EXECUTE
   ↓
OBSERVE
   ↓
EVALUATE
   ↓
MEMORY / SKILL UPDATE
   ↓
RECOVER OR CONTINUE
   ↓
DISCOVER
```

## Autonomous engineering lifecycle

```text
EVENT
  ↓
CONTROL PLANE
  ↓
ENGINEER WORKER
  ↓
REPRODUCE
  ↓
REGRESSION TEST
  ↓
ROOT-CAUSE FIX
  ↓
FULL VALIDATION
  ↓
ATOMIC COMMIT
  ↓
AUTOMATION BRANCH
  ↓
PULL REQUEST
  ↓
AUTOMATED PROMOTION
  ↓
POST-MERGE VERIFICATION
```

The repository is designed so normal engineering operation does not depend on an operator manually reproducing this sequence.

## Documentation synchronization

Executable behavior is authoritative. Documentation describes the behavior implemented in the repository and should be updated in the same logical change when architecture, interfaces, runtime routing, capability vocabulary, or operational workflow changes.

The machine-readable registries under `docs/` are source data for discovery and routing; they do not imply that every provider is directly browser-callable, permanently free, or anonymously accessible. Provider metadata records the relevant access and licensing fields so routing can make an informed selection.

## Legal and attribution

BowserAI / Nessy was created by Adam Joseph Rivers, CEO of Synthicsoft Labs LLC. The project is licensed under Apache License 2.0. See [`../NOTICE`](../NOTICE) and [`../LICENSE`](../LICENSE) for attribution and licensing terms.
