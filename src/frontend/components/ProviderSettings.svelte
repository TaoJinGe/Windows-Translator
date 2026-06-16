<script lang="ts">
  import { providerOptions } from "../types/models";
  import type { AppSettings } from "../types/settings";

  export let form: AppSettings;
  export let selectedProvider = providerOptions[0];
  export let selectProvider: (label: string) => void;
</script>

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
