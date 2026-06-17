mod api;
mod app_clipboard_plugin;
mod app_global_shortcut_plugin;
mod app_run;
mod app_setup;
mod app_setup_autostart;
mod app_setup_autostart_plugin;
mod app_setup_hotkey;
mod app_setup_settings;
mod app_setup_tray;
mod app_setup_window;
mod app_state;
mod app_window_close_event;
mod app_window_event;
mod app_window_focus_event;
mod autostart;
mod commands;
mod config;
mod hotkey;
mod storage;
mod tray;
mod window;

pub fn run() {
    app_run::run();
}
