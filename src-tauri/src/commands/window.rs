use tauri::AppHandle;

use crate::window::window_control;

#[tauri::command]
pub fn show_main_window(app: AppHandle) -> Result<(), String> {
    window_control::show(&app)
}

#[tauri::command]
pub fn hide_main_window(app: AppHandle) -> Result<(), String> {
    window_control::hide(&app)
}

#[tauri::command]
pub fn toggle_main_window(app: AppHandle) -> Result<(), String> {
    window_control::toggle(&app)
}
