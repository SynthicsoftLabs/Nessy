#!/usr/bin/env bash
# Copyright 2026 Synthicsoft Labs LLC
# Licensed under the Apache License, Version 2.0.
set -euo pipefail

EXPECTED="${GITHUB_SHA:?GITHUB_SHA is required}"
REPO="${GITHUB_REPOSITORY:?GITHUB_REPOSITORY is required}"

actual="$(git rev-parse HEAD)"
test "$actual" = "$EXPECTED"

tree="$(git rev-parse HEAD^{tree})"
test -n "$tree"

git fsck --full --strict

git diff --exit-code

git diff --cached --exit-code

git ls-remote origin "refs/heads/${GITHUB_REF_NAME}" | awk '{print $1}' | grep -Fx "$EXPECTED"

api_sha="$(curl --fail --silent --show-error --retry 5 --retry-delay 2 \
  -H 'Accept: application/vnd.github+json' \
  -H 'X-GitHub-Api-Version: 2022-11-28' \
  "https://api.github.com/repos/${REPO}/commits/${EXPECTED}" | python -c 'import json,sys; print(json.load(sys.stdin)["sha"])')"
test "$api_sha" = "$EXPECTED"

echo "INTERPOSITION CHECK PASSED"
echo "commit=$EXPECTED"
echo "tree=$tree"
