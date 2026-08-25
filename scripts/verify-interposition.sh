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

if [[ "${GITHUB_EVENT_NAME:-}" == "pull_request" ]]; then
  head_sha="$(python - <<'PY'
import json, os
with open(os.environ['GITHUB_EVENT_PATH'], encoding='utf-8') as f:
    event = json.load(f)
print(event['pull_request']['head']['sha'])
PY
)"
  test -n "$head_sha"
  git cat-file -e "${head_sha}^{commit}"
  git merge-base --is-ancestor "$head_sha" "$EXPECTED"
else
  branch="${GITHUB_REF_NAME:-}"
  test -n "$branch"
  remote_sha="$(git ls-remote origin "refs/heads/${branch}" | awk '{print $1}')"
  test "$remote_sha" = "$EXPECTED"
fi

api_sha="$(curl --fail --silent --show-error --retry 5 --retry-delay 2 \
  -H 'Accept: application/vnd.github+json' \
  -H 'X-GitHub-Api-Version: 2022-11-28' \
  "https://api.github.com/repos/${REPO}/commits/${EXPECTED}" | python -c 'import json,sys; print(json.load(sys.stdin)["sha"])')"
test "$api_sha" = "$EXPECTED"

echo "INTERPOSITION CHECK PASSED"
echo "commit=$EXPECTED"
echo "tree=$tree"
