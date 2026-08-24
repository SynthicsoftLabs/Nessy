# Copyright 2026 Synthicsoft Labs LLC
# Licensed under Apache-2.0 OR MIT.

check:
    cargo check --workspace --all-targets

test:
    cargo test --workspace --all-targets

fmt:
    cargo fmt --all -- --check

clippy:
    cargo clippy --workspace --all-targets --all-features -- -D warnings

build:
    cargo build --workspace --release

security:
    cargo audit
    cargo deny check

all: fmt check test clippy
