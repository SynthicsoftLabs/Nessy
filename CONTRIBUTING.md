<!-- Copyright 2026 Adam Joseph Rivers, CEO, Synthicsoft Labs LLC. Licensed under the Apache License, Version 2.0. -->

# Contributing

Nessy is maintained through a repository-native engineering contract. The normal change lifecycle is automated: inspect, reproduce, test, repair, validate, commit, PR, promote, and verify.

## Workflow

```text
EVENT
 ↓
CONTROL PLANE
 ↓
ENGINEER
 ↓
REPRODUCE
 ↓
REGRESSION TEST
 ↓
ROOT-CAUSE FIX
 ↓
FULL MATRIX
 ↓
ATOMIC COMMIT
 ↓
PR
 ↓
AUTOMATED PROMOTION
 ↓
POST-MERGE VERIFICATION
```

### Engineering rules

1. Read `git log --oneline --graph -n 20` before changing code.
2. Read `README.md`, `ARCHITECTURE.md`, `AUTOMATION.md`, `SECURITY.md`, and affected documentation before architectural changes.
3. Inspect `git blame` for files being changed and trace callers/dependents for interface changes.
4. Reproduce a bug before fixing it when repository evidence permits reproduction.
5. Add focused regression coverage before or alongside the implementation fix.
6. Prefer compiler-backed, AST-aware, and executable validation over text-only assertions.
7. Keep changes atomic and use strict Conventional Commit messages.
8. Never introduce hardcoded credentials, dead commented-out code, unresolved merge markers, or speculative unrelated refactors.
9. Run the complete validation matrix after a repair.
10. Work through automation branches and pull requests rather than treating `main` as a development workspace.

## Validation matrix

```bash
cargo fmt --all -- --check
cargo check --workspace --all-targets
cargo test --workspace --all-targets
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo audit
cargo deny check
```

Repository workflows additionally validate project licensing, SBOM generation, integrity/interposition, chat JavaScript, and deployment artifacts as applicable.

## Autonomous operation

The `Nessy Autonomous Control Plane` and `Nessy Autonomous Engineer` workflows are the repository's machine-driven engineering path. A workflow failure can become an autonomous repair objective. The engineer receives an explicit machine objective and repository target and does not prompt an operator during the normal cycle.

The control plane and worker use explicit repository targeting for GitHub CLI calls. Jobs that require Git first establish the repository checkout.

## Documentation

Changes that affect behavior, interfaces, runtime routing, providers, capability contracts, workflow topology, or user-facing features must update the relevant canonical documentation. See [`docs/README.md`](docs/README.md) and [`docs/DOCUMENTATION_COVERAGE.md`](docs/DOCUMENTATION_COVERAGE.md).

## Security and attribution

Credentials and generated secrets stay outside the source tree. Privileged execution remains behind explicit capability and policy boundaries.

BowserAI / Nessy was created by **Adam Joseph Rivers, CEO of Synthicsoft Labs LLC** and is licensed under the Apache License, Version 2.0. See [`NOTICE`](NOTICE) and [`LICENSE`](LICENSE).
