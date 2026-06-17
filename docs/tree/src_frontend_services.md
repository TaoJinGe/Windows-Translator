# TREE_MAP src_frontend_services

对应真实目录：`/src/frontend/services`

tauriRuntime.ts —— Tauri 运行环境判断服务，只负责识别桌面运行时（修改影响环境分支）
getSettingsApi.ts —— 设置读取 API 服务，只负责读取设置 invoke（修改影响设置加载）
saveSettingsApi.ts —— 设置保存 API 服务，只负责保存设置 invoke（修改影响设置保存）
getHistoryApi.ts —— 历史读取 API 服务，只负责读取历史 invoke（修改影响历史加载）
clearHistoryApi.ts —— 历史清空 API 服务，只负责清空历史 invoke（修改影响历史清空）
translationApi.ts —— 翻译 API 服务，只负责翻译 invoke 调用（修改影响翻译通信）
translationStreamApi.ts —— 翻译流事件服务，只负责监听翻译流式事件（修改影响流式输出通信）
translationRequestId.ts —— 翻译请求 ID 服务，只负责生成翻译请求唯一标识（修改影响翻译事件匹配）
translationApiKey.ts —— 翻译密钥校验服务，只负责检查翻译 API Key 可用性（修改影响翻译前置校验）
clipboardApi.ts —— 剪贴板 API 服务，只负责桌面剪贴板 invoke（修改影响桌面复制）
hideMainWindowApi.ts —— 隐藏窗口 API 服务，只负责隐藏主窗口 invoke（修改影响窗口隐藏）
showMainWindowApi.ts —— 显示窗口 API 服务，只负责显示主窗口 invoke（修改影响窗口显示）
openSettingsEventApi.ts —— 设置打开事件服务，只负责监听设置页打开事件（修改影响设置入口通信）
translator.ts —— 翻译服务模块，只负责组织单次翻译请求流程（修改影响翻译流程）
clipboard.ts —— 剪贴板服务模块，负责统一文本复制入口（修改影响复制行为）
languageSettings.ts —— 翻译语言保存服务，只负责持久化翻译语言对（修改影响语言设置保存）
translationRunner.ts —— 翻译执行服务，只负责聚合流式翻译结果文本（修改影响翻译结果生成）
settingsHotkeyRecorder.ts —— 设置快捷键录制服务，只负责把键盘事件转换为设置热键值（修改影响快捷键录制）
findSelectedProvider.ts —— 选中服务商服务，只负责按 API 地址查找服务商（修改影响服务商显示）
applyProviderDefaults.ts —— 服务商默认值服务，只负责套用服务商默认模型配置（修改影响服务商切换）
