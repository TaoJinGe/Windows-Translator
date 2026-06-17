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

/src/frontend/App.svelte —— 前端应用外壳组件，只负责全局布局承载（修改影响整体 UI）
/src/frontend/main.ts —— Svelte 应用挂载入口（修改影响前端启动）
/src/frontend/components/ —— UI 组件目录，翻译页/设置页/历史页在这里（修改影响界面）
/src/frontend/components/AppLifecycle.svelte —— 应用生命周期组件，只负责生命周期子组件挂载（修改影响启动监听）
/src/frontend/components/AppStartup.svelte —— 应用启动组件，只负责加载设置后显示窗口（修改影响启动流程）
/src/frontend/components/AppOpenSettingsListener.svelte —— 设置打开监听组件，只负责托盘设置事件响应（修改影响设置入口）
/src/frontend/components/AppEscapeHideShortcut.svelte —— Escape 隐藏组件，只负责 Escape 隐藏窗口快捷键（修改影响窗口快捷键）
/src/frontend/components/AppNav.svelte —— 应用导航组件，只负责主导航按钮组（修改影响导航 UI）
/src/frontend/components/ActiveViewPanel.svelte —— 当前页面组件，只负责按状态渲染页面组件（修改影响页面切换）
/src/frontend/components/TranslatePanel.svelte —— 翻译页容器组件，只编排翻译流程和局部状态（修改影响核心翻译交互）
/src/frontend/components/LanguageSelector.svelte —— 语言选择条组件，只负责源/目标语言选择与交换入口（修改影响语言栏 UI）
/src/frontend/components/TranslationInput.svelte —— 翻译原文输入框组件，只负责原文输入控件（修改影响原文输入 UI）
/src/frontend/components/TranslationResultBox.svelte —— 翻译结果框组件，只负责译文只读展示（修改影响结果展示 UI）
/src/frontend/components/TranslateActionBar.svelte —— 翻译操作栏组件，只负责操作区按钮布局（修改影响翻译操作区 UI）
/src/frontend/components/TranslateButton.svelte —— 翻译按钮组件，只负责翻译动作入口状态（修改影响翻译按钮 UI）
/src/frontend/components/CopyResultButton.svelte —— 复制结果按钮组件，只负责复制状态展示入口（修改影响复制按钮 UI）
/src/frontend/components/ClearButton.svelte —— 清空按钮组件，只负责清空动作入口（修改影响清空按钮 UI）
/src/frontend/components/SettingsPanel.svelte —— 设置页容器组件，只编排设置表单保存和子组件数据流（修改影响设置流程）
/src/frontend/components/ProviderSettings.svelte —— 模型设置组件，只负责模型连接表单（修改影响模型配置 UI）
/src/frontend/components/TranslationSettings.svelte —— 翻译默认设置组件，只负责默认互译语言选择（修改影响默认语言配置 UI）
/src/frontend/components/HotkeySettings.svelte —— 快捷键设置组组件，只负责三个快捷键输入的组合布局（修改影响快捷键配置 UI）
/src/frontend/components/HotkeyInput.svelte —— 单个快捷键输入组件，只负责快捷键录制输入控件（修改影响快捷键输入交互）
/src/frontend/components/WindowSettings.svelte —— 窗口设置组件，只负责窗口偏好表单（修改影响窗口配置 UI）
/src/frontend/components/SaveSettingsButton.svelte —— 保存设置按钮组件，只负责设置提交按钮状态（修改影响保存按钮 UI）
/src/frontend/components/HistoryPanel.svelte —— 历史页容器组件，只负责历史列表编排（修改影响历史 UI）
/src/frontend/components/HistoryRefreshButton.svelte —— 历史刷新按钮组件，只负责触发历史重新读取（修改影响刷新按钮交互）
/src/frontend/components/ClearHistoryButton.svelte —— 清空历史按钮组件，只负责触发历史清空（修改影响清空按钮交互）
/src/frontend/components/HistoryItem.svelte —— 历史条目组件，只负责单条历史记录展示布局（修改影响历史条目 UI）
/src/frontend/components/CopyHistoryTextButton.svelte —— 历史文本复制按钮组件，只负责历史文本复制按钮状态（修改影响历史复制按钮交互）
/src/frontend/components/IconButton.svelte —— 通用图标按钮外壳组件（修改影响复用按钮视觉）
/src/frontend/components/WindowHeader.svelte —— 预留窗口标题栏组件（修改影响全局头部 UI）
/src/frontend/services/ —— Tauri invoke 和业务服务封装目录（修改影响前后端调用）
/src/frontend/services/tauriRuntime.ts —— Tauri 运行环境判断服务，只负责识别桌面运行时（修改影响环境分支）
/src/frontend/services/getSettingsApi.ts —— 设置读取 API 服务，只负责读取设置 invoke（修改影响设置加载）
/src/frontend/services/saveSettingsApi.ts —— 设置保存 API 服务，只负责保存设置 invoke（修改影响设置保存）
/src/frontend/services/getHistoryApi.ts —— 历史读取 API 服务，只负责读取历史 invoke（修改影响历史加载）
/src/frontend/services/clearHistoryApi.ts —— 历史清空 API 服务，只负责清空历史 invoke（修改影响历史清空）
/src/frontend/services/translationApi.ts —— 翻译 API 服务，只负责翻译 invoke 调用（修改影响翻译通信）
/src/frontend/services/translationStreamApi.ts —— 翻译流事件服务，只负责监听翻译流式事件（修改影响流式输出通信）
/src/frontend/services/translationRequestId.ts —— 翻译请求 ID 服务，只负责生成翻译请求唯一标识（修改影响翻译事件匹配）
/src/frontend/services/translationApiKey.ts —— 翻译密钥校验服务，只负责检查翻译 API Key 可用性（修改影响翻译前置校验）
/src/frontend/services/clipboardApi.ts —— 剪贴板 API 服务，只负责桌面剪贴板 invoke（修改影响桌面复制）
/src/frontend/services/hideMainWindowApi.ts —— 隐藏窗口 API 服务，只负责隐藏主窗口 invoke（修改影响窗口隐藏）
/src/frontend/services/showMainWindowApi.ts —— 显示窗口 API 服务，只负责显示主窗口 invoke（修改影响窗口显示）
/src/frontend/services/openSettingsEventApi.ts —— 设置打开事件服务，只负责监听设置页打开事件（修改影响设置入口通信）
/src/frontend/services/languageSettings.ts —— 翻译语言保存服务，只负责持久化翻译语言对（修改影响语言设置保存）
/src/frontend/services/translationRunner.ts —— 翻译执行服务，只负责聚合流式翻译结果文本（修改影响翻译结果生成）
/src/frontend/services/settingsHotkeyRecorder.ts —— 设置快捷键录制服务，只负责把键盘事件转换为设置热键值（修改影响快捷键录制）
/src/frontend/services/findSelectedProvider.ts —— 选中服务商服务，只负责按 API 地址查找服务商（修改影响服务商显示）
/src/frontend/services/applyProviderDefaults.ts —— 服务商默认值服务，只负责套用服务商默认模型配置（修改影响服务商切换）
/src/frontend/stores/ —— Svelte 状态与动作目录（修改影响状态管理）
/src/frontend/stores/settingsStore.ts —— 设置状态 store，只负责保存 AppSettings 状态（修改影响设置状态）
/src/frontend/stores/loadSettings.ts —— 设置加载动作，只负责读取设置后写入 store（修改影响设置初始化）
/src/frontend/stores/persistSettings.ts —— 设置保存动作，只负责保存设置后写入 store（修改影响设置持久化）
/src/frontend/stores/historyStore.ts —— 历史状态 store，只负责保存历史列表状态（修改影响历史状态）
/src/frontend/stores/loadHistory.ts —— 历史加载动作，只负责读取历史后写入 store（修改影响历史刷新）
/src/frontend/stores/removeHistory.ts —— 历史清空动作，只负责清空历史后重置 store（修改影响历史清空）
/src/frontend/stores/appState.ts —— 页面状态 store，只负责当前页签状态（修改影响导航状态）
/src/frontend/types/ —— TypeScript 类型和选项数据目录（修改影响类型和下拉选项）
/src/frontend/utils/ —— 前端纯工具函数目录（修改影响文本、时间、快捷键判断）
/src/styles/app.css —— 样式入口文件，只负责样式分片导入顺序（修改影响样式加载）
/src/styles/base.css —— 基础样式文件，只负责全局渲染基线（修改影响全局基线）
/src/styles/layout.css —— 布局样式文件，只负责应用结构布局（修改影响布局）
/src/styles/buttons.css —— 按钮样式文件，只负责按钮视觉状态（修改影响按钮 UI）
/src/styles/forms.css —— 表单样式文件，只负责输入控件视觉（修改影响表单 UI）
/src/styles/translation.css —— 翻译页样式文件，只负责翻译页面视觉（修改影响翻译 UI）
/src/styles/settings.css —— 设置页样式文件，只负责设置表单视觉（修改影响设置 UI）
/src/styles/feedback.css —— 反馈文本样式文件，只负责提示文本视觉（修改影响提示 UI）
/src/styles/history.css —— 历史页样式文件，只负责历史列表视觉（修改影响历史 UI）
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
