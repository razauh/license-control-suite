# TC-TAURI-01 Migration Record

- Task ID: `TC-TAURI-01`
- Title: `Compose Unified Tauri Command Handler`
- Date: `2026-05-14`

## Scope Completed

- Added app-level command inventory in library API:
  - `src/lib.rs`
  - `APP_COMMAND_NAMES`
  - `app_command_names()`
- Added app-level invoke handler composition in entrypoint:
  - `src/main.rs`
  - `app_invoke_handler<R>()`
  - `configure_app_builder<R>(builder)`
- The handler includes exactly six commands from `user_reg::auth_licensing_tauri`:
  - `activate_license`
  - `validate_session`
  - `request_device_reset`
  - `get_device_reset_status`
  - `clear_local_session`
  - `get_auth_state`
- Added IPC inventory tests:
  - `tests/ipc/command_inventory.rs`

## Test-First Notes

- `tests/ipc/command_inventory.rs` was created before app-level handler wiring.
- Tests validate exact command set and no duplicate command names.

## Unresolved Issues

- Command inventory and compile verification are pending user-run execution.
- Final Tauri app shell/capabilities are still not present, so runtime integration remains pending later cards.
- Known unresolved issues:
  - User-reg core and cross-module regression verification pending user-run execution.

## Pending User-Run Verification Commands

```bash
cd /home/pc/Downloads/inf/plan/license-control-suite && cargo test command_inventory
cd /home/pc/Downloads/inf/plan/license-control-suite && cargo check
```
