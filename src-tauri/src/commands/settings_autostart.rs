use tauri::AppHandle;

use crate::autostart;
use crate::config::app_config::AppSettings;

pub fn sync_launch_at_startup(
    app: &AppHandle,
    old_settings: &AppSettings,
    settings: &AppSettings,
) -> Result<(), String> {
    if old_settings.launch_at_startup != settings.launch_at_startup {
        autostart::sync(app, settings.launch_at_startup)?;
    }

    Ok(())
}
