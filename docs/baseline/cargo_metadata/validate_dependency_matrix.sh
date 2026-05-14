#!/usr/bin/env bash
set -euo pipefail

matrix="/home/pc/Downloads/inf/plan/license-control-suite/docs/baseline/dependency_matrix.md"
meta_dir="/home/pc/Downloads/inf/plan/license-control-suite/docs/baseline/cargo_metadata"

required_matrix_lines=(
  "shared-contracts/Cargo.toml"
  "admin-dashboard/Cargo.toml"
  "auth-core/Cargo.toml"
  "user-reg/Cargo.toml"
  "user-reg/crates/auth-licensing-core/Cargo.toml"
  "user-reg/crates/auth-licensing-tauri/Cargo.toml"
  "user-reg/workers/licensing-worker/Cargo.toml"
  "thiserror = \"1\""
  "thiserror = \"2\""
  "tauri = \"2\""
  "shared-contracts = { git = \"ssh://git@github.com/razauh/shared-contracts.git\" }"
)

for line in "${required_matrix_lines[@]}"; do
  if ! rg -F -q "${line}" "${matrix}"; then
    echo "dependency matrix missing required fact: ${line}" >&2
    exit 1
  fi
done

required_meta_files=(
  "shared-contracts.metadata.json"
  "admin-dashboard.metadata.json"
  "auth-core.metadata.json"
  "user-reg.metadata.json"
)

for file in "${required_meta_files[@]}"; do
  path="${meta_dir}/${file}"
  if [[ ! -f "${path}" ]]; then
    echo "missing metadata file: ${path}" >&2
    exit 1
  fi
done

echo "TC-02 dependency matrix structure checks passed."
