# TREE_MAP src_frontend_services

对应真实目录：`/src/frontend/services`

tauriApi.ts —— Tauri invoke 封装，负责 settings/history/window/translation command 调用（修改影响前后端通信）
translator.ts —— 翻译业务服务，负责 API Key 检查和翻译请求组装（修改影响翻译流程）
clipboard.ts —— 剪贴板服务，负责桌面和浏览器预览复制逻辑（修改影响复制功能）
