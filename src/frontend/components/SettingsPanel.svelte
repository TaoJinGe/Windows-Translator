<script lang="ts">
  import { onDestroy } from "svelte";
  import { get } from "svelte/store";
  import { persistSettings, settingsStore } from "../stores/settingsStore";
  import { languagePairOptions } from "../types/languages";
  import { providerOptions } from "../types/models";
  import type { AppSettings } from "../types/settings";
  import { formatHotkey } from "../utils/hotkey";
  import { normalizeError } from "../utils/text";

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

  $: selectedProvider =
    providerOptions.find((item) => item.apiBaseUrl === form.apiBaseUrl) ??
    providerOptions[providerOptions.length - 1];
</script>

<form class="panel settings-panel" on:submit|preventDefault={save}>
  <h2>模型</h2>
  <label>
    模型公司
    <select value={selectedProvider.label} on:change={(event) => selectProvider(event.currentTarget.value)}>
      {#each providerOptions as provider}
        <option value={provider.label}>{provider.label}</option>
      {/each}
    </select>
  </label>
  <label>
    模型名称
    <select bind:value={form.model}>
      {#each selectedProvider.models as model}
        <option value={model.value}>{model.label}</option>
      {/each}
    </select>
  </label>
  <label>API Base URL <input bind:value={form.apiBaseUrl} /></label>
  <label>API Key <input type="password" bind:value={form.apiKey} /></label>
  {#if selectedProvider.label === "自定义 OpenAI 兼容"}
    <label>自定义模型名 <input bind:value={form.model} /></label>
  {/if}

  <h2>翻译</h2>
  <label>
    默认互译语言
    <select bind:value={form.defaultLanguagePair}>
      {#each languagePairOptions as pair}
        <option value={pair.value}>{pair.label}</option>
      {/each}
    </select>
  </label>

  <h2>快捷键</h2>
  <label>
    弹出主界面 / 隐藏到托盘
    <input
      class:recording={recordingField === "hotkey"}
      readonly
      value={recordingField === "hotkey" ? "请按下快捷键" : form.hotkey}
      on:focus={() => (recordingField = "hotkey")}
      on:click={() => (recordingField = "hotkey")}
      on:keydown={recordHotkey}
    />
  </label>
  <label>
    首页翻译
    <input
      class:recording={recordingField === "translateHotkey"}
      readonly
      value={recordingField === "translateHotkey" ? "请按下快捷键" : form.translateHotkey}
      on:focus={() => (recordingField = "translateHotkey")}
      on:click={() => (recordingField = "translateHotkey")}
      on:keydown={recordHotkey}
    />
  </label>
  <label>
    首页清空
    <input
      class:recording={recordingField === "clearHotkey"}
      readonly
      value={recordingField === "clearHotkey" ? "请按下快捷键" : form.clearHotkey}
      on:focus={() => (recordingField = "clearHotkey")}
      on:click={() => (recordingField = "clearHotkey")}
      on:keydown={recordHotkey}
    />
  </label>

  <h2>窗口</h2>
  <label>
    关闭按钮
    <select bind:value={form.closeAction}>
      <option value="tray">隐藏到托盘</option>
      <option value="minimize">最小化到任务栏</option>
    </select>
  </label>
  <label class="check"><input type="checkbox" bind:checked={form.alwaysOnTop} /> 窗口置顶</label>
  <label class="check"><input type="checkbox" bind:checked={form.streamOutput} /> 流式输出</label>
  <button type="submit">保存设置</button>
  {#if status}
    <p class="status">{status}</p>
  {/if}
</form>
