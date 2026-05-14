# TC-MOD-06 Migration Record

- Task ID: `TC-MOD-06`
- Title: `Migrate Licensing Worker Module`
- Date: `2026-05-14`

## Scope Completed

- Migrated worker source into:
  - `src/modules/user_reg/licensing_worker/mod.rs`
- Wired module export:
  - `src/modules/user_reg/mod.rs` exports `licensing_worker`
- Rewrote imports from:
  - `auth_licensing_core::...`
  - to `crate::modules::user_reg::auth_licensing_core::...`
- Added integration tests:
  - `tests/integration/licensing_worker_domain_int.rs`

## Test-First Notes

- Worker domain tests were ported before module export wiring was completed.
- Coverage includes activation, validation/reset approval behavior, authorization checks, not-found reset status, and audit token redaction.

## Unresolved Issues

- Licensing worker test execution is pending user-run verification.
- User-reg core regression verification is pending user-run verification.
- Known unresolved issues:
  - User-reg core and cross-module regression verification pending user-run execution.

## Pending User-Run Verification Commands

```bash
cd /home/pc/Downloads/inf/plan/license-control-suite && cargo test licensing_worker
cd /home/pc/Downloads/inf/plan/license-control-suite && cargo test user_reg_core
```
