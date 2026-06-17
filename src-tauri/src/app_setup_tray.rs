use crate::tray;

pub fn create_startup_tray(handle: &tauri::AppHandle) -> tauri::Result<()> {
    tray::app_tray::create(handle)
}
