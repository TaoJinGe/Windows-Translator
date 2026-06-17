# TREE_MAP src_frontend

对应真实目录：`/src/frontend`

App.svelte —— 前端应用外壳组件，只负责全局布局承载（修改影响全局 UI）
main.ts —— Svelte mount 入口，引入全局样式（修改影响前端启动）
components/ —— UI 组件目录，翻译页/设置页/历史页在此（修改影响界面）
services/ —— 服务封装目录，负责调用 Tauri commands（修改影响前后端通信）
stores/ —— Svelte stores 目录，负责前端状态文件分类（修改影响状态）
types/ —— 类型和选项数据目录，负责模型/语言/设置结构（修改影响类型）
utils/ —— 前端纯函数目录，负责通用工具文件分类（修改影响局部逻辑）
