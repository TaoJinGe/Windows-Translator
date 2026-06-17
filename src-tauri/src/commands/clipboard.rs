use tauri::AppHandle;
use tauri_plugin_clipboard_manager::ClipboardExt;

#[tauri::command]
pub fn copy_to_clipboard(app: AppHandle, text: String) -> Result<(), String> {
    app.clipboard()
        .write_text(text)
        .map_err(|_| "复制失败".to_string())
}
