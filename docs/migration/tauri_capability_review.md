# Tauri Capability Review (TC-TAURI-02)

- Task ID: `TC-TAURI-02`
- Date: `2026-05-14`

## Source Verification Findings

Inspected unified repository for final app shell files:

- `src-tauri/tauri.conf.json`
- `src-tauri/capabilities/*.json`
- setup hooks / plugin init / window/menu/tray lifecycle files

Current result:

- `src-tauri/` directory is not present.
- `tauri.conf.json` is not present.
- capability JSON files are not present.
- no final app lifecycle hooks were confirmed in a Tauri shell path.

Status: **Blocked (Requires source verification)**, aligned with PI-2 in source-of-truth docs.

## Test-First Artifacts Added

- `scripts/check_tauri_capabilities.sh`
  - fails with explicit `BLOCKED` status if shell/config/capability files are missing
  - checks six command names when capability JSON files exist
- `tests/ipc/tauri_capability_smoke.rs`
  - preserves six-command expectation for future capability mapping

## Risk Notes

- Command exposure permissions cannot be verified until final shell and capability files exist.
- This task intentionally does not invent capability schema without verified shell inputs.

## Unresolved Issues

- Tauri config/capability verification remains blocked pending source availability.
- Build verification (`cargo check` / `cargo tauri build --debug`) remains pending user-run execution.

## Pending User-Run Verification Commands

```bash
cd /home/pc/Downloads/inf/plan/license-control-suite && bash scripts/check_tauri_capabilities.sh
cd /home/pc/Downloads/inf/plan/license-control-suite && cargo check
# if src-tauri shell and tauri CLI become available:
cd /home/pc/Downloads/inf/plan/license-control-suite && cargo tauri build --debug
```
