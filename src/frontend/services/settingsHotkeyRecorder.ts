import type { AppSettings } from "../types/settings";
import { formatHotkey } from "../utils/hotkey";

export function recordSettingsHotkey(
  form: AppSettings,
  recordingField: keyof AppSettings | "",
  event: KeyboardEvent
): { form: AppSettings; recordingField: keyof AppSettings | "" } {
  event.preventDefault();
  event.stopPropagation();

  const hotkey = formatHotkey(event);

  if (!recordingField || !hotkey) {
    return { form, recordingField };
  }

  return {
    form: { ...form, [recordingField]: hotkey },
    recordingField: ""
  };
}
