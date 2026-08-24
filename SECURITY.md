<!--
Copyright 2026 Synthicsoft Labs LLC
Licensed under the Apache License, Version 2.0.
-->

# 🛡️ Nessy Security

> **Security is a fabric property.** Identity, execution, networking, storage, tools, and automation are secured at their boundaries and composed into one auditable runtime.

---

## ◈ Security Model

```text
                         ┌─────────────────────┐
                         │      BOWSERAI       │
                         │   TRUST BOUNDARY    │
                         └──────────┬──────────┘
                                    │
          ┌─────────────────────────┼─────────────────────────┐
          ▼                         ▼                         ▼
     ┌──────────┐             ┌──────────┐             ┌──────────┐
     │ IDENTITY │             │  KOOPA   │             │   MCP    │
     │   KEYS   │             │ EXECUTE  │             │  TOOLS   │
     └────┬─────┘             └────┬─────┘             └────┬─────┘
          │                        │                        │
          └────────────────────────┼────────────────────────┘
                                   ▼
                         ┌──────────────────┐
                         │ POLICY + AUDIT   │
                         │ VALIDATE · TRACE │
                         └────────┬─────────┘
                                  │
                    ┌─────────────┼─────────────┐
                    ▼             ▼             ▼
                 STORAGE       NETWORK       RUNTIME
```

## 🔐 Identity

Agent and user identity are explicit domain objects. Credentials are runtime inputs, never source-controlled artifacts. Identity material belongs in encrypted runtime storage, operating-system credential stores, or delegated credential providers.

The architecture supports key-based identities, WebAuthn/passkeys, DID-oriented identity, and OAuth2.1/PKCE integration through replaceable authentication adapters.

## 🧱 Execution

Koopa defines the execution boundary. Sandboxed execution backends can include WASI, containers, microVMs, remote execution, and policy-controlled native runners.

Execution requests carry explicit capability and policy information. Tool metadata distinguishes read-only behavior from destructive behavior instead of inferring privilege from natural-language descriptions.

## 🌐 Network

Network-capable components use explicit transport adapters, bounded request lifetimes, structured errors, retry policy, and health-aware routing. Provider failure is represented as a runtime condition so the orchestration fabric can select another compatible capability.

## 🧰 MCP

MCP tool descriptors are validated before registration and dispatch. Content-addressed representations can be used to identify tool definitions and support deterministic cache/integrity checks.

## 💾 State

Durable state is separated from ephemeral execution. Content-addressed artifacts are integrity checked, task identity persists independently of a runtime, and storage adapters can participate in redundant persistence topologies.

## 🔑 Secrets

Never commit private keys, access tokens, passwords, session cookies, or production secret material.

Preferred sources are:

1. encrypted runtime secret storage;
2. operating-system credential stores;
3. hardware-backed credentials where available;
4. environment injection for deployment systems;
5. delegated secret providers.

## 🔎 Supply Chain

Repository automation is designed to validate formatting, compilation, tests, dependency policy, security auditing, SBOM generation, and artifact integrity. Dependencies remain subject to the project's Apache-2.0 compatibility policy and repository security controls.

## 🧭 Security Operations

Security signals should remain machine-readable and auditable. Runtime health, authorization decisions, tool dispatch, provider selection, artifact integrity, and recovery events belong in structured telemetry without exposing secret material.

For a vulnerability, use GitHub's private security-advisory mechanism when available. Do not place undisclosed exploit details in a public issue.

---

## ✦ Security Principles

| Principle | Implementation direction |
|:--|:--|
| **Explicit identity** | Agents and credentials have durable identities |
| **Bounded authority** | Capabilities are granted explicitly |
| **Isolated execution** | Koopa owns execution boundaries |
| **Validated tools** | MCP metadata is checked before dispatch |
| **Durable integrity** | Content addressing verifies stored artifacts |
| **Provider independence** | Routing does not depend on one runtime |
| **Auditable automation** | CI and runtime events remain observable |
| **No source secrets** | Credentials stay outside Git |

> 🐢 **Nessy security follows the same architectural principle as the rest of the fabric: no single component should silently become the whole trust model.**
