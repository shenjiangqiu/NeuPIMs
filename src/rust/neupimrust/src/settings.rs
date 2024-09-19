use std::{ffi::c_char, sync::Mutex};
use tracing::info;
static SETTINGS: Mutex<Option<Settings>> = Mutex::new(None);

#[repr(C)]
#[derive(Debug, serde::Deserialize)]
pub struct Settings {
    fast_read: bool,
    fast_icnt: bool,
}
#[no_mangle]
pub extern "C" fn init_settings_with_file(file_path: *const c_char) {
    let file_path = unsafe { std::ffi::CStr::from_ptr(file_path) };
    let file_path = file_path.to_str().unwrap();
    init_settings_with_file_(file_path);
}

fn init_settings_with_file_(file_path: &str) {
    let settings = std::fs::read_to_string(file_path).unwrap();
    let settings = toml::from_str(&settings).unwrap();
    set_settings(settings);
}

#[no_mangle]
pub extern "C" fn init_settings() {
    let file_path = "sjq.toml";
    init_settings_with_file_(file_path);
}

fn set_settings(table: Settings) {
    let mut settings = SETTINGS.lock().unwrap();
    info!("set_settings: {:?}", table);
    *settings = Some(table);
}

#[no_mangle]
pub extern "C" fn get_settings() -> *const Settings {
    let settings = SETTINGS.lock().unwrap();
    match &*settings {
        Some(settings) => settings,
        None => std::ptr::null(),
    }
}
