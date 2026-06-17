# TREE_MAP src_frontend_stores

对应真实目录：`/src/frontend/stores`

appState.ts —— 页面状态 store，只负责当前页签状态（修改影响导航）
settingsStore.ts —— 设置状态 store，只负责保存 AppSettings 状态（修改影响设置状态）
loadSettings.ts —— 设置加载动作，只负责读取设置后写入 store（修改影响设置初始化）
persistSettings.ts —— 设置保存动作，只负责保存设置后写入 store（修改影响设置持久化）
historyStore.ts —— 历史状态 store，只负责保存历史列表状态（修改影响历史状态）
loadHistory.ts —— 历史加载动作，只负责读取历史后写入 store（修改影响历史刷新）
removeHistory.ts —— 历史清空动作，只负责清空历史后重置 store（修改影响历史清空）
