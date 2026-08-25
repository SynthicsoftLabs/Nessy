# BowserAI Autonomous Engineering

BowserAI/Nessy treats the repository itself as the autonomous engineering control plane. Validation, diagnosis, repair, testing, branch management, pull-request creation, promotion, and post-merge verification are designed to execute without an operator-driven setup phase.

## Engineering loop

The `Nessy Autonomous Engineer` workflow continuously applies the repository engineering contract:

1. Read repository lineage, README, architecture, affected files, and blame before editing.
2. Inspect workflow state, commit state, pull requests, and dependency relationships.
3. Reproduce defects before changing implementation.
4. Add or update regression coverage for the observed failure.
5. Trace callers and dependent interfaces before changing signatures.
6. Apply the smallest complete root-cause fix.
7. Execute the full validation matrix, including Rust build/test/lint, audit/deny, chat JavaScript checks, and repository integrity.
8. Iterate on actual failures until the tree is clean.
9. Inspect the final diff and enforce atomic Conventional Commits.
10. Keep `main` protected from direct agent pushes by using automation branches and pull requests.
11. Open or update pull requests, monitor checks, and automatically promote validated changes.
12. Verify the resulting `main` commit and repository integrity after promotion.

The workflow uses GitHub Copilot CLI in non-interactive mode through the repository's Actions runtime. GitHub documents Copilot CLI execution from Actions and the `GITHUB_TOKEN`-based organization flow, including `copilot-requests: write`. The agent is explicitly prohibited from prompting for user input and is given repository-native GitHub, Git, build, test, and editing tools.

## Failure-driven operation

A completed workflow failure automatically becomes an engineering input. The autonomous engineer retrieves the failing run context, reads the affected source and test surface, reproduces the failure, repairs the root cause, reruns the complete validation matrix, and promotes the verified result.

The same engine also runs on pull-request updates and on a recurring schedule so repository drift, stale documentation, latent test failures, and integration defects are continuously discovered rather than waiting for a maintainer to notice them.

## Autonomous execution verification

Every autonomous cycle is itself observable. The triggering event, target SHA, working branch, pull-request state, validation result, promotion result, and post-merge target SHA are written into the GitHub Actions step summary. A cycle is considered complete only after the promoted `main` commit has been independently re-read and repository integrity succeeds against that exact commit.

A fresh same-repository pull request is therefore a complete machine-generated verification trigger: the autonomous engineer must inspect the repository, perform the engineering cycle, update the pull request as required, wait for the validation matrix, promote the result, and verify the resulting `main` state without requesting operator input.

## Git contract

`main` is never the working branch for an autonomous change. Repairs are committed on an isolated automation branch and promoted through a pull request. Logical work remains atomic and uses Conventional Commit messages.

## Chat and runtime contract

The repository chat remains subject to the same engineering loop. Runtime routing, browser inference, response-quality gates, persistent history, UI wiring, and deployment artifacts are validated as executable surfaces rather than documentation claims.

## Recovery contract

When a gate fails, the failure becomes the next task. The automation does not replace the failed result with a green assertion; it fixes the underlying cause and reruns the gate. Repository integrity and post-merge verification are treated as first-class engineering checks.

## Human-free operation

Routine repository engineering is intended to proceed from repository state and machine-readable evidence. Human-operated setup is not part of the normal execution path. GitHub Actions, the repository Git graph, existing project tooling, and the configured coding-agent runtime form the control loop.

## Bootstrap

The host bootstrap path remains:

```bash
./scripts/bootstrap.sh
```

The repository-level engineering loop is independent of host bootstrap and operates from GitHub Actions.
