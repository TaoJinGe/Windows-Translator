import { invoke } from "@tauri-apps/api/core";
import { isTauriRuntime } from "./tauriRuntime";

export async function clearHistory(): Promise<void> {
  if (isTauriRuntime()) {
    await invoke("clear_history");
  }
}
