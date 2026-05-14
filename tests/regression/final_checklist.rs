use license_control_suite::app_command_names;
use std::fs;
use std::path::Path;

#[test]
fn no_external_shared_contracts_git_dependency_declared() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let cargo_toml = fs::read_to_string(root.join("Cargo.toml")).expect("Cargo.toml should exist");
    assert!(!cargo_toml.contains("git ="));
    assert!(!cargo_toml.contains("shared-contracts"));
}

#[test]
fn module_namespace_files_exist() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    assert!(root.join("src/modules/shared_contracts/mod.rs").exists());
    assert!(root.join("src/modules/admin_dashboard/mod.rs").exists());
    assert!(root.join("src/modules/auth_core/mod.rs").exists());
    assert!(root.join("src/modules/user_reg/mod.rs").exists());
}

#[test]
fn no_root_glob_exports_in_lib() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let lib_rs = fs::read_to_string(root.join("src/lib.rs")).expect("src/lib.rs should exist");
    assert!(!lib_rs.contains("pub use modules::*"));
}

#[test]
fn all_known_tauri_command_names_present() {
    let names = app_command_names();
    assert_eq!(names.len(), 6);
    assert!(names.contains(&"activate_license"));
    assert!(names.contains(&"validate_session"));
    assert!(names.contains(&"request_device_reset"));
    assert!(names.contains(&"get_device_reset_status"));
    assert!(names.contains(&"clear_local_session"));
    assert!(names.contains(&"get_auth_state"));
}
