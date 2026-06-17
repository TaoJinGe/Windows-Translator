pub fn manage_http_client(builder: tauri::Builder<tauri::Wry>) -> tauri::Builder<tauri::Wry> {
    builder.manage(reqwest::Client::new())
}
