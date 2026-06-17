import { writable } from "svelte/store";
import type { TranslationHistoryItem } from "../types/history";

export const historyStore = writable<TranslationHistoryItem[]>([]);
