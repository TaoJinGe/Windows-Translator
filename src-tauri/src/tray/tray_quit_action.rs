use tauri::AppHandle;

use crate::window::window_position_memory;

pub fn quit_app(app: &AppHandle) {
    window_position_memory::remember_main_window_position(app);
    app.exit(0);
}
