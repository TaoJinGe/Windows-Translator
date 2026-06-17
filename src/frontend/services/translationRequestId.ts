export function createTranslationRequestId(): string {
  if ("crypto" in window && "randomUUID" in window.crypto) {
    return window.crypto.randomUUID();
  }

  return `${Date.now()}-${Math.random().toString(16).slice(2)}`;
}
