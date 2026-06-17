import { saveSettings } from "../services/saveSettingsApi";
import type { AppSettings } from "../types/settings";
import { settingsStore } from "./settingsStore";

export async function persistSettings(settings: AppSettings): Promise<void> {
  const saved = await saveSettings(settings);
  settingsStore.set(saved);
}
