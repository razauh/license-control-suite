# TC-MOD-04 Migration Record

- Task ID: `TC-MOD-04`
- Title: `Migrate User-Reg Auth Licensing Core`
- Date: `2026-05-14`

## Scope Completed

- Migrated user-reg auth-licensing-core source into:
  - `src/modules/user_reg/auth_licensing_core/domain.rs`
  - `src/modules/user_reg/auth_licensing_core/service.rs`
  - `src/modules/user_reg/auth_licensing_core/state.rs`
  - `src/modules/user_reg/auth_licensing_core/traits.rs`
  - `src/modules/user_reg/auth_licensing_core/test_support.rs`
  - `src/modules/user_reg/auth_licensing_core/mod.rs`
- Wired module export:
  - `src/modules/user_reg/mod.rs` exports `auth_licensing_core`
- Added integration tests for core workflows:
  - `tests/integration/user_reg_core_service_int.rs`

## Test-First Notes

- Core workflow integration tests were added before module wiring was completed.
- Tests target activation, validation, and reset behavior with existing fake support types.

## Unresolved Issues

- User-reg core test execution is pending user-run verification.
- Cross-module regression verification (shared/auth/admin) remains pending user-run execution.
- Known unresolved issues:
  - Auth-core and regression verification are pending user-run execution.

## Pending User-Run Verification Commands

```bash
cd /home/pc/Downloads/inf/plan/license-control-suite && cargo test user_reg_core
cd /home/pc/Downloads/inf/plan/license-control-suite && cargo test auth_core
cd /home/pc/Downloads/inf/plan/license-control-suite && cargo test shared_contracts
cd /home/pc/Downloads/inf/plan/license-control-suite && cargo test admin_dashboard
```
