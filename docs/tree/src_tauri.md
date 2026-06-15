# TREE_MAP src_tauri

对应真实目录：`/src-tauri`

Cargo.toml —— Rust 依赖和 crate 配置，负责 Tauri 后端编译（修改影响构建）
Cargo.lock —— Rust 锁定文件，负责依赖版本复现（通常由 cargo 更新）
build.rs —— Tauri 构建脚本入口，负责资源和配置处理（修改影响打包）
tauri.conf.json —— Tauri 应用配置，负责窗口、打包、图标（修改影响桌面行为）
capabilities/ —— Tauri 权限配置目录，负责前端可调用能力（修改影响权限）
icons/ —— 图标资源目录，负责 EXE/MSI 图标（修改影响品牌显示）
gen/ —— Tauri schema 生成目录，自动生成（AI 禁止手改）
src/ —— Rust 源码目录，负责命令、存储、API、系统能力（修改影响后端）
target-package/ —— Rust 构建缓存目录，可再生成（AI 禁止改产物）
