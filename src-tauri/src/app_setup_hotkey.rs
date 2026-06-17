use crate::hotkey;

pub fn register_startup_hotkey(handle: &tauri::AppHandle, hotkey: &str) {
    let _ = hotkey::global_hotkey::register(handle, hotkey);
}
