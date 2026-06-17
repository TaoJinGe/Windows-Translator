import { copyToClipboard } from "./clipboardApi";
import { isTauriRuntime } from "./tauriRuntime";

export async function copyText(value: string): Promise<void> {
  if (!value.trim()) {
    return;
  }

  if (isTauriRuntime()) {
    await copyToClipboard(value);
    return;
  }

  await navigator.clipboard.writeText(value);
}
