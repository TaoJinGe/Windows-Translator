use tauri::{Manager, PhysicalPosition, Position, WebviewWindow, Window};

use crate::config::app_config::AppSettings;
use crate::storage::settings_store;

pub fn apply_saved_webview_position(window: &WebviewWindow, settings: &AppSettings) {
    if !settings.remember_window_position {
        return;
    }

    let (Some(x), Some(y)) = (settings.window_position_x, settings.window_position_y) else {
        return;
    };

    let _ = window.set_position(Position::Physical(PhysicalPosition { x, y }));
}

pub fn remember_current_position(window: &Window) {
    let app = window.app_handle();
    let Ok(settings) = settings_store::read(app) else {
        return;
    };

    if !settings.remember_window_position {
        return;
    }

    let Ok(position) = window.outer_position() else {
        return;
    };

    let _ = settings_store::write_window_position(app, position.x, position.y);
}

pub fn remember_main_window_position(app: &tauri::AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        remember_current_webview_position(&window);
    }
}

fn remember_current_webview_position(window: &WebviewWindow) {
    let app = window.app_handle();
    let Ok(settings) = settings_store::read(app) else {
        return;
    };

    if !settings.remember_window_position {
        return;
    }

    let Ok(position) = window.outer_position() else {
        return;
    };

    let _ = settings_store::write_window_position(app, position.x, position.y);
}
