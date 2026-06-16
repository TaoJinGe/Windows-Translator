# TREE_MAP src_frontend_services

对应真实目录：`/src/frontend/services`

tauriApi.ts —— Tauri API 封装，负责设置、历史、窗口、翻译 invoke 和事件监听（修改影响前后端通信）
translator.ts —— 翻译服务模块，负责 API Key 检查、请求组装、请求 ID 和流式回调（修改影响翻译流程）
clipboard.ts —— 剪贴板服务模块，负责桌面端和浏览器预览复制兼容（修改影响复制行为）
