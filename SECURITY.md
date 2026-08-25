<!-- Copyright 2026 Adam Joseph Rivers, CEO, Synthicsoft Labs LLC. Licensed under Apache-2.0. -->

# 🛡️ Nessy Security

Nessy is autonomous in operation, with explicit machine-readable identity, authority, execution, integrity, recovery, and audit semantics.

## 1. Security fabric

```text
IDENTITY → AUTHORITY → CAPABILITY → EXECUTION → CHECKPOINT
    ▲                                             │
    │                                             ▼
    └────── HEALTH → INTEGRITY → RECOVERY ← OBSERVE
```

The principal security domains are identity, tool authorization, execution isolation, network access, secrets, durable state, software supply chain, repository integrity, and autonomous recovery.

## 2. Identity and authority

Agent identity is explicit and durable. Credential material is runtime input rather than source-controlled data. Adapters may support key-based identities, DID-oriented identity, WebAuthn/passkeys, OAuth2.1/PKCE, and provider-specific credentials.

Authority is expressed through machine-readable capabilities and execution policy. The autonomous engineering loop does not depend on a human approval transition during normal operation.

## 3. Execution

Koopa supplies selectable execution backends including WASI, containers, microVMs, remote runners, and policy-controlled native execution. Execution adapters receive explicit capability and authorization context.

## 4. Network and providers

Runtime/provider adapters use authenticated transport where required, explicit repository/provider identity, deadlines, structured errors, health signals, retry/failover policy, and routing metadata. Provider redundancy is a reliability mechanism; it does not remove the requirement to preserve credential and authorization semantics.

## 5. MCP and tool security

MCP tools are validated when registered and dispatched. Tool identity, annotations, transport, authorization context, capability requirements, and audit metadata are part of the machine-readable execution contract.

## 6. Secrets

Private keys, API credentials, passwords, cookies, deployment credentials, and other secret material do not belong in the repository. Supported secret sources are encrypted stores, operating-system credential stores, hardware-backed credentials, deployment secret injection, and provider-managed secret systems.

## 7. Durable state and integrity

Task identity, checkpoints, artifacts, conversation state, and project state are separated from any individual runtime. Content-addressed data provides integrity checks. Repository history provides immutable lineage.

## 8. Repository security and supply chain

The repository validation fabric covers:

- `cargo fmt`;
- `cargo check`;
- `cargo test`;
- Clippy with warnings denied;
- dependency audit;
- cargo-deny advisory/license policy;
- project-license assertion;
- SBOM generation;
- checkout/integrity verification;
- interposition verification;
- chat JavaScript validation where applicable.

Dependency licensing policy distinguishes Nessy's own Apache-2.0 project licensing from third-party dependency licensing, which is evaluated through dependency policy tooling.

## 9. Autonomous security operations

```text
EVENT
 ↓
CONTROL PLANE
 ↓
INSPECT RUN / SHA / LOGS
 ↓
CLASSIFY
 ↓
REPRODUCE
 ↓
REPAIR
 ↓
VALIDATE
 ↓
PROMOTE
 ↓
VERIFY MAIN
```

A failed security or integrity gate becomes an autonomous engineering input. A failure is not converted into a success assertion.

## 10. Runtime and chat security

The browser chat is a public executable surface. Runtime configuration, provider selection, browser inference, persistent session state, and artifact deployment are all subject to repository validation. Client-side persistence is scoped to the browser's storage environment; provider credentials are not embedded into the public chat controller.

## 11. Autonomous engineering security

The autonomous worker is given repository-native tools and a non-interactive task contract. It must inspect lineage, reproduce defects, add regression coverage, validate the complete workspace, inspect the final diff, and verify promotion. It works through automation branches and pull requests rather than direct `main` development.

The control plane and worker use explicit repository targeting (`GH_REPO` / `--repo`) for GitHub CLI operations, and jobs requiring Git first establish a repository checkout.

## 12. Attribution and legal identity

BowserAI / Nessy was created by **Adam Joseph Rivers, CEO of Synthicsoft Labs LLC**. The project is licensed under Apache License 2.0. Attribution and legal notices are maintained in `NOTICE` and `LICENSE`.

## 13. GitHub platform controls

Repository workflows provide source-controlled security and integrity controls. GitHub branch protection and rulesets are separate platform-level controls. Documentation distinguishes those platform settings from repository-enforced policy so the repository never represents an unverified GitHub setting as active.
