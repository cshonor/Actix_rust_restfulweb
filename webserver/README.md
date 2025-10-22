# Actix Rust Web Server

一个基于 Actix Web 框架构建的 Rust Web 服务器项目，提供健康检查、用户订阅和问候功能。

## 功能特性

- 🚀 基于 Actix Web 4.x 的高性能异步 Web 服务器
- 💾 PostgreSQL 数据库集成 (使用 SQLx)
- 🔍 健康检查端点
- 👋 动态问候功能
- 📝 用户订阅功能
- 🧪 完整的测试套件
- 🐳 Docker 数据库支持

## 技术栈

- **Web 框架**: Actix Web 4
- **异步运行时**: Tokio
- **数据库**: PostgreSQL
- **ORM**: SQLx
- **HTTP 客户端**: Reqwest
- **序列化**: Serde

## 项目结构

```
webserver/
├── src/
│   ├── main.rs          # 应用程序入口点
│   ├── lib.rs           # 核心库文件，包含路由和处理器
│   ├── configuration.rs # 配置管理
│   └── routes/          # 路由模块
├── tests/
│   ├── testhealth.rs   # 健康检查测试
│   └── testUseTokio    # Tokio 使用测试
├── script/
│   └── init_db.sh      # 数据库初始化脚本
├── Cargo.toml          # 项目依赖配置
└── README.md           # 项目说明文档
```

## 快速开始

### 前置要求

- Rust 1.70+ 
- Docker (用于数据库)
- PostgreSQL 客户端工具

### 安装步骤

1. **克隆项目**
   ```bash
   git clone <repository-url>
   cd actix_rust/webserver
   ```

2. **安装依赖**
   ```bash
   cargo build
   ```

3. **启动数据库**
   ```bash
   # 使用提供的脚本启动 PostgreSQL
   chmod +x script/init_db.sh
   ./script/init_db.sh
   ```

4. **运行服务器**
   ```bash
   cargo run
   ```

服务器将在 `http://localhost:8080` 启动。

## API 端点

### 基础端点

- `GET /` - 返回 "Hello, World!"
- `GET /{name}` - 返回 "Hello, {name}!"
- `GET /health` - 健康检查端点
- `POST /subscribe` - 用户订阅端点

### 使用示例

```bash
# 基础问候
curl http://localhost:8080/

# 个性化问候
curl http://localhost:8080/Alice

# 健康检查
curl http://localhost:8080/health

# 用户订阅
curl -X POST http://localhost:8080/subscribe \
  -H "Content-Type: application/x-www-form-urlencoded" \
  -d "name=张三&email=zhangsan@example.com"
```

## 测试

运行测试套件：

```bash
# 运行所有测试
cargo test

# 运行特定测试
cargo test test_health_check
```

## 配置

### 环境变量

- `POSTGRES_USER` - 数据库用户名 (默认: postgres)
- `POSTGRES_PASSWORD` - 数据库密码 (默认: password)
- `POSTGRES_DB` - 数据库名称 (默认: newsletter)
- `POSTGRES_PORT` - 数据库端口 (默认: 5432)

### 数据库配置

项目使用 PostgreSQL 作为数据库。数据库配置通过环境变量或默认值进行管理。

## 开发

### 代码结构

- `src/lib.rs` - 包含主要的应用程序逻辑和路由定义
- `src/main.rs` - 应用程序入口点
- `tests/` - 测试文件目录

### 添加新功能

1. 在 `src/lib.rs` 中添加新的处理器函数
2. 在 `run()` 函数中注册新路由
3. 添加相应的测试

## 部署

### Docker 部署

项目包含数据库初始化脚本，支持 Docker 部署：

```bash
# 启动数据库
./script/init_db.sh

# 构建和运行应用
cargo build --release
./target/release/webserver
```

## 贡献

1. Fork 项目
2. 创建功能分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 打开 Pull Request

## 许可证

本项目采用 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情。

## 联系方式

如有问题或建议，请通过以下方式联系：

- 创建 Issue
- 发送邮件至 [your-email@example.com]

---

**注意**: 这是一个学习项目，用于演示 Actix Web 框架的基本用法。在生产环境中使用前，请确保进行适当的安全配置和性能优化。
