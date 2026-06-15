<script lang="ts">
  export let label: string;
  export let title = label;
  export let type: "button" | "submit" = "button";
</script>

<button class="icon-button" {type} {title} aria-label={label} on:click>
  <slot />
</button>
