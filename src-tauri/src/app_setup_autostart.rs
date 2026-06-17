use crate::autostart;

pub fn sync_startup_autostart(handle: &tauri::AppHandle, launch_at_startup: bool) {
    let _ = autostart::sync(handle, launch_at_startup);
}
