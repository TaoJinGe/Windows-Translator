# TREE_MAP src_tauri_api

对应真实目录：`/src-tauri/src/api`

mod.rs —— api 模块导出入口，负责开放 prompt 与 openai_client 模块（修改影响模块引用）
prompt.rs —— 翻译提示词构建模块，负责语言规则和输出约束（修改影响模型输出）
openai_client.rs —— OpenAI 兼容 HTTP 客户端，负责非流式/流式翻译、SSE 解析、错误处理和思考内容过滤（修改影响翻译 API 行为）
