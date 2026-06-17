pub mod clipboard;
pub mod history;
pub mod hotkey;
pub mod settings;
pub mod settings_autostart;
pub mod settings_hotkey;
pub mod settings_window;
pub mod translate;
pub mod translation_api_key;
pub mod translation_history_append;
pub mod translation_history_item;
pub mod window;

pub fn handlers() -> impl Fn(tauri::ipc::Invoke) -> bool {
    tauri::generate_handler![
        translate::translate_text,
        settings::get_settings,
        settings::save_settings,
        history::get_history,
        history::clear_history,
        clipboard::copy_to_clipboard,
        window::show_main_window,
        window::hide_main_window,
        window::toggle_main_window,
        hotkey::update_global_hotkey
    ]
}
