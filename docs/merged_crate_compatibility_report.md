# Merged Crate Compatibility Report

## 1. Executive Summary

The four repositories are **mergeable with moderate refactoring**. They do not show a fundamental Rust edition, crate-type, or Tauri-version incompatibility. The main risks are import-path normalization, dependency cleanup, public namespace collisions, and careful Tauri command integration.

The safest merge model is to preserve each original repository behind a wrapper module in the unified crate:

```text
src/
  admin_dashboard/
  auth_core/
  shared_contracts/
  user_reg/
```

Flattening all source files directly into `src/` is not safe without more cleanup because several module names and public type names overlap.

Top risks:

1. `admin-dashboard` and `auth-core` depend on external git `shared-contracts`; those imports must become internal module references.
2. `user-reg` is a workspace with three packages, not a single crate; its internal path dependencies must be collapsed into modules.
3. `thiserror` is split between major versions: `1` in three repos, `2` in `user-reg`.
4. Duplicate names exist across repositories: `AuthError`, `AuditEvent`, `DeviceResetRequest`, `SessionState`, `state.rs`, `compatibility.rs`, and several test helper names.
5. Tauri command registration currently exists only in `user-reg/auth-licensing-tauri` and must be composed into the final app-level `generate_handler!`, not registered as a competing handler.

## 2. Input Scope

Analyzed Graphify outputs:

| Output | Nodes | Edges |
|---|---:|---:|
| `/home/pc/Downloads/inf/plan/graphify-out/graph.json` | 657 | 1137 |
| `admin-dashboard/graphify-out/graph.json` | 92 | 96 |
| `auth-core/graphify-out/graph.json` | 104 | 116 |
| `shared-contracts/graphify-out/graph.json` | 88 | 95 |
| `user-reg/graphify-out/graph.json` | 405 | 854 |

Detected repositories/crates:

| Repository | Package/crate shape |
|---|---|
| `admin-dashboard` | single Rust library crate |
| `auth-core` | single Rust library crate |
| `shared-contracts` | single Rust library crate |
| `user-reg` | Cargo workspace with `auth-licensing-core`, `auth-licensing-tauri`, and `licensing-worker` |

Additional files inspected:

- Cargo manifests and `cargo metadata --no-deps`
- Root module files
- Tauri command/persistence/HTTP integration files in `user-reg`
- Graphify reports and duplicate graph labels

Limitations:

- The combined Graphify report was generated from AST and lightweight document extraction, with no external LLM token spend. It is useful for structure, names, and relationships, but semantic inferences should be verified from source before changing behavior.
- No final target Tauri app configuration was available, so final `tauri.conf.json`, capabilities, frontend assets, permissions, and plugin interactions cannot be fully confirmed.
- The analyzed repos do not appear to include a complete Tauri application entrypoint; `user-reg` provides Tauri integration library code, not a full app shell.

## 3. Repository-by-Repository Overview

### `admin-dashboard`

- Crate name: `admin-dashboard`, library target `admin_dashboard`
- Edition: `2021`
- Purpose: reusable admin dashboard core module for admin auth, reset queue operations, authorization, compatibility checks, operational health, and realtime recovery.
- Main modules: `adapters`, `auth`, `authz`, `compatibility`, `ops`, `queue`, `realtime`
- Main dependencies: `serde`, `serde_json`, `thiserror = "1"`, git dependency on `shared-contracts`
- Tauri-specific components: none found in source or metadata
- Build/configuration assumptions: standalone lib crate, uses external `shared_contracts::...` imports, no `build.rs`, no declared feature flags

Graph evidence:

- Graphify summary: 92 nodes, 96 edges, 8 communities
- Key nodes include `login_with_challenge()`, `approve()`, `reject()`, `compute_health()`, and `supported_shared_contracts_range()`

### `auth-core`

- Crate name: `auth-core`, library target `auth_core`
- Edition: `2021`
- Purpose: reusable auth/licensing module for activation, renewal, reset polling, local session handling, offline policy, and shared-contract compatibility checks.
- Main modules: `adapters`, `auth`, `compatibility`, `models`, `policy`, `reset`, `session`
- Main dependencies: `serde`, `serde_json`, `thiserror = "1"`, git dependency on `shared-contracts` with version requirement `1.0`
- Tauri-specific components: none found
- Build/configuration assumptions: standalone lib crate, uses external `shared_contracts::...` imports, explicit integration/unit/contract test entries, no `build.rs`, no declared feature flags

Graph evidence:

- Graphify summary: 104 nodes, 116 edges, 8 communities
- Key nodes include `renew()`, `submit_reset_request()`, `activate()`, `poll_until_terminal()`, and `offline_access()`

### `shared-contracts`

- Crate name: `shared-contracts`, library target `shared_contracts`
- Edition: `2021`
- Purpose: shared wire contracts for `auth-core` and `admin-dashboard`
- Main modules: `dto`, `errors`, `events`, `state`, `versioning`
- Main dependencies: `serde`, `serde_json`, `thiserror = "1"`
- Tauri-specific components: none found
- Build/configuration assumptions: standalone lib crate, source of DTO/state/error/event/versioning contracts, no `build.rs`, no declared feature flags

Graph evidence:

- Graphify summary: 88 nodes, 95 edges, 7 communities
- Key nodes include `ActivateRequest`, `ActivateResponse`, `AdminAuthVerifyRequest`, `DeviceResetStatusResponse`, `ApiError`, `AuditEvent`, and `CompatibilityInfo`

### `user-reg`

- Repository shape: Cargo workspace
- Workspace members:
  - `auth-licensing-core`, library target `auth_licensing_core`
  - `auth-licensing-tauri`, library target `auth_licensing_tauri`
  - `licensing-worker`, library target `licensing_worker`
- Edition: workspace edition `2021`
- Purpose: customer auth/licensing domain, Tauri IPC integration, local persistence, HTTP worker client, and local-testable licensing worker.
- Main dependencies: `async-trait`, `base64`, `keyring`, `reqwest`, `serde`, `serde_json`, `sha2`, `tauri = "2"`, `thiserror = "2"`, `tokio` as dev dependency, `wiremock` as dev dependency
- Tauri-specific components:
  - `#[tauri::command] activate_license`
  - `#[tauri::command] validate_session`
  - `#[tauri::command] request_device_reset`
  - `#[tauri::command] get_device_reset_status`
  - `#[tauri::command] clear_local_session`
  - `#[tauri::command] get_auth_state`
  - `command_handler()` using `tauri::generate_handler!`
  - `register_auth_commands(builder).invoke_handler(...)`
  - `AuthAppState` using `tauri::State`
  - `AppDataStateStore::from_app_handle` using `tauri::Manager`
- Build/configuration assumptions: workspace-level dependency inheritance and path dependencies; no full Tauri app shell found in analyzed files

Graph evidence:

- Graphify summary: 405 nodes, 854 edges, 10 communities
- Key nodes include `AuthService`, `AuthAppState`, `KeychainSecretStore`, `AppDataStateStore`, `HttpWorkerClient`, `WorkerApp`, and `InMemoryWorkerStore`

## 4. Cross-Repository Dependency Compatibility

### Compatible dependency areas

| Dependency | Evidence | Assessment |
|---|---|---|
| `serde` | all repos use major `1`, derive where needed | compatible |
| `serde_json` | all repos use major `1` | compatible |
| Rust edition | all packages use `2021` | compatible |
| Tauri | only `user-reg/auth-licensing-tauri` declares `tauri = "2"` | no direct cross-version conflict |

### Confirmed dependency issues and risks

| Area | Repositories | Evidence | Consequence | Severity | Status |
|---|---|---|---|---|---|
| `shared-contracts` external dependency | `admin-dashboard`, `auth-core`, `shared-contracts` | `admin-dashboard` has `shared-contracts = { git = "ssh://git@github.com/razauh/shared-contracts.git" }`; `auth-core` has same git dependency with `version = "1.0"`; source imports use `shared_contracts::...` | In a single crate, this should not remain an external git dependency if `shared-contracts` is merged as an internal module. Imports must be rewritten or compatibility re-exported. | High | Confirmed |
| `thiserror` major mismatch | standalone crates and `user-reg` | `admin-dashboard`, `auth-core`, `shared-contracts` use `thiserror = "1"`; `user-reg` workspace uses `thiserror = "2"` | One crate normally has one dependency named `thiserror`. Keeping both requires dependency renaming; standardizing on one version requires compile verification. | Medium | Confirmed |
| Workspace path dependencies | `user-reg` | `auth-licensing-tauri` and `licensing-worker` depend on `auth-licensing-core` by workspace/path | After collapsing to modules, imports like `auth_licensing_core::...` must resolve through module paths or a crate-level compatibility alias. | High | Confirmed |

No native-linking conflict is confirmed from metadata:

- `cargo metadata --no-deps` reports no `links` values.
- No `build.rs` files were found in the inspected tree.
- `keyring` may introduce platform-specific runtime/package requirements, but this is a possible packaging risk, not a confirmed linking conflict from available metadata.

## 5. Rust Crate and Module Merge Risks

The repositories can become modules in one crate if the merge preserves module boundaries. The risky approach is flattening all current `src/*.rs` files into one shared `src/` namespace.

### Confirmed namespace conflicts

| Conflict | Repositories | Evidence | Consequence | Severity | Status |
|---|---|---|---|---|---|
| Duplicate module names | `admin-dashboard`, `auth-core`, `shared-contracts`, `user-reg` | Graph/source labels include duplicate `adapters`, `auth`, `compatibility`, `state`, and multiple `lib.rs` files | Direct flattening into root `src/` causes file/module name conflicts. | Medium | Confirmed |
| Duplicate public type names | `auth-core`, `shared-contracts`, `user-reg` | Graph duplicate labels include `AuthError`, `AuditEvent`, `DeviceResetRequest`, `SessionState`, `InMemorySecretStore` | Root-level glob re-exports could produce ambiguous exports or accidental use of the wrong type. | Medium | Confirmed |
| Crate-root import assumptions | all repos except `shared-contracts` itself | source uses `crate::...` for local modules and external crate names like `shared_contracts::...` / `auth_licensing_core::...` | Moving code into nested modules changes `crate::...` resolution unless imports are rewritten or compatibility modules are introduced. | High | Confirmed |

Practical merge guidance:

- Keep each repository under a unique wrapper module.
- Avoid `pub use admin_dashboard::*`, `pub use auth_core::*`, or `pub use user_reg::*` at the unified crate root until API conflicts are intentionally resolved.
- Provide compatibility shims only where needed, for example `pub mod shared_contracts` at crate root and a module alias strategy for `auth_licensing_core`.

## 6. Tauri Integration Risks

Only `user-reg/auth-licensing-tauri` contains confirmed Tauri code. `admin-dashboard`, `auth-core`, and `shared-contracts` are plain library crates with no Tauri dependency, commands, plugins, setup hooks, menus, trays, windows, or capabilities found in the inspected source/metadata.

### Confirmed Tauri integration issue

| Area | Repositories | Evidence | Consequence | Severity | Status |
|---|---|---|---|---|---|
| Single invoke handler composition | `user-reg/auth-licensing-tauri` and final app | `register_auth_commands<R>(builder)` calls `builder.invoke_handler(command_handler::<R>())`; `command_handler()` uses `tauri::generate_handler![...]` | A final Tauri app must compose all module commands into one app-level `generate_handler!`. If other modules later add command handlers, multiple independent `invoke_handler` calls are not a safe composition model. | Medium | Confirmed |

### Tauri command surface

The command names that must be reserved or checked in the final app are:

- `activate_license`
- `validate_session`
- `request_device_reset`
- `get_device_reset_status`
- `clear_local_session`
- `get_auth_state`

No conflict with the other three repositories is confirmed because no other Tauri commands were found there.

### Possible Tauri risks requiring final app verification

- Final `tauri.conf.json` was not available.
- Final capabilities/permissions files were not available.
- Final frontend invoke names were not available.
- Final plugin list and setup hook ordering were not available.

Therefore, plugin, menu, tray, window, capability, and permission conflicts cannot be confirmed from current Graphify output.

## 7. Build, Packaging, and Platform Risks

### Confirmed build/configuration risks

| Area | Repositories | Evidence | Consequence | Severity | Status |
|---|---|---|---|---|---|
| Cargo workspace collapse | `user-reg` | `user-reg/Cargo.toml` is a workspace manifest with `[workspace.dependencies]` and three members | A single-crate merge cannot copy this manifest shape directly. Dependencies and module paths must be moved into the unified crate manifest/module tree. | High | Confirmed |
| External git dependency removal | `admin-dashboard`, `auth-core` | both use git `shared-contracts` | Unified crate should not fetch a merged internal module from git. Keeping both internal and git versions can create duplicate contract types. | High | Confirmed |

### Non-confirmed platform risks

| Risk | Evidence | Needed verification |
|---|---|---|
| Keyring backend availability | `KeychainSecretStore` uses `keyring = "3"` and OS credential APIs | Verify target Linux/macOS/Windows packaging and runtime secret-store availability |
| HTTP/TLS backend packaging | `reqwest` uses `default-features = false`, `features = ["json", "rustls-tls"]` | Likely lower native risk than system TLS, but final packaging should still compile target triples |
| External binary requirements | no `build.rs`, no external command invocation found | Verify final app build scripts if added later |

No overlapping frontend paths or asset paths were confirmed in the four analyzed repos. `user-reg` docs mention a Tauri/customer app concept, but the inspected project files do not expose a frontend directory or Tauri app config.

## 8. Runtime Compatibility Risks

### Confirmed runtime/state concerns

| Area | Repositories | Evidence | Consequence | Severity | Status |
|---|---|---|---|---|---|
| Generic app-data filenames | `user-reg/auth-licensing-tauri` | `AppDataStateStore` writes `session_state.json` and `reset_status.json` under `app.path().app_data_dir()` | If the final app already uses these filenames for other modules, state can collide or be overwritten. | Low/Medium | Confirmed |
| Generic keyring keys | `user-reg/auth-licensing-tauri` | `KeychainSecretStore` keys are `license_key`, `access_token`, `device_keypair`; service name is supplied by caller | If the final app reuses a service name already used by another auth module, secrets can collide. | Low/Medium | Confirmed |
| Blocking mutexes in async-adjacent code | `user-reg` | test stores and worker store use `std::sync::{Arc, Mutex}`; async traits call lock/unlock in methods | Current inspected methods do not prove a deadlock, but production sharing should verify no lock guard crosses `.await`. | Low | Possible |

### Serialization and IPC compatibility

There are two contract families:

- `shared-contracts` defines wire DTOs such as `ActivateRequest`, `ActivateResponse`, `DeviceResetRequest`, `DeviceResetStatusResponse`, `ApiError`, `AuditEvent`.
- `user-reg/auth-licensing-core` and `auth-licensing-tauri` define domain and IPC types such as `ActivationRequest`, `ActivationOutcome`, `DeviceResetStatus`, `ActivationView`, `AuthStateView`, `AuthCommandError`.

Graph duplicate labels show overlaps such as `DeviceResetRequest`, `AuditEvent`, and `SessionState`. This is not automatically wrong if modules remain separate. It becomes risky if the merge tries to unify these types or re-export all of them at crate root.

Needed verification:

- Compare serde shapes in `shared-contracts/src/dto.rs`, `shared-contracts/src/state.rs`, `user-reg/crates/auth-licensing-core/src/domain.rs`, `user-reg/crates/auth-licensing-core/src/state.rs`, and `user-reg/crates/auth-licensing-tauri/src/http_client.rs`.
- Confirm whether the final app expects one shared wire contract model or separate admin/customer/worker contract models.

## 9. Confirmed Issues

### CI-1: External `shared-contracts` dependency conflicts with internal module merge

- Affected repositories: `admin-dashboard`, `auth-core`, `shared-contracts`
- Area: Cargo dependency and imports
- Evidence: `admin-dashboard` and `auth-core` depend on git `shared-contracts`; source imports use `shared_contracts::dto`, `shared_contracts::errors`, `shared_contracts::state`, and `shared_contracts::versioning`
- Expected consequence: after merging `shared-contracts` internally, keeping the git dependency can create duplicate contract types or keep code coupled to the old external crate
- Severity: High
- Status: Confirmed

### CI-2: `user-reg` workspace layout cannot be copied directly into one crate

- Affected repository: `user-reg`
- Area: Cargo workspace/package configuration
- Evidence: root `[workspace]` with members `crates/auth-licensing-core`, `crates/auth-licensing-tauri`, `workers/licensing-worker`; workspace path dependency on `auth-licensing-core`
- Expected consequence: a single crate needs dependencies moved to one manifest and imports rewritten from package names to module paths
- Severity: High
- Status: Confirmed

### CI-3: `thiserror` major version mismatch

- Affected repositories: all
- Area: dependency version
- Evidence: `admin-dashboard`, `auth-core`, and `shared-contracts` use `thiserror = "1"`; `user-reg` workspace uses `thiserror = "2"`
- Expected consequence: unified crate must either standardize on one major or use renamed dependencies; standardizing may require compile fixes if derive behavior differs
- Severity: Medium
- Status: Confirmed

### CI-4: Duplicate module/file names block flattening

- Affected repositories: all
- Area: module layout
- Evidence: duplicate module/file labels include `adapters`, `auth`, `compatibility`, `state`, and multiple `lib.rs`
- Expected consequence: direct flattening into one `src/` directory causes path/name collisions
- Severity: Medium
- Status: Confirmed

### CI-5: Duplicate public type names make root glob exports unsafe

- Affected repositories: `auth-core`, `shared-contracts`, `user-reg`
- Area: public API namespace
- Evidence: duplicate graph labels include `AuthError`, `AuditEvent`, `DeviceResetRequest`, `SessionState`, and `InMemorySecretStore`
- Expected consequence: root `pub use module::*` can create ambiguous or misleading public API
- Severity: Medium
- Status: Confirmed

### CI-6: Tauri invoke handler must be composed at app level

- Affected repository: `user-reg/auth-licensing-tauri`
- Area: Tauri command registration
- Evidence: `register_auth_commands` calls `builder.invoke_handler(command_handler())`; command handler registers six commands
- Expected consequence: final Tauri app must have one composed handler. Future command modules cannot be bolted on through multiple independent `invoke_handler` calls.
- Severity: Medium
- Status: Confirmed

### CI-7: Generic local state and key names can collide in final app

- Affected repository: `user-reg/auth-licensing-tauri`
- Area: runtime storage/security
- Evidence: app data filenames `session_state.json`, `reset_status.json`; keyring keys `license_key`, `access_token`, `device_keypair`
- Expected consequence: possible collision with existing app state/secrets if the final unified app already uses those names under the same app data directory or service name
- Severity: Low/Medium
- Status: Confirmed

## 10. Possible Issues Requiring Source Verification

### PI-1: DTO/domain model semantic overlap

- Affected repositories: `shared-contracts`, `user-reg`
- Area: serialization/wire contracts
- Evidence: duplicate or similar graph labels include `DeviceResetRequest`, `AuditEvent`, `SessionState`, and activation/reset concepts
- Expected consequence: developers could accidentally mix admin/shared contract types with customer/worker domain types
- Severity: Medium
- Status: Requires source verification
- Inspect: `shared-contracts/src/dto.rs`, `shared-contracts/src/state.rs`, `shared-contracts/src/events.rs`, `user-reg/crates/auth-licensing-core/src/domain.rs`, `user-reg/crates/auth-licensing-core/src/state.rs`, `user-reg/crates/auth-licensing-tauri/src/http_client.rs`

### PI-2: Final Tauri capabilities and permissions may not expose commands

- Affected repositories: final unified Tauri app and `user-reg/auth-licensing-tauri`
- Area: Tauri capabilities/permissions
- Evidence: command functions exist, but final app capability files were not present in analyzed output
- Expected consequence: commands may compile but be unavailable or denied at runtime if not permitted
- Severity: Medium
- Status: Requires source verification
- Inspect: final app `tauri.conf.json`, `capabilities/*.json`, permission files

### PI-3: Existing final app command names may collide

- Affected repositories: final unified Tauri app and `user-reg/auth-licensing-tauri`
- Area: IPC command namespace
- Evidence: user-reg reserves six command names; no final app handler was available
- Expected consequence: duplicate command names would create ambiguous frontend IPC behavior
- Severity: Medium
- Status: Requires source verification
- Inspect: final app command registry and frontend `invoke(...)` calls

### PI-4: Keyring backend and OS packaging behavior

- Affected repository: `user-reg/auth-licensing-tauri`
- Area: platform packaging/runtime dependencies
- Evidence: `KeychainSecretStore` uses `keyring = "3"` and OS credential APIs
- Expected consequence: Linux/macOS/Windows packaging may need secret-store/keychain prerequisites or fallback behavior
- Severity: Medium
- Status: Requires source verification
- Inspect: final target OS packaging, CI target triples, runtime install docs

### PI-5: Blocking mutexes under async runtime

- Affected repository: `user-reg`
- Area: async/runtime compatibility
- Evidence: `InMemorySecretStore`, `InMemoryWorkerStore`, and test support use `Arc<Mutex<...>>`; several traits are async
- Expected consequence: currently not proven unsafe, but final shared runtime should verify no mutex guard is held across `.await`
- Severity: Low
- Status: Requires source verification
- Inspect: full implementations in `test_support.rs`, `persistence.rs`, and `workers/licensing-worker/src/lib.rs`

### PI-6: Final frontend/assets/build path conflicts

- Affected repositories: final unified app
- Area: frontend/build configuration
- Evidence: analyzed repos do not include a complete frontend/Tauri app config
- Expected consequence: asset path and build output conflicts cannot be confirmed or dismissed from Graphify alone
- Severity: Low/Medium
- Status: Requires source verification
- Inspect: final frontend directory, `tauri.conf.json`, bundler config, generated output paths

## 11. Non-Issues / Compatible Areas

- All packages use Rust edition `2021`.
- All analyzed packages are library targets; no conflicting binary entrypoints were found.
- No `build.rs` files were found in the analyzed project tree.
- No Cargo `links` metadata was reported by `cargo metadata --no-deps`.
- No direct Tauri version conflict was found; only `user-reg/auth-licensing-tauri` depends on `tauri = "2"`.
- No Tauri plugin, menu, tray, window, setup-hook, or app-entrypoint conflicts were confirmed in the available source.
- No declared feature flags were found in the package manifests that create a confirmed feature incompatibility.
- `serde` and `serde_json` align on major version `1`.
- `reqwest` is configured with `default-features = false` and `rustls-tls`, which avoids a confirmed native TLS conflict in the analyzed metadata.
- The duplicate test helper names such as `FakeApi` and `FakeStore` are not production conflicts if tests remain module-scoped.

## 12. Merge Readiness Verdict

Final verdict: **Mergeable with moderate refactoring**.

Justification:

- The repositories are not fundamentally incompatible: same Rust edition, library crate targets, no confirmed native-link conflicts, and no competing Tauri versions.
- The main confirmed risks are mechanical but non-trivial: dependency unification, import rewriting, workspace collapse, namespace preservation, and composed Tauri command registration.
- A low-risk merge should preserve repository boundaries as internal modules. A flattening merge would be high-risk because confirmed module and type name collisions already exist.
- The final Tauri app shell must be inspected before runtime/security/capability compatibility can be fully signed off.

## 13. Recommended Next Steps

1. Create the unified crate with explicit wrapper modules for each original repository.
2. Move `shared-contracts` first and expose it as `crate::shared_contracts`.
3. Rewrite `admin-dashboard` and `auth-core` imports from external `shared_contracts::...` to internal module paths or provide a compatibility re-export.
4. Collapse `user-reg` workspace packages into nested modules, preserving current internal boundaries:
   - `user_reg::auth_licensing_core`
   - `user_reg::auth_licensing_tauri`
   - `user_reg::licensing_worker`
5. Avoid root-level glob re-exports until duplicate public names are intentionally resolved.
6. Standardize `thiserror` version in the unified manifest, then run compile/tests to verify derives and error conversions.
7. Compose Tauri commands in the final app-level `generate_handler!`; do not rely on multiple independent `invoke_handler` calls.
8. Decide final app storage/keyring names before production use, preferably namespaced by module or app feature.
9. Verify final `tauri.conf.json`, capabilities, frontend invoke names, and platform packaging after the module merge.
10. Run full test suite after each major merge phase: shared contracts, admin/auth libraries, user-reg core, Tauri integration.
