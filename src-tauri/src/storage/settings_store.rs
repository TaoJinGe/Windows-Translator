use std::fs;

use tauri::AppHandle;

use crate::config::app_config::AppSettings;

use super::paths;

pub fn read(app: &AppHandle) -> Result<AppSettings, String> {
    let path = paths::settings_path(app)?;

    if !path.exists() {
        return Ok(AppSettings::default());
    }

    let content = fs::read_to_string(path).map_err(|_| "无法读取设置".to_string())?;
    let settings: AppSettings =
        serde_json::from_str(&content).map_err(|_| "设置文件格式无效".to_string())?;

    Ok(settings.normalize())
}

pub fn write(app: &AppHandle, settings: &AppSettings) -> Result<(), String> {
    let path = paths::settings_path(app)?;
    let content = serde_json::to_string_pretty(settings).map_err(|_| "无法保存设置".to_string())?;

    fs::write(path, content).map_err(|_| "无法写入设置".to_string())
}
