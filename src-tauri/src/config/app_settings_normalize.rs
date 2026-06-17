use super::app_config::AppSettings;
use super::default_languages::{DEFAULT_LANGUAGE_PAIR, SOURCE_LANG, TARGET_LANG};
use super::default_settings_version::SETTINGS_VERSION;
use super::language_pair::{is_concrete_language, is_valid_language_pair};

impl AppSettings {
    pub fn normalize(mut self) -> Self {
        if self.settings_version == 0 && self.source_lang == "auto" && self.target_lang == "zh-CN" {
            self.target_lang = TARGET_LANG.to_string();
        }

        self.settings_version = SETTINGS_VERSION;

        if self.source_lang.trim().is_empty() {
            self.source_lang = SOURCE_LANG.to_string();
        }

        if self.target_lang.trim().is_empty() {
            self.target_lang = TARGET_LANG.to_string();
        }

        if self.default_language_pair.trim().is_empty() {
            self.default_language_pair = if is_concrete_language(&self.default_source_lang)
                && is_concrete_language(&self.default_target_lang)
            {
                format!("{}|{}", self.default_source_lang, self.default_target_lang)
            } else {
                DEFAULT_LANGUAGE_PAIR.to_string()
            };
        }

        if !is_valid_language_pair(&self.default_language_pair) {
            self.default_language_pair = DEFAULT_LANGUAGE_PAIR.to_string();
        }

        self.always_on_top = false;
        self
    }
}
