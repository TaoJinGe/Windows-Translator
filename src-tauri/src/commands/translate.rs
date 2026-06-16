use serde::{Deserialize, Serialize};
use tauri::{AppHandle, State};

use crate::api::openai_client;
use crate::commands::history::TranslationHistoryItem;
use crate::storage::history_store;
use crate::storage::settings_store;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TranslateRequest {
    pub request_id: Option<String>,
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
    client: State<'_, reqwest::Client>,
    request: TranslateRequest,
) -> Result<TranslateResponse, String> {
    let settings = settings_store::read(&app)?;

    if settings.api_key.trim().is_empty() {
        return Err("请先填写 API Key".to_string());
    }

    let translated_text = if settings.stream_output {
        openai_client::translate_stream(&app, &client, &settings, &request).await?
    } else {
        openai_client::translate(&client, &settings, &request).await?
    };
    let item = TranslationHistoryItem {
        id: uuid::Uuid::new_v4().to_string(),
        source_text: request.source_text.clone(),
        translated_text: translated_text.clone(),
        source_lang: request.source_lang.clone(),
        target_lang: request.target_lang.clone(),
        model: settings.model,
        created_at: chrono::Utc::now().to_rfc3339(),
    };

    let history_app = app.clone();
    tauri::async_runtime::spawn_blocking(move || {
        let _ = history_store::append(&history_app, item);
    });

    Ok(TranslateResponse { translated_text })
}
