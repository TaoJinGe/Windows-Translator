<script lang="ts">
  import ActiveViewPanel from "./components/ActiveViewPanel.svelte";
  import AppLifecycle from "./components/AppLifecycle.svelte";
  import AppNav from "./components/AppNav.svelte";
</script>

<AppLifecycle />

<main class="app-shell">
  <header class="window-header">
    <h1>Windows Translator</h1>
    <AppNav />
  </header>

  <ActiveViewPanel />
</main>
