# ROOT_MAP_1

覆盖范围：根目录、前端源码、脚本、公共资源。不要覆盖 `src-tauri/`。

## 根目录

/package.json —— npm 脚本和前端/Tauri CLI 依赖入口（修改影响构建命令）
/package-lock.json —— npm 依赖锁定文件（修改影响依赖复现）
/vite.config.ts —— Vite 和 Svelte 构建配置（修改影响前端开发与打包）
/tsconfig.json —— TypeScript 严格检查配置（修改影响前端类型检查）
/index.html —— Svelte 应用 HTML 挂载入口（修改影响页面加载）
/README.md —— 项目使用说明和构建命令（修改影响用户文档）
/.gitignore —— 忽略依赖、构建产物、本地文件（修改影响版本控制）

## 前端源码

/src/frontend/App.svelte —— 前端主布局和页面切换入口（修改影响整体 UI）
/src/frontend/main.ts —— Svelte 应用挂载入口（修改影响前端启动）
/src/frontend/components/ —— UI 组件目录，翻译页/设置页/历史页在这里（修改影响界面）
/src/frontend/components/TranslatePanel.svelte —— 翻译页容器组件，只编排翻译流程和局部状态（修改影响核心翻译交互）
/src/frontend/components/LanguageSelector.svelte —— 语言选择条组件，只负责源/目标语言选择与交换入口（修改影响语言栏 UI）
/src/frontend/components/TranslateButton.svelte —— 翻译按钮组件，只负责翻译动作入口和加载态（修改影响翻译按钮 UI）
/src/frontend/components/CopyResultButton.svelte —— 复制结果按钮组件，只负责复制状态展示入口（修改影响复制按钮 UI）
/src/frontend/components/ClearButton.svelte —— 清空按钮组件，只负责清空动作入口（修改影响清空按钮 UI）
/src/frontend/components/SettingsPanel.svelte —— 设置页容器组件，只编排设置表单保存和快捷键录制状态（修改影响设置流程）
/src/frontend/components/ProviderSettings.svelte —— 模型设置组件，只负责服务商、模型、API 地址和密钥输入（修改影响模型配置 UI）
/src/frontend/components/TranslationSettings.svelte —— 翻译默认设置组件，只负责默认互译语言选择（修改影响默认语言配置 UI）
/src/frontend/components/HotkeySettings.svelte —— 快捷键设置组组件，只负责三个快捷键输入的组合布局（修改影响快捷键配置 UI）
/src/frontend/components/HotkeyInput.svelte —— 单个快捷键输入组件，只负责录制态输入框显示和键盘事件入口（修改影响快捷键输入交互）
/src/frontend/components/WindowSettings.svelte —— 窗口设置组件，只负责关闭行为、置顶、流式输出选项（修改影响窗口配置 UI）
/src/frontend/components/SaveSettingsButton.svelte —— 保存设置按钮组件，只负责提交入口和保存状态展示（修改影响保存按钮 UI）
/src/frontend/components/HistoryPanel.svelte —— 历史页组件，只负责历史读取、刷新和清空展示（修改影响历史 UI）
/src/frontend/components/IconButton.svelte —— 通用图标按钮外壳组件（修改影响复用按钮视觉）
/src/frontend/components/WindowHeader.svelte —— 预留窗口标题栏组件（修改影响全局头部 UI）
/src/frontend/services/ —— Tauri invoke 和业务服务封装目录（修改影响前后端调用）
/src/frontend/stores/ —— Svelte 状态管理目录（修改影响设置、历史、页面状态）
/src/frontend/types/ —— TypeScript 类型和选项数据目录（修改影响类型和下拉选项）
/src/frontend/utils/ —— 前端纯工具函数目录（修改影响文本、时间、快捷键判断）
/src/styles/app.css —— 全局样式文件（修改影响全部界面视觉）
/src/shared/ —— 预留共享代码目录，目前为空（新增影响跨端共享逻辑）

## 辅助目录

/public/favicon.svg —— 浏览器预览和图标生成源图（修改影响应用图标）
/scripts/package.ps1 —— 构建并复制 EXE/MSI 到 release 的脚本（修改影响发布流程）
/release/ —— EXE/MSI 发布产物目录，可由脚本再生成（AI 禁止修改产物）
/dist/ —— Vite 构建输出目录，可再生成（AI 禁止修改产物）
/node_modules/ —— npm 依赖目录，可再安装（AI 禁止修改依赖内容）
/.playwright-mcp/ —— 浏览器测试日志目录，可删除（AI 禁止作为源码修改）

## Tree Maps

/docs/tree/root.md —— 根目录文件和生成目录索引
/docs/tree/src.md —— src 顶层结构索引
/docs/tree/src_frontend.md —— 前端入口和子目录索引
/docs/tree/src_frontend_components.md —— 前端组件索引
/docs/tree/src_frontend_services.md —— 前端服务索引
/docs/tree/src_frontend_stores.md —— 前端状态索引
/docs/tree/src_frontend_types.md —— 前端类型/选项索引
/docs/tree/src_frontend_utils.md —— 前端工具函数索引
/docs/tree/src_styles.md —— 样式目录索引
/docs/tree/scripts.md —— 脚本目录索引
/docs/tree/public.md —— 公共资源目录索引
