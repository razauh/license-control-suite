# TC-MOD-03 Migration Record

- Task ID: `TC-MOD-03`
- Title: `Migrate Auth Core Module`
- Date: `2026-05-14`

## Scope Completed

- Migrated auth-core source into:
  - `src/modules/auth_core/adapters.rs`
  - `src/modules/auth_core/auth.rs`
  - `src/modules/auth_core/compatibility.rs`
  - `src/modules/auth_core/models.rs`
  - `src/modules/auth_core/policy.rs`
  - `src/modules/auth_core/reset.rs`
  - `src/modules/auth_core/session.rs`
- Updated `src/modules/auth_core/mod.rs` to export migrated modules.
- Rewrote imports:
  - external `shared_contracts::...` to `crate::modules::shared_contracts::...`
  - local `crate::...` to module-local `super::...` where required.
- Ported tests into unified integration suite:
  - `tests/integration/auth_core_activation_int.rs`
  - `tests/integration/auth_core_reset_int.rs`
  - `tests/integration/auth_core_compatibility_unit.rs`
  - `tests/integration/auth_core_policy_unit.rs`
  - `tests/integration/auth_core_session_unit.rs`
  - `tests/integration/auth_core_contracts_parity.rs`

## Test-First Notes

- Auth-core integration/unit/contract parity tests were added in unified test paths before final import adaptation.

## Unresolved Issues

- Auth-core test execution is pending user-run verification.
- Shared-contracts and admin-dashboard regression checks are pending user-run verification.
- Known unresolved issues:
  - Admin-dashboard and shared-contracts regression verification pending user-run execution.

## Pending User-Run Verification Commands

```bash
cd /home/pc/Downloads/inf/plan/license-control-suite && cargo test auth_core
cd /home/pc/Downloads/inf/plan/license-control-suite && cargo test shared_contracts
cd /home/pc/Downloads/inf/plan/license-control-suite && cargo test admin_dashboard
```
