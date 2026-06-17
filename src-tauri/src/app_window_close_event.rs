use tauri::WindowEvent;

use crate::window::window_control;

pub fn handle_close_requested(window: &tauri::Window, event: &WindowEvent) {
    if let WindowEvent::CloseRequested { api, .. } = event {
        api.prevent_close();
        window_control::handle_close_requested(window);
    }
}
