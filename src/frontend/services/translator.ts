import { get } from "svelte/store";
import { settingsStore } from "../stores/settingsStore";
import { assertTranslationApiKey } from "./translationApiKey";
import { translateText } from "./translationApi";
import { createTranslationRequestId } from "./translationRequestId";
import { listenTranslationStream } from "./translationStreamApi";

export async function requestTranslation(
  sourceText: string,
  sourceLang: string,
  targetLang: string,
  onStreamDelta?: (delta: string) => void
): Promise<string> {
  const settings = get(settingsStore);

  assertTranslationApiKey(settings);

  const requestId = createTranslationRequestId();
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
