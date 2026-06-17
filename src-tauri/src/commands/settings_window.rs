use tauri::{AppHandle, Manager};

pub fn apply_always_on_top(app: &AppHandle, always_on_top: bool) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("main") {
        window
            .set_always_on_top(always_on_top)
            .map_err(|_| "窗口置顶设置失败".to_string())?;
    }

    Ok(())
}
