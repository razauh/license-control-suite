pub mod modules;

pub const APP_COMMAND_NAMES: [&str; 6] = [
    "activate_license",
    "validate_session",
    "request_device_reset",
    "get_device_reset_status",
    "clear_local_session",
    "get_auth_state",
];

pub fn app_command_names() -> &'static [&'static str] {
    &APP_COMMAND_NAMES
}
