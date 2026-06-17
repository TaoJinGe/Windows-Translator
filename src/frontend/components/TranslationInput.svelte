<script lang="ts">
  export let value = "";
  export let handleKeydown: (event: KeyboardEvent) => void;
</script>

<textarea
  bind:value
  on:keydown={handleKeydown}
  placeholder="输入要翻译的文本"
></textarea>
