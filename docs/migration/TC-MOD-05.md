# TC-MOD-05 Migration Record

- Task ID: `TC-MOD-05`
- Title: `Migrate User-Reg Tauri Integration Module`
- Date: `2026-05-14`

## Scope Completed

- Migrated Tauri integration source into:
  - `src/modules/user_reg/auth_licensing_tauri/mod.rs`
  - `src/modules/user_reg/auth_licensing_tauri/commands.rs`
  - `src/modules/user_reg/auth_licensing_tauri/http_client.rs`
  - `src/modules/user_reg/auth_licensing_tauri/persistence.rs`
- Wired module export:
  - `src/modules/user_reg/mod.rs` exports `auth_licensing_tauri`
- Rewrote `auth_licensing_core::...` imports to unified internal path:
  - `crate::modules::user_reg::auth_licensing_core::...`
- Ported tests:
  - `tests/integration/user_reg_tauri_e2e_acceptance.rs`
  - `tests/ipc/user_reg_tauri_security_redaction.rs`
  - `tests/ipc/user_reg_tauri_command_inventory.rs`

## Test-First Notes

- Tauri integration tests were ported before module export wiring was finalized.
- Command inventory test enforces exact six command names expected by task card.

## Unresolved Issues

- User-reg tauri integration tests are pending user-run verification.
- IPC and regression verification across prior modules remain pending user-run execution.
- Known unresolved issues:
  - User-reg core and cross-module regression verification pending user-run execution.

## Pending User-Run Verification Commands

```bash
cd /home/pc/Downloads/inf/plan/license-control-suite && cargo test user_reg_tauri
cd /home/pc/Downloads/inf/plan/license-control-suite && cargo test ipc
cd /home/pc/Downloads/inf/plan/license-control-suite && cargo test user_reg_core
cd /home/pc/Downloads/inf/plan/license-control-suite && cargo test auth_core
cd /home/pc/Downloads/inf/plan/license-control-suite && cargo test shared_contracts
cd /home/pc/Downloads/inf/plan/license-control-suite && cargo test admin_dashboard
```
