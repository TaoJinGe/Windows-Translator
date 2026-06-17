use serde::Serialize;
use tauri::{AppHandle, Emitter};

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
struct TranslationStreamEvent {
    request_id: String,
    delta: String,
    done: bool,
}

pub fn emit_stream_delta(app: &AppHandle, request_id: &str, delta: String) {
    let _ = app.emit(
        "translation-stream",
        TranslationStreamEvent {
            request_id: request_id.to_string(),
            delta,
            done: false,
        },
    );
}

pub fn emit_stream_done(app: &AppHandle, request_id: &str) {
    let _ = app.emit(
        "translation-stream",
        TranslationStreamEvent {
            request_id: request_id.to_string(),
            delta: String::new(),
            done: true,
        },
    );
}
