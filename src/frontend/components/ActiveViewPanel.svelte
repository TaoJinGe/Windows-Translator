<script lang="ts">
  import { activeView } from "../stores/appState";
  import HistoryPanel from "./HistoryPanel.svelte";
  import SettingsPanel from "./SettingsPanel.svelte";
  import TranslatePanel from "./TranslatePanel.svelte";
</script>

{#if $activeView === "translate"}
  <TranslatePanel />
{:else if $activeView === "settings"}
  <SettingsPanel />
{:else}
  <HistoryPanel />
{/if}
