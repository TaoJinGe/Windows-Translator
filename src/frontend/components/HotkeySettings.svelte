<script lang="ts">
  import type { AppSettings } from "../types/settings";
  import HotkeyInput from "./HotkeyInput.svelte";

  export let form: AppSettings;
  export let recordingField: keyof AppSettings | "";
  export let startRecording: (field: keyof AppSettings) => void;
  export let recordHotkey: (event: KeyboardEvent) => void;
</script>

<h2>快捷键</h2>
<HotkeyInput
  label="弹出主界面 / 隐藏到托盘"
  value={form.hotkey}
  active={recordingField === "hotkey"}
  startRecording={() => startRecording("hotkey")}
  {recordHotkey}
/>
<HotkeyInput
  label="首页翻译"
  value={form.translateHotkey}
  active={recordingField === "translateHotkey"}
  startRecording={() => startRecording("translateHotkey")}
  {recordHotkey}
/>
<HotkeyInput
  label="首页清空"
  value={form.clearHotkey}
  active={recordingField === "clearHotkey"}
  startRecording={() => startRecording("clearHotkey")}
  {recordHotkey}
/>
