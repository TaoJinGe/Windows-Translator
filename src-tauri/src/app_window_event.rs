use tauri::WindowEvent;

use crate::{app_window_close_event, app_window_focus_event};

pub fn handle_window_event(window: &tauri::Window, event: &WindowEvent) {
    app_window_close_event::handle_close_requested(window, event);
    app_window_focus_event::handle_focus_lost(window, event);
}
