export interface TranslateRequest {
  sourceText: string;
  sourceLang: string;
  targetLang: string;
}

export interface TranslateResponse {
  translatedText: string;
}
