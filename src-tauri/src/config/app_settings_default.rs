use super::app_config::AppSettings;
use super::default_api::API_BASE_URL;
use super::default_hotkeys::{CLEAR_HOTKEY, HOTKEY, TRANSLATE_HOTKEY};
use super::default_languages::{DEFAULT_LANGUAGE_PAIR, SOURCE_LANG, TARGET_LANG};
use super::default_model::MODEL;
use super::default_settings_version::SETTINGS_VERSION;
use super::default_window::{
    ALWAYS_ON_TOP, CLOSE_ACTION, LAUNCH_AT_STARTUP, REMEMBER_WINDOW_POSITION, STREAM_OUTPUT,
};

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            settings_version: SETTINGS_VERSION,
            api_base_url: API_BASE_URL.to_string(),
            api_key: String::new(),
            model: MODEL.to_string(),
            source_lang: SOURCE_LANG.to_string(),
            target_lang: TARGET_LANG.to_string(),
            default_language_pair: DEFAULT_LANGUAGE_PAIR.to_string(),
            default_source_lang: String::new(),
            default_target_lang: String::new(),
            hotkey: HOTKEY.to_string(),
            translate_hotkey: TRANSLATE_HOTKEY.to_string(),
            clear_hotkey: CLEAR_HOTKEY.to_string(),
            always_on_top: ALWAYS_ON_TOP,
            stream_output: STREAM_OUTPUT,
            launch_at_startup: LAUNCH_AT_STARTUP,
            remember_window_position: REMEMBER_WINDOW_POSITION,
            window_position_x: None,
            window_position_y: None,
            close_action: CLOSE_ACTION.to_string(),
        }
    }
}
