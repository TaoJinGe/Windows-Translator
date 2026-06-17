<script lang="ts">
  import { copyText } from "../services/clipboard";

  export let text = "";
  export let idleLabel = "";
  export let copiedLabel = "";

  let copied = false;

  async function copyHistoryText(): Promise<void> {
    await copyText(text);
    copied = true;

    window.setTimeout(() => {
      copied = false;
    }, 1200);
  }
</script>

<button type="button" class:copied on:click={copyHistoryText}>
  {copied ? copiedLabel : idleLabel}
</button>
