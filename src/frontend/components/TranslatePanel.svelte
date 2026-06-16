<script lang="ts">
  import { copyText } from "../services/clipboard";
  import { requestTranslation } from "../services/translator";
  import { persistSettings, settingsStore } from "../stores/settingsStore";
  import { matchesHotkey } from "../utils/hotkey";
  import { hasText, normalizeError } from "../utils/text";
  import ClearButton from "./ClearButton.svelte";
  import CopyResultButton from "./CopyResultButton.svelte";
  import LanguageSelector from "./LanguageSelector.svelte";
  import TranslateButton from "./TranslateButton.svelte";

  let sourceText = "";
  let translatedText = "";
  let loading = false;
  let errorMessage = "";
  let savingLanguage = false;

  async function translate(): Promise<void> {
    if (!hasText(sourceText) || loading) {
      return;
    }

    loading = true;
    errorMessage = "";
    translatedText = "";

    try {
      translatedText = await requestTranslation(
        sourceText,
        $settingsStore.sourceLang,
        $settingsStore.targetLang,
        (delta) => {
          translatedText += delta;
        }
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
  }

  async function copyResult(): Promise<void> {
    try {
      await copyText(translatedText);
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
  <LanguageSelector
    sourceLang={$settingsStore.sourceLang}
    targetLang={$settingsStore.targetLang}
    on:change={(event) => updateLanguage(event.detail.sourceLang, event.detail.targetLang)}
    on:swap={swapLanguages}
  />
  <textarea
    bind:value={sourceText}
    on:keydown={handleKeydown}
    placeholder="输入要翻译的文本"
  ></textarea>
  <div class="actions">
    <TranslateButton {loading} on:click={translate} />
    <CopyResultButton on:click={copyResult} />
    <ClearButton on:click={clearAll} />
  </div>
  {#if errorMessage}
    <p class="error">{errorMessage}</p>
  {/if}
  <textarea class="result" bind:value={translatedText} readonly placeholder="翻译结果"></textarea>
</section>
