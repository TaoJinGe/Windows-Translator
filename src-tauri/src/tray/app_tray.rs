use tauri::tray::TrayIconBuilder;
use tauri::AppHandle;

use super::tray_icon_event::handle_tray_icon_event;
use super::tray_menu::build_tray_menu;
use super::tray_menu_event::handle_tray_menu_event;

pub fn create(app: &AppHandle) -> tauri::Result<()> {
    let menu = build_tray_menu(app)?;
    let mut builder = TrayIconBuilder::new()
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_menu_event(handle_tray_menu_event)
        .on_tray_icon_event(handle_tray_icon_event);

    if let Some(icon) = app.default_window_icon() {
        builder = builder.icon(icon.clone());
    }

    builder.build(app)?;
    Ok(())
}
