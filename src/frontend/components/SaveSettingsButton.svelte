<script lang="ts">
  export let status = "";
</script>

<button type="submit">保存设置</button>
{#if status}
  <p class="status">{status}</p>
{/if}
