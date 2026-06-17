use tauri::AppHandle;

use crate::window::window_control;

pub fn toggle_window(app: &AppHandle) {
    let _ = window_control::toggle(app);
}
