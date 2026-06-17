import { listen } from "@tauri-apps/api/event";
import { isTauriRuntime } from "./tauriRuntime";

export async function onOpenSettings(callback: () => void): Promise<() => void> {
  if (!isTauriRuntime()) {
    return () => {};
  }

  return listen("open-settings", callback);
}
