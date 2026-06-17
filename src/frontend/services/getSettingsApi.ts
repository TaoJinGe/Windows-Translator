import { invoke } from "@tauri-apps/api/core";
import { defaultSettings, type AppSettings } from "../types/settings";
import { isTauriRuntime } from "./tauriRuntime";

export async function getSettings(): Promise<AppSettings> {
  if (!isTauriRuntime()) {
    return defaultSettings;
  }

  return invoke<AppSettings>("get_settings");
}
