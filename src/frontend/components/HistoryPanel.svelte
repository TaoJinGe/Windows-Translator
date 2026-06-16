<script lang="ts">
  import { copyText } from "../services/clipboard";
  import { historyStore, loadHistory, removeHistory } from "../stores/historyStore";
  import { formatShortTime } from "../utils/time";

  let copiedKey = "";

  async function refresh(): Promise<void> {
    await loadHistory();
  }

  async function copyHistoryText(key: string, value: string): Promise<void> {
    await copyText(value);
    copiedKey = key;

    window.setTimeout(() => {
      if (copiedKey === key) {
        copiedKey = "";
      }
    }, 1200);
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
          <div class="history-meta">
            <time>{formatShortTime(item.createdAt)}</time>
            <span>{item.sourceLang} → {item.targetLang}</span>
          </div>
          <div class="history-text">
            <div>
              <strong>原文</strong>
              <p>{item.sourceText}</p>
            </div>
            <div>
              <strong>译文</strong>
              <p>{item.translatedText}</p>
            </div>
          </div>
          <div class="history-actions">
            <button
              type="button"
              class:copied={copiedKey === `${item.id}-source`}
              on:click={() => copyHistoryText(`${item.id}-source`, item.sourceText)}
            >
              {copiedKey === `${item.id}-source` ? "已复制原文" : "复制原文"}
            </button>
            <button
              type="button"
              class:copied={copiedKey === `${item.id}-result`}
              on:click={() => copyHistoryText(`${item.id}-result`, item.translatedText)}
            >
              {copiedKey === `${item.id}-result` ? "已复制译文" : "复制译文"}
            </button>
          </div>
        </li>
      {/each}
    </ul>
  {/if}
</section>
