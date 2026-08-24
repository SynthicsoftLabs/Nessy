<!--
Copyright 2026 Synthicsoft Labs LLC
Licensed under the Apache License, Version 2.0.
-->
# BowserAI Architecture

## Runtime layers

```text
Clients / MCP transports
        │
        ▼
  bowserd process
        │
 ┌──────┼────────┐
 ▼      ▼        ▼
Turtle Koopa   Core
 │      │        │
 ▼      ▼        ▼
state  execution protocols
 │
 ▼
future: identity / storage / inference / observability
```

### Core
`bowser-core` owns stable domain identifiers, task lifecycle types, tool descriptors, and validation rules. It has no network or filesystem authority.

### Turtle
`turtle` owns scheduling and session/task lifecycle. Distributed CRDT and libp2p implementations belong behind this boundary rather than leaking transport details into application code.

### Koopa
`koopa` defines the execution capability boundary. The initial backend is deliberately deny-by-default; future WASI, container, and microVM adapters implement the same trait.

### Bowserd
`bowserd` is the process composition root. It initializes telemetry, constructs runtime services, owns lifecycle, and handles graceful shutdown.

## Extension order

1. Protocol and identity contracts.
2. Persistent state and cryptographic secret storage.
3. Local inference provider adapters.
4. MCP transport and tool registry.
5. Distributed synchronization and relay.
6. Sandbox backends.
7. Client applications and deployment packaging.

Every layer should preserve the dependency direction toward `bowser-core` and avoid introducing application-specific assumptions into shared domain types.
