use serde::{Deserialize, Serialize};
use tauri::{AppHandle, State};

use crate::api::openai_client;
use crate::storage::settings_store;

use super::translation_api_key::assert_api_key;
use super::translation_history_append::append_translation_history;

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

    assert_api_key(&settings)?;

    let translated_text = if settings.stream_output {
        openai_client::translate_stream(&app, &client, &settings, &request).await?
    } else {
        openai_client::translate(&client, &settings, &request).await?
    };

    append_translation_history(&app, &settings.model, &request, &translated_text);

    Ok(TranslateResponse { translated_text })
}
