import { invoke } from "@tauri-apps/api/core";
import type { AppSettings } from "../types/settings";
import { isTauriRuntime } from "./tauriRuntime";

export async function saveSettings(settings: AppSettings): Promise<AppSettings> {
  if (!isTauriRuntime()) {
    return settings;
  }

  return invoke<AppSettings>("save_settings", { settings });
}
