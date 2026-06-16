<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import { languageOptions } from "../types/languages";

  export let sourceLang: string;
  export let targetLang: string;

  const dispatch = createEventDispatcher<{
    change: { sourceLang: string; targetLang: string };
    swap: void;
  }>();
</script>

<div class="language-row">
  <select
    aria-label="源语言"
    value={sourceLang}
    on:change={(event) =>
      dispatch("change", { sourceLang: event.currentTarget.value, targetLang })}
  >
    {#each languageOptions as language}
      <option value={language.value}>{language.label}</option>
    {/each}
  </select>
  <button type="button" class="swap-button" on:click={() => dispatch("swap")} title="交换语言">
    ⇄
  </button>
  <select
    aria-label="目标语言"
    value={targetLang}
    on:change={(event) =>
      dispatch("change", { sourceLang, targetLang: event.currentTarget.value })}
  >
    {#each languageOptions as language}
      <option value={language.value}>{language.label}</option>
    {/each}
  </select>
</div>
