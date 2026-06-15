# TREE_MAP src_tauri_api

对应真实目录：`/src-tauri/src/api`

mod.rs —— api 模块导出入口（修改影响模块引用）
prompt.rs —— 翻译系统提示词，负责目标语言和自动检测规则（修改影响模型输出）
openai_client.rs —— OpenAI 兼容 HTTP 客户端，负责请求、错误、译文解析和 think 过滤（修改影响翻译 API）
