use tauri::AppHandle;

use crate::config::app_config::AppSettings;
use crate::hotkey::global_hotkey;

pub fn sync_settings_hotkey(
    app: &AppHandle,
    old_settings: &AppSettings,
    settings: &AppSettings,
) -> Result<(), String> {
    if old_settings.hotkey != settings.hotkey {
        global_hotkey::replace(app, &old_settings.hotkey, &settings.hotkey)?;
    }

    Ok(())
}
