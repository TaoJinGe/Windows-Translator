use std::fs;
use std::path::Path;

use tauri::AppHandle;

use crate::commands::history::TranslationHistoryItem;

use super::paths;

const MAX_HISTORY: usize = 50;

pub fn read(app: &AppHandle) -> Result<Vec<TranslationHistoryItem>, String> {
    let path = paths::history_path(app)?;
    read_from_path(&path)
}

fn read_from_path(path: &Path) -> Result<Vec<TranslationHistoryItem>, String> {
    if !path.exists() {
        return Ok(Vec::new());
    }

    let content = fs::read_to_string(path).map_err(|_| "无法读取历史记录".to_string())?;
    serde_json::from_str(&content).map_err(|_| "历史记录格式无效".to_string())
}

pub fn append(app: &AppHandle, item: TranslationHistoryItem) -> Result<(), String> {
    let path = paths::history_path(app)?;
    append_to_path(&path, item)
}

fn append_to_path(path: &Path, item: TranslationHistoryItem) -> Result<(), String> {
    let mut items = read_from_path(path)?;
    items.insert(0, item);
    items.truncate(MAX_HISTORY);
    write_to_path(path, &items)
}

pub fn clear(app: &AppHandle) -> Result<(), String> {
    let path = paths::history_path(app)?;
    write_to_path(&path, &[])
}

fn write_to_path(path: &Path, items: &[TranslationHistoryItem]) -> Result<(), String> {
    let content =
        serde_json::to_string_pretty(items).map_err(|_| "无法保存历史记录".to_string())?;

    fs::write(path, content).map_err(|_| "无法写入历史记录".to_string())
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    use super::{append_to_path, read_from_path, write_to_path};
    use crate::commands::history::TranslationHistoryItem;

    #[test]
    fn append_keeps_latest_50_items() {
        let path = test_path("history-limit.json");

        for index in 0..55 {
            append_to_path(&path, item(index)).unwrap();
        }

        let items = read_from_path(&path).unwrap();

        assert_eq!(items.len(), 50);
        assert_eq!(items[0].id, "item-54");
        assert_eq!(items[49].id, "item-5");

        let _ = fs::remove_file(path);
    }

    #[test]
    fn clear_writes_empty_history() {
        let path = test_path("history-clear.json");

        append_to_path(&path, item(1)).unwrap();
        write_to_path(&path, &[]).unwrap();

        assert!(read_from_path(&path).unwrap().is_empty());

        let _ = fs::remove_file(path);
    }

    fn item(index: usize) -> TranslationHistoryItem {
        TranslationHistoryItem {
            id: format!("item-{index}"),
            source_text: format!("source-{index}"),
            translated_text: format!("translated-{index}"),
            source_lang: "zh-CN".to_string(),
            target_lang: "en".to_string(),
            model: "test-model".to_string(),
            created_at: format!("2026-06-16T00:00:{index:02}Z"),
        }
    }

    fn test_path(name: &str) -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();

        std::env::temp_dir().join(format!("windows-translator-{nanos}-{name}"))
    }
}
