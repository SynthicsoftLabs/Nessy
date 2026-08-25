<!-- Copyright 2026 Adam Joseph Rivers, CEO, Synthicsoft Labs LLC. Licensed under Apache-2.0. -->

# Nessy Autonomous Engineering

Nessy treats the repository itself as the autonomous engineering control plane. The normal lifecycle is designed to run from repository state and machine-readable evidence without an operator-driven setup phase.

## 1. Authoritative topology

```text
GitHub event
   ↓
Nessy Autonomous Control Plane
   ↓
Nessy Autonomous Engineer
   ↓
Diagnosis → Regression Test → Root-Cause Fix → Full Validation
   ↓
Atomic Commit → Automation Branch → Pull Request
   ↓
Automated Promotion
   ↓
Post-Merge Verification
```

There is one authoritative dispatcher and one engineering worker. Legacy competing supervisor/promotion controllers have been retired to prevent duplicate automation loops.

## 2. Event sources

The control plane responds to machine events including:

- `push` to `main`;
- pull-request creation, synchronization, and reopening;
- new repository issues;
- completion of relevant CI, Preflight, Integrity, Interposition Integrity, chat artifact, and CodeQL workflows;
- scheduled health/maintenance cycles;
- explicit machine dispatch.

A workflow event becomes an engineering objective. A failed check becomes a reproducible failure task rather than a reason to mask the failure.

## 3. Control-plane responsibilities

The controller resolves:

- event type;
- target reference;
- issue/task identity;
- objective text;
- repository identity;
- current GitHub workflow/PR context.

It records a machine receipt, launches the worker through the repository's supported Actions dispatch path, and verifies that the worker became observable.

All GitHub CLI operations explicitly target the repository with `GH_REPO` / `--repo`. Jobs that require Git also check out the repository at the event SHA before using Git commands.

## 4. Engineering worker

The worker is non-interactive. Its contract is:

1. Read `git log --oneline --graph -n 20`.
2. Read `README.md`, `AUTOMATION.md`, `CONTRIBUTING.md`, affected source, workflows, and relevant architecture/security documentation.
3. Inspect `git blame` for files being changed.
4. Inspect dependency graph, workflow runs, active PRs, issues, repository state, and existing tests.
5. Reproduce a reported failure before changing behavior when a failure can be reproduced from repository evidence.
6. Add focused regression coverage for defects.
7. Trace callers, dependency edges, workflow consumers, and public interfaces before changing them.
8. Implement the smallest complete root-cause fix.
9. Run the complete repository validation matrix.
10. Correct actual failures and rerun the affected gate followed by the complete matrix.
11. Inspect the final diff for atomicity, idempotency, credentials, merge markers, generated artifacts, and unrelated changes.
12. Create one logical Conventional Commit.
13. Work on an automation branch rather than directly on `main`.
14. Create or update a pull request and promote only after validation succeeds.
15. Verify the resulting `main` SHA independently after promotion.
16. Record the exact task, commit/PR, validation outcome, and resulting `main` SHA back to the machine task when one exists.

## 5. Validation matrix

The standard autonomous matrix includes:

```text
cargo fmt --all -- --check
cargo check --workspace --all-targets
cargo test --workspace --all-targets
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo audit
cargo deny check
Node syntax/tests for repository chat surfaces
repository integrity verification
interposition verification
relevant project-specific scripts
```

The authoritative GitHub workflows additionally cover project-license assertions, SBOM generation, advisory policy, and other repository checks.

## 6. Failure-driven recovery

A failed workflow is itself a machine-generated task input.

```text
FAILED CHECK
    ↓
CAPTURE RUN / SHA / LOGS
    ↓
CLASSIFY ROOT CAUSE
    ↓
REPRODUCE
    ↓
REGRESSION TEST
    ↓
FIX
    ↓
REVALIDATE
    ↓
PROMOTE
    ↓
VERIFY
```

The system does not manufacture a success result when a gate fails.

## 7. Chat/runtime engineering

The repository chat is included in the same engineering contract. Its executable surfaces include:

- DOM and controller wiring;
- persistent IndexedDB conversation state;
- runtime/provider routing;
- browser inference model loading;
- model progress reporting;
- response-quality regression detection;
- runtime diagnostics;
- chat artifact/deployment validation.

A chat regression is therefore handled as a normal repository engineering task rather than an isolated UI change.

## 8. Model/provider expansion

Provider and model discovery is represented in machine-readable registries. The free/open registry at `docs/models/free-models.json` is informed by `12britz/awesome-free-models`; current catalog scope includes open-weight models, free API providers, routers, local runtimes, coding models, multimodal systems, embeddings, RAG, agent frameworks, MCP servers, evaluation/observability, and hosting resources.

Provider entries remain metadata-driven so routing can account for capability, credentials, access status, license, modalities, and context requirements.

## 9. Git contract

- `main` is not the working branch for autonomous changes.
- Autonomous changes use `automation/*` branches.
- One logical change receives one Conventional Commit.
- PR promotion is automated after repository validation.
- Post-merge verification reads the resulting `main` SHA directly.
- Git history remains the authoritative lineage.

## 10. Observability

Every autonomous cycle writes machine-readable evidence to the GitHub Actions summary. Trigger type, target reference, objective, worker run, validation outcome, promotion state, and post-merge target are recorded when available.

## 11. GitHub platform policy

The repository implements its engineering and validation policy in source-controlled workflows. GitHub branch protection/rulesets are an additional platform-level control surface and are not represented as repository files. The repository does not claim platform rules are enabled unless GitHub reports them as enabled.

## 12. Documentation synchronization

When the automation architecture changes, update:

- `README.md`;
- `ARCHITECTURE.md`;
- `AUTOMATION.md`;
- `SECURITY.md`;
- `CONTRIBUTING.md`;
- `docs/README.md`;
- `docs/DOCUMENTATION_COVERAGE.md`.

This keeps operational behavior, architecture, security, engineering policy, and user-facing documentation synchronized.
