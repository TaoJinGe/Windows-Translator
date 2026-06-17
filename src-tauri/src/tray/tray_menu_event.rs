use tauri::menu::MenuEvent;
use tauri::AppHandle;

use super::tray_quit_action::quit_app;
use super::tray_settings_action::open_settings;
use super::tray_show_action::toggle_window;

pub fn handle_tray_menu_event(app: &AppHandle, event: MenuEvent) {
    match event.id().as_ref() {
        "show" => toggle_window(app),
        "settings" => open_settings(app),
        "quit" => quit_app(app),
        _ => {}
    }
}
