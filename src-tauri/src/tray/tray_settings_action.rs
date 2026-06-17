use tauri::{AppHandle, Emitter};

use crate::window::window_control;

pub fn open_settings(app: &AppHandle) {
    let _ = window_control::show(app);
    let _ = app.emit("open-settings", ());
}
