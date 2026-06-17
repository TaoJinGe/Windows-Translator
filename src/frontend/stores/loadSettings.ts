import { getSettings } from "../services/getSettingsApi";
import { settingsStore } from "./settingsStore";

export async function loadSettings(): Promise<void> {
  settingsStore.set(await getSettings());
}
