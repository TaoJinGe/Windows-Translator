# TREE_MAP src_tauri_src

对应真实目录：`/src-tauri/src`

main.rs —— Windows 子系统和二进制入口，调用 library run（修改影响启动）
lib.rs —— Tauri library 入口，只负责转发应用运行（修改影响桌面启动）
app_run.rs —— Tauri 运行入口，只负责执行 Builder 启动链（修改影响应用运行）
app_state.rs —— 应用状态注册模块，只负责注入 HTTP 客户端状态（修改影响后端状态）
app_clipboard_plugin.rs —— 剪贴板插件模块，只负责安装剪贴板插件（修改影响复制插件）
app_global_shortcut_plugin.rs —— 全局快捷键插件模块，只负责安装快捷键插件（修改影响快捷键插件）
app_setup.rs —— 应用 setup 入口，只负责串联启动步骤（修改影响启动流程）
app_setup_autostart_plugin.rs —— 自启动插件 setup 模块，只负责安装自启动插件（修改影响启动项插件）
app_setup_settings.rs —— 启动设置读取模块，只负责读取启动设置（修改影响启动配置）
app_setup_window.rs —— 启动窗口状态模块，只负责应用启动窗口状态（修改影响启动窗口）
app_setup_autostart.rs —— 启动自启动同步模块，只负责同步开机启动状态（修改影响启动项）
app_setup_tray.rs —— 启动托盘模块，只负责创建启动托盘（修改影响托盘启动）
app_setup_hotkey.rs —— 启动快捷键模块，只负责注册启动快捷键（修改影响快捷键启动）
app_window_event.rs —— 窗口事件入口，只负责分发窗口事件（修改影响窗口事件）
app_window_close_event.rs —— 窗口关闭事件模块，只负责处理关闭请求（修改影响关闭行为）
app_window_focus_event.rs —— 窗口焦点事件模块，只负责处理失焦置顶状态（修改影响焦点行为）
autostart.rs —— 开机自启动同步模块，按 AppSettings 启用或关闭系统自启动（修改影响启动项）
commands/ —— Tauri command 模块目录，暴露给前端 invoke（修改影响 API 边界）
config/ —— 配置结构和默认值目录，负责 AppSettings（修改影响设置）
storage/ —— 本地 JSON 存储目录，负责持久化文件分类（修改影响持久化）
api/ —— 翻译 API 调用目录，负责模型通信文件分类（修改影响翻译）
hotkey/ —— 全局快捷键目录，负责热键模块分类（修改影响快捷键）
tray/ —— 系统托盘目录，负责托盘模块分类（修改影响后台常驻）
window/ —— 窗口控制目录，负责窗口行为模块分类（修改影响窗口体验）
