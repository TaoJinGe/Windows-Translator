# TREE_MAP src_tauri_api

对应真实目录：`/src-tauri/src/api`

mod.rs —— api 模块导出入口，只负责开放 API 子模块（修改影响模块引用）
prompt.rs —— 翻译提示词构建模块，只负责生成系统提示词（修改影响模型输出）
openai_client.rs —— OpenAI HTTP 客户端，只负责翻译请求调用编排（修改影响模型通信）
openai_request.rs —— OpenAI 请求构造模块，只负责生成聊天补全请求体（修改影响请求格式）
openai_types.rs —— OpenAI 类型模块，只负责聊天补全请求响应结构（修改影响接口解析）
sse_parser.rs —— SSE 解析模块，只负责解析流式响应事件（修改影响流式解析）
stream_event.rs —— 翻译流事件模块，只负责向前端发送翻译流事件（修改影响流式通知）
thinking_filter.rs —— 思考内容过滤模块，只负责清理模型思考标签内容（修改影响译文清理）
