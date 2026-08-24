#!/usr/bin/env bash
# Copyright 2026 Synthicsoft Labs LLC
# Licensed under the Apache License, Version 2.0.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
PREFIX="${BOWSER_PREFIX:-${HOME}/.bowser}"
BIN_DIR="${PREFIX}/bin"
CONFIG_DIR="${PREFIX}/config"
STATE_DIR="${PREFIX}/state"
CACHE_DIR="${PREFIX}/cache"
LOG_DIR="${PREFIX}/log"
SECRETS_DIR="${PREFIX}/secrets"

mkdir -p "${BIN_DIR}" "${CONFIG_DIR}" "${STATE_DIR}" "${CACHE_DIR}" "${LOG_DIR}" "${SECRETS_DIR}"
chmod 700 "${PREFIX}" "${SECRETS_DIR}"

if ! command -v cargo >/dev/null 2>&1; then
  echo "cargo is required to build BowserAI" >&2
  exit 1
fi

export CARGO_TERM_COLOR=always
export RUST_BACKTRACE=1

cargo build --release --workspace --manifest-path "${ROOT}/Cargo.toml"
cp "${ROOT}/target/release/bowserd" "${BIN_DIR}/bowserd"
chmod 755 "${BIN_DIR}/bowserd"

cat > "${CONFIG_DIR}/environment" <<EOF
BOWSER_HOME=${PREFIX}
BOWSER_CONFIG_DIR=${CONFIG_DIR}
BOWSER_STATE_DIR=${STATE_DIR}
BOWSER_CACHE_DIR=${CACHE_DIR}
BOWSER_LOG_DIR=${LOG_DIR}
BOWSER_SECRETS_DIR=${SECRETS_DIR}
KAIROS_URL=${KAIROS_URL:-https://the-real-kairos.com}
EOF
chmod 600 "${CONFIG_DIR}/environment"

if command -v systemctl >/dev/null 2>&1 && systemctl --user status >/dev/null 2>&1; then
  mkdir -p "${HOME}/.config/systemd/user"
  cat > "${HOME}/.config/systemd/user/bowserd.service" <<EOF
[Unit]
Description=BowserAI Autonomous Runtime
After=network-online.target
Wants=network-online.target

[Service]
Type=simple
EnvironmentFile=${CONFIG_DIR}/environment
ExecStart=${BIN_DIR}/bowserd
Restart=always
RestartSec=3
NoNewPrivileges=true
PrivateTmp=true
ProtectSystem=strict
ProtectHome=read-only
ReadWritePaths=${PREFIX}

[Install]
WantedBy=default.target
EOF
  systemctl --user daemon-reload
  systemctl --user enable --now bowserd.service
  echo "BowserAI autonomous daemon enabled and started."
else
  echo "BowserAI installed at ${BIN_DIR}/bowserd."
  echo "No user service manager was detected; use the platform service adapter supplied by the deployment target."
fi
