<script lang="ts">
  import { loadHistory } from "../stores/loadHistory";

  async function refresh(): Promise<void> {
    await loadHistory();
  }
</script>

<button type="button" on:click={refresh}>刷新</button>
