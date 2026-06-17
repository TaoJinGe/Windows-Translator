use tauri::{AppHandle, Manager, Window};

use crate::storage::settings_store;
use crate::window::window_position_memory;

pub fn show(app: &AppHandle) -> Result<(), String> {
    let window = app
        .get_webview_window("main")
        .ok_or_else(|| "找不到主窗口".to_string())?;

    window.show().map_err(|_| "无法显示窗口".to_string())?;
    window.set_focus().map_err(|_| "无法聚焦窗口".to_string())
}

pub fn hide(app: &AppHandle) -> Result<(), String> {
    let window = app
        .get_webview_window("main")
        .ok_or_else(|| "找不到主窗口".to_string())?;

    window.hide().map_err(|_| "无法隐藏窗口".to_string())
}

pub fn toggle(app: &AppHandle) -> Result<(), String> {
    let window = app
        .get_webview_window("main")
        .ok_or_else(|| "找不到主窗口".to_string())?;

    if window.is_visible().unwrap_or(false) && window.is_focused().unwrap_or(false) {
        hide(app)
    } else {
        show(app)
    }
}

pub fn handle_close_requested(window: &Window) {
    window_position_memory::remember_current_position(window);

    let app = window.app_handle();
    let close_action = settings_store::read(app)
        .map(|settings| settings.close_action)
        .unwrap_or_else(|_| "tray".to_string());

    if close_action == "minimize" {
        let _ = window.minimize();
    } else {
        let _ = window.hide();
    }
}
