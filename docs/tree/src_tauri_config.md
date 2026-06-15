# TREE_MAP src_tauri_config

对应真实目录：`/src-tauri/src/config`

mod.rs —— config 模块导出入口（修改影响模块引用）
defaults.rs —— 默认配置常量，负责 API 地址、模型、语言、快捷键默认值（修改影响初始设置）
app_config.rs —— AppSettings Rust 结构和默认/迁移逻辑（修改影响配置文件兼容）
