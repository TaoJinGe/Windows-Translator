import { getHistory } from "../services/getHistoryApi";
import { historyStore } from "./historyStore";

export async function loadHistory(): Promise<void> {
  historyStore.set(await getHistory());
}
