export interface AppSettings {
  settingsVersion: number;
  apiBaseUrl: string;
  apiKey: string;
  model: string;
  sourceLang: string;
  targetLang: string;
  defaultLanguagePair: string;
  hotkey: string;
  translateHotkey: string;
  clearHotkey: string;
  alwaysOnTop: boolean;
  streamOutput: boolean;
  launchAtStartup: boolean;
  closeAction: "tray" | "minimize";
}

export const defaultSettings: AppSettings = {
  settingsVersion: 2,
  apiBaseUrl: "https://api.openai.com/v1",
  apiKey: "",
  model: "gpt-4o-mini",
  sourceLang: "auto",
  targetLang: "auto",
  defaultLanguagePair: "zh-CN|en",
  hotkey: "Ctrl+Alt+T",
  translateHotkey: "Ctrl+Enter",
  clearHotkey: "Ctrl+Backspace",
  alwaysOnTop: false,
  streamOutput: false,
  launchAtStartup: false,
  closeAction: "tray"
};
