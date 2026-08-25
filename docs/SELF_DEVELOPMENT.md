<!-- Copyright 2026 Adam Joseph Rivers, CEO, Synthicsoft Labs LLC. Licensed under Apache-2.0. -->

# Nessy Self-Development Program

Nessy continuously develops itself from repository state without requiring a human to assemble a development task list. The autonomous control plane is the scheduler; the autonomous engineer is the implementation worker; CI, integrity, and repository state are the feedback system.

## Development loop

```text
OBSERVE REPOSITORY
      ↓
DISCOVER GAPS / OPPORTUNITIES
      ↓
RANK BY VALUE + HEALTH + DEPENDENCY IMPACT
      ↓
SELECT ONE ATOMIC DEVELOPMENT OBJECTIVE
      ↓
RESEARCH EXISTING IMPLEMENTATION / EXTERNAL CAPABILITIES
      ↓
WRITE REGRESSION / ACCEPTANCE COVERAGE
      ↓
IMPLEMENT ROOT-CAUSE OR FEATURE CHANGE
      ↓
SYNCHRONIZE DOCUMENTATION + REGISTRIES
      ↓
RUN COMPLETE VALIDATION
      ↓
CREATE ATOMIC AUTOMATION BRANCH / PR
      ↓
AUTOMATIC PROMOTION
      ↓
VERIFY MAIN
      ↓
FEED RESULTS BACK INTO NEXT CYCLE
```

## Development surfaces

The worker continuously evaluates the complete repository, including:

- Core Rust crates and public interfaces.
- Capability graph coverage and capability composition.
- Turtle orchestration, scheduling, recovery, and distributed state.
- Koopa execution backends and execution-provider integration.
- Kairos runtime integration and always-on connectivity.
- Inference routing, provider health, model discovery, licensing metadata, and fallback paths.
- MCP transports, tool registration, annotations, caching, and dispatch.
- Identity, credentials, durable state, content addressing, and persistence.
- Browser chat UI, controller behavior, context persistence, runtime diagnostics, and response-quality tests.
- GitHub backend, repository automation, CI/CD, integrity verification, SBOM, and reproducibility.
- Security controls, dependency health, supply-chain policy, and auditability.
- Documentation, examples, architecture diagrams, capability matrices, model registries, and legal attribution.
- Research ingestion and normalization for newly discovered models, agents, runtimes, tools, and techniques.
- Release engineering, versioning, artifacts, distribution, and operational observability.

## Objective selection

When no externally supplied failure or task exists, the worker must derive the next objective from repository evidence. Priority order:

1. Broken functionality or failing validation.
2. Security, integrity, supply-chain, or data-loss defects.
3. Missing tests or regression coverage for existing behavior.
4. Incomplete or broken user-facing functionality.
5. Missing redundancy, failover, recovery, or observability.
6. Capability gaps required by existing architecture.
7. High-value improvements with clear repository-level acceptance criteria.
8. Documentation and registry drift.
9. Performance, ergonomics, aesthetics, and maintainability improvements.

One cycle should produce one logically atomic change. Larger initiatives are decomposed into subsequent machine-generated cycles rather than bundled into an unreviewable commit.

## Research ingestion

Research is treated as an engineering input. The worker may inspect current public model/provider catalogs, repository sources, official project documentation, and other machine-accessible references; extract capabilities; normalize them into Nessy's capability vocabulary; update registries; add an adapter when warranted; and add evaluation coverage.

External model or provider entries are recorded as discovery metadata. Their presence in a registry does not by itself assert that a service is browser-callable, anonymous, permanently free, or available without credentials.

## Self-maintained development state

The repository itself is the durable state surface. The worker should use:

- Git history and branches for implementation lineage.
- Issues and pull requests for active objectives and execution records.
- CI and workflow history for health signals.
- `docs/AGI_CAPABILITY_MATRIX.md` for normalized capability coverage.
- `docs/models/free-models.json` and runtime registries for provider/model discovery.
- `docs/DOCUMENTATION_COVERAGE.md` for documentation synchronization.
- This document for the self-development policy.

No separate human-maintained task board is required for normal autonomous operation.

## Completion criteria

A development cycle is complete only when the selected objective has a concrete implementation or documented machine-verifiable disposition, relevant regression/acceptance coverage exists, the complete repository validation matrix passes, documentation and registries are synchronized, the change is represented by one logical Conventional Commit, promotion succeeds, and the resulting `main` state has been re-read and integrity-verified.
