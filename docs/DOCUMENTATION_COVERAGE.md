<!-- Copyright 2026 Adam Joseph Rivers, CEO, Synthicsoft Labs LLC. Licensed under Apache-2.0. -->

# Documentation Coverage

This document is the completeness map for the repository's public documentation.

| Requirement surface | Canonical source |
|:--|:--|
| Product identity and creator attribution | `README.md`, `NOTICE`, `LICENSE` |
| System architecture | `ARCHITECTURE.md` |
| AGI capability vocabulary and frontier research | `docs/AGI_CAPABILITY_MATRIX.md` |
| Runtime/provider discovery | `docs/chat/runtime.json`, `docs/models/free-models.json` |
| Browser chat | `docs/chat/index.html`, `docs/chat/app.js` |
| Durable chat history and session UX | `docs/chat/app.js`, `README.md` |
| Autonomous engineering | `AUTOMATION.md`, `.github/workflows/nessy-autonomous-control-plane.yml`, `.github/workflows/nessy-autonomous-engineer.yml` |
| CI/build/test policy | `CONTRIBUTING.md`, `.github/workflows/ci.yml`, `.github/workflows/preflight.yml` |
| Repository integrity/interposition | `SECURITY.md`, `.github/workflows/integrity.yml`, `.github/workflows/interposition.yml` |
| Security architecture | `SECURITY.md` |
| Attribution/licensing | `NOTICE`, `LICENSE`, `README.md` |
| Repository structure | `README.md`, `docs/README.md` |
| Release/distribution surface | `README.md`, workflow/deployment files |

## Current behavior represented

Nessy documentation covers the following operational surfaces:

- GitHub-backed project/control state.
- Persistent repository-native chat.
- Runtime discovery and provider failover.
- Browser inference and provider routing.
- Free/open model discovery and model metadata.
- AGI capability registration and composition.
- Always-on Kairos integration.
- Turtle orchestration and recovery.
- Koopa execution backends.
- MCP tool integration.
- Durable storage and content addressing.
- Agent identity and authorization.
- Observability, integrity, SBOM, dependency auditing, and policy gates.
- Autonomous issue, pull-request, workflow, and scheduled maintenance triggers.
- Automated diagnosis, regression testing, repair, validation, promotion, and post-merge verification.

## Documentation rule

When an implementation change introduces a new user-visible capability, runtime contract, provider, workflow, data format, or operational lifecycle, the same logical change must update the applicable canonical documentation and this coverage map.
