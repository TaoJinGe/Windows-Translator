import { invoke } from "@tauri-apps/api/core";
import type { TranslationHistoryItem } from "../types/history";
import { isTauriRuntime } from "./tauriRuntime";

export async function getHistory(): Promise<TranslationHistoryItem[]> {
  if (!isTauriRuntime()) {
    return [];
  }

  return invoke<TranslationHistoryItem[]>("get_history");
}
