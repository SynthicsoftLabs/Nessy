<!--
Copyright 2026 Synthicsoft Labs LLC
Licensed under the Apache License, Version 2.0; and MIT License.
-->
# Contributing

## Workflow

1. Create a focused branch from `main`.
2. Keep changes scoped to one architectural concern.
3. Run `just all` before opening a pull request.
4. Include tests for changed behavior and update architectural documentation when boundaries change.
5. Never commit credentials or generated secrets.

## Code standards

Rust code is formatted with `rustfmt` and must pass Clippy with warnings denied. Public interfaces should have explicit ownership, error, and lifecycle semantics.

## Pull requests

Describe the architectural change, security implications, test commands, and any compatibility impact. CI is authoritative for formatting, compilation, tests, linting, dependency advisories, and SBOM generation.
