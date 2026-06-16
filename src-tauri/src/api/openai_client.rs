use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter};

use crate::commands::translate::TranslateRequest;
use crate::config::app_config::AppSettings;

use super::prompt;

#[derive(Debug, Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<ChatMessage>,
    temperature: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    stream: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ChatMessage {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct ChatResponse {
    choices: Vec<ChatChoice>,
}

#[derive(Debug, Deserialize)]
struct ChatChoice {
    message: ChatMessage,
}

#[derive(Debug, Deserialize)]
struct ChatStreamResponse {
    choices: Vec<ChatStreamChoice>,
}

#[derive(Debug, Deserialize)]
struct ChatStreamChoice {
    delta: ChatStreamDelta,
}

#[derive(Debug, Deserialize)]
struct ChatStreamDelta {
    content: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
struct TranslationStreamEvent {
    request_id: String,
    delta: String,
    done: bool,
}

pub async fn translate(
    client: &reqwest::Client,
    settings: &AppSettings,
    request: &TranslateRequest,
) -> Result<String, String> {
    let url = chat_url(settings);
    let body = build_request(settings, request, false);
    let response = client
        .post(url)
        .bearer_auth(&settings.api_key)
        .json(&body)
        .send()
        .await
        .map_err(|_| "Network connection failed".to_string())?;

    let status = response.status();

    if !status.is_success() {
        return Err(format!("API request failed: {}", status.as_u16()));
    }

    let data = response
        .json::<ChatResponse>()
        .await
        .map_err(|_| "Translation result is empty".to_string())?;

    let translated = data
        .choices
        .first()
        .map(|choice| strip_thinking(&choice.message.content))
        .filter(|text| !text.is_empty())
        .ok_or_else(|| "Translation result is empty".to_string())?;

    Ok(translated)
}

pub async fn translate_stream(
    app: &AppHandle,
    client: &reqwest::Client,
    settings: &AppSettings,
    request: &TranslateRequest,
) -> Result<String, String> {
    let request_id = request
        .request_id
        .clone()
        .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
    let url = chat_url(settings);
    let body = build_request(settings, request, true);
    let response = client
        .post(url)
        .bearer_auth(&settings.api_key)
        .json(&body)
        .send()
        .await
        .map_err(|_| "Network connection failed".to_string())?;

    let status = response.status();

    if !status.is_success() {
        return Err(format!("API request failed: {}", status.as_u16()));
    }

    let mut stream = response.bytes_stream();
    let mut buffer = Vec::new();
    let mut translated = String::new();

    while let Some(chunk) = stream.next().await {
        let chunk = chunk.map_err(|_| "Network connection failed".to_string())?;
        buffer.extend_from_slice(&chunk);

        while let Some(event_end) = find_event_end(&buffer) {
            let event_bytes = buffer.drain(..event_end).collect::<Vec<_>>();
            trim_event_separator(&mut buffer);
            let event = String::from_utf8(event_bytes)
                .map_err(|_| "Translation stream format is invalid".to_string())?;

            for delta in parse_stream_event(&event)? {
                translated.push_str(&delta);
                emit_stream_delta(app, &request_id, delta);
            }
        }
    }

    let translated = strip_thinking(&translated);
    emit_stream_done(app, &request_id);

    if translated.is_empty() {
        return Err("Translation result is empty".to_string());
    }

    Ok(translated)
}

fn chat_url(settings: &AppSettings) -> String {
    format!(
        "{}/chat/completions",
        settings.api_base_url.trim_end_matches('/')
    )
}

fn build_request(settings: &AppSettings, request: &TranslateRequest, stream: bool) -> ChatRequest {
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

fn parse_stream_event(event: &str) -> Result<Vec<String>, String> {
    let mut deltas = Vec::new();

    for line in event.lines() {
        let Some(data) = line.strip_prefix("data:") else {
            continue;
        };
        let data = data.trim();

        if data.is_empty() || data == "[DONE]" {
            continue;
        }

        let chunk = serde_json::from_str::<ChatStreamResponse>(data)
            .map_err(|_| "Translation stream format is invalid".to_string())?;

        for choice in chunk.choices {
            if let Some(content) = choice.delta.content {
                if !content.is_empty() {
                    deltas.push(content);
                }
            }
        }
    }

    Ok(deltas)
}

fn find_event_end(buffer: &[u8]) -> Option<usize> {
    buffer
        .windows(2)
        .position(|window| window == b"\n\n")
        .or_else(|| buffer.windows(4).position(|window| window == b"\r\n\r\n"))
}

fn trim_event_separator(buffer: &mut Vec<u8>) {
    if buffer.starts_with(b"\r\n\r\n") {
        buffer.drain(..4);
    } else if buffer.starts_with(b"\n\n") {
        buffer.drain(..2);
    }
}

fn emit_stream_delta(app: &AppHandle, request_id: &str, delta: String) {
    let _ = app.emit(
        "translation-stream",
        TranslationStreamEvent {
            request_id: request_id.to_string(),
            delta,
            done: false,
        },
    );
}

fn emit_stream_done(app: &AppHandle, request_id: &str) {
    let _ = app.emit(
        "translation-stream",
        TranslationStreamEvent {
            request_id: request_id.to_string(),
            delta: String::new(),
            done: true,
        },
    );
}

fn strip_thinking(value: &str) -> String {
    let mut remaining = value.trim().to_string();

    loop {
        let lower = remaining.to_lowercase();
        let Some(start) = lower.find("<think>") else {
            break;
        };
        let Some(end_offset) = lower[start..].rfind("</think>") else {
            remaining.replace_range(start.., "");
            break;
        };
        let end = start + end_offset;

        remaining.replace_range(start..end + "</think>".len(), "");
    }

    let lower = remaining.to_lowercase();
    if let Some(end) = lower.rfind("</think>") {
        remaining.replace_range(..end + "</think>".len(), "");
    }

    remaining.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::{parse_stream_event, strip_thinking};

    #[test]
    fn parses_stream_delta() {
        let event = r#"data: {"choices":[{"delta":{"content":"Hi"}}]}"#;

        assert_eq!(parse_stream_event(event).unwrap(), vec!["Hi"]);
    }

    #[test]
    fn ignores_stream_done() {
        assert!(parse_stream_event("data: [DONE]").unwrap().is_empty());
    }

    #[test]
    fn removes_thinking_that_quotes_closing_tag() {
        let value = "<think>
Do not output <think>, </think> or any chain-of-thought content.
Wait, I need to reconsider.
</think>

AI Translator";

        assert_eq!(strip_thinking(value), "AI Translator");
    }

    #[test]
    fn removes_unclosed_thinking() {
        let value = "<think>
I should not show this.";

        assert_eq!(strip_thinking(value), "");
    }

    #[test]
    fn removes_stray_closing_tag_prefix() {
        let value = "Hidden reasoning leaked.
</think>
Final translation";

        assert_eq!(strip_thinking(value), "Final translation");
    }
}
