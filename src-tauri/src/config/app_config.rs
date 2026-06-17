use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct AppSettings {
    pub settings_version: u32,
    pub api_base_url: String,
    pub api_key: String,
    pub model: String,
    pub source_lang: String,
    pub target_lang: String,
    pub default_language_pair: String,
    #[serde(skip_serializing)]
    pub default_source_lang: String,
    #[serde(skip_serializing)]
    pub default_target_lang: String,
    pub hotkey: String,
    pub translate_hotkey: String,
    pub clear_hotkey: String,
    pub always_on_top: bool,
    pub stream_output: bool,
    pub launch_at_startup: bool,
    pub close_action: String,
}
