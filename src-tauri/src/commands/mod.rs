pub mod history;
pub mod settings;
pub mod translate;
pub mod window;

pub fn handlers() -> impl Fn(tauri::ipc::Invoke) -> bool {
    tauri::generate_handler![
        translate::translate_text,
        settings::get_settings,
        settings::save_settings,
        history::get_history,
        history::clear_history,
        window::copy_to_clipboard,
        window::show_main_window,
        window::hide_main_window,
        window::toggle_main_window,
        window::update_global_hotkey
    ]
}
