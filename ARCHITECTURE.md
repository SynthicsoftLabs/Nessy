<!--
Copyright 2026 Synthicsoft Labs LLC
Licensed under the Apache License, Version 2.0.
-->
# BowserAI Architecture

BowserAI is an autonomous, distributed execution fabric. Local execution is one capability, not the system boundary. Remote always-on runtimes, peer runtimes, cloud providers, and local providers participate through common capability contracts.

## Runtime fabric

```text
                         BOWSERAI
                            │
                ┌───────────┴───────────┐
                │   Capability Router   │
                └───────────┬───────────┘
                            │
        ┌───────────────────┼───────────────────┐
        ▼                   ▼                   ▼
     KAIROS             PEER RUNTIMES       INFERENCE
   always-on daemon       libp2p mesh       provider pool
        │                   │                   │
        └───────────────────┼───────────────────┘
                            ▼
                     TURTLE ORCHESTRATOR
                  queues / CRDT / recovery
                            │
                ┌───────────┼───────────┐
                ▼           ▼           ▼
             KOOPA       STORAGE      MCP
             execute      durable      tools
                │           │           │
       ┌────────┼──────┐    ├── SQLite  ├── HTTP
       ▼        ▼      ▼    ├── Postgres├── stdio
      WASI   Container MicroVM├── CAS/IPFS└── WebSocket
                              └── replicas
```

## Autonomous runtime selection

`capability` defines runtime descriptors and required capabilities. A task can carry multiple eligible runtimes. Selection uses capability compatibility, health, priority, and runtime identity. The design permits health-driven failover and migration without coupling tasks to a single provider.

## Kairos

`kairos` is a first-class runtime integration. Its endpoint is configurable through `KAIROS_URL` and defaults to `https://the-real-kairos.com`. The adapter is deliberately protocol-neutral at this layer so the concrete Kairon daemon transport can be implemented without baking an invented wire contract into the core.

## Inference

`inference` defines a provider-neutral interface. Providers are registered dynamically and are attempted in sequence with health checks and failure propagation. Local models, Kairos-backed inference, OpenAI-compatible gateways, and additional providers belong behind this interface.

## Turtle

`turtle` owns durable orchestration semantics: task identity, queueing, claiming, lifecycle transitions, recovery, and eventually CRDT/libp2p replication. Transport and provider implementations remain outside the domain model.

## Koopa

`koopa` defines the execution boundary. Backends include WASI, containers, microVMs, remote execution, and native policy-controlled runners. The backend selection mechanism is capability-driven rather than tied to one runtime.

## State and redundancy

State is designed around durable task identities, checkpoints, content-addressed artifacts, replicated metadata, and recoverable execution plans. Storage providers can be layered without changing task semantics.

## Failure domains

No single provider, runtime, transport, storage engine, model host, or execution backend is intended to be authoritative for the whole platform. Health, routing, persistence, and recovery are cross-cutting services so that an unavailable component can be replaced by another compatible component.

## Extension order

The implementation proceeds across all layers rather than making one narrow path the product: identity, storage, inference, Kairos integration, distributed synchronization, sandbox backends, MCP transports, observability, clients, deployment, reproducible distribution, and automated recovery are all part of the platform architecture.

Every layer preserves dependency direction toward `bowser-core` and capability contracts. Shared domain types do not acquire provider-specific assumptions.
