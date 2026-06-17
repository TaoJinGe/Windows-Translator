use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub(super) struct ChatRequest {
    pub(super) model: String,
    pub(super) messages: Vec<ChatMessage>,
    pub(super) temperature: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) stream: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(super) struct ChatMessage {
    pub(super) role: String,
    pub(super) content: String,
}

#[derive(Debug, Deserialize)]
pub(super) struct ChatResponse {
    pub(super) choices: Vec<ChatChoice>,
}

#[derive(Debug, Deserialize)]
pub(super) struct ChatChoice {
    pub(super) message: ChatMessage,
}

#[derive(Debug, Deserialize)]
pub(super) struct ChatStreamResponse {
    pub(super) choices: Vec<ChatStreamChoice>,
}

#[derive(Debug, Deserialize)]
pub(super) struct ChatStreamChoice {
    pub(super) delta: ChatStreamDelta,
}

#[derive(Debug, Deserialize)]
pub(super) struct ChatStreamDelta {
    pub(super) content: Option<String>,
}
