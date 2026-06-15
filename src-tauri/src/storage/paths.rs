use std::fs;
use std::path::PathBuf;

use tauri::{AppHandle, Manager};

pub fn config_dir(app: &AppHandle) -> Result<PathBuf, String> {
    let dir = app
        .path()
        .app_config_dir()
        .map_err(|_| "无法读取配置目录".to_string())?;

    fs::create_dir_all(&dir).map_err(|_| "无法创建配置目录".to_string())?;
    Ok(dir)
}

pub fn settings_path(app: &AppHandle) -> Result<PathBuf, String> {
    Ok(config_dir(app)?.join("settings.json"))
}

pub fn history_path(app: &AppHandle) -> Result<PathBuf, String> {
    Ok(config_dir(app)?.join("history.json"))
}
