```markdown
# My-App

> 一个同时支持「在线云模式」和「离线本地模式」的跨平台桌面应用  
> 技术栈：Rust（Axum + Tauri） + Vue3 + Vite

---

## ✨ 特性

- **一处编码，两处运行**  
  所有业务逻辑都封装在 `shared` crate，在线服务器与桌面客户端零重复代码。
- **一键切换模式**  
  前端可在「云服务器」与「本地 Rust」之间动态切换，无需重启。
- **真正的离线可用**  
  本地模式完全脱离网络，数据与功能全部内嵌。
- **跨平台发布**  
  Tauri 自动输出 Windows / macOS / Linux 安装包，云服务器可容器化部署。

---

## 🚀 快速开始

### 1️⃣ 克隆与安装

```bash
git clone https://github.com/yourname/my-app.git
cd my-app
# 安装前端依赖
npm install
# 拉取 Rust 依赖
cargo build --release
```

### 2️⃣ 启动在线服务器（可选）

```bash
cargo run -p server
# 服务将运行在 http://localhost:3030
```

### 3️⃣ 启动桌面客户端

```bash
npm run tauri dev
# 或打包
npm run tauri build
```

---

## 🏗️ 项目结构

| 目录 | 职责 |
|------|------|
| `shared/` | 公共库：DTO、业务逻辑、数据库实体 |
| `server/` | 在线 HTTP API（Axum） |
| `src-tauri/` | 本地 Tauri 后端（Rust） |
| `src/` | Vue3 + Vite 前端 |
| `docs.md` | 架构与扩展规范 |

---

## 📦 构建 & 发布

| 目标 | 命令 |
|------|------|
| 前端构建产物 | `npm run build` |
| 桌面安装包 | `npm run tauri build` |
| 云服务器镜像 | `docker build -t my-app-server ./server` |

---

## 🧑‍💻 开发者指南

### 新增业务接口

1. 在 `shared/src/service.rs` 写纯逻辑函数  
2. 在 `server/src/main.rs` 加一条 Axum 路由  
3. 在 `src-tauri/src/main.rs` 加一个 `#[tauri::command]`  
4. 前端调用：`axios.get('/api/hello')` 或 `invoke('hello_local')`


## 🛠️ 技术栈

| 层级 | 技术 |
|------|------|
| 在线服务器 | Rust + Axum |
| 本地后端 | Rust + Tauri |
| 前端 | Vue3 + Vite + JavaScript |
| 构建 | Cargo workspace + Tauri CLI |

---

## 🤝 贡献

1. Fork 本仓库  
2. 新建分支 `feat/xxx`  
3. 提交 PR，遵循 [Conventional Commits](https://www.conventionalcommits.org/)

---

## 📄 许可证

MIT © 2025 Your Name
```