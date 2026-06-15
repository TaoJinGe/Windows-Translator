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
/src-tauri/target-package/ —— Rust 构建缓存目录，可再生成（AI 禁止修改产物）

## Rust 源码

/src-tauri/src/main.rs —— Windows release 子系统入口和 lib 调用（修改影响启动方式）
/src-tauri/src/lib.rs —— Tauri 应用组装、插件、托盘、快捷键、窗口事件入口（修改影响桌面核心）
/src-tauri/src/commands/ —— 暴露给前端 invoke 的 command 目录（修改影响 API 边界）
/src-tauri/src/config/ —— AppSettings 和默认值目录（修改影响本地配置结构）
/src-tauri/src/storage/ —— 设置和历史 JSON 存储目录（修改影响持久化）
/src-tauri/src/api/ —— OpenAI 兼容翻译请求目录（修改影响模型调用）
/src-tauri/src/hotkey/ —— 全局快捷键注册目录（修改影响快捷键）
/src-tauri/src/tray/ —— 系统托盘菜单目录（修改影响常驻和退出）
/src-tauri/src/window/ —— 窗口显示、隐藏、关闭行为目录（修改影响窗口体验）

## 文档系统

/docs/AI_SYSTEM_RULES.md —— 文件系统索引协议唯一规则文件（修改影响 AI 执行规则）
/docs/ROOT_MAP_1.md —— 根目录和前端索引（修改影响导航入口）
/docs/ROOT_MAP_2.md —— 后端和文档索引（修改影响导航入口）
/docs/PROJECT_OVERVIEW.md —— 项目概览和运行说明（修改影响项目理解）
/docs/ARCHITECTURE.md —— 前后端架构说明（修改影响系统理解）
/docs/FILE_MAP.md —— 原有全量文件职责地图（修改影响旧导航文档）
/docs/DEVELOPMENT_LOG.md —— 阶段开发日志（修改影响变更追踪）
/docs/TODO.md —— 后续任务列表（修改影响计划）
/docs/DECISIONS.md —— 技术决策记录（修改影响决策追溯）
/docs/tree/ —— 子目录 Tree Map 目录（修改影响二级索引）

## Tree Maps

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
