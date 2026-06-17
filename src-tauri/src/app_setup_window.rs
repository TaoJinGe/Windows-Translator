use tauri::Manager;

pub fn apply_startup_window_state(handle: &tauri::AppHandle, always_on_top: bool) {
    if let Some(window) = handle.get_webview_window("main") {
        let _ = window.set_always_on_top(always_on_top);
    }
}
