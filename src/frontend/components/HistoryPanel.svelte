<script lang="ts">
  import { historyStore, loadHistory, removeHistory } from "../stores/historyStore";
  import { formatShortTime } from "../utils/time";

  async function refresh(): Promise<void> {
    await loadHistory();
  }
</script>

<section class="panel history-panel">
  <div class="actions">
    <button type="button" on:click={refresh}>刷新</button>
    <button type="button" on:click={removeHistory}>清空历史</button>
  </div>
  {#if $historyStore.length === 0}
    <p class="muted">暂无历史记录</p>
  {:else}
    <ul>
      {#each $historyStore as item}
        <li>
          <time>{formatShortTime(item.createdAt)}</time>
          <strong>{item.targetLang}</strong>
          <p>{item.translatedText}</p>
        </li>
      {/each}
    </ul>
  {/if}
</section>
