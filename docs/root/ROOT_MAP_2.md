# ROOT_MAP_2

覆盖范围：Rust/Tauri 后端源码、Tauri 配置、项目文档。不要覆盖 `src/frontend/`。

## Tauri 根目录

/src-tauri/Cargo.toml —— Rust crate 和 Tauri 插件依赖配置（修改影响后端编译）
/src-tauri/Cargo.lock —— Rust 依赖锁定文件（修改影响构建复现）
/src-tauri/build.rs —— Tauri 构建脚本入口（修改影响资源生成）
/src-tauri/tauri.conf.json —— Tauri 窗口、打包和图标配置（修改影响桌面应用行为）
/src-tauri/capabilities/ —— Tauri v2 权限能力配置（修改影响前端调用权限）
/src-tauri/icons/ —— Tauri 图标资源目录（修改影响 EXE/MSI 图标）
/src-tauri/gen/ —— Tauri 自动生成 schema 目录（AI 禁止手动修改）
/src-tauri/target/ —— Rust 构建缓存目录，可再生成（AI 禁止修改产物）
/src-tauri/target-package/ —— Tauri 发布构建缓存目录，可再生成（AI 禁止修改产物）

## Rust 源码

/src-tauri/src/main.rs —— Windows release 子系统入口和 lib 调用（修改影响启动方式）
/src-tauri/src/lib.rs —— Tauri library 入口，只负责转发应用运行（修改影响桌面启动）
/src-tauri/src/app_run.rs —— Tauri 运行入口，只负责执行 Builder 启动链（修改影响应用运行）
/src-tauri/src/app_state.rs —— 应用状态注册模块，只负责注入 HTTP 客户端状态（修改影响后端状态）
/src-tauri/src/app_clipboard_plugin.rs —— 剪贴板插件模块，只负责安装剪贴板插件（修改影响复制插件）
/src-tauri/src/app_global_shortcut_plugin.rs —— 全局快捷键插件模块，只负责安装快捷键插件（修改影响快捷键插件）
/src-tauri/src/app_setup.rs —— 应用 setup 入口，只负责串联启动步骤（修改影响启动流程）
/src-tauri/src/app_setup_autostart_plugin.rs —— 自启动插件 setup 模块，只负责安装自启动插件（修改影响启动项插件）
/src-tauri/src/app_setup_settings.rs —— 启动设置读取模块，只负责读取启动设置（修改影响启动配置）
/src-tauri/src/app_setup_window.rs —— 启动窗口状态模块，只负责应用启动窗口状态（修改影响启动窗口）
/src-tauri/src/app_setup_autostart.rs —— 启动自启动同步模块，只负责同步开机启动状态（修改影响启动项）
/src-tauri/src/app_setup_tray.rs —— 启动托盘模块，只负责创建启动托盘（修改影响托盘启动）
/src-tauri/src/app_setup_hotkey.rs —— 启动快捷键模块，只负责注册启动快捷键（修改影响快捷键启动）
/src-tauri/src/app_window_event.rs —— 窗口事件入口，只负责分发窗口事件（修改影响窗口事件）
/src-tauri/src/app_window_close_event.rs —— 窗口关闭事件模块，只负责处理关闭请求（修改影响关闭行为）
/src-tauri/src/app_window_focus_event.rs —— 窗口焦点事件模块，只负责处理失焦置顶状态（修改影响焦点行为）
/src-tauri/src/autostart.rs —— 开机自启动同步模块，负责按设置启用或关闭系统自启动（修改影响启动项）
/src-tauri/src/commands/ —— 暴露给前端 invoke 的 command 目录（修改影响 API 边界）
/src-tauri/src/commands/clipboard.rs —— 剪贴板 command，只负责桌面剪贴板写入（修改影响复制能力）
/src-tauri/src/commands/hotkey.rs —— 快捷键 command，只负责全局快捷键更新入口（修改影响快捷键能力）
/src-tauri/src/commands/settings.rs —— 设置 command，只负责设置读取保存入口（修改影响设置 API）
/src-tauri/src/commands/settings_autostart.rs —— 设置自启动同步模块，只负责开机启动状态同步（修改影响启动项）
/src-tauri/src/commands/settings_hotkey.rs —— 设置快捷键同步模块，只负责保存设置时替换全局快捷键（修改影响快捷键）
/src-tauri/src/commands/settings_window.rs —— 设置窗口同步模块，只负责保存设置时应用置顶状态（修改影响窗口置顶）
/src-tauri/src/commands/translate.rs —— 翻译 command，只负责翻译命令入口编排（修改影响翻译 API）
/src-tauri/src/commands/translation_api_key.rs —— 翻译密钥校验模块，只负责后端 API Key 检查（修改影响翻译前置校验）
/src-tauri/src/commands/translation_history_item.rs —— 翻译历史条目模块，只负责构建历史记录数据（修改影响历史记录内容）
/src-tauri/src/commands/translation_history_append.rs —— 翻译历史写入模块，只负责异步追加历史记录（修改影响历史写入）
/src-tauri/src/commands/window.rs —— 窗口 command，只负责主窗口显示状态入口（修改影响窗口能力）
/src-tauri/src/config/ —— AppSettings 和默认值目录（修改影响本地配置结构）
/src-tauri/src/config/default_settings_version.rs —— 设置版本默认值模块，只负责配置版本常量（修改影响配置迁移）
/src-tauri/src/config/default_api.rs —— API 默认值模块，只负责默认 API 地址常量（修改影响初始 API）
/src-tauri/src/config/default_model.rs —— 模型默认值模块，只负责默认模型常量（修改影响初始模型）
/src-tauri/src/config/default_languages.rs —— 语言默认值模块，只负责默认语言常量（修改影响初始语言）
/src-tauri/src/config/default_hotkeys.rs —— 快捷键默认值模块，只负责默认快捷键常量（修改影响初始快捷键）
/src-tauri/src/config/default_window.rs —— 窗口默认值模块，只负责窗口偏好默认常量（修改影响初始窗口行为）
/src-tauri/src/storage/ —— 设置和历史 JSON 存储目录（修改影响持久化）
/src-tauri/src/api/ —— OpenAI 兼容翻译请求目录（修改影响模型调用）
/src-tauri/src/api/openai_client.rs —— OpenAI HTTP 客户端，只负责翻译请求调用编排（修改影响模型通信）
/src-tauri/src/api/openai_request.rs —— OpenAI 请求构造模块，只负责生成聊天补全请求体（修改影响请求格式）
/src-tauri/src/api/openai_types.rs —— OpenAI 类型模块，只负责聊天补全请求响应结构（修改影响接口解析）
/src-tauri/src/api/sse_parser.rs —— SSE 解析模块，只负责解析流式响应事件（修改影响流式解析）
/src-tauri/src/api/stream_event.rs —— 翻译流事件模块，只负责向前端发送翻译流事件（修改影响流式通知）
/src-tauri/src/api/thinking_filter.rs —— 思考内容过滤模块，只负责清理模型思考标签内容（修改影响译文清理）
/src-tauri/src/hotkey/ —— 全局快捷键注册目录（修改影响快捷键）
/src-tauri/src/tray/ —— 系统托盘菜单目录（修改影响常驻和退出）
/src-tauri/src/tray/app_tray.rs —— 托盘创建入口，只负责组装托盘构建器（修改影响托盘初始化）
/src-tauri/src/tray/tray_menu.rs —— 托盘菜单模块，只负责构建菜单项（修改影响托盘菜单）
/src-tauri/src/tray/tray_menu_event.rs —— 托盘菜单事件模块，只负责菜单事件分发（修改影响菜单交互）
/src-tauri/src/tray/tray_icon_event.rs —— 托盘图标事件模块，只负责图标点击事件分发（修改影响图标交互）
/src-tauri/src/tray/tray_show_action.rs —— 托盘显示动作模块，只负责切换主窗口显示状态（修改影响显示入口）
/src-tauri/src/tray/tray_settings_action.rs —— 托盘设置动作模块，只负责打开设置页事件（修改影响设置入口）
/src-tauri/src/tray/tray_quit_action.rs —— 托盘退出动作模块，只负责退出应用动作（修改影响退出入口）
/src-tauri/src/window/ —— 窗口显示、隐藏、关闭行为目录（修改影响窗口体验）

## 文档系统

/docs/AI_SYSTEM_RULES.md —— 文件系统索引协议唯一规则文件（修改影响 AI 执行规则）
/docs/root/ —— 总目录 Root Map 目录，集中管理一级文件系统索引（修改影响 AI 导航入口）
/docs/root/ROOT_MAP_1.md —— 根目录、前端源码、脚本和公共资源索引（修改影响前端导航入口）
/docs/root/ROOT_MAP_2.md —— Rust/Tauri 后端源码、Tauri 配置和文档索引（修改影响后端与文档导航入口）
/docs/tree/ —— 子目录 Tree Map 目录（修改影响二级索引）

## Tree Maps

/docs/tree/docs_tree.md —— docs/tree 子目录索引文件目录索引
/docs/tree/root_dir.md —— docs/root 总目录文件索引
/docs/tree/src_tauri.md —— src-tauri 顶层结构索引
/docs/tree/src_tauri_src.md —— Rust 源码顶层索引
/docs/tree/src_tauri_commands.md —— Tauri commands 索引
/docs/tree/src_tauri_config.md —— Rust 配置模块索引
/docs/tree/src_tauri_storage.md —— Rust 存储模块索引
/docs/tree/src_tauri_api.md —— Rust API 模块索引
/docs/tree/src_tauri_hotkey.md —— Rust 快捷键模块索引
/docs/tree/src_tauri_tray.md —— Rust 托盘模块索引
/docs/tree/src_tauri_window.md —— Rust 窗口模块索引
/docs/tree/docs.md —— docs 文档目录索引
