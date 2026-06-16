use std::fs;
use std::path::Path;

use tauri::AppHandle;

use crate::config::app_config::AppSettings;

use super::paths;

pub fn read(app: &AppHandle) -> Result<AppSettings, String> {
    let path = paths::settings_path(app)?;
    read_from_path(&path)
}

fn read_from_path(path: &Path) -> Result<AppSettings, String> {
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
    write_to_path(&path, settings)
}

fn write_to_path(path: &Path, settings: &AppSettings) -> Result<(), String> {
    let content = serde_json::to_string_pretty(settings).map_err(|_| "无法保存设置".to_string())?;

    fs::write(path, content).map_err(|_| "无法写入设置".to_string())
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    use super::{read_from_path, write_to_path};
    use crate::config::app_config::AppSettings;

    #[test]
    fn writes_and_reads_settings() {
        let path = test_path("settings-roundtrip.json");
        let settings = AppSettings {
            api_key: "test-key".to_string(),
            model: "test-model".to_string(),
            hotkey: "Ctrl+Alt+Y".to_string(),
            ..AppSettings::default()
        };

        write_to_path(&path, &settings).unwrap();
        let saved = read_from_path(&path).unwrap();

        assert_eq!(saved.api_key, "test-key");
        assert_eq!(saved.model, "test-model");
        assert_eq!(saved.hotkey, "Ctrl+Alt+Y");

        let _ = fs::remove_file(path);
    }

    #[test]
    fn missing_settings_file_returns_defaults() {
        let path = test_path("settings-missing.json");
        let settings = read_from_path(&path).unwrap();

        assert_eq!(settings.hotkey, AppSettings::default().hotkey);
        assert_eq!(settings.model, AppSettings::default().model);
    }

    fn test_path(name: &str) -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();

        std::env::temp_dir().join(format!("windows-translator-{nanos}-{name}"))
    }
}
