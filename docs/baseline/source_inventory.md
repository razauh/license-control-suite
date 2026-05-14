# TC-00 Source Inventory

## Task

- Task ID: `TC-00`
- Title: `Evidence Baseline and Source Inventory`
- Date: `2026-05-14`

## Legacy Repository Roots (verified)

1. `/home/pc/Downloads/inf/plan/shared-contracts`
2. `/home/pc/Downloads/inf/plan/admin-dashboard`
3. `/home/pc/Downloads/inf/plan/auth-core`
4. `/home/pc/Downloads/inf/plan/user-reg`

## Repository Shape Summary

- `shared-contracts`: standalone Rust library crate
  - Root files: `.gitignore`, `.graphifyignore`, `Cargo.lock`, `Cargo.toml`, `README.md`, `TESTING.md`
  - Key directories: `src/`, `tests/` (`compatibility/`, `contract/`, `fixtures/`)
- `admin-dashboard`: standalone Rust library crate
  - Root files: `.gitignore`, `.graphifyignore`, `Cargo.lock`, `Cargo.toml`, `README.md`, `TESTING.md`
  - Key directories: `src/`, `tests/`
- `auth-core`: standalone Rust library crate
  - Root files: `.gitignore`, `.graphifyignore`, `Cargo.lock`, `Cargo.toml`, `README.md`, `TESTING.md`
  - Key directories: `src/`, `tests/` (`unit/`, `integration/`, `contract/`, `fixtures/`)
- `user-reg`: Cargo workspace root
  - Root files: `.gitignore`, `.graphifyignore`, `Cargo.lock`, `Cargo.toml`
  - Workspace members confirmed by manifests:
    - `crates/auth-licensing-core/Cargo.toml`
    - `crates/auth-licensing-tauri/Cargo.toml`
    - `workers/licensing-worker/Cargo.toml`
  - Key directories: `crates/`, `workers/`, `docs/`

## Required Inventory Inputs Confirmed

- Legacy `Cargo.toml` files: present in all required crate/workspace roots.
- Legacy `src/` directories: present in all four repository roots (workspace member `src/` for `user-reg`).
- Legacy `tests/` directories: present where expected in standalone crates and `user-reg` tauri crate.
- Legacy documentation files:
  - `README.md` and `TESTING.md` confirmed in `shared-contracts`, `admin-dashboard`, `auth-core`.
  - `user-reg` has `docs/` directory; root `README.md`/`TESTING.md` were not found.

## Source-Verification Findings (from plan/report + filesystem)

- No `build.rs` files found in the four legacy repositories.
- No frontend source directory (for example `frontend/` or `src-tauri/`) found in the analyzed repositories.
- Tauri integration source exists under `user-reg/crates/auth-licensing-tauri/`, but a full Tauri app shell/config was not confirmed in the analyzed roots.

## Notes

- No legacy source code was copied in TC-00.
- This file is evidence-only baseline documentation.

## Unresolved Issues

- User-run verification commands for TC-00 are pending confirmation.

## Pending User-Run Verification Commands

```bash
bash /home/pc/Downloads/inf/plan/license-control-suite/docs/baseline/check_source_inventory.sh
find /home/pc/Downloads/inf/plan/{shared-contracts,admin-dashboard,auth-core,user-reg} -maxdepth 4 -type f
test -f /home/pc/Downloads/inf/plan/graphify-out/graph.json
```
