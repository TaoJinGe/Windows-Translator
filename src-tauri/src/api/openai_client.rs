use futures_util::StreamExt;
use tauri::AppHandle;

use crate::commands::translate::TranslateRequest;
use crate::config::app_config::AppSettings;

use super::openai_request::{build_request, chat_url};
use super::openai_types::ChatResponse;
use super::sse_parser::{find_event_end, parse_stream_event, trim_event_separator};
use super::stream_event::{emit_stream_delta, emit_stream_done};
use super::thinking_filter::strip_thinking;

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
