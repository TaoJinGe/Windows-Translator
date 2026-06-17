<script lang="ts">
  import type { AppSettings } from "../types/settings";

  export let form: AppSettings;
</script>

<h2>窗口</h2>
<label>
  关闭按钮
  <select bind:value={form.closeAction}>
    <option value="tray">隐藏到托盘</option>
    <option value="minimize">最小化到任务栏</option>
  </select>
</label>
<label class="check"><input type="checkbox" bind:checked={form.alwaysOnTop} /> 窗口置顶</label>
<label class="check"><input type="checkbox" bind:checked={form.streamOutput} /> 流式输出</label>
<label class="check"><input type="checkbox" bind:checked={form.launchAtStartup} /> 开机自启动</label>
<label class="check"><input type="checkbox" bind:checked={form.rememberWindowPosition} /> 上次关闭位置打开</label>
