<script lang="ts">
  import { onDestroy } from "svelte";
  import { get } from "svelte/store";
  import { persistSettings, settingsStore } from "../stores/settingsStore";
  import { providerOptions } from "../types/models";
  import type { AppSettings } from "../types/settings";
  import { formatHotkey } from "../utils/hotkey";
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
    const provider = providerOptions.find((item) => item.label === label);

    if (!provider) {
      return;
    }

    form.apiBaseUrl = provider.apiBaseUrl || form.apiBaseUrl;
    form.model = provider.models[0]?.value || form.model;
  }

  function recordHotkey(event: KeyboardEvent): void {
    event.preventDefault();
    event.stopPropagation();

    const hotkey = formatHotkey(event);

    if (!recordingField || !hotkey) {
      return;
    }

    form = { ...form, [recordingField]: hotkey };
    recordingField = "";
  }

  function startRecording(field: keyof AppSettings): void {
    recordingField = field;
  }

  $: selectedProvider =
    providerOptions.find((item) => item.apiBaseUrl === form.apiBaseUrl) ??
    providerOptions[providerOptions.length - 1];
</script>

<form class="panel settings-panel" on:submit|preventDefault={save}>
  <ProviderSettings {form} {selectedProvider} {selectProvider} />
  <TranslationSettings {form} />
  <HotkeySettings {form} {recordingField} {startRecording} {recordHotkey} />
  <WindowSettings {form} />
  <SaveSettingsButton {status} />
</form>
