use tauri::AppHandle;
use tauri_plugin_clipboard_manager::ClipboardExt;

use crate::hotkey::global_hotkey;
use crate::window::window_control;

#[tauri::command]
pub fn copy_to_clipboard(app: AppHandle, text: String) -> Result<(), String> {
    app.clipboard()
        .write_text(text)
        .map_err(|_| "复制失败".to_string())
}

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

#[tauri::command]
pub fn update_global_hotkey(app: AppHandle, hotkey: String) -> Result<(), String> {
    global_hotkey::update(&app, &hotkey)
}
