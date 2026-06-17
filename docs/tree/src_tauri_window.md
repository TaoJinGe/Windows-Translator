# TREE_MAP src_tauri_window

对应真实目录：`/src-tauri/src/window`

mod.rs —— window 模块导出入口（修改影响模块引用）
window_control.rs —— 窗口显示、隐藏、切换、关闭按钮行为处理（修改影响窗口体验）
window_position_memory.rs —— 窗口位置记忆模块，只负责记录和恢复主窗口坐标（修改影响启动位置）
