use crate::{
    app_clipboard_plugin, app_global_shortcut_plugin, app_setup, app_state, app_window_event,
    commands,
};

pub fn run() {
    let builder = tauri::Builder::default();
    let builder = app_state::manage_http_client(builder);
    let builder = app_clipboard_plugin::install_clipboard_plugin(builder);
    let builder = app_global_shortcut_plugin::install_global_shortcut_plugin(builder);

    builder
        .setup(app_setup::setup_app)
        .on_window_event(app_window_event::handle_window_event)
        .invoke_handler(commands::handlers())
        .run(tauri::generate_context!())
        .expect("failed to run app");
}
