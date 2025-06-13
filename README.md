# AtomDesk - AtomGit桌面客户端

[![Tauri](https://img.shields.io/badge/Tauri-v2-blue)](https://tauri.app/)
[![Vue](https://img.shields.io/badge/Vue-v3.5-green)](https://vuejs.org/)
[![TypeScript](https://img.shields.io/badge/TypeScript-v5.6-blue)](https://www.typescriptlang.org/)
[![Tailwind CSS](https://img.shields.io/badge/Tailwind-v4.1-blueviolet)](https://tailwindcss.com/)

## 项目介绍

AtomDesk是一款基于Tauri和Vue 3开发的AtomGit桌面客户端应用，为开发者提供了便捷的代码仓库管理体验。通过AtomDesk，您可以在桌面环境中高效地管理代码仓库、查看Issues、跟踪项目进度，无需频繁切换浏览器标签页。

## 功能特点

- **仓库管理**：浏览、搜索和管理您的代码仓库
- **Issues跟踪**：查看和管理项目Issues
- **项目管理**：跟踪项目进度和里程碑
- **活动概览**：实时查看您的最近活动和通知
- **OAuth认证**：安全的AtomGit账号授权登录
- **桌面通知**：重要事件的实时桌面通知

## 技术栈

- **前端框架**：Vue 3 + TypeScript
- **构建工具**：Vite
- **桌面框架**：Tauri 2
- **状态管理**：Pinia
- **路由管理**：Vue Router
- **UI组件**：Tailwind CSS + 自定义组件
- **HTTP客户端**：Axios (前端) + Reqwest (Rust后端)
- **认证方式**：OAuth 2.0

## 开发环境要求

- Node.js 16+
- Rust 1.60+
- 系统依赖（根据Tauri要求）

## 安装与运行

### 1. 克隆仓库

```bash
git clone https://github.com/yourusername/atomdesk.git
cd atomdesk
```

### 2. 安装依赖

```bash
pnpm install
```

### 3. 配置环境变量

复制`.env.example`文件并重命名为`.env`，然后填写必要的配置：

```
VITE_APP_BASE_API = 'https://api.atomgit.com'
VITE_APP_CLIENT_ID = '您的AtomGit OAuth客户端ID'
VITE_APP_CLIENT_SECRET = '您的AtomGit OAuth客户端密钥'
```

### 4. 开发模式运行

```bash
pnpm tauri dev
```

### 5. 构建生产版本

```bash
pnpm tauri build
```

## 项目结构

```
├── src/                  # 前端源代码
│   ├── assets/           # 静态资源
│   ├── components/       # 组件
│   ├── passport/         # 认证相关组件
│   ├── router/           # 路由配置
│   ├── services/         # 服务
│   ├── stores/           # Pinia状态管理
│   ├── utils/            # 工具函数
│   ├── view/             # 页面视图
│   ├── App.vue           # 根组件
│   └── main.ts           # 入口文件
├── src-tauri/            # Tauri/Rust后端代码
│   ├── src/              # Rust源代码
│   │   ├── http_client.rs # HTTP客户端实现
│   │   ├── lib.rs        # 库入口
│   │   └── main.rs       # 主程序入口
│   └── tauri.conf.json   # Tauri配置
└── public/               # 公共资源
```

## 推荐的IDE设置

- [VS Code](https://code.visualstudio.com/) + [Volar](https://marketplace.visualstudio.com/items?itemName=Vue.volar) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## 贡献指南

1. Fork 本仓库
2. 创建您的特性分支 (`git checkout -b feature/amazing-feature`)
3. 提交您的更改 (`git commit -m 'Add some amazing feature'`)
4. 推送到分支 (`git push origin feature/amazing-feature`)
5. 打开一个 Pull Request

## 许可证

[MIT](LICENSE)

## 联系方式

如有任何问题或建议，请通过Issues或Pull Requests与我们联系。
