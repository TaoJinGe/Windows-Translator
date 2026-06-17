use crate::config::app_config::AppSettings;
use crate::storage::settings_store;

pub fn read_startup_settings(handle: &tauri::AppHandle) -> AppSettings {
    settings_store::read(handle).unwrap_or_default()
}
