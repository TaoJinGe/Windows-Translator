use crate::{
    app_setup_autostart, app_setup_autostart_plugin, app_setup_hotkey, app_setup_settings,
    app_setup_tray, app_setup_window,
};

pub fn setup_app(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    app_setup_autostart_plugin::install_autostart_plugin(app)?;
    let handle = app.handle().clone();
    let settings = app_setup_settings::read_startup_settings(&handle);

    app_setup_window::apply_startup_window_preferences(&handle, &settings);
    app_setup_autostart::sync_startup_autostart(&handle, settings.launch_at_startup);
    app_setup_tray::create_startup_tray(&handle)?;
    app_setup_hotkey::register_startup_hotkey(&handle, &settings.hotkey);
    Ok(())
}
