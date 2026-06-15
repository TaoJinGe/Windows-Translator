import { writable } from "svelte/store";
import { getSettings, saveSettings } from "../services/tauriApi";
import { defaultSettings, type AppSettings } from "../types/settings";

export const settingsStore = writable<AppSettings>(defaultSettings);

export async function loadSettings(): Promise<void> {
  settingsStore.set(await getSettings());
}

export async function persistSettings(settings: AppSettings): Promise<void> {
  const saved = await saveSettings(settings);
  settingsStore.set(saved);
}
