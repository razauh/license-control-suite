#!/usr/bin/env bash
set -euo pipefail

doc="/home/pc/Downloads/inf/plan/license-control-suite/docs/baseline/tauri_command_inventory.md"
fe_doc="/home/pc/Downloads/inf/plan/license-control-suite/docs/baseline/frontend_asset_inventory.md"

search_file() {
  local pattern="$1"
  local file="$2"
  if command -v rg >/dev/null 2>&1; then
    rg -q --fixed-strings "${pattern}" "${file}"
  else
    grep -Fq "${pattern}" "${file}"
  fi
}

required_commands=(
  "activate_license"
  "validate_session"
  "request_device_reset"
  "get_device_reset_status"
  "clear_local_session"
  "get_auth_state"
)

for cmd in "${required_commands[@]}"; do
  if ! search_file "\`${cmd}\`" "${doc}"; then
    echo "missing command in inventory: ${cmd}" >&2
    exit 1
  fi
done

required_markers=(
  "session_state.json"
  "reset_status.json"
  "license_key"
  "access_token"
  "device_keypair"
)

for marker in "${required_markers[@]}"; do
  if ! search_file "\`${marker}\`" "${doc}"; then
    echo "missing persistence marker in tauri inventory: ${marker}" >&2
    exit 1
  fi
done

if ! search_file "No confirmed frontend app shell" "${fe_doc}"; then
  echo "frontend inventory missing source-verification status" >&2
  exit 1
fi

echo "TC-03 tauri/frontend inventory checks passed."
