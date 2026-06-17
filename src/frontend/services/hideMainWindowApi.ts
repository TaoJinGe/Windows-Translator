import { invoke } from "@tauri-apps/api/core";
import { isTauriRuntime } from "./tauriRuntime";

export async function hideMainWindow(): Promise<void> {
  if (isTauriRuntime()) {
    await invoke("hide_main_window");
  }
}
