<script lang="ts">
  import { languagePairOptions } from "../types/languages";
  import type { AppSettings } from "../types/settings";

  export let form: AppSettings;
</script>

<h2>翻译</h2>
<label>
  默认互译语言
  <select bind:value={form.defaultLanguagePair}>
    {#each languagePairOptions as pair}
      <option value={pair.value}>{pair.label}</option>
    {/each}
  </select>
</label>
