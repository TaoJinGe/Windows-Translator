<script lang="ts">
  import { copyText } from "../services/clipboard";
  import { saveTranslationLanguages } from "../services/languageSettings";
  import { runTranslation } from "../services/translationRunner";
  import { settingsStore } from "../stores/settingsStore";
  import { matchesHotkey } from "../utils/hotkey";
  import { hasText, normalizeError } from "../utils/text";
  import LanguageSelector from "./LanguageSelector.svelte";
  import TranslateActionBar from "./TranslateActionBar.svelte";
  import TranslationInput from "./TranslationInput.svelte";
  import TranslationResultBox from "./TranslationResultBox.svelte";

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
      translatedText = await runTranslation(
        sourceText,
        $settingsStore.sourceLang,
        $settingsStore.targetLang,
        (value) => {
          translatedText = value;
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
      await saveTranslationLanguages($settingsStore, sourceLang, targetLang);
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
  <TranslationInput bind:value={sourceText} {handleKeydown} />
  <TranslateActionBar {loading} {translate} {copyResult} {clearAll} />
  {#if errorMessage}
    <p class="error">{errorMessage}</p>
  {/if}
  <TranslationResultBox bind:value={translatedText} />
</section>
