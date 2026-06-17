use tauri::Manager;

use crate::config::app_config::AppSettings;
use crate::window::window_position_memory;

pub fn apply_startup_window_preferences(handle: &tauri::AppHandle, settings: &AppSettings) {
    if let Some(window) = handle.get_webview_window("main") {
        let _ = window.set_always_on_top(settings.always_on_top);
        window_position_memory::apply_saved_webview_position(&window, settings);
    }
}
