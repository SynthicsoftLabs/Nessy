# Nessy Automation

Nessy uses one repository-owned autonomous engineering control plane. The control plane runs only from `main`, serializes execution, creates one deterministic automation branch, performs one engineering cycle, creates one pull request, promotes it automatically, verifies the promoted commit, and leaves branch cleanup to the repository cleanup workflow.

## Execution model

`main push` / scheduled cycle / explicit dispatch

→ acquire the repository-level autonomy concurrency lock

→ checkout the current `main` commit

→ reuse the deterministic `automation/nessy-autonomous` branch after removing any stale copy

→ inspect Git history, README, automation policy, source, tests, workflows, registries, and active GitHub state

→ reproduce concrete failures and add regression coverage

→ execute the provider/model fabric with automatic fallback

→ validate the complete repository matrix

→ commit one logical change

→ push the deterministic automation branch

→ create or update one PR

→ automatic squash promotion

→ post-promotion immutable-state verification

→ recurring branch cleanup

## Branch discipline

`main` is the source of truth. Autonomous execution never writes directly to `main`. The engineering branch is always `automation/nessy-autonomous` and is reused rather than generated from workflow run IDs. Open pull-request heads are preserved by the cleanup workflow; obsolete `automation/*` branches are removed automatically.

## Provider redundancy

The coding engine uses the repository model/provider registry rather than a single vendor. LLM7 is the primary OpenAI-compatible route and supplies an explicit nonempty placeholder key when its anonymous path is used. Pollinations is secondary when a configured API key is available. Copilot is optional and is not part of the critical path.

## Validation

The autonomous executor performs formatting, check, tests, clippy, dependency audit, dependency policy, relevant Node tests/syntax, and project-specific checks before committing. Immutable repository verification is run only after the autonomous commit is pushed, because that verifier intentionally requires a clean Git tree.

## Recovery

A failed provider routes to the next configured provider. A failed validation aborts the cycle before commit. A changed `main` is reconciled before committing. A pre-existing open automation PR prevents a second autonomous cycle from competing with it. The concurrency group prevents overlapping autonomous workers.
