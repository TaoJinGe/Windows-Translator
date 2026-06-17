use crate::config::app_config::AppSettings;

pub fn assert_api_key(settings: &AppSettings) -> Result<(), String> {
    if settings.api_key.trim().is_empty() {
        return Err("请先填写 API Key".to_string());
    }

    Ok(())
}
