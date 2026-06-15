<script lang="ts">
  import IconButton from "./IconButton.svelte";

  export let title: string;
</script>

<header class="window-header">
  <h1>{title}</h1>
  <nav aria-label="窗口导航">
    <IconButton label="翻译">译</IconButton>
    <IconButton label="历史">史</IconButton>
    <IconButton label="设置">设</IconButton>
  </nav>
</header>
