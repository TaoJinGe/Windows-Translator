<script lang="ts">
  import { onMount } from "svelte";
  import { onOpenSettings } from "../services/openSettingsEventApi";
  import { activeView } from "../stores/appState";

  onMount(() => {
    const unlisten = onOpenSettings(() => activeView.set("settings"));

    return () => {
      void unlisten.then((stop) => stop());
    };
  });
</script>
