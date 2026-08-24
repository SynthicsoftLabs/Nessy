<!--
Copyright 2026 Synthicsoft Labs LLC
Licensed under the Apache License, Version 2.0.
See LICENSE for the Apache License text and project notices.
-->
# 🐢 BowserAI / Nessy

Nessy is the repository foundation for BowserAI: a local-first, distributed AI execution platform built around a Rust orchestration core, explicit protocol boundaries, pluggable execution backends, and auditable state.

## Architecture

The implementation is organized as a Cargo workspace so protocol, orchestration, sandbox, identity, storage, routing, and shared domain types can evolve independently while retaining a single reproducible build surface.

```text
Nessy/
├── crates/
│   ├── bowser-core/       # shared domain types and errors
│   ├── turtle/            # orchestration/session state
│   ├── koopa/             # execution/sandbox abstractions
│   └── bowserd/            # executable daemon
├── docs/
├── .github/workflows/
└── Cargo.toml
```

## Development

Requirements: Rust stable and `just` (or invoke the equivalent Cargo commands directly).

```bash
just check
just test
just fmt
just clippy
```

## Design principles

- Local-first execution and explicit provider boundaries.
- Strongly typed protocol contracts instead of stringly typed orchestration.
- No credentials in source control.
- Deterministic builds and machine-verifiable CI gates.
- Security-sensitive operations are represented explicitly in the domain model.
- Optional distributed components remain behind replaceable traits.

## License

BowserAI/Nessy is released under the Apache License, Version 2.0. See `LICENSE` and `NOTICE` for details.
