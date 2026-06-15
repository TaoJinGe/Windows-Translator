use tauri::menu::{MenuBuilder, MenuItemBuilder};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri::{AppHandle, Emitter};

use crate::window::window_control;

pub fn create(app: &AppHandle) -> tauri::Result<()> {
    let show = MenuItemBuilder::with_id("show", "显示/隐藏").build(app)?;
    let settings = MenuItemBuilder::with_id("settings", "设置").build(app)?;
    let quit = MenuItemBuilder::with_id("quit", "退出").build(app)?;
    let menu = MenuBuilder::new(app)
        .items(&[&show, &settings, &quit])
        .build()?;

    let mut builder = TrayIconBuilder::new()
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_menu_event(|app, event| match event.id().as_ref() {
            "show" => {
                let _ = window_control::toggle(app);
            }
            "settings" => {
                let _ = window_control::show(app);
                let _ = app.emit("open-settings", ());
            }
            "quit" => app.exit(0),
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                let _ = window_control::toggle(tray.app_handle());
            }
        });

    if let Some(icon) = app.default_window_icon() {
        builder = builder.icon(icon.clone());
    }

    builder.build(app)?;
    Ok(())
}
