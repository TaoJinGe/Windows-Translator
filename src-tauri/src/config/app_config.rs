use serde::{Deserialize, Serialize};

use super::defaults;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct AppSettings {
    pub api_base_url: String,
    pub api_key: String,
    pub model: String,
    pub source_lang: String,
    pub target_lang: String,
    pub hotkey: String,
    pub translate_hotkey: String,
    pub clear_hotkey: String,
    pub always_on_top: bool,
    pub stream_output: bool,
    pub close_action: String,
}

impl AppSettings {
    pub fn normalize(mut self) -> Self {
        if self.source_lang.trim().is_empty() {
            self.source_lang = defaults::SOURCE_LANG.to_string();
        }

        if self.target_lang.trim().is_empty() {
            self.target_lang = defaults::TARGET_LANG.to_string();
        }

        self.always_on_top = false;
        self
    }
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            api_base_url: defaults::API_BASE_URL.to_string(),
            api_key: String::new(),
            model: defaults::MODEL.to_string(),
            source_lang: defaults::SOURCE_LANG.to_string(),
            target_lang: defaults::TARGET_LANG.to_string(),
            hotkey: defaults::HOTKEY.to_string(),
            translate_hotkey: "Ctrl+Enter".to_string(),
            clear_hotkey: "Ctrl+Backspace".to_string(),
            always_on_top: false,
            stream_output: false,
            close_action: "tray".to_string(),
        }
    }
}
