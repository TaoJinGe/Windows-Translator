<script lang="ts">
  import type { HistoryItem as HistoryEntry } from "../types/history";
  import { formatShortTime } from "../utils/time";
  import CopyHistoryTextButton from "./CopyHistoryTextButton.svelte";

  export let item: HistoryEntry;
</script>

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
    <CopyHistoryTextButton text={item.sourceText} idleLabel="复制原文" copiedLabel="已复制原文" />
    <CopyHistoryTextButton text={item.translatedText} idleLabel="复制译文" copiedLabel="已复制译文" />
  </div>
</li>
