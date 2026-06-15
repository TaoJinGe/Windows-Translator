export interface AppSettings {
  apiBaseUrl: string;
  apiKey: string;
  model: string;
  sourceLang: string;
  targetLang: string;
  hotkey: string;
  translateHotkey: string;
  clearHotkey: string;
  alwaysOnTop: boolean;
  streamOutput: boolean;
  closeAction: "tray" | "minimize";
}

export const defaultSettings: AppSettings = {
  apiBaseUrl: "https://api.openai.com/v1",
  apiKey: "",
  model: "gpt-4o-mini",
  sourceLang: "auto",
  targetLang: "zh-CN",
  hotkey: "Ctrl+Alt+T",
  translateHotkey: "Ctrl+Enter",
  clearHotkey: "Ctrl+Backspace",
  alwaysOnTop: false,
  streamOutput: false,
  closeAction: "tray"
};
