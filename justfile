# Copyright 2026 Synthicsoft Labs LLC
# Licensed under the Apache License, Version 2.0.

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

license:
    @! grep -RInE 'Apache-2.0 OR MIT|Apache License, Version 2.0; and MIT|dual Apache|Apache-2.0/MIT|and/or the MIT License' --exclude-dir=.git --exclude='ci.yml' --exclude='integrity.yml' --exclude='verify-repository.sh' --exclude='justfile' --exclude='LICENSE' .

all: fmt check test clippy security license
