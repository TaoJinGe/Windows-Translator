# TREE_MAP src_tauri_src

对应真实目录：`/src-tauri/src`

main.rs —— Windows 子系统和二进制入口，调用 library run（修改影响启动）
lib.rs —— Tauri 应用组装入口，注册插件、托盘、快捷键、窗口事件（修改影响桌面核心）
autostart.rs —— 开机自启动同步模块，按 AppSettings 启用或关闭系统自启动（修改影响启动项）
commands/ —— Tauri command 模块目录，暴露给前端 invoke（修改影响 API 边界）
config/ —— 配置结构和默认值目录，负责 AppSettings（修改影响设置）
storage/ —— 本地 JSON 存储目录，负责设置和历史读写（修改影响持久化）
api/ —— 翻译 API 调用目录，负责 OpenAI 兼容请求和 prompt（修改影响翻译）
hotkey/ —— 全局快捷键目录，负责注册和更新热键（修改影响快捷键）
tray/ —— 系统托盘目录，负责菜单和退出（修改影响后台常驻）
window/ —— 窗口控制目录，负责显示、隐藏、关闭行为（修改影响窗口体验）
