use tauri::{AppHandle, Manager};

use crate::config::app_config::AppSettings;
use crate::hotkey::global_hotkey;
use crate::storage::settings_store;

#[tauri::command]
pub fn get_settings(app: AppHandle) -> Result<AppSettings, String> {
    settings_store::read(&app)
}

#[tauri::command]
pub fn save_settings(app: AppHandle, settings: AppSettings) -> Result<AppSettings, String> {
    let old_settings = settings_store::read(&app).unwrap_or_default();

    if old_settings.hotkey != settings.hotkey {
        global_hotkey::replace(&app, &old_settings.hotkey, &settings.hotkey)?;
    }

    settings_store::write(&app, &settings)?;

    if let Some(window) = app.get_webview_window("main") {
        window
            .set_always_on_top(settings.always_on_top)
            .map_err(|_| "窗口置顶设置失败".to_string())?;
    }

    Ok(settings)
}
