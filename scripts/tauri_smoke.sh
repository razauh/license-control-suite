#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "${repo_root}"

if [[ ! -d "src-tauri" ]] || ! find "src-tauri" -maxdepth 2 -type f -name "tauri.conf.*" | grep -q .; then
  echo "[tauri_smoke] BLOCKED: no verified Tauri app shell exists yet (missing src-tauri and/or tauri.conf.*)."
  exit 2
fi

echo "[tauri_smoke] Tauri shell files detected."

if [[ ! -d "src-tauri/capabilities" ]]; then
  echo "[tauri_smoke] BLOCKED: src-tauri/capabilities is missing."
  exit 2
fi

echo "[tauri_smoke] Capability directory detected."
echo "[tauri_smoke] NOTE: execute 'cargo tauri build --debug' and keyring manual smoke outside this script."
