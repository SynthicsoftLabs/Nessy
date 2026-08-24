<!--
Copyright 2026 Synthicsoft Labs LLC
Licensed under the Apache License, Version 2.0.
-->
# Contributing

## Workflow

Changes are produced and validated by automation. The repository's CI gates are authoritative for formatting, compilation, tests, linting, dependency advisories, licensing, and SBOM generation.

1. Automation creates a focused change.
2. Automation validates the complete workspace.
3. Failed gates block promotion and may trigger automated repair.
4. Successful changes may be promoted according to repository automation policy.
5. Credentials and generated secrets are never committed.

## Code standards

Rust code is formatted with `rustfmt` and must pass Clippy with warnings denied. Public interfaces should have explicit ownership, error, and lifecycle semantics.

## Security

Privileged execution remains deny-by-default. Network access, credentials, filesystem access, and external providers must remain behind explicit capability boundaries.
