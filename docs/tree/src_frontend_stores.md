# TREE_MAP src_frontend_stores

对应真实目录：`/src/frontend/stores`

appState.ts —— 页面状态 store，负责 translate/history/settings tab（修改影响导航）
settingsStore.ts —— 设置 store，负责加载和保存 AppSettings（修改影响配置持久化）
historyStore.ts —— 历史 store，负责读取和清空历史记录（修改影响历史页）
