#!/usr/bin/env bash
set -euo pipefail

SCRIPT="scripts/nessy-autonomous-engineer.sh"

assert_contains() {
  local needle="$1"
  grep -Fq "$needle" "$SCRIPT"
}

assert_contains 'if [[ -n "${LLM7_API_KEY:-}" ]]; then'
assert_contains 'if [[ "$ENGINE_SUCCESS" -eq 0 && -n "${POLLINATIONS_API_KEY:-}" ]]; then'
assert_contains 'curl -fsSL https://ollama.com/install.sh | sh'
assert_contains 'ollama pull "$model"'
assert_contains 'ollama_chat/${LOCAL_MODEL}'
assert_contains 'smollm2:360m-instruct-q4_0'
assert_contains 'No autonomous coding provider completed successfully.'

bash -n "$SCRIPT"

echo "autonomous provider fallback assertions passed"
