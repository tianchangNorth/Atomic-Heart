# Atomic Heart - 现代化 Git 仓库管理应用

[![Tauri](https://img.shields.io/badge/Tauri-v2-blue)](https://tauri.app/)
[![Vue](https://img.shields.io/badge/Vue-v3.5-green)](https://vuejs.org/)
[![TypeScript](https://img.shields.io/badge/TypeScript-v5.6-blue)](https://www.typescriptlang.org/)
[![Tailwind CSS](https://img.shields.io/badge/Tailwind-v4.1-blueviolet)](https://tailwindcss.com/)
[![Rust](https://img.shields.io/badge/Rust-1.77+-orange)](https://www.rust-lang.org/)

## 项目介绍

Atomic Heart（原子之心）是一款基于 Tauri 和 Vue 3 开发的现代化 Git 仓库管理桌面应用。它为开发者提供了强大而直观的本地 Git 仓库管理体验，支持多平台 Git 服务集成，包括 GitHub、AtomGit、GitLab、Gitee 等。

## 核心功能

### 🚀 Git 仓库管理
- **本地仓库管理**：导入、浏览和管理本地 Git 仓库
- **仓库克隆**：支持 HTTPS 和 SSH 协议克隆远程仓库
- **智能认证**：双协议认证系统，自动识别并使用合适的认证方式
- **多平台支持**：GitHub、AtomGit、GitLab、Gitee 等主流 Git 平台

### 📝 代码管理
- **文件状态查看**：实时显示工作区和暂存区文件状态
- **差异查看**：高亮显示文件变更内容
- **提交管理**：创建提交、查看提交历史
- **暂存操作**：灵活的文件暂存和取消暂存

### 🌿 分支管理
- **分支列表**：查看本地和远程分支
- **分支操作**：创建、切换、删除分支
- **远程分支**：检出远程分支到本地

### 🔄 同步操作
- **智能同步**：fetch、pull、push 操作
- **冲突处理**：友好的合并冲突提示
- **进度跟踪**：实时显示同步进度和状态

### 🔐 安全认证
- **OAuth 2.0**：安全的第三方平台授权登录
- **Token 管理**：Personal Access Token 存储和管理
- **SSH 密钥**：SSH 密钥检测和验证
- **深度链接**：支持浏览器回调和应用唤起

### 🎨 用户体验
- **现代化界面**：基于 shadcn-vue 的精美 UI 组件
- **暗黑模式**：支持亮色/暗色主题切换
- **响应式设计**：适配不同屏幕尺寸
- **单实例模式**：避免重复启动应用实例

## 技术栈

### 前端技术
- **框架**：Vue 3 + TypeScript + 组合式 API
- **构建工具**：Vite 6
- **状态管理**：Pinia
- **路由管理**：Vue Router 4
- **UI 框架**：Tailwind CSS + shadcn-vue
- **包管理器**：pnpm

### 后端技术
- **桌面框架**：Tauri 2
- **编程语言**：Rust 1.77+
- **Git 操作**：git2 + 系统 Git 命令
- **HTTP 客户端**：reqwest
- **数据存储**：Tauri Store API
- **认证系统**：OAuth 2.0 + Personal Access Token

### 核心插件
- **tauri-plugin-deep-link**：深度链接支持
- **tauri-plugin-single-instance**：单实例模式
- **tauri-plugin-store**：数据持久化
- **tauri-plugin-dialog**：文件对话框
- **tauri-plugin-opener**：系统集成

## 开发环境要求

### 基础环境
- **Node.js**：18+ (推荐使用 LTS 版本)
- **Rust**：1.77+ (支持最新的 Tauri 2 特性)
- **pnpm**：8+ (推荐的包管理器)
- **Git**：2.30+ (用于 Git 操作)

### 系统依赖
根据 Tauri 要求安装对应平台的系统依赖：

#### Windows
- Microsoft Visual Studio C++ Build Tools
- WebView2 Runtime

#### macOS
- Xcode Command Line Tools

#### Linux (Ubuntu/Debian)
```bash
sudo apt update
sudo apt install libwebkit2gtk-4.0-dev \
  build-essential \
  curl \
  wget \
  file \
  libssl-dev \
  libgtk-3-dev \
  libayatana-appindicator3-dev \
  librsvg2-dev
```

## 快速开始

### 1. 克隆仓库

```bash
git clone https://github.com/yourusername/atomic-heart.git
cd atomic-heart
```

### 2. 安装依赖

```bash
# 安装前端依赖
pnpm install

# 安装 Rust 依赖（自动执行）
cd src-tauri
cargo fetch
cd ..
```

### 3. 配置环境变量

复制 `.env.example` 文件并重命名为 `.env`，然后填写必要的配置：

```env
VITE_APP_BASE_API = 'xxxxxx'
VITE_APP_CLIENT_ID = 'xxxxxx'
VITE_APP_CLIENT_SECRET = 'xxxxxxxx'
```

### 4. 开发模式运行

```bash
# 启动开发服务器
pnpm run tauri:dev

### 5. 构建生产版本

```bash
# 构建所有平台
pnpm run tauri:build
```

## 项目结构

```
atomic-heart/
├── src/                          # 前端源代码
│   ├── assets/                   # 静态资源
│   │   ├── icons/               # 图标文件
│   │   └── images/              # 图片资源
│   ├── components/              # Vue 组件
│   │   ├── ui/                  # shadcn-vue UI 组件
│   │   ├── git/                 # Git 相关组件
│   │   │   ├── CommitManager.vue    # 提交管理
│   │   │   ├── SyncManager.vue      # 同步管理
│   │   │   ├── BranchManager.vue    # 分支管理
│   │   │   └── RepoClone.vue        # 仓库克隆
│   │   └── common/              # 通用组件
│   ├── composables/             # 组合式函数
│   │   ├── useLocalRepositories.ts # 本地仓库管理
│   │   ├── useGitOperations.ts     # Git 操作
│   │   └── useAuth.ts              # 认证管理
│   ├── passport/                # 认证相关组件
│   ├── router/                  # 路由配置
│   ├── services/                # 服务层
│   │   ├── api/                 # API 服务
│   │   ├── deepLinkService.ts   # 深度链接服务
│   │   └── authService.ts       # 认证服务
│   ├── stores/                  # Pinia 状态管理
│   ├── utils/                   # 工具函数
│   │   ├── token/               # Token 管理
│   │   └── git/                 # Git 工具
│   ├── view/                    # 页面视图
│   │   ├── Home.vue             # 首页
│   │   ├── Repositories.vue     # 仓库列表
│   │   ├── LocalRepositories.vue # 本地仓库
│   │   └── RepositoryDetail.vue  # 仓库详情
│   ├── App.vue                  # 根组件
│   └── main.ts                  # 入口文件
├── src-tauri/                   # Tauri/Rust 后端代码
│   ├── src/                     # Rust 源代码
│   │   ├── commands/            # Tauri 命令
│   │   │   ├── git.rs          # Git 操作命令
│   │   │   └── mod.rs          # 模块导出
│   │   ├── git/                 # Git 核心功能
│   │   │   ├── auth.rs         # 认证管理
│   │   │   ├── operations.rs   # Git 操作
│   │   │   ├── types.rs        # 类型定义
│   │   │   └── mod.rs          # 模块导出
│   │   ├── utils/               # 工具模块
│   │   │   ├── system_command.rs # 系统命令
│   │   │   └── mod.rs          # 模块导出
│   │   ├── http_client.rs       # HTTP 客户端
│   │   ├── lib.rs              # 库入口
│   │   └── main.rs             # 主程序入口
│   ├── capabilities/            # Tauri 权限配置
│   ├── icons/                   # 应用图标
│   ├── Cargo.toml              # Rust 依赖配置
│   └── tauri.conf.json         # Tauri 配置
├── public/                      # 公共资源
├── docs/                        # 项目文档
├── .env.example                 # 环境变量示例
├── package.json                 # Node.js 依赖配置
├── tailwind.config.js           # Tailwind CSS 配置
├── vite.config.ts              # Vite 配置
└── README.md                   # 项目说明
```

## 开发工具推荐

### IDE 配置
- **VS Code** + 推荐插件：
  - [Volar](https://marketplace.visualstudio.com/items?itemName=Vue.volar) - Vue 3 支持
  - [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) - Tauri 开发支持
  - [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) - Rust 语言支持
  - [Tailwind CSS IntelliSense](https://marketplace.visualstudio.com/items?itemName=bradlc.vscode-tailwindcss) - Tailwind 智能提示

### 调试工具
- **Tauri DevTools**：内置的开发者工具
- **Vue DevTools**：Vue 组件调试
- **Rust 调试**：通过 rust-analyzer 进行断点调试

## 使用指南

### 首次使用
1. **启动应用**：运行 `pnpm run tauri:dev` 启动开发版本
2. **登录认证**：点击登录按钮，选择 AtomGit 平台进行 OAuth 认证
3. **导入仓库**：通过"本地仓库"页面导入现有的 Git 仓库
4. **克隆仓库**：通过"仓库"页面克隆远程仓库到本地

### 常用操作
- **查看文件状态**：在仓库详情页查看工作区和暂存区状态
- **提交代码**：暂存文件后创建提交
- **同步代码**：使用 fetch/pull/push 与远程仓库同步
- **分支管理**：创建、切换、删除分支

### 深度链接
应用支持深度链接功能，可以通过 URL 直接打开特定功能：
- `atomic-heart://auth/callback` - OAuth 认证回调
- `atomic-heart://repository/open?url=<repo_url>` - 打开仓库
- `atomic-heart://clone?url=<repo_url>` - 克隆仓库

## 故障排除

### 常见问题

#### 1. 编译错误
```bash
# 清理缓存并重新安装
rm -rf node_modules
rm -rf src-tauri/target
pnpm install
```

#### 2. Git 操作失败
- 检查 Git 是否正确安装(某些情况下可能需要系统git的参与)
- 验证 SSH 密钥配置
- 确认网络连接正常

#### 3. 认证问题
- 检查 OAuth 客户端 ID 配置
- 确认回调 URL 设置正确
- 验证 Personal Access Token 有效性

#### 4. 深度链接不工作
- 确保应用已正确安装（生产版本）
- 检查系统是否注册了 `atomic-heart://` 协议
- 在开发模式下使用 `register_all()` 函数

# 查看前端控制台
# 在应用中按 F12 打开开发者工具

## 贡献指南

我们欢迎所有形式的贡献！

### 开发流程
1. **Fork** 本仓库到你的 GitHub 账户
2. **Clone** 你的 fork 到本地
3. **创建分支** `git checkout -b feature/your-feature-name`
4. **开发功能** 并确保代码质量
5. **运行测试** `pnpm test` 和 `pnpm lint`
6. **提交代码** `git commit -m "feat: add your feature"`
7. **推送分支** `git push origin feature/your-feature-name`
8. **创建 PR** 在 GitHub 上创建 Pull Request

### 代码规范
- **前端**：遵循 Vue 3 + TypeScript 最佳实践
- **后端**：遵循 Rust 编码规范
- **提交信息**：使用 [Conventional Commits](https://www.conventionalcommits.org/) 格式
- **代码格式化**：使用 Prettier (前端) 和 rustfmt (后端)

### 问题反馈
- **Bug 报告**：使用 GitHub Issues，提供详细的复现步骤
- **功能请求**：描述需求场景和期望的解决方案
- **文档改进**：帮助完善文档和示例

## 路线图

### 已完成 ✅
- [x] 基础项目架构
- [x] OAuth 认证系统
- [x] 本地仓库管理
- [x] Git 基础操作（提交、暂存）
- [x] 分支管理
- [x] 同步操作（fetch/pull/push）
- [x] 深度链接支持
- [x] 暗黑模式

### 开发中 🚧
- [ ] 合并冲突解决
- [ ] 标签管理
- [ ] 子模块支持
- [ ] 性能优化

### 计划中 📋
- [ ] 插件系统
- [ ] 自定义主题
- [ ] 多语言支持
- [ ] 移动端支持
- [ ] 云同步配置

## 许可证

本项目采用 [MIT 许可证](LICENSE)。

## 致谢

感谢以下开源项目的支持：
- [Tauri](https://tauri.app/) - 跨平台桌面应用框架
- [Vue.js](https://vuejs.org/) - 渐进式 JavaScript 框架
- [shadcn-vue](https://www.shadcn-vue.com/) - 精美的 Vue UI 组件库
- [git2](https://github.com/rust-lang/git2-rs) - Rust Git 库

## 联系方式

- **AtomGit Issues**：[提交问题或建议](https://atomgit.com/tianchang/AtomDesk/issues)
- **Discussions**：[参与讨论](https://atomgit.com/tianchang/AtomDesk/discussions)
- **Email**：tianchang488@gamil.com

---

⭐ 如果这个项目对你有帮助，请给我们一个 Star！
