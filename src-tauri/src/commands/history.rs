use serde::{Deserialize, Serialize};
use tauri::AppHandle;

use crate::storage::history_store;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TranslationHistoryItem {
    pub id: String,
    pub source_text: String,
    pub translated_text: String,
    pub source_lang: String,
    pub target_lang: String,
    pub model: String,
    pub created_at: String,
}

#[tauri::command]
pub fn get_history(app: AppHandle) -> Result<Vec<TranslationHistoryItem>, String> {
    history_store::read(&app)
}

#[tauri::command]
pub fn clear_history(app: AppHandle) -> Result<(), String> {
    history_store::clear(&app)
}
