use std::fs;

use tauri::AppHandle;

use crate::commands::history::TranslationHistoryItem;

use super::paths;

const MAX_HISTORY: usize = 50;

pub fn read(app: &AppHandle) -> Result<Vec<TranslationHistoryItem>, String> {
    let path = paths::history_path(app)?;

    if !path.exists() {
        return Ok(Vec::new());
    }

    let content = fs::read_to_string(path).map_err(|_| "无法读取历史记录".to_string())?;
    serde_json::from_str(&content).map_err(|_| "历史记录格式无效".to_string())
}

pub fn append(app: &AppHandle, item: TranslationHistoryItem) -> Result<(), String> {
    let mut items = read(app)?;
    items.insert(0, item);
    items.truncate(MAX_HISTORY);
    write(app, &items)
}

pub fn clear(app: &AppHandle) -> Result<(), String> {
    write(app, &[])
}

fn write(app: &AppHandle, items: &[TranslationHistoryItem]) -> Result<(), String> {
    let path = paths::history_path(app)?;
    let content = serde_json::to_string_pretty(items).map_err(|_| "无法保存历史记录".to_string())?;

    fs::write(path, content).map_err(|_| "无法写入历史记录".to_string())
}
