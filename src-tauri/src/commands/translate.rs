use serde::{Deserialize, Serialize};
use tauri::AppHandle;

use crate::commands::history::TranslationHistoryItem;
use crate::api::openai_client;
use crate::storage::history_store;
use crate::storage::settings_store;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TranslateRequest {
    pub source_text: String,
    pub source_lang: String,
    pub target_lang: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TranslateResponse {
    pub translated_text: String,
}

#[tauri::command]
pub async fn translate_text(
    app: AppHandle,
    request: TranslateRequest,
) -> Result<TranslateResponse, String> {
    let settings = settings_store::read(&app)?;

    if settings.api_key.trim().is_empty() {
        return Err("请先填写 API Key".to_string());
    }

    let translated_text = openai_client::translate(&settings, &request).await?;
    let item = TranslationHistoryItem {
        id: uuid::Uuid::new_v4().to_string(),
        source_text: request.source_text.clone(),
        translated_text: translated_text.clone(),
        source_lang: request.source_lang.clone(),
        target_lang: request.target_lang.clone(),
        model: settings.model,
        created_at: chrono::Utc::now().to_rfc3339(),
    };

    history_store::append(&app, item)?;
    Ok(TranslateResponse { translated_text })
}
