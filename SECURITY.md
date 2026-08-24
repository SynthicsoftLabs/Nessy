<!-- Copyright 2026 Synthicsoft Labs LLC; Licensed under Apache-2.0. -->

# 🛡️ Nessy Security

> **Autonomous does not mean undefined.** Nessy can operate without a human in the loop while retaining explicit security semantics, authenticated identity, auditable state, and selectable execution policy.

## ◈ Security Fabric

```text
                       ┌─────────────────────┐
                       │      BOWSERAI       │
                       │ AUTONOMOUS FABRIC   │
                       └──────────┬──────────┘
                                  │
        ┌─────────────────────────┼─────────────────────────┐
        ▼                         ▼                         ▼
   ┌──────────┐             ┌──────────┐             ┌──────────┐
   │ IDENTITY │             │  KOOPA   │             │   MCP    │
   │   KEYS   │             │ EXECUTE  │             │  TOOLS   │
   └────┬─────┘             └────┬─────┘             └────┬─────┘
        └─────────────────────────┼─────────────────────────┘
                                  ▼
                         ┌──────────────────┐
                         │ POLICY / AUDIT   │
                         │ VALIDATE · TRACE │
                         └────────┬─────────┘
                                  │
                  ┌───────────────┼───────────────┐
                  ▼               ▼               ▼
               STORAGE         NETWORK         RUNTIME
```

## 🔐 Identity

Agent identity is explicit and durable. Credentials are runtime inputs and are not committed to source control. The identity subsystem can support key-based identities, DID-oriented identity, WebAuthn/passkeys, and OAuth2.1/PKCE through adapters.

## 🧱 Execution

Koopa provides selectable execution backends including WASI, containers, microVMs, remote runners, and policy-controlled native execution. Autonomous operation does not require a human approval step between task transitions; authorization is expressed through machine-readable capabilities and runtime policy.

## 🌐 Network

Network adapters use authenticated transport, request deadlines, structured errors, retry policy, health signals, and provider failover. The capability fabric can move work between available compatible runtimes without changing task identity.

## 🧰 MCP

MCP tools are validated at registration and dispatch. Tool metadata, transport identity, authorization context, and content-addressed definitions form the machine-readable basis for tool execution.

## 💾 Durable State

Task identity and checkpoints are independent of the runtime executing a task. Content-addressed artifacts provide integrity verification. GitHub, persistent databases, and content-addressed storage can participate in redundant persistence topologies.

## 🔑 Secrets

Private keys, access tokens, passwords, cookies, and production credentials remain outside source control. Runtime secret sources include encrypted storage, operating-system credential stores, hardware-backed credentials, deployment secret injection, and delegated secret providers.

## 🔎 Supply Chain

Repository automation validates formatting, compilation, tests, dependency policy, security auditing, SBOM generation, and artifact integrity. Dependencies remain governed by the project's Apache-2.0 licensing requirements.

## 🛰️ Autonomous Security Operations

```text
DISCOVER → AUTHENTICATE → AUTHORIZE → EXECUTE → CHECKPOINT
    ▲                                             │
    └──────────── HEALTH / RECOVERY ◄─────────────┘
```

Security telemetry remains machine-readable and auditable. Health, authorization, tool dispatch, provider selection, artifact integrity, and recovery events can be recorded without exposing secret material.

## ✦ Security Principles

| Principle | Nessy implementation |
|:--|:--|
| **Autonomous operation** | No mandatory human approval loop in normal execution |
| **Explicit authority** | Capabilities and policies are machine-readable |
| **Identity** | Durable agent and credential identity |
| **Execution isolation** | Koopa execution backends |
| **Tool validation** | MCP registration and dispatch validation |
| **Integrity** | Content-addressed state and artifacts |
| **Provider independence** | Capability-driven runtime selection |
| **Recovery** | Persistent checkpoints and automatic re-routing |
| **Auditability** | Structured runtime and repository telemetry |
| **Secret separation** | Credentials remain outside source |

> 🐢 **Nessy is autonomous in operation while remaining explicit about identity, authority, state, and execution semantics.**
