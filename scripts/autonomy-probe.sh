#!/usr/bin/env bash
# Copyright 2026 Adam Joseph Rivers, CEO, Synthicsoft Labs LLC.
# Licensed under the Apache License, Version 2.0.
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo_root"

required=(
  '.github/workflows/nessy-autonomous-engineer.yml'
  '.github/workflows/nessy-autonomous-supervisor.yml'
  '.github/workflows/interposition.yml'
  'scripts/verify-interposition.sh'
  'AUTOMATION.md'
)

for path in "${required[@]}"; do
  test -s "$path" || { echo "missing required autonomy artifact: $path" >&2; exit 1; }
done

grep -q 'workflow_dispatch:' .github/workflows/nessy-autonomous-engineer.yml
grep -q 'workflow_run:' .github/workflows/nessy-autonomous-supervisor.yml
grep -q 'actions: write' .github/workflows/nessy-autonomous-supervisor.yml
grep -q 'copilot-requests: write' .github/workflows/nessy-autonomous-engineer.yml
grep -q 'INTERPOSITION CHECK PASSED' scripts/verify-interposition.sh
grep -q 'Human-free operation' AUTOMATION.md

printf '%s\n' 'AUTONOMY PROBE PASSED'
printf 'head=%s\n' "$(git rev-parse HEAD)"
printf 'tree=%s\n' "$(git rev-parse HEAD^{tree})"
