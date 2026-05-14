use license_control_suite::modules;

#[test]
fn module_namespaces_are_exposed() {
    let _ = core::mem::size_of_val(&modules::shared_contracts::NAMESPACE);
    let _ = core::mem::size_of_val(&modules::admin_dashboard::NAMESPACE);
    let _ = core::mem::size_of_val(&modules::auth_core::NAMESPACE);
    let _ = core::mem::size_of_val(&modules::user_reg::NAMESPACE);
}
