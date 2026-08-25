#!/usr/bin/env bash
set -euo pipefail

: "${GH_TOKEN:?GH_TOKEN is required}"
: "${GH_REPO:?GH_REPO is required}"
EVENT_NAME="${GITHUB_EVENT_NAME:-workflow_dispatch}"
EVENT_PATH="${GITHUB_EVENT_PATH:?GITHUB_EVENT_PATH is required}"
AUTOMATION_BRANCH="${NESSY_AUTOMATION_BRANCH:-automation/nessy-autonomous}"
TARGET_REF="main"
ISSUE_NUMBER=""
OBJECTIVE=""
START_SHA="$(git rev-parse HEAD)"

case "$EVENT_NAME" in
  push)
    TARGET_REF="main"
    OBJECTIVE="Run the complete autonomous repository development cycle against commit ${GITHUB_SHA}. Inspect the complete repository, repair every concrete defect found, and implement the highest-value missing capability supported by repository evidence." ;;
  schedule|workflow_dispatch)
    TARGET_REF="main"
    OBJECTIVE="Run the complete autonomous Nessy self-development cycle. Inspect the complete repository and use docs/SELF_DEVELOPMENT.md to select and execute the highest-value atomic objective." ;;
  *)
    TARGET_REF="main"
    OBJECTIVE="Run the complete autonomous Nessy engineering and health cycle from the current main state." ;;
esac

OPEN_PR="$(gh pr list --repo "$GH_REPO" --head "$AUTOMATION_BRANCH" --base main --state open --limit 1 --json number --jq '.[0].number // empty')"
if [[ -n "$OPEN_PR" ]]; then
  echo "Autonomous PR #$OPEN_PR already owns $AUTOMATION_BRANCH; leaving the existing cycle authoritative."
  exit 0
fi

git fetch origin main --prune
REMOTE_MAIN="$(git rev-parse origin/main)"
if [[ "$REMOTE_MAIN" != "$START_SHA" ]]; then
  git reset --hard origin/main
  START_SHA="$REMOTE_MAIN"
fi

gh api --method DELETE "/repos/$GH_REPO/git/refs/heads/$AUTOMATION_BRANCH" >/dev/null 2>&1 || true
git branch -D "$AUTOMATION_BRANCH" >/dev/null 2>&1 || true
git switch -c "$AUTOMATION_BRANCH"

if [[ -z "$ISSUE_NUMBER" ]]; then
  ISSUE_NUMBER="$(gh issue list --repo "$GH_REPO" --state open --limit 20 --json number,title --jq '[.[] | select(.title | startswith("Autonomous")) | .number] | .[-1] // empty')"
fi
if [[ -n "$ISSUE_NUMBER" ]]; then
  gh issue comment "$ISSUE_NUMBER" --repo "$GH_REPO" --body "Nessy Autonomous Control Plane accepted '$EVENT_NAME' for main commit '$START_SHA'. Machine engineering cycle started on '$AUTOMATION_BRANCH'."
fi

cat > /tmp/nessy-context.md <<EOF
# Nessy Autonomous Engineering Context

Event: $EVENT_NAME
Target ref: $TARGET_REF
Working branch: $AUTOMATION_BRANCH
Start SHA: $START_SHA
Objective: $OBJECTIVE

## Recent history
$(git log --oneline --graph -n 20)

## Current status
$(git status --short)

## Open pull requests
$(gh pr list --repo "$GH_REPO" --state open --limit 30 --json number,title,headRefName,baseRefName,mergeStateStatus,url)

## Recent workflow runs
$(gh run list --repo "$GH_REPO" --limit 30 --json databaseId,name,status,conclusion,headBranch,headSha,event,createdAt)
EOF
printf '\n## Self-development policy\n\n' >> /tmp/nessy-context.md
cat docs/SELF_DEVELOPMENT.md >> /tmp/nessy-context.md

if [[ -n "$ISSUE_NUMBER" ]]; then
  gh issue view "$ISSUE_NUMBER" --repo "$GH_REPO" --json number,title,body,url > /tmp/nessy-issue.json
else
  printf '%s\n' '{"number":null,"title":"autonomous-maintenance","body":""}' > /tmp/nessy-issue.json
fi

cat > /tmp/nessy-aider-prompt.md <<EOF
You are Nessy Autonomous Engineer operating inside $GH_REPO.

Execute the complete repository engineering and self-development cycle without requesting human input.

OBJECTIVE:
$OBJECTIVE

Read /tmp/nessy-context.md, /tmp/nessy-issue.json, README.md, AUTOMATION.md, CONTRIBUTING.md, docs/SELF_DEVELOPMENT.md, affected source, tests, workflows, and git blame before editing.

Required cycle:
1. Establish the exact objective from repository evidence.
2. Trace callers, dependency edges, workflow consumers, public interfaces, and documentation consumers.
3. Reproduce observed failures before fixing them; add focused regression or acceptance coverage where practical.
4. Research relevant current project/model/provider capabilities when warranted and normalize useful findings into Nessy's registries.
5. Implement the complete root-cause repair or feature increment. No TODOs, placeholders, dead commented code, hardcoded secrets, speculative rewrites, or unrelated changes.
6. Synchronize README, canonical documentation, capability matrices, runtime registries, model registries, and legal/attribution records when affected.
7. Run the complete validation matrix: cargo fmt --all -- --check; cargo check --workspace --all-targets; cargo test --workspace --all-targets; cargo clippy --workspace --all-targets --all-features -- -D warnings; cargo audit; cargo deny check; relevant Node syntax/tests; and repository project scripts.
8. On any validation failure, inspect the actual error and repair the cause, then rerun the affected gate and the complete matrix.
9. Inspect the final diff for atomicity, idempotency, documentation synchronization, merge markers, credentials, unrelated edits, generated artifacts, and accidental API drift.
10. Do not commit or push; the outer executor owns exactly one commit and promotion.
11. Leave the working tree containing the complete validated implementation.

Do not ask for confirmation. Continue to the next configured mechanism if an optional provider or tool is unavailable. Do not claim success without evidence.
EOF

run_aider() {
  local base_url="$1"
  local api_key="$2"
  local model="$3"
  local label="$4"
  local log="/tmp/nessy-aider-${label}.log"
  echo "Nessy provider attempt: $label"
  set +e
  OPENAI_API_KEY="$api_key" aider \
    --openai-api-base "$base_url" \
    --model "$model" \
    --message-file /tmp/nessy-aider-prompt.md \
    --yes-always \
    --no-auto-commits \
    --no-stream \
    --no-show-model-warnings \
    --no-gitignore >"$log" 2>&1
  local status=$?
  set -e
  cat "$log"
  if [[ "$status" -ne 0 ]]; then return 1; fi
  if grep -Eqi 'AuthenticationError|api_key.*must be set|Traceback \(most recent call last\)|Process completed with exit code [1-9]' "$log"; then return 1; fi
  return 0
}

ENGINE_SUCCESS=0

if [[ -n "${LLM7_API_KEY:-}" ]]; then
  if run_aider "${NESSY_LLM7_BASE_URL:-https://api.llm7.io/v1}" "$LLM7_API_KEY" "${NESSY_AIDER_MODEL:-openai/default}" "LLM7"; then
    ENGINE_SUCCESS=1
  else
    echo "LLM7 failed; routing to the next configured provider." >&2
  fi
else
  echo "LLM7 credential unavailable; routing directly to the keyless fallback chain."
fi

if [[ "$ENGINE_SUCCESS" -eq 0 && -n "${POLLINATIONS_API_KEY:-}" ]]; then
  if run_aider "${NESSY_POLLINATIONS_BASE_URL:-https://gen.pollinations.ai/v1}" "$POLLINATIONS_API_KEY" "${NESSY_POLLINATIONS_MODEL:-openai/default}" "Pollinations"; then
    ENGINE_SUCCESS=1
  else
    echo "Pollinations failed; routing to the local provider." >&2
  fi
fi

start_local_ollama() {
  local model="${NESSY_LOCAL_MODEL:-qwen2.5-coder:0.5b}"
  local context_length="${NESSY_OLLAMA_CONTEXT_LENGTH:-32768}"

  if ! command -v ollama >/dev/null 2>&1; then
    echo "Installing repository-selected Ollama runtime for keyless local inference."
    curl -fsSL https://ollama.com/install.sh | sh
  fi

  if ! pgrep -x ollama >/dev/null 2>&1; then
    echo "Starting local Ollama service."
    OLLAMA_CONTEXT_LENGTH="$context_length" nohup ollama serve >/tmp/nessy-ollama.log 2>&1 &
  fi

  for _ in $(seq 1 60); do
    if curl -fsS http://127.0.0.1:11434/api/tags >/dev/null 2>&1; then
      break
    fi
    sleep 1
  done

  curl -fsS http://127.0.0.1:11434/api/tags >/dev/null
  echo "Pulling keyless local coding model: $model"
  ollama pull "$model"

  export OLLAMA_API_BASE="${NESSY_OLLAMA_API_BASE:-http://127.0.0.1:11434}"
  export OLLAMA_CONTEXT_LENGTH="$context_length"
  LOCAL_MODEL="$model"
}

if [[ "$ENGINE_SUCCESS" -eq 0 ]]; then
  if start_local_ollama; then
    if run_aider "${NESSY_OLLAMA_API_BASE:-http://127.0.0.1:11434}" "ollama" "ollama_chat/${LOCAL_MODEL}" "Ollama-${LOCAL_MODEL//[:\/]/-}"; then
      ENGINE_SUCCESS=1
    else
      echo "Primary local model failed; trying the compact SmolLM2 fallback." >&2
      if [[ "$LOCAL_MODEL" != "smollm2:360m-instruct-q4_0" ]]; then
        ollama pull smollm2:360m-instruct-q4_0
        if run_aider "${NESSY_OLLAMA_API_BASE:-http://127.0.0.1:11434}" "ollama" "ollama_chat/smollm2:360m-instruct-q4_0" "Ollama-Smollm2"; then
          ENGINE_SUCCESS=1
        fi
      fi
    fi
  fi
fi

if [[ "$ENGINE_SUCCESS" -eq 0 ]]; then
  echo "No autonomous coding provider completed successfully." >&2
  exit 1
fi

git diff --check
cargo fmt --all -- --check
cargo check --workspace --all-targets
cargo test --workspace --all-targets
cargo clippy --workspace --all-targets --all-features -- -D warnings
command -v cargo-audit >/dev/null || cargo install cargo-audit --locked
command -v cargo-deny >/dev/null || cargo install cargo-deny --locked
cargo audit
cargo deny check
[ ! -f docs/chat/app.js ] || node --check docs/chat/app.js
[ ! -f tests/chat-response-quality.test.js ] || node tests/chat-response-quality.test.js
[ ! -f tests/chat-engineering.test.js ] || node tests/chat-engineering.test.js
[ ! -f tests/chat-model-routing.test.js ] || node tests/chat-model-routing.test.js
[ ! -f tests/autonomous-provider-fallback.test.sh ] || bash tests/autonomous-provider-fallback.test.sh

if [[ -z "$(git status --porcelain)" ]]; then
  echo "Autonomous cycle produced no repository changes."
  exit 0
fi

git fetch origin main --prune
LATEST_MAIN="$(git rev-parse origin/main)"
CURRENT_BASE="$(git merge-base HEAD origin/main)"
if [[ "$CURRENT_BASE" != "$LATEST_MAIN" ]]; then
  git rebase origin/main
fi

git diff --check
git status --short
git config user.name 'Nessy Autonomous Engineer'
git config user.email 'nessy-autonomous-engineer@users.noreply.github.com'
git add -A
git commit -m "chore(automation): apply autonomous repository cycle"
git push --set-upstream origin "$AUTOMATION_BRANCH"

PR_NUMBER="$(gh pr list --repo "$GH_REPO" --head "$AUTOMATION_BRANCH" --base main --state open --limit 1 --json number --jq '.[0].number // empty')"
if [[ -z "$PR_NUMBER" ]]; then
  PR_URL="$(gh pr create --repo "$GH_REPO" --head "$AUTOMATION_BRANCH" --base main --title "$(git log -1 --format=%s)" --body "Autonomous Nessy engineering cycle for: $OBJECTIVE")"
  PR_NUMBER="${PR_URL##*/}"
fi

gh pr merge "$PR_NUMBER" --repo "$GH_REPO" --auto --squash --delete-branch || \
gh pr merge "$PR_NUMBER" --repo "$GH_REPO" --squash --delete-branch

PROMOTED_SHA="$(gh pr view "$PR_NUMBER" --repo "$GH_REPO" --json mergeCommit --jq '.mergeCommit.oid // empty')"
if [[ -z "$PROMOTED_SHA" ]]; then
  echo "PR #$PR_NUMBER did not expose a merge commit yet." >&2
  exit 1
fi

GITHUB_SHA="$PROMOTED_SHA" GITHUB_REF_NAME="main" GITHUB_EVENT_NAME="push" GITHUB_REPOSITORY="$GH_REPO" bash scripts/verify-interposition.sh

if [[ -n "$ISSUE_NUMBER" ]]; then
  gh issue comment "$ISSUE_NUMBER" --repo "$GH_REPO" --body "Nessy autonomous cycle completed and promoted PR #$PR_NUMBER at commit '$PROMOTED_SHA'."
fi

echo "Nessy autonomous engineering cycle complete: branch=$AUTOMATION_BRANCH pr=$PR_NUMBER promoted=$PROMOTED_SHA"
