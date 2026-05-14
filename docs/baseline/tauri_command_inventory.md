# TC-03 Tauri Command Inventory

- Task ID: `TC-03`
- Date: `2026-05-14`

## Sources Inspected

- `/home/pc/Downloads/inf/plan/user-reg/crates/auth-licensing-tauri/src/commands.rs`
- `/home/pc/Downloads/inf/plan/user-reg/crates/auth-licensing-tauri/src/persistence.rs`
- `/home/pc/Downloads/inf/plan/user-reg/crates/auth-licensing-tauri/src/http_client.rs`
- `/home/pc/Downloads/inf/plan/user-reg/crates/auth-licensing-tauri/tests/e2e_acceptance.rs`
- `/home/pc/Downloads/inf/plan/user-reg/crates/auth-licensing-tauri/tests/security_redaction.rs`

## Confirmed Tauri Commands (6)

From `#[tauri::command]` functions and `command_names()` in `commands.rs`:

1. `activate_license`
2. `validate_session`
3. `request_device_reset`
4. `get_device_reset_status`
5. `clear_local_session`
6. `get_auth_state`

`command_handler()` uses `tauri::generate_handler![...]` with these same six commands.

## Persistence Filenames and Key Names

From `persistence.rs`:

- Local state files:
  - `session_state.json`
  - `reset_status.json`
- Keyring entry keys:
  - `license_key`
  - `access_token`
  - `device_keypair`

## IPC/HTTP DTO Assumptions (baseline notes)

From `http_client.rs`:

- HTTP client maps worker responses into domain outcomes using typed request/response bodies.
- Error mapping is based on both HTTP status code and optional response `code`.
- Request debug formatting redacts sensitive fields for:
  - activation request body,
  - validation request body,
  - reset request body.

## Risk Mapping Notes

- CI-6: single handler composition concern is valid because module provides local handler/registration helpers.
- CI-7: generic storage and key names are confirmed and should be namespaced at unified-app integration stage.
- PI-2/PI-3: command names and contract surface are now explicitly inventoried for later collision checks.

## Unresolved Issues

- Frontend/app shell source remains unverified in legacy roots.
- User-run verification commands for TC-03 are pending confirmation.

## Pending User-Run Verification Commands

```bash
bash /home/pc/Downloads/inf/plan/license-control-suite/docs/baseline/check_tauri_inventory.sh
rg -n "#\\[tauri::command\\]|generate_handler|invoke_handler|session_state.json|reset_status.json|license_key|access_token|device_keypair" /home/pc/Downloads/inf/plan/user-reg
find /home/pc/Downloads/inf/plan -name 'tauri.conf.*' -o -name package.json -o -path '*/src-tauri/*'
```
