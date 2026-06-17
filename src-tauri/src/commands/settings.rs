use tauri::AppHandle;

use crate::config::app_config::AppSettings;
use crate::storage::settings_store;

use super::settings_autostart::sync_launch_at_startup;
use super::settings_hotkey::sync_settings_hotkey;
use super::settings_window::apply_always_on_top;

#[tauri::command]
pub fn get_settings(app: AppHandle) -> Result<AppSettings, String> {
    settings_store::read(&app)
}

#[tauri::command]
pub fn save_settings(app: AppHandle, settings: AppSettings) -> Result<AppSettings, String> {
    let old_settings = settings_store::read(&app).unwrap_or_default();

    sync_settings_hotkey(&app, &old_settings, &settings)?;
    sync_launch_at_startup(&app, &old_settings, &settings)?;

    settings_store::write(&app, &settings)?;
    apply_always_on_top(&app, settings.always_on_top)?;

    Ok(settings)
}
