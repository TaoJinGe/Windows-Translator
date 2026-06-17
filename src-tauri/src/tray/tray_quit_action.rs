use tauri::AppHandle;

pub fn quit_app(app: &AppHandle) {
    app.exit(0);
}
