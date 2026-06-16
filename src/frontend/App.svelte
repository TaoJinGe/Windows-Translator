<script lang="ts">
  import { onMount } from "svelte";
  import HistoryPanel from "./components/HistoryPanel.svelte";
  import SettingsPanel from "./components/SettingsPanel.svelte";
  import TranslatePanel from "./components/TranslatePanel.svelte";
  import { hideMainWindow, onOpenSettings, showMainWindow } from "./services/tauriApi";
  import { activeView, type ActiveView } from "./stores/appState";
  import { loadSettings } from "./stores/settingsStore";

  const tabs: { id: ActiveView; label: string }[] = [
    { id: "translate", label: "翻译" },
    { id: "history", label: "历史" },
    { id: "settings", label: "设置" }
  ];

  onMount(() => {
    void initializeWindow();
    const unlisten = onOpenSettings(() => activeView.set("settings"));

    return () => {
      void unlisten.then((stop) => stop());
    };
  });

  async function initializeWindow(): Promise<void> {
    try {
      await loadSettings();
    } catch (error) {
      console.error(error);
    } finally {
      await showMainWindow();
    }
  }

  function handleKeydown(event: KeyboardEvent): void {
    if (event.key === "Escape") {
      void hideMainWindow();
    }
  }
</script>

<svelte:window on:keydown={handleKeydown} />

<main class="app-shell">
  <header class="window-header">
    <h1>Windows Translator</h1>
    <nav aria-label="主导航">
      {#each tabs as tab}
        <button
          type="button"
          class:active={$activeView === tab.id}
          on:click={() => activeView.set(tab.id)}
        >
          {tab.label}
        </button>
      {/each}
    </nav>
  </header>

  {#if $activeView === "translate"}
    <TranslatePanel />
  {:else if $activeView === "settings"}
    <SettingsPanel />
  {:else}
    <HistoryPanel />
  {/if}
</main>
