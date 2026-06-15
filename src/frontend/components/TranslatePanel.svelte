<script lang="ts">
  import { copyText } from "../services/clipboard";
  import { requestTranslation } from "../services/translator";
  import { persistSettings, settingsStore } from "../stores/settingsStore";
  import { languageOptions } from "../types/languages";
  import { matchesHotkey } from "../utils/hotkey";
  import { hasText, normalizeError } from "../utils/text";

  let sourceText = "";
  let translatedText = "";
  let loading = false;
  let errorMessage = "";
  let savingLanguage = false;
  let copyStatus = "复制";

  async function translate(): Promise<void> {
    if (!hasText(sourceText) || loading) {
      return;
    }

    loading = true;
    errorMessage = "";

    try {
      translatedText = await requestTranslation(
        sourceText,
        $settingsStore.sourceLang,
        $settingsStore.targetLang
      );
    } catch (error) {
      errorMessage = normalizeError(error);
    } finally {
      loading = false;
    }
  }

  function clearAll(): void {
    sourceText = "";
    translatedText = "";
    errorMessage = "";
    copyStatus = "复制";
  }

  async function copyResult(): Promise<void> {
    try {
      await copyText(translatedText);
      copyStatus = "已复制";
      window.setTimeout(() => {
        copyStatus = "复制";
      }, 1200);
    } catch (error) {
      errorMessage = normalizeError(error);
    }
  }

  function handleKeydown(event: KeyboardEvent): void {
    if (matchesHotkey(event, $settingsStore.translateHotkey)) {
      event.preventDefault();
      void translate();
    }

    if (matchesHotkey(event, $settingsStore.clearHotkey)) {
      event.preventDefault();
      clearAll();
    }
  }

  async function updateLanguage(sourceLang: string, targetLang: string): Promise<void> {
    if (savingLanguage) {
      return;
    }

    savingLanguage = true;

    try {
      await persistSettings({ ...$settingsStore, sourceLang, targetLang });
    } catch (error) {
      errorMessage = normalizeError(error);
    } finally {
      savingLanguage = false;
    }
  }

  function swapLanguages(): void {
    if ($settingsStore.sourceLang === "auto") {
      return;
    }

    void updateLanguage($settingsStore.targetLang, $settingsStore.sourceLang);
  }
</script>

<section class="panel">
  <div class="language-row">
    <select
      aria-label="源语言"
      value={$settingsStore.sourceLang}
      on:change={(event) =>
        updateLanguage(event.currentTarget.value, $settingsStore.targetLang)}
    >
      {#each languageOptions as language}
        <option value={language.value}>{language.label}</option>
      {/each}
    </select>
    <button type="button" class="swap-button" on:click={swapLanguages} title="交换语言">⇄</button>
    <select
      aria-label="目标语言"
      value={$settingsStore.targetLang}
      on:change={(event) =>
        updateLanguage($settingsStore.sourceLang, event.currentTarget.value)}
    >
      {#each languageOptions as language}
        <option value={language.value}>{language.label}</option>
      {/each}
    </select>
  </div>
  <textarea
    bind:value={sourceText}
    on:keydown={handleKeydown}
    placeholder="输入要翻译的文本"
  ></textarea>
  <div class="actions">
    <button type="button" on:click={translate} disabled={loading}>
      {loading ? "翻译中" : "翻译"}
    </button>
    <button type="button" class:copied={copyStatus === "已复制"} on:click={copyResult}>
      {copyStatus}
    </button>
    <button type="button" on:click={clearAll}>清空</button>
  </div>
  {#if errorMessage}
    <p class="error">{errorMessage}</p>
  {/if}
  <textarea class="result" bind:value={translatedText} readonly placeholder="翻译结果"></textarea>
</section>
