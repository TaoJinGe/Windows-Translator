import { listen } from "@tauri-apps/api/event";
import type { TranslationStreamEvent } from "../types/translation";
import { isTauriRuntime } from "./tauriRuntime";

export async function listenTranslationStream(
  callback: (event: TranslationStreamEvent) => void
): Promise<() => void> {
  if (!isTauriRuntime()) {
    return () => {};
  }

  return listen<TranslationStreamEvent>("translation-stream", (event) => {
    callback(event.payload);
  });
}
