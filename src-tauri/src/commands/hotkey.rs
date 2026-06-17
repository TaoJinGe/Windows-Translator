use tauri::AppHandle;

use crate::hotkey::global_hotkey;

#[tauri::command]
pub fn update_global_hotkey(app: AppHandle, hotkey: String) -> Result<(), String> {
    global_hotkey::update(&app, &hotkey)
}
