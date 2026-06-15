export function formatHotkey(event: KeyboardEvent): string {
  const parts: string[] = [];

  if (event.ctrlKey) parts.push("Ctrl");
  if (event.altKey) parts.push("Alt");
  if (event.shiftKey) parts.push("Shift");
  if (event.metaKey) parts.push("Super");

  const key = normalizeKey(event.key);

  if (key) {
    parts.push(key);
  }

  return parts.join("+");
}

export function matchesHotkey(event: KeyboardEvent, hotkey: string): boolean {
  return formatHotkey(event).toLowerCase() === hotkey.trim().toLowerCase();
}

export function normalizeKey(key: string): string {
  const ignored = ["Control", "Alt", "Shift", "Meta"];

  if (ignored.includes(key)) {
    return "";
  }

  if (key.length === 1) {
    return key.toUpperCase();
  }

  return key;
}
