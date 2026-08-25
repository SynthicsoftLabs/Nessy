# BowserAI Autonomous Engineering

BowserAI/Nessy treats the repository itself as the autonomous engineering control plane. The complete lifecycle from event intake through diagnosis, repair, validation, promotion, and post-merge verification is repository-owned and machine-driven.

## Control-plane topology

`Nessy Autonomous Control Plane` is the sole dispatcher. `Nessy Autonomous Engineer` is the worker.

```text
repository event
      |
      v
Autonomous Control Plane
      |
      +--> issue / PR context
      +--> workflow result / failure context
      +--> main push health cycle
      +--> scheduled maintenance
      |
      v
Autonomous Engineer
      |
      +--> read history / README / blame / dependency graph
      +--> reproduce / regression test
      +--> root-cause implementation
      +--> complete validation matrix
      +--> atomic Conventional Commit
      +--> automation branch + PR
      +--> automatic promotion
      +--> post-merge verification
      |
      v
verified main state
```

The control plane listens to repository pushes, pull-request changes, issue creation, completion of the repository validation workflows, scheduled health cycles, and direct workflow dispatch. It resolves machine context, dispatches the worker with an explicit repository/ref/objective, and records an execution receipt when an issue is involved.

The engineer workflow is intentionally worker-only and accepts machine objectives through `workflow_dispatch`. It does not ask an operator for configuration or confirmation.

## Engineering loop

1. Establish repository lineage and current state before editing.
2. Read README, architecture, contribution rules, affected source, tests, workflows, and blame context.
3. Determine the actual objective from machine evidence and reproduce observed failures.
4. Add focused regression coverage for defects.
5. Trace callers, dependency edges, workflow consumers, and public interfaces before changing behavior.
6. Apply the smallest complete root-cause fix.
7. Execute the complete validation matrix: Rust formatting, check, tests, Clippy, RustSec audit, cargo-deny policy, chat JavaScript validation, repository integrity, and relevant project scripts.
8. Repair failed gates and repeat the complete matrix until the tree is clean.
9. Inspect the final diff for atomicity, idempotency, documentation, credentials, merge markers, generated artifacts, and unrelated changes.
10. Commit one logical change using a strict Conventional Commit.
11. Never work directly on `main`; create or update an automation branch and use a pull request for promotion.
12. Monitor validation and automatically promote only the verified result.
13. Re-read `main` after promotion and verify repository integrity against the resulting SHA.
14. Record the machine result to the driving issue when an issue exists.

## Failure ownership

A failed validation workflow is a first-class engineering input. The control plane dispatches the same engineer against the affected ref with the workflow identity, result, SHA, and explicit repair objective. This turns failures into machine-generated engineering tasks instead of human triage queues.

The control plane also runs scheduled maintenance so defects are discovered independently of user reports or individual commits.

## Automation safety and determinism

All GitHub CLI calls in automation specify `--repo "$GITHUB_REPOSITORY"`; jobs that need repository files explicitly check out the target ref. This prevents implicit local-Git discovery from becoming a systemic failure mode.

The worker validates its final tree before promotion. Success is recorded only after the resulting GitHub state can be directly inspected. Intended state is never substituted for observed state.

## Scope

This repository-owned loop covers the work performed during the Nessy build sequence: repository construction, documentation, chat and runtime surfaces, model/provider redundancy, licensing policy, CI/CD, integrity checks, security ingestion, autonomous supervision, regression repair, branch/PR management, release validation, and continuous maintenance.

## Human-free operation

Normal operation requires no human-driven setup, configuration, steering, task handoff, or approval inside the engineering loop. Machine-readable repository events and the configured GitHub Actions runtime are the inputs to the control plane.

## Bootstrap

The host bootstrap path remains available through:

```bash
./scripts/bootstrap.sh
```

The autonomous repository engineering control plane is independent of host bootstrap and operates from GitHub Actions.
