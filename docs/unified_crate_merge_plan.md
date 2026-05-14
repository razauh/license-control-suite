# Unified Crate Merge Plan

## 1. Executive Summary

Objective: create a completely new unified Rust/Tauri repository under `/home/pc/Downloads/inf/plan/` and migrate the four existing Rust repositories into it as internal modules while preserving behavior.

Recommended new repository path:

```text
/home/pc/Downloads/inf/plan/license-control-suite/
```

Main strategy:

- Create a new Rust/Tauri project instead of merging into any existing repository.
- Preserve each original repository behind a clear module boundary.
- Migrate test-first, one module at a time.
- Do not flatten all source files into one `src/` directory.
- Do not remove behavior to resolve conflicts.

Merge-readiness: the compatibility report at `/home/pc/Downloads/inf/plan/docs/merged_crate_compatibility_report.md` concludes the four repositories are **mergeable with moderate refactoring**. The main risks are import-path normalization, dependency cleanup, duplicate public names, user-reg workspace collapse, and Tauri command composition.

Safest merge sequence:

1. `shared-contracts`
2. `admin-dashboard`
3. `auth-core`
4. `user-reg`

This order follows dependency complexity: `shared-contracts` is the root contract module, `admin-dashboard` and `auth-core` consume it, and `user-reg` is last because it contains a workspace, Tauri integration, persistence, HTTP client code, and worker code.

## 2. Source-of-Truth Inputs

Primary sources:

- Compatibility report: `/home/pc/Downloads/inf/plan/docs/merged_crate_compatibility_report.md`
- Graphify output: `/home/pc/Downloads/inf/plan/graphify-out/`

Graphify data to use:

| Graphify source | Nodes | Edges |
|---|---:|---:|
| `graphify-out/graph.json` | 657 | 1137 |
| `admin-dashboard/graphify-out/graph.json` | 92 | 96 |
| `auth-core/graphify-out/graph.json` | 104 | 116 |
| `shared-contracts/graphify-out/graph.json` | 88 | 95 |
| `user-reg/graphify-out/graph.json` | 405 | 854 |

How to use these inputs:

- Use the compatibility report as the decision record for confirmed issues, possible issues, and non-issues.
- Use Graphify to verify module names, duplicate labels, relationship clusters, Tauri command nodes, and source-file references.
- Use original Cargo manifests as source of truth for dependency versions and crate shape.
- Use original tests as behavior baselines before adapting code.

Limitations:

- The report states no full final Tauri app shell, `tauri.conf.json`, capability files, frontend directory, or frontend config was confirmed in the analyzed repositories.
- Any final frontend, assets, permission, capability, or packaging plan requiring those files must say: **Requires source verification**.
- The Graphify report was generated from AST and lightweight document extraction. It is reliable for source shape and names, but inferred semantic edges should be verified from source before behavior changes.

## 3. Professional Name Candidates

| Name | Fits because | Repository | Crate | Tauri app | Internal package | Risks |
|---|---|---:|---:|---:|---:|---|
| `license-control-suite` | Captures licensing, auth, admin, and worker functions without overfitting to one module | yes | yes | yes | acceptable | Slightly broad |
| `auth-license-hub` | Emphasizes auth and licensing as the common theme | yes | yes | yes | acceptable | “Hub” can sound generic |
| `license-ops-core` | Good for operational/admin + licensing backend | yes | yes | maybe | yes | Understates Tauri app/UI role |
| `entitlement-control` | Professional domain term for licenses and access | yes | yes | yes | yes | May be too enterprise/generic |
| `activation-suite` | Matches activation/reset flows in graph | yes | maybe | yes | maybe | Too narrow for admin dashboard |
| `license-platform` | Clear umbrella for multiple modules | yes | yes | yes | acceptable | Very generic |
| `auth-entitlement-suite` | Accurate for auth + licensing + entitlement | yes | yes | yes | acceptable | Longer name |
| `device-license-control` | Captures device binding/reset behavior | yes | maybe | maybe | maybe | Too tied to one user-reg domain |
| `access-license-core` | Good technical crate name | yes | yes | maybe | yes | Less polished as app name |
| `licensing-admin-suite` | Fits admin-dashboard + licensing | yes | yes | yes | acceptable | Understates customer auth |
| `license-runtime` | Good for internal backend crate | maybe | yes | no | yes | Too low-level for repo/app |
| `entitlement-hub` | Short and professional | yes | yes | maybe | yes | Less obvious Tauri/auth scope |

Top 3:

1. `license-control-suite`
   - Best overall repository/app name.
   - Short, professional, technically appropriate.
   - Broad enough for `shared-contracts`, `admin-dashboard`, `auth-core`, and `user-reg`.
2. `auth-license-hub`
   - Good alternative if product identity should emphasize authentication more strongly.
   - Professional, but “hub” is less precise than “suite”.
3. `license-ops-core`
   - Best crate/internal package style name.
   - Useful if the final product is backend-heavy, but slightly too narrow for a Tauri app.

Recommendation:

- Repository name: `license-control-suite`
- Main crate/package name: `license-control-suite`
- Rust crate import name: `license_control_suite`
- Tauri app display name: `License Control Suite`

## 4. Proposed New Repository Structure

Planned structure:

```text
/home/pc/Downloads/inf/plan/license-control-suite/
  Cargo.toml
  Cargo.lock
  README.md
  TESTING.md
  src/
    lib.rs
    main.rs
    modules/
      mod.rs
      shared_contracts/
        mod.rs
        dto.rs
        errors.rs
        events.rs
        state.rs
        versioning.rs
      admin_dashboard/
        mod.rs
        adapters.rs
        auth.rs
        authz.rs
        compatibility.rs
        ops.rs
        queue.rs
        realtime.rs
      auth_core/
        mod.rs
        adapters.rs
        auth.rs
        compatibility.rs
        models.rs
        policy.rs
        reset.rs
        session.rs
      user_reg/
        mod.rs
        auth_licensing_core/
          mod.rs
          domain.rs
          service.rs
          state.rs
          traits.rs
          test_support.rs
        auth_licensing_tauri/
          mod.rs
          commands.rs
          http_client.rs
          persistence.rs
        licensing_worker/
          mod.rs
  src-tauri/
    tauri.conf.json
    capabilities/
    icons/
  frontend/
    README.md
  tests/
    baseline/
    contracts/
    integration/
    ipc/
    regression/
  fixtures/
    shared_contracts/
  docs/
    baseline/
    migration/
    merged_crate_compatibility_report.md
    unified_crate_merge_plan.md
  scripts/
    baseline.sh
    check_all.sh
    tauri_smoke.sh
```

Important structure rules:

- `src/modules/shared_contracts` corresponds to original `shared-contracts/src`.
- `src/modules/admin_dashboard` corresponds to original `admin-dashboard/src`.
- `src/modules/auth_core` corresponds to original `auth-core/src`.
- `src/modules/user_reg` contains the three original `user-reg` workspace packages as nested modules.
- `src-tauri/` is a planned final app shell. The compatibility report found Tauri integration library code, but no complete Tauri app shell or final `tauri.conf.json`.
- `frontend/` is a placeholder until actual frontend source is verified. The analyzed repositories did not expose frontend source files, package manifests, or bundler config.

## 5. Repository Mapping

### `shared-contracts`

| Item | Value |
|---|---|
| Original root | `/home/pc/Downloads/inf/plan/shared-contracts` |
| Detected crate | `shared-contracts`, lib target `shared_contracts` |
| Main source | `shared-contracts/src` |
| Important config | `shared-contracts/Cargo.toml`, `Cargo.lock`, `.gitignore`, `.graphifyignore` |
| Tauri files | none found |
| Frontend/assets | none found |
| Build scripts | no `build.rs` found |
| Tests | `shared-contracts/tests`, including compatibility, contract, and fixtures |
| Destination | `license-control-suite/src/modules/shared_contracts` |

Copy unchanged:

- `src/dto.rs`
- `src/errors.rs`
- `src/events.rs`
- `src/state.rs`
- `src/versioning.rs`
- test fixtures under `tests/fixtures`
- README/TESTING documentation into `docs/baseline/shared-contracts/`

Copy and modify:

- `src/lib.rs` -> `src/modules/shared_contracts/mod.rs`
- test paths/imports to use unified crate path

Do not copy directly:

- `.git/`
- `target/`
- `graphify-out/`
- `logs/`

Manual review:

- `Cargo.toml` dependency section for merged manifest.
- serde wire-shape tests because report PI-1 flags DTO/domain overlap.

### `admin-dashboard`

| Item | Value |
|---|---|
| Original root | `/home/pc/Downloads/inf/plan/admin-dashboard` |
| Detected crate | `admin-dashboard`, lib target `admin_dashboard` |
| Main source | `admin-dashboard/src` |
| Important config | `admin-dashboard/Cargo.toml`, `.cargo/config.toml`, `.gitignore`, `.graphifyignore` |
| Tauri files | none found |
| Frontend/assets | none found |
| Build scripts | no `build.rs` found |
| Tests | `admin-dashboard/tests` |
| Destination | `license-control-suite/src/modules/admin_dashboard` |

Copy unchanged first:

- `src/authz.rs`
- `src/ops.rs`
- `src/realtime.rs`
- tests as baseline references under `tests/baseline/admin_dashboard/`

Copy and modify:

- `src/lib.rs` -> `mod.rs`
- `src/adapters.rs`, `src/auth.rs`, `src/compatibility.rs`, `src/queue.rs` because they import `shared_contracts::...`
- tests that import external crate names

Do not copy directly:

- git dependency on `shared-contracts` from `Cargo.toml`
- build output/logs/Graphify output

Manual review:

- `.cargo/config.toml` before carrying forward.
- public names exposed from `admin_dashboard` to avoid root glob export conflicts.

### `auth-core`

| Item | Value |
|---|---|
| Original root | `/home/pc/Downloads/inf/plan/auth-core` |
| Detected crate | `auth-core`, lib target `auth_core` |
| Main source | `auth-core/src` |
| Important config | `auth-core/Cargo.toml`, `.gitignore`, `.graphifyignore` |
| Tauri files | none found |
| Frontend/assets | none found |
| Build scripts | no `build.rs` found |
| Tests | `auth-core/tests/unit`, `tests/integration`, `tests/contract` |
| Destination | `license-control-suite/src/modules/auth_core` |

Copy unchanged first:

- `src/models.rs`
- `src/policy.rs`
- `src/session.rs`
- test files as baseline references

Copy and modify:

- `src/lib.rs` -> `mod.rs`
- `src/adapters.rs`, `src/auth.rs`, `src/compatibility.rs`, `src/reset.rs` because they import `shared_contracts::...`
- tests with external crate imports

Do not copy directly:

- git dependency on `shared-contracts`
- build output/logs/Graphify output

Manual review:

- duplicate `AuthError` versus `user-reg/auth_licensing_core::AuthError`.
- duplicate `SessionState` versus user-reg session state.

### `user-reg`

| Item | Value |
|---|---|
| Original root | `/home/pc/Downloads/inf/plan/user-reg` |
| Detected crate shape | workspace |
| Workspace members | `auth-licensing-core`, `auth-licensing-tauri`, `licensing-worker` |
| Main source | `user-reg/crates/*/src`, `user-reg/workers/licensing-worker/src` |
| Important config | `user-reg/Cargo.toml`, member `Cargo.toml` files |
| Tauri files | `crates/auth-licensing-tauri/src/commands.rs`, `persistence.rs`, `http_client.rs` |
| Frontend/assets | none confirmed |
| Build scripts | no `build.rs` found |
| Tests | `auth-licensing-tauri/tests`, inline tests in source |
| Destination | `license-control-suite/src/modules/user_reg` |

Copy unchanged first:

- docs into `docs/baseline/user-reg/`
- domain/state tests as behavior references

Copy and modify:

- `crates/auth-licensing-core/src/lib.rs` -> `auth_licensing_core/mod.rs`
- `crates/auth-licensing-tauri/src/lib.rs` -> `auth_licensing_tauri/mod.rs`
- `workers/licensing-worker/src/lib.rs` -> `licensing_worker/mod.rs`
- imports from `auth_licensing_core::...` to `crate::modules::user_reg::auth_licensing_core::...`, or local `super::auth_licensing_core::...`
- Tauri command registration so final app owns one composed handler

Do not copy directly:

- workspace manifest shape as-is
- `target/`
- `graphify-out/`
- `.git/`

Manual review:

- `KeychainSecretStore` service names and keys.
- app data filenames `session_state.json`, `reset_status.json`.
- Tauri capabilities/permissions once final app shell exists.
- mutex use in async-adjacent code.

## 6. Compatibility Report Findings to Address

| ID | Report item | Affected areas | Merge response | Stage | Verification |
|---|---|---|---|---|---|
| CI-1 | External `shared-contracts` dependency | `admin-dashboard`, `auth-core` | migrate `shared-contracts` first; remove git dependency; rewrite imports to internal module | before admin/auth migration | contract parity tests compile without external git dependency |
| CI-2 | `user-reg` workspace layout | `user-reg` | collapse workspace members into nested modules under `user_reg` | during user-reg migration | `cargo check` and user-reg core tests pass in unified crate |
| CI-3 | `thiserror` mismatch | all | start with `thiserror = "2"`; if compile fails, use renamed `thiserror1` only as temporary compatibility bridge | dependency reconciliation | `cargo check`, error conversion tests |
| CI-4 | duplicate module/file names | all | preserve wrapper module namespaces | skeleton + all migrations | namespace compile tests |
| CI-5 | duplicate public names | `auth-core`, `shared-contracts`, `user-reg` | avoid root glob exports; require explicit module paths | skeleton + API review | compile tests fail if root ambiguous exports added |
| CI-6 | Tauri handler composition | `user-reg/auth-licensing-tauri` | final app owns a single composed `generate_handler!` | Tauri integration | IPC command invocation tests |
| CI-7 | generic state/key names | `user-reg/auth-licensing-tauri` | namespace storage paths/service names or document chosen app-specific values | runtime integration | temp-dir persistence and keyring smoke tests |
| PI-1 | DTO/domain semantic overlap | `shared-contracts`, `user-reg` | add serialization round-trip and type-path tests | after shared/user-reg migration | contract comparison tests |
| PI-2 | Tauri capabilities/permissions | final app | inspect/create final `tauri.conf.json` and capabilities | Tauri integration | command permission smoke tests |
| PI-3 | command name collision | final app | reserve six user-reg command names and scan final frontend invocations | Tauri integration | command inventory test |
| PI-4 | keyring packaging | user-reg Tauri | add platform checklist and ignored/manual smoke where OS keychain required | packaging | manual or CI platform smoke |
| PI-5 | blocking mutex async use | user-reg | inspect no lock guard crosses `.await`; add concurrent usage tests | runtime integration | concurrency tests |
| PI-6 | frontend/assets conflicts | final app | mark as source verification until frontend exists | frontend phase | frontend build and asset path tests |

## 7. Merge Strategy

Selected approach: **one new Rust/Tauri project with internal Rust modules**.

The new project should be a single repository and a single main Rust crate. The Rust code should be organized into internal modules that mirror the four original repositories. This satisfies the user requirement to merge all four into one newly created project while preserving module boundaries.

Why not a raw Cargo workspace:

- The goal is a single Rust/Tauri project with original repos as internal modules.
- The compatibility report says `user-reg` already has workspace assumptions that must be collapsed.
- Keeping all original packages as workspace members would avoid some import rewrites but would not satisfy the intended single-crate/module structure.

Why not flatten source:

- The compatibility report confirms duplicate module/file names: `adapters`, `auth`, `compatibility`, `state`, and multiple `lib.rs`.
- Graphify confirms duplicate public labels: `AuthError`, `AuditEvent`, `DeviceResetRequest`, `SessionState`, `InMemorySecretStore`.

Trade-off:

- Internal modules require import rewrites.
- But they preserve behavior and avoid accidental namespace collisions.
- This is safer than bulk copying because each module can be compile-gated and tested independently.

## 8. TDD-Based Migration Workflow

Every migration phase follows this rhythm:

1. Capture baseline behavior from original repository.
2. Write or define equivalent failing tests in new repository.
3. Copy/adapt the minimum source needed.
4. Make tests pass.
5. Refactor only after tests pass.
6. Re-run all prior migrated module tests.
7. Record any deviation in `docs/migration/`.

No phase may proceed if:

- new repo does not compile,
- previous module tests regress,
- a compatibility-report issue remains untracked,
- imports are fixed by deleting behavior,
- root glob exports are introduced to bypass namespace problems.

## 9. Phase 0 - Preparation and Evidence Baseline

Objective: capture original behavior before creating migration code.

Tasks:

- Create new repo directory later at `/home/pc/Downloads/inf/plan/license-control-suite/`.
- Copy documentation only at first:
  - compatibility report
  - this merge plan
  - original README/TESTING docs
  - Graphify summaries
- Run and save original test/build commands:
  - `cargo test` in `shared-contracts`
  - `cargo test` in `admin-dashboard`
  - `cargo test` in `auth-core`
  - `cargo test --workspace` in `user-reg`
- Save logs under `docs/baseline/original-runs/`.
- Save `cargo metadata --no-deps --format-version 1` for each repo.
- Inventory Tauri commands from `user-reg/crates/auth-licensing-tauri/src/commands.rs`.
- Inventory dependencies from all Cargo manifests.

TDD output:

- `docs/baseline/baseline_checklist.md`
- `docs/baseline/dependency_matrix.md`
- `docs/baseline/original_command_inventory.md`
- `docs/baseline/tauri_command_inventory.md`
- original build/test logs

Pass criteria:

- Every original repo has a recorded baseline result.
- Any failing original test is documented before migration, not discovered later.

## 10. Phase 1 - New Repository Skeleton

Objective: create a compiling empty project before code migration.

Cargo strategy:

- Create `license-control-suite/Cargo.toml`.
- Use edition `2021`.
- Add only minimal dependencies first: `serde`, `serde_json`, `thiserror`, and `tauri` if Tauri skeleton is created immediately.
- Add `resolver = "2"` if the final manifest uses a workspace-like dependency resolution section or Tauri needs it.

Initial Rust layout:

```rust
pub mod modules;
```

`src/modules/mod.rs` initially declares empty modules:

```rust
pub mod shared_contracts;
pub mod admin_dashboard;
pub mod auth_core;
pub mod user_reg;
```

Tauri entry strategy:

- Add a minimal `src/main.rs` or `src-tauri/src/main.rs` depending on selected Tauri scaffold.
- Do not wire user-reg commands until Phase 4.
- If Tauri CLI scaffold is unavailable, keep Tauri skeleton documented as blocked and proceed with Rust library migration first.

Frontend entry strategy:

- Create `frontend/README.md` explaining no frontend files were confirmed.
- Do not invent frontend routes/components.

Tests before implementation:

- `cargo check` for empty crate.
- compile test that imports all namespace modules.
- optional Tauri skeleton check if `src-tauri/` exists.

Pass criteria:

- New repo compiles before any migrated source code exists.
- Module namespace is present and does not flatten source.

## 11. Phase 2 - Dependency Reconciliation

Objective: create one dependency set that supports all modules.

Dependency sources:

- `shared-contracts`: `serde`, `serde_json`, `thiserror = "1"`
- `admin-dashboard`: `serde`, `serde_json`, `thiserror = "1"`, git `shared-contracts`
- `auth-core`: `serde`, `serde_json`, `thiserror = "1"`, git `shared-contracts`
- `user-reg`: `async-trait`, `base64`, `reqwest`, `keyring`, `serde`, `serde_json`, `sha2`, `tauri = "2"`, `thiserror = "2"`, `tokio`, `tempfile`, `wiremock`

Target dependency policy:

- Remove external git `shared-contracts`.
- Use internal `crate::modules::shared_contracts`.
- Use `serde = { version = "1", features = ["derive"] }`.
- Use `serde_json = "1"`.
- Prefer `thiserror = "2"` initially.
- Keep `reqwest = { version = "0.12", default-features = false, features = ["json", "rustls-tls"] }`.
- Keep `tauri = { version = "2", features = [] }`.
- Keep dev dependencies `tokio`, `tempfile`, `wiremock` for tests.

Feature handling:

- No declared feature flags were found in original manifests.
- Do not add feature gates unless required by compilation.

Native/platform handling:

- No `build.rs` or Cargo `links` metadata was confirmed.
- `keyring` requires platform validation in packaging phase.

Tests:

- `cargo check`
- `cargo tree -d`
- `cargo metadata --format-version 1`
- dependency regression test confirming no external `shared-contracts` git source remains

Pass criteria:

- One lockfile resolves.
- No external `shared-contracts` dependency appears.
- Dependency duplicates are documented if unavoidable.

## 12. Phase 3 - Module-by-Module Migration Plan

### 3.1 Migrate `shared-contracts`

Module name: `shared_contracts`

Original files to inspect:

- `shared-contracts/src/lib.rs`
- `shared-contracts/src/dto.rs`
- `shared-contracts/src/errors.rs`
- `shared-contracts/src/events.rs`
- `shared-contracts/src/state.rs`
- `shared-contracts/src/versioning.rs`
- `shared-contracts/tests`
- `shared-contracts/tests/fixtures`

Target destination:

- `src/modules/shared_contracts/`
- `tests/contracts/shared_contracts_*.rs`
- `fixtures/shared_contracts/`

APIs to preserve:

- DTOs listed by Graphify, including `ActivateRequest`, `ActivateResponse`, `AdminAuthVerifyRequest`, `DeviceResetStatusResponse`
- errors including `ApiError`, `ErrorCode`, `ErrorBody`
- events including `AuditEvent`
- state types including `LicenseState`, `ResetRequestState`
- `CompatibilityInfo`, `SemverChange`

Tests to write before migration:

- compile test importing `license_control_suite::modules::shared_contracts::*` through explicit submodules
- fixture round-trip tests copied from original contract tests
- versioning compatibility tests

Expected failing tests:

- imports fail until modules are copied and `mod.rs` is wired.

Implementation steps:

1. Copy source files into `src/modules/shared_contracts/`.
2. Convert `src/lib.rs` to `mod.rs`.
3. Keep submodule names unchanged.
4. Copy fixtures.
5. Port tests to new crate path.

Pass criteria:

- all shared-contract tests pass in unified repo.
- no external dependency on original `shared-contracts`.

Rollback criteria:

- if public serde shapes differ from original fixture tests, stop and restore copied files before adapting behavior.

Risks addressed:

- CI-1, CI-4, CI-5, PI-1.

### 3.2 Migrate `admin-dashboard`

Module name: `admin_dashboard`

Original files to inspect:

- `admin-dashboard/src/lib.rs`
- `admin-dashboard/src/adapters.rs`
- `admin-dashboard/src/auth.rs`
- `admin-dashboard/src/authz.rs`
- `admin-dashboard/src/compatibility.rs`
- `admin-dashboard/src/ops.rs`
- `admin-dashboard/src/queue.rs`
- `admin-dashboard/src/realtime.rs`
- `admin-dashboard/tests`

Target destination:

- `src/modules/admin_dashboard/`
- `tests/integration/admin_dashboard_*.rs`

Dependencies required:

- internal `shared_contracts`
- `serde`, `serde_json`, `thiserror`

APIs to preserve:

- `login_with_challenge()`
- `approve()`
- `reject()`
- `compute_health()`
- `backoff_step_sec()`
- compatibility range functions

Tests to write before migration:

- admin auth challenge/verify behavior.
- queue approve/reject behavior.
- ops health threshold behavior.
- realtime backoff/delta behavior.
- compatibility range test against internal `shared_contracts`.

Expected failing tests:

- imports fail until `admin_dashboard` source is copied and `shared_contracts::...` imports are rewritten.

Implementation steps:

1. Copy source files.
2. Convert `src/lib.rs` to module `mod.rs`.
3. Rewrite imports from `shared_contracts::...` to `crate::modules::shared_contracts::...`.
4. Rewrite local `crate::...` imports to `super::...` or module-local paths.
5. Port tests.

Pass criteria:

- migrated admin tests pass.
- shared-contract tests still pass.
- no root public name conflicts.

Rollback criteria:

- if admin behavior changes, revert admin module changes while keeping shared contracts intact.

Risks addressed:

- CI-1, CI-3, CI-4, CI-5.

### 3.3 Migrate `auth-core`

Module name: `auth_core`

Original files to inspect:

- `auth-core/src/lib.rs`
- `auth-core/src/adapters.rs`
- `auth-core/src/auth.rs`
- `auth-core/src/compatibility.rs`
- `auth-core/src/models.rs`
- `auth-core/src/policy.rs`
- `auth-core/src/reset.rs`
- `auth-core/src/session.rs`
- `auth-core/tests/unit`
- `auth-core/tests/integration`
- `auth-core/tests/contract`

Target destination:

- `src/modules/auth_core/`
- `tests/integration/auth_core_*.rs`

Dependencies required:

- internal `shared_contracts`
- `serde`, `serde_json`, `thiserror`

APIs to preserve:

- `activate()`
- `renew()`
- `submit_reset_request()`
- `poll_until_terminal()`
- `offline_access()`
- session and compatibility behavior

Tests to write before migration:

- activation success/invalid behavior.
- reset submit/poll terminal behavior.
- offline grace policy behavior.
- session model behavior.
- contract parity tests using internal `shared_contracts`.

Expected failing tests:

- imports fail until source copied and path rewrites applied.

Implementation steps:

1. Copy source files.
2. Convert `src/lib.rs` to module `mod.rs`.
3. Rewrite imports from `shared_contracts::...`.
4. Rewrite local `crate::...` imports.
5. Port tests and test fixtures.

Pass criteria:

- `shared_contracts`, `admin_dashboard`, and `auth_core` tests all pass.
- no accidental mixing of `auth_core::AuthError` with user-reg `AuthError`.

Rollback criteria:

- if `auth_core` migration breaks admin/shared tests, revert only auth-core changes and inspect import leakage.

Risks addressed:

- CI-1, CI-3, CI-4, CI-5, PI-1.

### 3.4 Migrate `user-reg`

Module name: `user_reg`

Original files to inspect:

- `user-reg/Cargo.toml`
- `user-reg/crates/auth-licensing-core/src/*`
- `user-reg/crates/auth-licensing-tauri/src/*`
- `user-reg/workers/licensing-worker/src/lib.rs`
- `user-reg/crates/auth-licensing-tauri/tests`
- `user-reg/docs`

Target destination:

- `src/modules/user_reg/auth_licensing_core/`
- `src/modules/user_reg/auth_licensing_tauri/`
- `src/modules/user_reg/licensing_worker/`
- `tests/integration/user_reg_*.rs`
- `tests/ipc/user_reg_commands.rs`

Dependencies required:

- `async-trait`
- `base64`
- `keyring`
- `reqwest`
- `serde`
- `serde_json`
- `sha2`
- `tauri`
- `thiserror`
- `tokio`, `tempfile`, `wiremock` for tests

APIs and commands to preserve:

- `AuthService`
- `HttpWorkerClient`
- `AppDataStateStore`
- `KeychainSecretStore`
- `WorkerApp`
- Tauri commands:
  - `activate_license`
  - `validate_session`
  - `request_device_reset`
  - `get_device_reset_status`
  - `clear_local_session`
  - `get_auth_state`

Tests to write before migration:

- core activation/session/reset service tests.
- persistence temp-dir tests.
- HTTP worker client tests with `wiremock`.
- command return-shape tests using service helpers.
- worker route/domain tests.

Expected failing tests:

- imports fail until workspace package names are replaced by module paths.
- command handler tests fail until Tauri wiring is adapted.

Implementation steps:

1. Copy `auth-licensing-core` source as nested module.
2. Rewrite internal `crate::...` paths inside nested module.
3. Copy and adapt tests.
4. Copy `auth-licensing-tauri`.
5. Rewrite `auth_licensing_core::...` imports to internal module paths.
6. Copy `licensing-worker`.
7. Rewrite worker imports.
8. Defer final app-level Tauri handler composition to Phase 4.

Pass criteria:

- user-reg core tests pass.
- Tauri command helper tests pass.
- worker tests pass.
- previously migrated module tests still pass.

Rollback criteria:

- if user-reg migration requires broad API redesign, stop and isolate failing import/type boundary rather than changing behavior.

Risks addressed:

- CI-2, CI-3, CI-4, CI-5, CI-6, CI-7, PI-1, PI-4, PI-5.

## 13. Phase 4 - Tauri Command, Plugin, and App Lifecycle Integration

Objective: integrate Tauri app lifecycle without conflicting builders or command handlers.

Confirmed Tauri source:

- `user-reg/crates/auth-licensing-tauri/src/commands.rs`
- `user-reg/crates/auth-licensing-tauri/src/persistence.rs`
- `user-reg/crates/auth-licensing-tauri/src/http_client.rs`

Unified command registration strategy:

- Final app owns one app-level `generate_handler!`.
- Include the six user-reg commands in that one handler.
- Do not call multiple independent `builder.invoke_handler(...)` methods as module composition.

Duplicate command-name handling:

- Reserve command names from the compatibility report.
- Before adding final app commands, scan for collisions in frontend `invoke(...)` usage.
- Requires source verification: inspect final frontend path when available.

Plugin initialization:

- No plugins were confirmed in analyzed repos.
- If final Tauri app adds plugins, initialize them once in the app shell, not inside individual migrated modules.

Setup hooks, menus, trays, windows:

- No conflicts confirmed.
- Requires source verification: inspect final `src-tauri` app shell when created.

Capabilities/permissions:

- Create or inspect `src-tauri/capabilities/*.json`.
- Ensure the six commands are available to the intended windows.

Tests:

- command inventory test confirms six command names.
- IPC payload/response tests for activation, validation, reset request, reset status, clear session, auth state.
- app startup smoke test.
- permission/capability smoke test once `tauri.conf.json` exists.

Pass criteria:

- one command handler contains all commands.
- app starts.
- frontend can invoke commands in test environment or documented smoke harness.

## 14. Phase 5 - Frontend and Asset Integration

Evidence status:

- No source frontend files were confirmed.
- The only `.html` files found are Graphify outputs, not app frontend files.
- No `package.json`, Vite config, Svelte/React files, Tauri app shell config, or assets were confirmed in analyzed repositories.

Plan:

- Create `frontend/README.md` initially.
- Do not invent app UI, routes, components, or assets.
- When actual frontend source is supplied, copy it under `frontend/`.
- Namespace any IPC calls by verified command names.
- Keep build output out of Rust source directories.

If frontend exists later, tests must include:

- package manager install/build.
- route smoke tests.
- asset path tests.
- IPC invoke contract tests for six user-reg commands.
- UI regression screenshots if feasible.

Pass criteria:

- frontend builds.
- asset paths resolve.
- IPC calls match command names and payload shapes.

## 15. Phase 6 - Runtime Integration and Shared State

Async runtime strategy:

- Use Tauri v2 runtime for command execution.
- Keep `tokio` as test dependency for async module tests.
- Do not introduce a second runtime unless required by compile error and documented.

Shared state:

- Register `AuthAppState` once in final Tauri builder.
- Keep admin/auth/shared modules pure library modules unless future source proves they need Tauri state.

File paths:

- Review `AppDataStateStore` use of:
  - `session_state.json`
  - `reset_status.json`
- Prefer namespaced paths in final app if any existing app state uses same names.

Keyring:

- Review `KeychainSecretStore` keys:
  - `license_key`
  - `access_token`
  - `device_keypair`
- Choose app-specific service name before production.

Events:

- No event names were confirmed.
- If events are added later, namespace them by module, for example `auth:session_changed`.

Serialization:

- Add round-trip tests for shared-contract DTOs.
- Add round-trip tests for user-reg IPC views.
- Add explicit tests that similar names are not accidentally interchanged.

Error strategy:

- Preserve module-local error types.
- Do not unify `AuthError` names globally.
- Wrap at Tauri IPC boundary using `AuthCommandError`.

Tests:

- concurrent activation/reset service tests.
- persistence temp-dir tests.
- serialization round-trip tests.
- error propagation tests.
- mutex review test or static code review checklist: no lock guard across `.await`.

## 16. Phase 7 - Packaging and Platform Validation

Tauri bundle strategy:

- Use final `src-tauri` app shell once created.
- Keep Rust library modules independent from bundle config.
- Do not copy Graphify HTML as app frontend assets.

External binary handling:

- No external binaries confirmed.
- Requires source verification if future build scripts or frontend tooling are added.

Native libraries:

- No Cargo `links` metadata confirmed.
- `keyring` may require OS credential backend support.
- Tauri may require OS-specific WebView dependencies.

OS-specific checks:

- Linux: WebKit/WebView and secret service/keyring smoke.
- Windows: credential manager/keyring smoke.
- macOS: keychain smoke and signing/notarization only if release distribution requires it.

Tests:

- `cargo build --release`
- `cargo test`
- Tauri build smoke test
- app launch smoke test
- resource loading test
- keyring manual smoke, ignored by default if no OS keychain in CI

Pass criteria:

- release build succeeds.
- app launches.
- no missing runtime resources.
- platform-specific blockers documented with exact failing command/log.

## 17. Phase 8 - Final Regression and Acceptance

Full test expectations:

- all migrated module tests pass.
- all original behavior parity tests pass or documented original baseline failure exists.
- all Tauri command tests pass.
- dependency tree contains no external git `shared-contracts`.
- no root glob exports added.
- packaging smoke passes or is blocked only by documented missing platform prerequisites.

Module-level acceptance:

- `shared_contracts`: contract fixtures and versioning pass.
- `admin_dashboard`: auth, queue, authz, ops, realtime, compatibility pass.
- `auth_core`: activation, renewal, reset, session, policy, compatibility pass.
- `user_reg`: core, Tauri command helper, persistence, HTTP client, worker tests pass.

Integration acceptance:

- admin/auth modules use internal shared contracts.
- user-reg modules compile as internal modules, not workspace packages.
- Tauri handler composition is singular and app-level.

Documentation acceptance:

- migration notes document all import rewrites.
- compatibility-report issue checklist shows every CI/PI item addressed or explicitly blocked.

## 18. Detailed Task Breakdown

1. **Create baseline evidence archive**
   - Objective: preserve current behavior.
   - Source: all four original repos.
   - Target: `license-control-suite/docs/baseline/`.
   - Prerequisite: none.
   - TDD: run original tests before migration.
   - Implementation: save command outputs and metadata.
   - Expected output: baseline logs and dependency matrix.
   - Completion: all four repos have recorded results.
   - Risks addressed: all behavior-regression risks.

2. **Create new repository skeleton**
   - Objective: new repo, not existing repo.
   - Source: none.
   - Target: `/home/pc/Downloads/inf/plan/license-control-suite/`.
   - Prerequisite: task 1.
   - TDD: write namespace compile test.
   - Implementation: create Cargo project, `src/modules`, docs dirs.
   - Expected output: compiling empty module skeleton.
   - Completion: `cargo check` passes.
   - Risks addressed: CI-4, CI-5.

3. **Merge dependency manifest**
   - Objective: one dependency set.
   - Source: all Cargo manifests.
   - Target: new `Cargo.toml`.
   - Prerequisite: task 2.
   - TDD: dependency check fails until manifest includes required deps.
   - Implementation: add dependencies, remove external git shared-contracts.
   - Expected output: resolved lockfile.
   - Completion: `cargo tree -d` reviewed; no git `shared-contracts`.
   - Risks addressed: CI-1, CI-3.

4. **Migrate shared contracts**
   - Objective: establish internal contract module.
   - Source: `shared-contracts/src`, tests, fixtures.
   - Target: `src/modules/shared_contracts`, `tests/contracts`, `fixtures/shared_contracts`.
   - Prerequisite: task 3.
   - TDD: port contract tests first.
   - Implementation: copy/adapt source, keep wire behavior.
   - Expected output: shared contract tests pass.
   - Completion: no external shared-contracts needed.
   - Risks addressed: CI-1, PI-1.

5. **Migrate admin dashboard**
   - Objective: preserve admin behavior.
   - Source: `admin-dashboard/src`, tests.
   - Target: `src/modules/admin_dashboard`, `tests/integration/admin_dashboard_*`.
   - Prerequisite: task 4.
   - TDD: port admin tests before adapting source.
   - Implementation: copy source, rewrite shared_contracts imports.
   - Expected output: admin tests pass.
   - Completion: shared + admin tests pass together.
   - Risks addressed: CI-1, CI-4, CI-5.

6. **Migrate auth core**
   - Objective: preserve auth/licensing core behavior.
   - Source: `auth-core/src`, tests.
   - Target: `src/modules/auth_core`, `tests/integration/auth_core_*`.
   - Prerequisite: task 4 and preferably task 5.
   - TDD: port activation/reset/policy/session tests first.
   - Implementation: copy source, rewrite imports.
   - Expected output: auth-core tests pass.
   - Completion: shared + admin + auth tests pass.
   - Risks addressed: CI-1, CI-4, CI-5, PI-1.

7. **Migrate user-reg core**
   - Objective: collapse `auth-licensing-core`.
   - Source: `user-reg/crates/auth-licensing-core/src`.
   - Target: `src/modules/user_reg/auth_licensing_core`.
   - Prerequisite: task 3.
   - TDD: port core service/domain tests.
   - Implementation: adapt module paths.
   - Expected output: core tests pass.
   - Completion: no package dependency on `auth-licensing-core`.
   - Risks addressed: CI-2, PI-5.

8. **Migrate user-reg Tauri integration**
   - Objective: preserve Tauri command behavior.
   - Source: `user-reg/crates/auth-licensing-tauri/src`.
   - Target: `src/modules/user_reg/auth_licensing_tauri`.
   - Prerequisite: task 7.
   - TDD: port command helper, persistence, HTTP client tests.
   - Implementation: rewrite imports, keep command functions.
   - Expected output: command helper tests pass.
   - Completion: six commands compile.
   - Risks addressed: CI-2, CI-6, CI-7, PI-2, PI-3, PI-4.

9. **Migrate licensing worker**
   - Objective: preserve worker behavior.
   - Source: `user-reg/workers/licensing-worker/src/lib.rs`.
   - Target: `src/modules/user_reg/licensing_worker/mod.rs`.
   - Prerequisite: task 7.
   - TDD: port worker tests.
   - Implementation: rewrite `auth_licensing_core` imports.
   - Expected output: worker tests pass.
   - Completion: worker compiles as module.
   - Risks addressed: CI-2, PI-5.

10. **Compose Tauri app handler**
    - Objective: one command registration point.
    - Source: migrated `commands.rs`.
    - Target: app-level Tauri builder.
    - Prerequisite: task 8.
    - TDD: command inventory and IPC tests.
    - Implementation: build single `generate_handler!`.
    - Expected output: command invocation tests pass.
    - Completion: app starts with commands registered.
    - Risks addressed: CI-6, PI-2, PI-3.

11. **Verify runtime storage and serialization**
    - Objective: prevent runtime collisions.
    - Source: `persistence.rs`, DTO/domain files.
    - Target: runtime tests.
    - Prerequisite: tasks 4, 7, 8.
    - TDD: write round-trip and temp-dir persistence tests.
    - Implementation: namespace paths if collision confirmed.
    - Expected output: no state/key collisions in tests.
    - Completion: tests pass; storage names documented.
    - Risks addressed: CI-7, PI-1.

12. **Run final regression and packaging smoke**
    - Objective: prove merged app readiness.
    - Source: full new repo.
    - Target: CI/check scripts.
    - Prerequisite: all migration tasks.
    - TDD: full suite must pass before cleanup.
    - Implementation: run check scripts, release build, Tauri smoke.
    - Expected output: final acceptance logs.
    - Completion: all acceptance criteria met.
    - Risks addressed: all.

## 19. File Copy and Modification Matrix

| Source | Repo | Destination | Action | Reason | Related concern |
|---|---|---|---|---|---|
| `shared-contracts/src/*.rs` | shared-contracts | `src/modules/shared_contracts/` | copy and modify `lib.rs` to `mod.rs` | internal contract module | CI-1 |
| `shared-contracts/tests/fixtures` | shared-contracts | `fixtures/shared_contracts/` | copy unchanged | preserve contract fixtures | PI-1 |
| `shared-contracts/tests` | shared-contracts | `tests/contracts/` | copy and modify imports | preserve wire contract tests | PI-1 |
| `admin-dashboard/src/*.rs` | admin-dashboard | `src/modules/admin_dashboard/` | copy and modify imports | remove external `shared_contracts` | CI-1 |
| `admin-dashboard/tests` | admin-dashboard | `tests/integration/admin_dashboard_*` | copy and modify imports | behavior parity | CI-4 |
| `auth-core/src/*.rs` | auth-core | `src/modules/auth_core/` | copy and modify imports | remove external `shared_contracts` | CI-1 |
| `auth-core/tests` | auth-core | `tests/integration/auth_core_*` | copy and modify imports | behavior parity | CI-4 |
| `user-reg/crates/auth-licensing-core/src` | user-reg | `src/modules/user_reg/auth_licensing_core/` | copy and modify paths | collapse workspace | CI-2 |
| `user-reg/crates/auth-licensing-tauri/src` | user-reg | `src/modules/user_reg/auth_licensing_tauri/` | copy and modify paths | preserve Tauri commands | CI-6 |
| `user-reg/workers/licensing-worker/src/lib.rs` | user-reg | `src/modules/user_reg/licensing_worker/mod.rs` | copy and modify paths | collapse worker package | CI-2 |
| `user-reg/crates/auth-licensing-tauri/tests` | user-reg | `tests/ipc/`, `tests/integration/` | copy and modify imports | Tauri/IPC behavior | CI-6 |
| original `Cargo.toml` files | all | `docs/baseline/`, new `Cargo.toml` | inspect/copy excerpts only | dependency matrix | CI-3 |
| `.git/` | all | none | do not copy | not source | none |
| `target/` | all | none | do not copy | build output | none |
| `graphify-out/` | all | docs reference only | do not copy into source | analysis output | none |
| logs | all | `docs/baseline/original-runs/` if useful | inspect/copy as evidence only | baseline evidence | regression |

## 20. Test Plan Summary

| Test | Type | Phase | Module | Expected behavior | Pass criteria | Risk |
|---|---|---|---|---|---|---|
| original repo cargo tests | baseline | 0 | all | current behavior known | logs saved | regression |
| namespace compile test | compile | 1 | all | modules exist without flattening | `cargo check` passes | CI-4 |
| no external shared-contracts | dependency | 2 | admin/auth/shared | git dependency removed | `cargo tree` clean | CI-1 |
| thiserror compile gate | compile | 2 | all | errors derive/convert | `cargo check` passes | CI-3 |
| shared contract fixture tests | contract | 3 | shared_contracts | JSON wire shapes stable | fixture tests pass | PI-1 |
| admin dashboard behavior tests | regression | 3 | admin_dashboard | auth/queue/ops/realtime preserved | migrated tests pass | CI-1 |
| auth core behavior tests | regression | 3 | auth_core | activation/reset/session/policy preserved | migrated tests pass | CI-1 |
| user-reg core service tests | regression | 3 | user_reg core | domain workflows preserved | tests pass | CI-2 |
| Tauri command helper tests | IPC | 3/4 | user_reg Tauri | command payload/response stable | tests pass | CI-6 |
| app command inventory | IPC | 4 | Tauri app | six commands registered once | inventory passes | PI-3 |
| persistence temp-dir tests | runtime | 6 | user_reg Tauri | state files read/write safely | tests pass | CI-7 |
| serialization round trips | contract | 6 | shared/user_reg | DTO/domain shapes explicit | tests pass | PI-1 |
| concurrent usage tests | runtime | 6 | user_reg | no mutex/runtime issue found | tests pass | PI-5 |
| release build | packaging | 7 | all | optimized build works | command succeeds | packaging |
| Tauri smoke | packaging | 7 | app | app starts | launch/build succeeds | PI-2 |

## 21. Risk Register

| Risk | Status | Severity | Mitigation | Test coverage | Stage |
|---|---|---:|---|---|---|
| external `shared-contracts` dependency | confirmed | High | migrate shared first, remove git dep | cargo tree + contract tests | Phase 2/3 |
| `user-reg` workspace collapse | confirmed | High | nested modules and path rewrites | cargo check + user-reg tests | Phase 3 |
| `thiserror` major mismatch | confirmed | Medium | standardize or rename fallback | compile gate | Phase 2 |
| duplicate module names | confirmed | Medium | wrapper modules | namespace compile | Phase 1 |
| duplicate public names | confirmed | Medium | no root glob exports | API compile tests | Phase 1/3 |
| Tauri handler composition | confirmed | Medium | one app-level handler | IPC tests | Phase 4 |
| generic state/key names | confirmed | Low/Medium | namespace or document app-specific values | persistence tests | Phase 6 |
| DTO/domain overlap | possible | Medium | explicit type paths + round trips | contract tests | Phase 6 |
| Tauri capabilities | possible | Medium | inspect/create capabilities | permission smoke | Phase 4 |
| command-name collision | possible | Medium | inventory command names | command inventory | Phase 4 |
| keyring packaging | possible | Medium | platform smoke/manual tests | packaging checklist | Phase 7 |
| blocking mutex async use | possible | Low | inspect no lock across await | concurrency tests | Phase 6 |
| frontend/assets conflicts | possible | Low/Medium | source verification before copy | frontend build | Phase 5 |

## 22. Merge Order Recommendation

Recommended order:

```text
shared-contracts -> admin-dashboard -> auth-core -> user-reg
```

Justification:

- `shared-contracts` is the contract root for `admin-dashboard` and `auth-core`.
- `admin-dashboard` is a plain library crate with no Tauri dependency and smaller behavior surface.
- `auth-core` is also a plain library crate but has broader auth/reset/session behavior.
- `user-reg` is last because the compatibility report flags it as highest complexity: workspace collapse, Tauri commands, local persistence, keyring, HTTP client, and worker module.

## 23. What Not to Do

- Do not merge into `admin-dashboard`, `auth-core`, `shared-contracts`, or `user-reg`.
- Do not copy all files blindly.
- Do not copy `.git`, `target`, `graphify-out`, or logs into source.
- Do not flatten all modules into one unstructured `src/`.
- Do not resolve duplicate names by deleting behavior.
- Do not introduce root `pub use module::*` exports.
- Do not keep external git `shared-contracts` after internal migration.
- Do not merge multiple Tauri builders or handlers without one app lifecycle plan.
- Do not ignore Tauri capabilities/permissions.
- Do not treat Graphify inferred semantic edges as behavior proof without source verification.
- Do not test only at the end.

## 24. Final Implementation Checklist

- [ ] New repo created at `/home/pc/Downloads/inf/plan/license-control-suite/`
- [ ] Baseline logs captured for all four original repositories
- [ ] Dependency matrix saved
- [ ] `shared_contracts` migrated and tested
- [ ] `admin_dashboard` migrated and tested
- [ ] `auth_core` migrated and tested
- [ ] `user_reg::auth_licensing_core` migrated and tested
- [ ] `user_reg::auth_licensing_tauri` migrated and tested
- [ ] `user_reg::licensing_worker` migrated and tested
- [ ] External git `shared-contracts` removed
- [ ] `thiserror` version decision verified by compile/tests
- [ ] Duplicate public names kept behind module boundaries
- [ ] One Tauri command handler composed
- [ ] Tauri capabilities/permissions verified or explicitly blocked
- [ ] Frontend source verified or marked not present
- [ ] Runtime storage/keyring names reviewed
- [ ] Full test suite passes
- [ ] Tauri app starts
- [ ] Release/package smoke passes or platform blocker documented
- [ ] Migration docs updated

## Terminal-Safe Summary

Plan path: `/home/pc/Downloads/inf/plan/docs/unified_crate_merge_plan.md`

Recommended new repo name: `license-control-suite`

Top 3 alternative names:

1. `license-control-suite`
2. `auth-license-hub`
3. `license-ops-core`

Number of migration phases: `9` (`Phase 0` through `Phase 8`)

Recommended module migration order:

```text
shared-contracts -> admin-dashboard -> auth-core -> user-reg
```

Number of major risks addressed: `13`

Final merge strategy:

```text
new Rust/Tauri repository with preserved internal module boundaries
```
