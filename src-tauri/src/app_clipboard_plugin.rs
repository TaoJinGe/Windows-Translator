pub fn install_clipboard_plugin(
    builder: tauri::Builder<tauri::Wry>,
) -> tauri::Builder<tauri::Wry> {
    builder.plugin(tauri_plugin_clipboard_manager::init())
}
