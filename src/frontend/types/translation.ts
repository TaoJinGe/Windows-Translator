export interface TranslateRequest {
  requestId?: string;
  sourceText: string;
  sourceLang: string;
  targetLang: string;
}

export interface TranslateResponse {
  translatedText: string;
}

export interface TranslationStreamEvent {
  requestId: string;
  delta: string;
  done: boolean;
}
