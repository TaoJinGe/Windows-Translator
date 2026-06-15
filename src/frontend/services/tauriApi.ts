import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { defaultSettings, type AppSettings } from "../types/settings";
import type { TranslationHistoryItem } from "../types/history";
import type { TranslateRequest, TranslateResponse } from "../types/translation";

export function isTauriRuntime(): boolean {
  return "__TAURI_INTERNALS__" in window;
}

export async function getSettings(): Promise<AppSettings> {
  if (!isTauriRuntime()) {
    return defaultSettings;
  }

  return invoke<AppSettings>("get_settings");
}

export async function saveSettings(settings: AppSettings): Promise<AppSettings> {
  if (!isTauriRuntime()) {
    return settings;
  }

  return invoke<AppSettings>("save_settings", { settings });
}

export async function translateText(request: TranslateRequest): Promise<TranslateResponse> {
  return invoke<TranslateResponse>("translate_text", { request });
}

export async function getHistory(): Promise<TranslationHistoryItem[]> {
  if (!isTauriRuntime()) {
    return [];
  }

  return invoke<TranslationHistoryItem[]>("get_history");
}

export async function clearHistory(): Promise<void> {
  if (isTauriRuntime()) {
    await invoke("clear_history");
  }
}

export async function copyToClipboard(text: string): Promise<void> {
  if (isTauriRuntime()) {
    await invoke("copy_to_clipboard", { text });
  }
}

export async function hideMainWindow(): Promise<void> {
  if (isTauriRuntime()) {
    await invoke("hide_main_window");
  }
}

export async function onOpenSettings(callback: () => void): Promise<() => void> {
  if (!isTauriRuntime()) {
    return () => {};
  }

  return listen("open-settings", callback);
}
