<script lang="ts">
  import { activeView, type ActiveView } from "../stores/appState";

  const tabs: { id: ActiveView; label: string }[] = [
    { id: "translate", label: "翻译" },
    { id: "history", label: "历史" },
    { id: "settings", label: "设置" }
  ];
</script>

<nav aria-label="主导航">
  {#each tabs as tab}
    <button
      type="button"
      class:active={$activeView === tab.id}
      on:click={() => activeView.set(tab.id)}
    >
      {tab.label}
    </button>
  {/each}
</nav>
