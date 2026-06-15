# TREE_MAP src_tauri_commands

对应真实目录：`/src-tauri/src/commands`

mod.rs —— command 汇总入口，负责 generate_handler 列表（修改影响前端可调用命令）
settings.rs —— 设置 command，负责读取/保存设置和更新快捷键/置顶（修改影响设置流程）
translate.rs —— 翻译 command，负责 API Key 检查、翻译调用、写历史（修改影响翻译流程）
history.rs —— 历史 command 和数据结构，负责读取/清空历史（修改影响历史功能）
window.rs —— 窗口和剪贴板 command，负责复制、显示、隐藏、切换、快捷键更新（修改影响系统能力）
