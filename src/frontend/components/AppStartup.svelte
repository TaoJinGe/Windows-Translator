<script lang="ts">
  import { onMount } from "svelte";
  import { showMainWindow } from "../services/showMainWindowApi";
  import { loadSettings } from "../stores/loadSettings";

  onMount(() => {
    void initializeWindow();
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
</script>
