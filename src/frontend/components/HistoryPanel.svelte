<script lang="ts">
  import { historyStore } from "../stores/historyStore";
  import ClearHistoryButton from "./ClearHistoryButton.svelte";
  import HistoryItem from "./HistoryItem.svelte";
  import HistoryRefreshButton from "./HistoryRefreshButton.svelte";
</script>

<section class="panel history-panel">
  <div class="actions">
    <HistoryRefreshButton />
    <ClearHistoryButton />
  </div>
  {#if $historyStore.length === 0}
    <p class="muted">暂无历史记录</p>
  {:else}
    <ul>
      {#each $historyStore as item}
        <HistoryItem {item} />
      {/each}
    </ul>
  {/if}
</section>
