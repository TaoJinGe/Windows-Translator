use tauri::menu::{Menu, MenuBuilder, MenuItemBuilder};
use tauri::{AppHandle, Wry};

pub fn build_tray_menu(app: &AppHandle) -> tauri::Result<Menu<Wry>> {
    let show = MenuItemBuilder::with_id("show", "显示/隐藏").build(app)?;
    let settings = MenuItemBuilder::with_id("settings", "设置").build(app)?;
    let quit = MenuItemBuilder::with_id("quit", "退出").build(app)?;

    MenuBuilder::new(app)
        .items(&[&show, &settings, &quit])
        .build()
}
