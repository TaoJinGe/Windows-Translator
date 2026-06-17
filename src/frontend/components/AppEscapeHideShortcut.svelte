<script lang="ts">
  import { hideMainWindow } from "../services/hideMainWindowApi";

  function handleKeydown(event: KeyboardEvent): void {
    if (event.key === "Escape") {
      void hideMainWindow();
    }
  }
</script>

<svelte:window on:keydown={handleKeydown} />
