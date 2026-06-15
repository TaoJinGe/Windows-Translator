use serde::{Deserialize, Serialize};

use crate::commands::translate::TranslateRequest;
use crate::config::app_config::AppSettings;

use super::prompt;

#[derive(Debug, Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<ChatMessage>,
    temperature: f32,
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

pub async fn translate(
    settings: &AppSettings,
    request: &TranslateRequest,
) -> Result<String, String> {
    let url = format!("{}/chat/completions", settings.api_base_url.trim_end_matches('/'));
    let body = build_request(settings, request);
    let client = reqwest::Client::new();
    let response = client
        .post(url)
        .bearer_auth(&settings.api_key)
        .json(&body)
        .send()
        .await
        .map_err(|_| "网络连接失败".to_string())?;

    let status = response.status();

    if !status.is_success() {
        return Err(format!("API 请求失败：{}", status.as_u16()));
    }

    let data = response
        .json::<ChatResponse>()
        .await
        .map_err(|_| "翻译结果为空".to_string())?;

    let translated = data
        .choices
        .first()
        .map(|choice| strip_thinking(&choice.message.content))
        .filter(|text| !text.is_empty())
        .ok_or_else(|| "翻译结果为空".to_string())?;

    Ok(translated)
}

fn build_request(settings: &AppSettings, request: &TranslateRequest) -> ChatRequest {
    ChatRequest {
        model: settings.model.clone(),
        temperature: 0.2,
        messages: vec![
            ChatMessage {
                role: "system".to_string(),
                content: prompt::system_prompt(&request.source_lang, &request.target_lang),
            },
            ChatMessage {
                role: "user".to_string(),
                content: request.source_text.clone(),
            },
        ],
    }
}

fn strip_thinking(value: &str) -> String {
    let mut remaining = value.trim().to_string();

    loop {
        let lower = remaining.to_lowercase();
        let Some(start) = lower.find("<think>") else {
            break;
        };
        let Some(end) = lower.find("</think>") else {
            remaining.replace_range(start.., "");
            break;
        };

        remaining.replace_range(start..end + "</think>".len(), "");
    }

    remaining.trim().to_string()
}
