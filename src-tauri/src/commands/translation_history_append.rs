use tauri::AppHandle;

use crate::storage::history_store;

use super::translate::TranslateRequest;
use super::translation_history_item::build_translation_history_item;

pub fn append_translation_history(
    app: &AppHandle,
    model: &str,
    request: &TranslateRequest,
    translated_text: &str,
) {
    let item = build_translation_history_item(model, request, translated_text);
    let history_app = app.clone();

    tauri::async_runtime::spawn_blocking(move || {
        let _ = history_store::append(&history_app, item);
    });
}
