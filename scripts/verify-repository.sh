#!/usr/bin/env bash
# Copyright 2026 Synthicsoft Labs LLC
# Licensed under the Apache License, Version 2.0.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"
command -v cargo >/dev/null || { echo "cargo is required" >&2; exit 1; }

echo "[1/9] repository status"
git diff --exit-code
git diff --cached --exit-code

echo "[2/9] checkout identity"
if [[ -n "${GITHUB_SHA:-}" ]]; then test "$GITHUB_SHA" = "$(git rev-parse HEAD)"; fi

echo "[3/9] unresolved merge markers"
! grep -RInE '^(<<<<<<<|=======|>>>>>>>)' --exclude-dir=.git .

echo "[4/9] workspace metadata"
cargo metadata --no-deps --format-version 1 > /tmp/nessy-metadata.json
python3 - <<'PY'
import json
with open('/tmp/nessy-metadata.json', encoding='utf-8') as f:
    data = json.load(f)
packages = data.get('packages', [])
assert packages, 'workspace metadata contains no packages'
bad = [(p['name'], p.get('license')) for p in packages if p.get('license') != 'Apache-2.0']
assert not bad, f'non-Apache project package licenses: {bad}'
PY

echo "[5/9] formatting"
cargo fmt --all -- --check

echo "[6/9] dependency graph"
cargo metadata --format-version 1 >/dev/null

echo "[7/9] workspace check"
cargo check --workspace --all-targets

echo "[8/9] workspace tests"
cargo test --workspace --all-targets

echo "[9/9] clippy"
cargo clippy --workspace --all-targets --all-features -- -D warnings

echo "Repository verification complete."
