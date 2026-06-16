<script lang="ts">
  export let loading = false;
</script>

<button type="button" on:click disabled={loading}>
  {loading ? "翻译中" : "翻译"}
</button>
