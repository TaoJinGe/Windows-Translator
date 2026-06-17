# TREE_MAP src_frontend_components

对应真实目录：`/src/frontend/components`

AppLifecycle.svelte —— 应用生命周期组件，只负责生命周期子组件挂载（修改影响启动监听）
AppStartup.svelte —— 应用启动组件，只负责加载设置后显示窗口（修改影响启动流程）
AppOpenSettingsListener.svelte —— 设置打开监听组件，只负责托盘设置事件响应（修改影响设置入口）
AppEscapeHideShortcut.svelte —— Escape 隐藏组件，只负责 Escape 隐藏窗口快捷键（修改影响窗口快捷键）
AppNav.svelte —— 应用导航组件，只负责主导航按钮组（修改影响导航 UI）
ActiveViewPanel.svelte —— 当前页面组件，只负责按状态渲染页面组件（修改影响页面切换）
TranslatePanel.svelte —— 主翻译面板容器，只负责翻译页子组件编排（修改影响核心翻译 UI 与交互）
LanguageSelector.svelte —— 语言选择条组件，只负责源/目标语言选择与交换入口（修改影响语言栏 UI 与交互）
TranslationInput.svelte —— 翻译原文输入框组件，只负责原文输入控件（修改影响原文输入 UI）
TranslationResultBox.svelte —— 翻译结果框组件，只负责译文只读展示（修改影响结果展示 UI）
TranslateActionBar.svelte —— 翻译操作栏组件，只负责操作区按钮布局（修改影响操作区 UI）
TranslateButton.svelte —— 翻译按钮组件，只负责翻译动作入口状态（修改影响翻译按钮 UI）
CopyResultButton.svelte —— 复制结果按钮组件，只负责复制状态展示入口（修改影响复制按钮 UI）
ClearButton.svelte —— 清空按钮组件，只负责清空动作入口（修改影响清空按钮 UI）
SettingsPanel.svelte —— 设置面板容器，只负责设置表单子组件编排（修改影响配置流程）
ProviderSettings.svelte —— 模型设置组件，只负责模型连接表单（修改影响模型配置 UI）
TranslationSettings.svelte —— 翻译默认设置组件，只负责默认互译语言选择（修改影响默认语言配置 UI）
HotkeySettings.svelte —— 快捷键设置组组件，只负责快捷键输入项组合布局（修改影响快捷键配置 UI）
HotkeyInput.svelte —— 单个快捷键输入组件，只负责快捷键录制输入控件（修改影响快捷键输入交互）
WindowSettings.svelte —— 窗口设置组件，只负责窗口偏好表单（修改影响窗口配置 UI）
SaveSettingsButton.svelte —— 保存设置按钮组件，只负责设置提交按钮状态（修改影响保存按钮 UI）
HistoryPanel.svelte —— 历史面板容器，只负责历史列表编排（修改影响历史 UI）
HistoryRefreshButton.svelte —— 历史刷新按钮组件，只负责触发历史重新读取（修改影响刷新按钮交互）
ClearHistoryButton.svelte —— 清空历史按钮组件，只负责触发历史清空（修改影响清空按钮交互）
HistoryItem.svelte —— 历史条目组件，只负责单条历史记录展示布局（修改影响历史条目 UI）
CopyHistoryTextButton.svelte —— 历史文本复制按钮组件，只负责历史文本复制按钮状态（修改影响历史复制按钮交互）
WindowHeader.svelte —— 预留标题栏组件，负责全局窗口头部结构（修改影响全局头部 UI）
IconButton.svelte —— 通用图标按钮组件，负责复用按钮外壳状态（修改影响按钮视觉与交互）
