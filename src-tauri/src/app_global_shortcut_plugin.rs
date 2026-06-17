pub fn install_global_shortcut_plugin(
    builder: tauri::Builder<tauri::Wry>,
) -> tauri::Builder<tauri::Wry> {
    builder.plugin(tauri_plugin_global_shortcut::Builder::new().build())
}
