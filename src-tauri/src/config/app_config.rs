use serde::{Deserialize, Serialize};

use super::defaults;

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

impl AppSettings {
    pub fn normalize(mut self) -> Self {
        if self.settings_version == 0 && self.source_lang == "auto" && self.target_lang == "zh-CN" {
            self.target_lang = defaults::TARGET_LANG.to_string();
        }

        self.settings_version = defaults::SETTINGS_VERSION;

        if self.source_lang.trim().is_empty() {
            self.source_lang = defaults::SOURCE_LANG.to_string();
        }

        if self.target_lang.trim().is_empty() {
            self.target_lang = defaults::TARGET_LANG.to_string();
        }

        if self.default_language_pair.trim().is_empty() {
            self.default_language_pair = if is_concrete_language(&self.default_source_lang)
                && is_concrete_language(&self.default_target_lang)
            {
                format!("{}|{}", self.default_source_lang, self.default_target_lang)
            } else {
                defaults::DEFAULT_LANGUAGE_PAIR.to_string()
            };
        }

        if !is_valid_language_pair(&self.default_language_pair) {
            self.default_language_pair = defaults::DEFAULT_LANGUAGE_PAIR.to_string();
        }

        self.always_on_top = false;
        self
    }

    pub fn default_pair_languages(&self) -> (&str, &str) {
        self.default_language_pair
            .split_once('|')
            .filter(|(first, second)| is_concrete_language(first) && is_concrete_language(second))
            .unwrap_or(("zh-CN", "en"))
    }
}

fn is_concrete_language(value: &str) -> bool {
    !value.trim().is_empty() && value != "auto"
}

fn is_valid_language_pair(value: &str) -> bool {
    value
        .split_once('|')
        .is_some_and(|(first, second)| is_concrete_language(first) && is_concrete_language(second))
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            settings_version: defaults::SETTINGS_VERSION,
            api_base_url: defaults::API_BASE_URL.to_string(),
            api_key: String::new(),
            model: defaults::MODEL.to_string(),
            source_lang: defaults::SOURCE_LANG.to_string(),
            target_lang: defaults::TARGET_LANG.to_string(),
            default_language_pair: defaults::DEFAULT_LANGUAGE_PAIR.to_string(),
            default_source_lang: String::new(),
            default_target_lang: String::new(),
            hotkey: defaults::HOTKEY.to_string(),
            translate_hotkey: "Ctrl+Enter".to_string(),
            clear_hotkey: "Ctrl+Backspace".to_string(),
            always_on_top: false,
            stream_output: false,
            launch_at_startup: false,
            close_action: "tray".to_string(),
        }
    }
}
