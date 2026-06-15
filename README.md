# Windows Translator

轻量 Windows AI 翻译工具 MVP，使用 Tauri 2、Svelte、TypeScript、Rust 和 Vite。

当前实现范围：Phase 1 到 Phase 3。

## 开发

```bash
npm install
npm run dev
npm run tauri dev
```

## 打包

```bash
npm run build
npm run tauri build
```

推荐使用：

```bash
npm run package
```

该命令会构建 Tauri 应用，并把最终 EXE 和 MSI 复制到根目录 `release/`。

需要本机安装 Rust 工具链后才能运行 Tauri 命令。
