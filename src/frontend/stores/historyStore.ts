import { writable } from "svelte/store";
import { clearHistory, getHistory } from "../services/tauriApi";
import type { TranslationHistoryItem } from "../types/history";

export const historyStore = writable<TranslationHistoryItem[]>([]);

export async function loadHistory(): Promise<void> {
  historyStore.set(await getHistory());
}

export async function removeHistory(): Promise<void> {
  await clearHistory();
  historyStore.set([]);
}
