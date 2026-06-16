<script lang="ts">
  export let label: string;
  export let value: string;
  export let active = false;
  export let startRecording: () => void;
  export let recordHotkey: (event: KeyboardEvent) => void;
</script>

<label>
  {label}
  <input
    class:recording={active}
    readonly
    value={active ? "请按下快捷键" : value}
    on:focus={startRecording}
    on:click={startRecording}
    on:keydown={recordHotkey}
  />
</label>
