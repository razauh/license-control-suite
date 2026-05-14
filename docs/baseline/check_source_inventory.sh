#!/usr/bin/env bash
set -euo pipefail

roots=(
  "/home/pc/Downloads/inf/plan/shared-contracts"
  "/home/pc/Downloads/inf/plan/admin-dashboard"
  "/home/pc/Downloads/inf/plan/auth-core"
  "/home/pc/Downloads/inf/plan/user-reg"
)

for root in "${roots[@]}"; do
  if [[ ! -d "${root}" ]]; then
    echo "missing legacy root: ${root}" >&2
    exit 1
  fi
done

graph="/home/pc/Downloads/inf/plan/graphify-out/graph.json"
if [[ ! -f "${graph}" ]]; then
  echo "missing graphify graph: ${graph}" >&2
  exit 1
fi

node_count="$(rg -c '"id"\s*:' "${graph}")"
edge_count="$(rg -c '"source"\s*:' "${graph}")"

if [[ "${node_count}" != "657" ]]; then
  echo "unexpected node count: got ${node_count}, expected 657" >&2
  exit 1
fi

if [[ "${edge_count}" != "1137" ]]; then
  echo "unexpected edge count: got ${edge_count}, expected 1137" >&2
  exit 1
fi

if [[ ! -f "/home/pc/Downloads/inf/plan/user-reg/Cargo.toml" ]]; then
  echo "missing user-reg workspace manifest" >&2
  exit 1
fi

if [[ ! -f "/home/pc/Downloads/inf/plan/user-reg/crates/auth-licensing-core/Cargo.toml" ]]; then
  echo "missing user-reg member: auth-licensing-core" >&2
  exit 1
fi

if [[ ! -f "/home/pc/Downloads/inf/plan/user-reg/crates/auth-licensing-tauri/Cargo.toml" ]]; then
  echo "missing user-reg member: auth-licensing-tauri" >&2
  exit 1
fi

if [[ ! -f "/home/pc/Downloads/inf/plan/user-reg/workers/licensing-worker/Cargo.toml" ]]; then
  echo "missing user-reg member: licensing-worker" >&2
  exit 1
fi

echo "TC-00 baseline inventory checks passed."
