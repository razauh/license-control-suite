use license_control_suite::modules::user_reg::auth_licensing_tauri::{
    command_handler, register_auth_commands,
};

pub fn app_invoke_handler<R>() -> impl Fn(tauri::ipc::Invoke<R>) -> bool + Send + Sync + 'static
where
    R: tauri::Runtime,
{
    command_handler::<R>()
}

pub fn configure_app_builder<R>(builder: tauri::Builder<R>) -> tauri::Builder<R>
where
    R: tauri::Runtime,
{
    register_auth_commands(builder)
}

fn main() {
    println!("license-control-suite skeleton");
}
