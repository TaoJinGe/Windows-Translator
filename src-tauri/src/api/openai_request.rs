use crate::commands::translate::TranslateRequest;
use crate::config::app_config::AppSettings;

use super::prompt;
use super::openai_types::{ChatMessage, ChatRequest};

pub(super) fn chat_url(settings: &AppSettings) -> String {
    format!(
        "{}/chat/completions",
        settings.api_base_url.trim_end_matches('/')
    )
}

pub(super) fn build_request(
    settings: &AppSettings,
    request: &TranslateRequest,
    stream: bool,
) -> ChatRequest {
    let (default_source_lang, default_target_lang) = settings.default_pair_languages();

    ChatRequest {
        model: settings.model.clone(),
        temperature: 0.2,
        stream: stream.then_some(true),
        messages: vec![
            ChatMessage {
                role: "system".to_string(),
                content: prompt::system_prompt(
                    &request.source_lang,
                    &request.target_lang,
                    default_source_lang,
                    default_target_lang,
                ),
            },
            ChatMessage {
                role: "user".to_string(),
                content: request.source_text.clone(),
            },
        ],
    }
}
