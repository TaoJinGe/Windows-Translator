use crate::commands::history::TranslationHistoryItem;

use super::translate::TranslateRequest;

pub fn build_translation_history_item(
    model: &str,
    request: &TranslateRequest,
    translated_text: &str,
) -> TranslationHistoryItem {
    TranslationHistoryItem {
        id: uuid::Uuid::new_v4().to_string(),
        source_text: request.source_text.clone(),
        translated_text: translated_text.to_string(),
        source_lang: request.source_lang.clone(),
        target_lang: request.target_lang.clone(),
        model: model.to_string(),
        created_at: chrono::Utc::now().to_rfc3339(),
    }
}
