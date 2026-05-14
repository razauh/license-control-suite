# Dependency Decisions (TC-06)

- Date: `2026-05-14`
- Task: `TC-06 - Reconcile Cargo Dependencies`

## Inputs Used

- `docs/baseline/dependency_matrix.md`
- `/home/pc/Downloads/inf/plan/docs/unified_crate_merge_plan.md`
- `/home/pc/Downloads/inf/plan/docs/merged_crate_compatibility_report.md`

## Decisions

1. Unified crate dependency baseline now uses `thiserror = "2"`.
   - Rationale: `user-reg` workspace already standardizes on `thiserror = "2"`, and TC-06 instructs to prefer `thiserror = "2"` initially.
2. Do not add external git `shared-contracts` dependency.
   - Rationale: compatibility risk CI-1 requires internal module migration, not external crate fetch.
3. Added planned runtime dependencies from source manifests:
   - `async-trait = "0.1"`
   - `base64 = "0.22"`
   - `keyring = "3"`
   - `reqwest = { version = "0.12", default-features = false, features = ["json", "rustls-tls"] }`
   - `serde = { version = "1", features = ["derive"] }`
   - `serde_json = "1"`
   - `sha2 = "0.10"`
   - `tauri = { version = "2", features = [] }`
   - `thiserror = "2"`
   - `tokio = { version = "1", features = ["macros", "rt", "sync"] }`
4. Added planned dev dependencies from source manifests:
   - `tokio = { version = "1", features = ["macros", "rt", "sync"] }`
   - `tempfile = "3"`
   - `wiremock = "0.6"`

## Known Pending Verification

- `cargo check` not run yet.
- `cargo tree -d` not run yet.
- `cargo metadata --format-version 1` not run yet.
- `Cargo.lock` has not been generated/updated yet because cargo commands were not executed.
