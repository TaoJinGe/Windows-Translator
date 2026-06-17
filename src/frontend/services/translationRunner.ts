import { requestTranslation } from "./translator";

type StreamUpdate = (value: string) => void;

export async function runTranslation(
  sourceText: string,
  sourceLang: string,
  targetLang: string,
  onStreamUpdate: StreamUpdate
): Promise<string> {
  let streamedText = "";

  return requestTranslation(sourceText, sourceLang, targetLang, (delta) => {
    streamedText += delta;
    onStreamUpdate(streamedText);
  });
}
