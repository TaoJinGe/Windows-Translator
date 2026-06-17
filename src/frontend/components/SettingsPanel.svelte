<script lang="ts">
  import { onDestroy } from "svelte";
  import { get } from "svelte/store";
  import { applyProviderDefaults } from "../services/applyProviderDefaults";
  import { findSelectedProvider } from "../services/findSelectedProvider";
  import { recordSettingsHotkey } from "../services/settingsHotkeyRecorder";
  import { persistSettings } from "../stores/persistSettings";
  import { settingsStore } from "../stores/settingsStore";
  import type { AppSettings } from "../types/settings";
  import { normalizeError } from "../utils/text";
  import HotkeySettings from "./HotkeySettings.svelte";
  import ProviderSettings from "./ProviderSettings.svelte";
  import SaveSettingsButton from "./SaveSettingsButton.svelte";
  import TranslationSettings from "./TranslationSettings.svelte";
  import WindowSettings from "./WindowSettings.svelte";

  let form: AppSettings = get(settingsStore);
  let status = "";
  let recordingField: keyof AppSettings | "" = "";

  const unsubscribe = settingsStore.subscribe((value) => {
    form = { ...value };
  });

  onDestroy(unsubscribe);

  async function save(): Promise<void> {
    status = "";

    try {
      await persistSettings(form);
      status = "设置已保存";
    } catch (error) {
      status = normalizeError(error);
      window.alert(status);
    }
  }

  function selectProvider(label: string): void {
    form = applyProviderDefaults(form, label);
  }

  function recordHotkey(event: KeyboardEvent): void {
    const recorded = recordSettingsHotkey(form, recordingField, event);
    form = recorded.form;
    recordingField = recorded.recordingField;
  }

  function startRecording(field: keyof AppSettings): void {
    recordingField = field;
  }

  $: selectedProvider =
    findSelectedProvider(form.apiBaseUrl);
</script>

<form class="panel settings-panel" on:submit|preventDefault={save}>
  <ProviderSettings {form} {selectedProvider} {selectProvider} />
  <TranslationSettings {form} />
  <HotkeySettings {form} {recordingField} {startRecording} {recordHotkey} />
  <WindowSettings {form} />
  <SaveSettingsButton {status} />
</form>
