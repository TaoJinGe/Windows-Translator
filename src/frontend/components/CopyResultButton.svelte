<script lang="ts">
  export let status = "复制";
</script>

<button type="button" class:copied={status === "已复制"} on:click>
  {status}
</button>
