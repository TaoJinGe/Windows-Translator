pub fn install_autostart_plugin(app: &mut tauri::App) -> tauri::Result<()> {
    #[cfg(desktop)]
    app.handle().plugin(tauri_plugin_autostart::init(
        tauri_plugin_autostart::MacosLauncher::LaunchAgent,
        None,
    ))?;

    Ok(())
}
