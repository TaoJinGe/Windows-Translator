use std::str::FromStr;

use tauri::AppHandle;
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut, ShortcutState};

use crate::window::window_control;

pub fn register(app: &AppHandle, hotkey: &str) -> Result<(), String> {
    let shortcut = parse(hotkey)?;
    let handle = app.clone();

    app.global_shortcut()
        .on_shortcut(shortcut, move |_app, _shortcut, event| {
            if event.state == ShortcutState::Pressed {
                let _ = window_control::toggle(&handle);
            }
        })
        .map_err(|_| "快捷键已被占用，请换一个组合键".to_string())
}

pub fn update(app: &AppHandle, hotkey: &str) -> Result<(), String> {
    app.global_shortcut()
        .unregister_all()
        .map_err(|_| "快捷键更新失败".to_string())?;

    register(app, hotkey)
}

pub fn replace(app: &AppHandle, old_hotkey: &str, new_hotkey: &str) -> Result<(), String> {
    app.global_shortcut()
        .unregister_all()
        .map_err(|_| "快捷键更新失败".to_string())?;

    match register(app, new_hotkey) {
        Ok(()) => Ok(()),
        Err(error) => {
            let _ = register(app, old_hotkey);
            Err(error)
        }
    }
}

fn parse(hotkey: &str) -> Result<Shortcut, String> {
    let normalized = normalize(hotkey);

    if normalized.is_empty() {
        return Err("快捷键注册失败，请换一个组合键".to_string());
    }

    Shortcut::from_str(&normalized).map_err(|_| "快捷键格式无效".to_string())
}

fn normalize(hotkey: &str) -> String {
    hotkey
        .split('+')
        .map(str::trim)
        .filter(|part| !part.is_empty())
        .map(|part| match part.to_lowercase().as_str() {
            "ctrl" | "control" => "CommandOrControl".to_string(),
            "alt" => "Alt".to_string(),
            "shift" => "Shift".to_string(),
            "super" | "meta" | "win" | "windows" => "Super".to_string(),
            other => normalize_key(other),
        })
        .collect::<Vec<_>>()
        .join("+")
}

fn normalize_key(key: &str) -> String {
    if key.len() == 1 {
        key.to_uppercase()
    } else {
        key.to_string()
    }
}
