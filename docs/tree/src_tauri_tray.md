# TREE_MAP src_tauri_tray

对应真实目录：`/src-tauri/src/tray`

mod.rs —— tray 模块导出入口（修改影响模块引用）
app_tray.rs —— 托盘创建入口，只负责组装托盘构建器（修改影响托盘初始化）
tray_menu.rs —— 托盘菜单模块，只负责构建菜单项（修改影响托盘菜单）
tray_menu_event.rs —— 托盘菜单事件模块，只负责菜单事件分发（修改影响菜单交互）
tray_icon_event.rs —— 托盘图标事件模块，只负责图标点击事件分发（修改影响图标交互）
tray_show_action.rs —— 托盘显示动作模块，只负责切换主窗口显示状态（修改影响显示入口）
tray_settings_action.rs —— 托盘设置动作模块，只负责打开设置页事件（修改影响设置入口）
tray_quit_action.rs —— 托盘退出动作模块，只负责退出应用动作（修改影响退出入口）
