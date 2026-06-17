import { clearHistory } from "../services/clearHistoryApi";
import { historyStore } from "./historyStore";

export async function removeHistory(): Promise<void> {
  await clearHistory();
  historyStore.set([]);
}
