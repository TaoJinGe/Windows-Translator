use tauri::WindowEvent;

pub fn handle_focus_lost(window: &tauri::Window, event: &WindowEvent) {
    if matches!(event, WindowEvent::Focused(false)) {
        let _ = window.set_always_on_top(false);
    }
}
