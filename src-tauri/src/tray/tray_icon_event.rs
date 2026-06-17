use tauri::tray::{MouseButton, MouseButtonState, TrayIcon, TrayIconEvent};

use super::tray_show_action::toggle_window;

pub fn handle_tray_icon_event(tray: &TrayIcon, event: TrayIconEvent) {
    if let TrayIconEvent::Click {
        button: MouseButton::Left,
        button_state: MouseButtonState::Up,
        ..
    } = event
    {
        toggle_window(tray.app_handle());
    }
}
