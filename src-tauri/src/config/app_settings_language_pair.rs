use super::app_config::AppSettings;
use super::language_pair::is_concrete_language;

impl AppSettings {
    pub fn default_pair_languages(&self) -> (&str, &str) {
        self.default_language_pair
            .split_once('|')
            .filter(|(first, second)| is_concrete_language(first) && is_concrete_language(second))
            .unwrap_or(("zh-CN", "en"))
    }
}
