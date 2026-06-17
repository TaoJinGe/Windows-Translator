# TREE_MAP src_tauri_config

对应真实目录：`/src-tauri/src/config`

mod.rs —— config 模块导出入口（修改影响模块引用）
default_settings_version.rs —— 设置版本默认值模块，只负责配置版本常量（修改影响配置迁移）
default_api.rs —— API 默认值模块，只负责默认 API 地址常量（修改影响初始 API）
default_model.rs —— 模型默认值模块，只负责默认模型常量（修改影响初始模型）
default_languages.rs —— 语言默认值模块，只负责默认语言常量（修改影响初始语言）
default_hotkeys.rs —— 快捷键默认值模块，只负责默认快捷键常量（修改影响初始快捷键）
default_window.rs —— 窗口默认值模块，只负责窗口偏好默认常量（修改影响初始窗口行为）
app_config.rs —— AppSettings Rust 结构模块，只负责配置字段结构（修改影响配置文件兼容）
app_settings_default.rs —— AppSettings 默认值模块，只负责构建默认设置（修改影响初始配置）
app_settings_normalize.rs —— AppSettings 归一化模块，只负责迁移设置值（修改影响配置迁移）
app_settings_language_pair.rs —— AppSettings 语言对模块，只负责解析默认语言对（修改影响语言对读取）
language_pair.rs —— 语言对校验模块，只负责语言对有效性判断（修改影响语言对校验）
