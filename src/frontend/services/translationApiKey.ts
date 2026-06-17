import type { AppSettings } from "../types/settings";

export function assertTranslationApiKey(settings: AppSettings): void {
  if (!settings.apiKey.trim()) {
    throw new Error("请先填写 API Key");
  }
}
