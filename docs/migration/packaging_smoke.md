# Packaging Smoke Report (TC-PKG-01)

- Task ID: `TC-PKG-01`
- Date: `2026-05-14`

## Scope

Define and document debug/release/Tauri packaging smoke validation for the unified repository.

## Source Verification Findings

- `src-tauri/` is not present in the unified repository.
- `src-tauri/tauri.conf.*` is not present.
- `src-tauri/capabilities/` is not present.

Result: Tauri packaging smoke is currently **blocked by missing app shell inputs**.

## Test-First Artifacts Added

- `tests/baseline/packaging_smoke_artifacts.rs`
  - asserts packaging smoke report exists
  - asserts `scripts/tauri_smoke.sh` exists and carries explicit blocked signaling

## Script Updates

- `scripts/tauri_smoke.sh` now:
  - returns `BLOCKED` when `src-tauri` or `tauri.conf.*` are missing
  - returns `BLOCKED` when `src-tauri/capabilities` is missing
  - keeps execution non-destructive and documentation-focused

## Pending User-Run Verification

```bash
cd /home/pc/Downloads/inf/plan/license-control-suite && cargo test
cd /home/pc/Downloads/inf/plan/license-control-suite && cargo build --release
cd /home/pc/Downloads/inf/plan/license-control-suite && bash scripts/tauri_smoke.sh
```

## Expected Success Criteria

- `cargo test` completes successfully.
- `cargo build --release` completes successfully.
- `scripts/tauri_smoke.sh` either:
  - reports shell/capabilities detected (if source appears), or
  - exits with explicit `BLOCKED` status that documents missing inputs.

## Known Unresolved Issues

- Tauri packaging smoke cannot fully execute until verified `src-tauri` shell/capability inputs are present.
- Keyring packaging validation remains pending user-run, platform-dependent execution.
