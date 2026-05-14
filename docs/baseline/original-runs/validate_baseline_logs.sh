#!/usr/bin/env bash
set -euo pipefail

base_dir="/home/pc/Downloads/inf/plan/license-control-suite/docs/baseline/original-runs"
required_logs=(
  "shared-contracts.log"
  "admin-dashboard.log"
  "auth-core.log"
  "user-reg.log"
)

required_fields=(
  "Command:"
  "Working Directory:"
  "Exit Code:"
  "Timestamp:"
)

for log in "${required_logs[@]}"; do
  path="${base_dir}/${log}"
  if [[ ! -f "${path}" ]]; then
    echo "missing baseline log: ${path}" >&2
    exit 1
  fi

  for field in "${required_fields[@]}"; do
    if ! rg -q "^${field}" "${path}"; then
      echo "missing field '${field}' in ${path}" >&2
      exit 1
    fi
  done
done

echo "TC-01 baseline log structure checks passed."
