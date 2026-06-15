# TREE_MAP root

对应真实目录：`/`

package.json —— npm 脚本和依赖入口，负责开发/打包命令（修改影响构建）
package-lock.json —— npm 锁定文件，负责依赖版本复现（通常不手改）
vite.config.ts —— Vite 配置，负责 Svelte 前端构建（修改影响 dev/build）
tsconfig.json —— TypeScript 配置，负责严格类型规则（修改影响前端检查）
index.html —— HTML 入口，负责挂载前端应用（修改影响页面加载）
README.md —— 用户说明文档，负责运行和打包说明（修改影响文档）
.gitignore —— 忽略规则，负责排除依赖和产物（修改影响版本控制）
Windows_Translator_SPEC.md —— 产品规格，负责需求来源（修改影响需求基线）
src/ —— 前端源码和样式顶层目录（修改影响 UI）
src-tauri/ —— Rust/Tauri 桌面端目录（修改影响系统能力）
docs/ —— 项目文档和索引目录（修改影响 AI 导航）
scripts/ —— 本地构建脚本目录（修改影响发布流程）
public/ —— 公共资源目录（修改影响静态资源）
release/ —— 最终产物目录，可再生成（AI 禁止改产物）
dist/ —— Vite 构建输出，可再生成（AI 禁止改产物）
node_modules/ —— npm 依赖目录，可再安装（AI 禁止改依赖）
.playwright-mcp/ —— 测试日志目录，可删除（AI 禁止当源码改）
