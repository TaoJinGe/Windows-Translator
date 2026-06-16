import { get } from "svelte/store";
import { settingsStore } from "../stores/settingsStore";
import { listenTranslationStream, translateText } from "./tauriApi";

function createRequestId(): string {
  if ("crypto" in window && "randomUUID" in window.crypto) {
    return window.crypto.randomUUID();
  }

  return `${Date.now()}-${Math.random().toString(16).slice(2)}`;
}

export async function requestTranslation(
  sourceText: string,
  sourceLang: string,
  targetLang: string,
  onStreamDelta?: (delta: string) => void
): Promise<string> {
  const settings = get(settingsStore);

  if (!settings.apiKey.trim()) {
    throw new Error("请先填写 API Key");
  }

  const requestId = createRequestId();
  let unlisten: (() => void) | undefined;

  if (settings.streamOutput && onStreamDelta) {
    unlisten = await listenTranslationStream((event) => {
      if (event.requestId === requestId && event.delta) {
        onStreamDelta(event.delta);
      }
    });
  }

  try {
    const response = await translateText({
      requestId,
      sourceText,
      sourceLang,
      targetLang
    });

    return response.translatedText;
  } finally {
    unlisten?.();
  }
}
