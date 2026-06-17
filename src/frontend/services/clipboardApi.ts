import { invoke } from "@tauri-apps/api/core";
import { isTauriRuntime } from "./tauriRuntime";

export async function copyToClipboard(text: string): Promise<void> {
  if (isTauriRuntime()) {
    await invoke("copy_to_clipboard", { text });
  }
}
