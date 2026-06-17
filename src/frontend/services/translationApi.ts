import { invoke } from "@tauri-apps/api/core";
import type { TranslateRequest, TranslateResponse } from "../types/translation";

export async function translateText(request: TranslateRequest): Promise<TranslateResponse> {
  return invoke<TranslateResponse>("translate_text", { request });
}
