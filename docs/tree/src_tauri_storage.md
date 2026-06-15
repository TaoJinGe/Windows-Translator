# TREE_MAP src_tauri_storage

对应真实目录：`/src-tauri/src/storage`

mod.rs —— storage 模块导出入口（修改影响模块引用）
paths.rs —— 应用配置目录和 JSON 路径解析（修改影响文件位置）
settings_store.rs —— 设置 JSON 读写，负责 AppSettings 持久化（修改影响设置保存）
history_store.rs —— 历史 JSON 读写，负责最近 50 条记录（修改影响历史存储）
