# TREE_MAP src_tauri_commands

对应真实目录：`/src-tauri/src/commands`

mod.rs —— command 汇总入口，负责 generate_handler 列表（修改影响前端可调用命令）
settings.rs —— 设置 command，只负责设置读取保存入口（修改影响设置 API）
settings_autostart.rs —— 设置自启动同步模块，只负责开机启动状态同步（修改影响启动项）
settings_hotkey.rs —— 设置快捷键同步模块，只负责保存设置时替换全局快捷键（修改影响快捷键）
settings_window.rs —— 设置窗口同步模块，只负责保存设置时应用置顶状态（修改影响窗口置顶）
translate.rs —— 翻译 command，只负责翻译命令入口编排（修改影响翻译 API）
translation_api_key.rs —— 翻译密钥校验模块，只负责后端 API Key 检查（修改影响翻译前置校验）
translation_history_item.rs —— 翻译历史条目模块，只负责构建历史记录数据（修改影响历史记录内容）
translation_history_append.rs —— 翻译历史写入模块，只负责异步追加历史记录（修改影响历史写入）
history.rs —— 历史 command，只负责历史读取清空入口（修改影响历史功能）
clipboard.rs —— 剪贴板 command，只负责桌面剪贴板写入（修改影响复制能力）
hotkey.rs —— 快捷键 command，只负责全局快捷键更新入口（修改影响快捷键能力）
window.rs —— 窗口 command，只负责主窗口显示状态入口（修改影响窗口能力）
