use license_control_suite::app_command_names;

#[test]
fn six_commands_remain_reserved_for_tauri_capability_mapping() {
    assert_eq!(app_command_names().len(), 6);
}
