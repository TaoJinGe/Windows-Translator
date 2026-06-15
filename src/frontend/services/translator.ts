import { get } from "svelte/store";
import { settingsStore } from "../stores/settingsStore";
import { translateText } from "./tauriApi";

export async function requestTranslation(
  sourceText: string,
  sourceLang: string,
  targetLang: string
): Promise<string> {
  const settings = get(settingsStore);

  if (!settings.apiKey.trim()) {
    throw new Error("请先填写 API Key");
  }

  const response = await translateText({
    sourceText,
    sourceLang,
    targetLang
  });

  return response.translatedText;
}
