mod api;
mod autostart;
mod commands;
mod config;
mod hotkey;
mod storage;
mod tray;
mod window;

use tauri::{Manager, WindowEvent};

pub fn run() {
    tauri::Builder::default()
        .manage(reqwest::Client::new())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .setup(|app| {
            #[cfg(desktop)]
            app.handle().plugin(tauri_plugin_autostart::init(
                tauri_plugin_autostart::MacosLauncher::LaunchAgent,
                None,
            ))?;

            let handle = app.handle().clone();
            let settings = storage::settings_store::read(&handle).unwrap_or_default();

            if let Some(window) = handle.get_webview_window("main") {
                let _ = window.set_always_on_top(settings.always_on_top);
            }

            let _ = autostart::sync(&handle, settings.launch_at_startup);
            tray::app_tray::create(&handle)?;
            let _ = hotkey::global_hotkey::register(&handle, &settings.hotkey);
            Ok(())
        })
        .on_window_event(|window, event| {
            if let WindowEvent::CloseRequested { api, .. } = event {
                api.prevent_close();
                window::window_control::handle_close_requested(window);
            }

            if matches!(event, WindowEvent::Focused(false)) {
                let _ = window.set_always_on_top(false);
            }
        })
        .invoke_handler(commands::handlers())
        .run(tauri::generate_context!())
        .expect("failed to run app");
}
