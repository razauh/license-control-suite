# TC-02 Dependency Matrix

- Task ID: `TC-02`
- Date: `2026-05-14`
- Status: Implementation complete; execution verification pending user-run commands.

## Manifest Inventory (required 7 paths)

1. `/home/pc/Downloads/inf/plan/shared-contracts/Cargo.toml` (`shared-contracts/Cargo.toml`)
2. `/home/pc/Downloads/inf/plan/admin-dashboard/Cargo.toml` (`admin-dashboard/Cargo.toml`)
3. `/home/pc/Downloads/inf/plan/auth-core/Cargo.toml` (`auth-core/Cargo.toml`)
4. `/home/pc/Downloads/inf/plan/user-reg/Cargo.toml` (`user-reg/Cargo.toml`)
5. `/home/pc/Downloads/inf/plan/user-reg/crates/auth-licensing-core/Cargo.toml` (`user-reg/crates/auth-licensing-core/Cargo.toml`)
6. `/home/pc/Downloads/inf/plan/user-reg/crates/auth-licensing-tauri/Cargo.toml` (`user-reg/crates/auth-licensing-tauri/Cargo.toml`)
7. `/home/pc/Downloads/inf/plan/user-reg/workers/licensing-worker/Cargo.toml` (`user-reg/workers/licensing-worker/Cargo.toml`)

## Baseline Dependency Facts

- All package/workspace package editions are `2021`.
- Standalone crates use `thiserror = "1"`:
  - `shared-contracts`: `thiserror = "1"`
  - `admin-dashboard`: `thiserror = "1"`
  - `auth-core`: `thiserror = "1"`
- `user-reg` workspace uses `thiserror = "2"` in `[workspace.dependencies]`: `thiserror = "2"`.
- Only user-reg tauri crate declares tauri through workspace dependency: `tauri = "2"`.
- External shared-contracts git dependency exists in two crates:
  - `shared-contracts = { git = "ssh://git@github.com/razauh/shared-contracts.git" }` (admin-dashboard)
  - `shared-contracts = { git = "ssh://git@github.com/razauh/shared-contracts.git", version = "1.0" }` (auth-core)
- No confirmed `build.rs` in analyzed repos.
- No confirmed Cargo `links` metadata in inspected manifests.

## Consolidated Table

| Manifest | Kind | Edition | Key dependencies | Risk facts |
|---|---|---|---|---|
| `shared-contracts/Cargo.toml` | crate | 2021 | `serde`, `serde_json`, `thiserror = "1"` | Baseline contract crate |
| `admin-dashboard/Cargo.toml` | crate | 2021 | `serde`, `serde_json`, `thiserror = "1"`, git `shared-contracts` | External git contract dep (CI-1) |
| `auth-core/Cargo.toml` | crate | 2021 | `serde`, `serde_json`, `thiserror = "1"`, git `shared-contracts` | External git contract dep (CI-1) |
| `user-reg/Cargo.toml` | workspace | 2021 (`workspace.package`) | `thiserror = "2"`, `tauri = "2"`, `reqwest`, `keyring`, `tokio` | Workspace collapse + thiserror major mismatch (CI-2, CI-3) |
| `user-reg/crates/auth-licensing-core/Cargo.toml` | workspace member | inherited | `thiserror.workspace = true` | Resolves to workspace `thiserror = "2"` |
| `user-reg/crates/auth-licensing-tauri/Cargo.toml` | workspace member | inherited | `tauri.workspace = true`, `thiserror.workspace = true` | Only tauri-using member |
| `user-reg/workers/licensing-worker/Cargo.toml` | workspace member | inherited | `thiserror.workspace = true` | Resolves to workspace `thiserror = "2"` |

## Metadata Capture Status

Per task card, these files are reserved for `cargo metadata --no-deps --format-version 1` output:

- `docs/baseline/cargo_metadata/shared-contracts.metadata.json`
- `docs/baseline/cargo_metadata/admin-dashboard.metadata.json`
- `docs/baseline/cargo_metadata/auth-core.metadata.json`
- `docs/baseline/cargo_metadata/user-reg.metadata.json`

They are currently placeholders pending user-run metadata extraction.

## Pending User-Run Verification

```bash
cd /home/pc/Downloads/inf/plan/shared-contracts && cargo metadata --no-deps --format-version 1 > /home/pc/Downloads/inf/plan/license-control-suite/docs/baseline/cargo_metadata/shared-contracts.metadata.json
cd /home/pc/Downloads/inf/plan/admin-dashboard && cargo metadata --no-deps --format-version 1 > /home/pc/Downloads/inf/plan/license-control-suite/docs/baseline/cargo_metadata/admin-dashboard.metadata.json
cd /home/pc/Downloads/inf/plan/auth-core && cargo metadata --no-deps --format-version 1 > /home/pc/Downloads/inf/plan/license-control-suite/docs/baseline/cargo_metadata/auth-core.metadata.json
cd /home/pc/Downloads/inf/plan/user-reg && cargo metadata --no-deps --format-version 1 > /home/pc/Downloads/inf/plan/license-control-suite/docs/baseline/cargo_metadata/user-reg.metadata.json
```

No metadata commands were run by the agent.
