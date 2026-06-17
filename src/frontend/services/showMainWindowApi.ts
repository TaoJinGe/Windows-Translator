import { invoke } from "@tauri-apps/api/core";
import { isTauriRuntime } from "./tauriRuntime";

export async function showMainWindow(): Promise<void> {
  if (isTauriRuntime()) {
    await invoke("show_main_window");
  }
}
