# TC-00 Graphify Inventory

## Task

- Task ID: `TC-00`
- Title: `Evidence Baseline and Source Inventory`
- Date: `2026-05-14`

## Graphify Artifacts Verified

- `/home/pc/Downloads/inf/plan/graphify-out/graph.json`
- `/home/pc/Downloads/inf/plan/graphify-out/GRAPH_REPORT.md`

## Combined Graph Baseline (recorded)

- Node count: `657`
- Edge count: `1137`
- Source:
  - `GRAPH_REPORT.md` summary line (`657 nodes · 1137 edges`)
  - `graph.json` structural count check (`"id"` entries = `657`, `"source"` entries = `1137`)

## Per-Repository Graph Sizes (source-of-truth)

From `docs/merged_crate_compatibility_report.md` and `docs/unified_crate_merge_plan.md`:

- `admin-dashboard`: `92 nodes`, `96 edges`
- `auth-core`: `104 nodes`, `116 edges`
- `shared-contracts`: `88 nodes`, `95 edges`
- `user-reg`: `405 nodes`, `854 edges`

## TDD Inventory Checks Added

- Script: `docs/baseline/check_source_inventory.sh`
- Check coverage:
  - verifies all four legacy roots exist,
  - verifies `graphify-out/graph.json` exists,
  - verifies combined graph counts (`657` nodes, `1137` edges),
  - verifies `user-reg` workspace members exist by `Cargo.toml` presence.

## Notes

- TC-00 requires evidence capture only. No migration/copy implementation was performed.
