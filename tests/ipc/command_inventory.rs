use license_control_suite::app_command_names;
use std::collections::BTreeSet;

#[test]
fn command_inventory_has_exactly_six_known_commands() {
    let names = app_command_names();
    assert_eq!(names.len(), 6);
    assert_eq!(
        names,
        &[
            "activate_license",
            "validate_session",
            "request_device_reset",
            "get_device_reset_status",
            "clear_local_session",
            "get_auth_state",
        ]
    );
}

#[test]
fn command_inventory_has_no_duplicates() {
    let names = app_command_names();
    let unique: BTreeSet<_> = names.iter().copied().collect();
    assert_eq!(unique.len(), names.len());
}
