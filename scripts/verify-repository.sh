#!/usr/bin/env bash
# Copyright 2026 Synthicsoft Labs LLC
# Licensed under the Apache License, Version 2.0.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

command -v cargo >/dev/null || { echo "cargo is required" >&2; exit 1; }

echo "[1/8] repository status"
git diff --exit-code
git diff --cached --exit-code

echo "[2/8] unresolved merge markers"
if grep -RInE '^(<<<<<<<|=======|>>>>>>>)' --exclude-dir=.git .; then exit 1; fi

echo "[3/8] formatting"
cargo fmt --all -- --check

echo "[4/8] dependency graph"
cargo metadata --format-version 1 >/dev/null

echo "[5/8] workspace check"
cargo check --workspace --all-targets
echo "[6/8] workspace tests"
cargo test --workspace --all-targets
echo "[7/8] clippy"
cargo clippy --workspace --all-targets --all-features -- -D warnings

echo "[8/8] project license"
grep -q '^license = "Apache-2.0"' Cargo.toml
grep -q 'license.workspace = true' crates/bowserd/Cargo.toml
if grep -RInE 'Apache-2.0 OR MIT|Apache License, Version 2.0; and MIT|dual Apache|Apache-2.0/MIT|and/or the MIT License' --exclude-dir=.git --exclude='ci.yml' --exclude='integrity.yml' --exclude='verify-repository.sh' --exclude='justfile' --exclude='LICENSE' .; then exit 1; fi

echo "Repository verification complete."
