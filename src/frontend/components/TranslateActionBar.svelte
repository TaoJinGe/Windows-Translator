<script lang="ts">
  import ClearButton from "./ClearButton.svelte";
  import CopyResultButton from "./CopyResultButton.svelte";
  import TranslateButton from "./TranslateButton.svelte";

  export let loading = false;
  export let translate: () => void;
  export let copyResult: () => void;
  export let clearAll: () => void;
</script>

<div class="actions">
  <TranslateButton {loading} on:click={translate} />
  <CopyResultButton on:click={copyResult} />
  <ClearButton on:click={clearAll} />
</div>
