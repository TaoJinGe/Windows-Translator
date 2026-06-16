# TREE_MAP src_frontend_components

对应真实目录：`/src/frontend/components`

TranslatePanel.svelte —— 主翻译面板，负责语言选择、互换、翻译、流式结果、复制和清空入口（修改影响核心翻译 UI 与交互）
LanguageSelector.svelte —— 语言选择条组件，只负责源/目标语言选择与交换入口（修改影响语言栏 UI 与交互）
TranslateButton.svelte —— 翻译按钮组件，只负责翻译动作入口和加载态（修改影响翻译按钮 UI）
CopyResultButton.svelte —— 复制结果按钮组件，只负责复制状态展示入口（修改影响复制按钮 UI）
ClearButton.svelte —— 清空按钮组件，只负责清空动作入口（修改影响清空按钮 UI）
SettingsPanel.svelte —— 设置面板容器，负责模型、默认语言、快捷键和窗口设置的保存编排（修改影响配置流程）
ProviderSettings.svelte —— 模型设置组件，只负责服务商、模型、API 地址和密钥输入（修改影响模型配置 UI）
TranslationSettings.svelte —— 翻译默认设置组件，只负责默认互译语言选择（修改影响默认语言配置 UI）
HotkeySettings.svelte —— 快捷键设置组组件，只负责快捷键输入项组合布局（修改影响快捷键配置 UI）
HotkeyInput.svelte —— 单个快捷键输入组件，只负责录制态展示和键盘事件入口（修改影响快捷键输入交互）
WindowSettings.svelte —— 窗口设置组件，只负责关闭行为、置顶、流式输出选项（修改影响窗口配置 UI）
SaveSettingsButton.svelte —— 保存设置按钮组件，只负责提交入口和保存状态展示（修改影响保存按钮 UI）
HistoryPanel.svelte —— 历史面板，负责读取、刷新、清空历史记录（修改影响历史 UI 与数据展示）
WindowHeader.svelte —— 预留标题栏组件，负责全局窗口头部结构（修改影响全局头部 UI）
IconButton.svelte —— 通用图标按钮组件，负责可复用按钮外壳和禁用状态（修改影响按钮视觉与交互）
