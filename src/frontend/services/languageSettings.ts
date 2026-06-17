import { persistSettings } from "../stores/persistSettings";
import type { AppSettings } from "../types/settings";

export async function saveTranslationLanguages(
  settings: AppSettings,
  sourceLang: string,
  targetLang: string
): Promise<void> {
  await persistSettings({ ...settings, sourceLang, targetLang });
}
