#[cfg(desktop)]
use tauri::AppHandle;

#[cfg(desktop)]
use tauri_plugin_autostart::ManagerExt;

#[cfg(desktop)]
pub fn sync(app: &AppHandle, enabled: bool) -> Result<(), String> {
    let manager = app.autolaunch();
    let result = if enabled {
        manager.enable()
    } else {
        manager.disable()
    };

    result.map_err(|_| "开机自启动设置失败".to_string())
}

#[cfg(not(desktop))]
pub fn sync(_: &tauri::AppHandle, _: bool) -> Result<(), String> {
    Ok(())
}
