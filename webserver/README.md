# Actix Rust Web Server

一个基于 Actix Web 框架构建的现代化 Rust Web 服务器项目，提供健康检查、用户订阅、问候功能和完整的日志追踪系统。

## 功能特性

- 🚀 基于 Actix Web 4.x 的高性能异步 Web 服务器
- 💾 PostgreSQL 数据库集成 (使用 SQLx)
- 🔍 健康检查端点
- 👋 动态问候功能
- 📝 用户订阅功能
- 📊 完整的结构化日志系统 (Tracing + Bunyan)
- 🐳 Docker 容器化支持
- 🧪 完整的测试套件
- ⚙️ 灵活的配置管理

## 技术栈

- **Web 框架**: Actix Web 4
- **异步运行时**: Tokio
- **数据库**: PostgreSQL
- **ORM**: SQLx
- **HTTP 客户端**: Reqwest
- **序列化**: Serde
- **日志系统**: Tracing + Tracing-Subscriber + Bunyan Formatter
- **中间件**: Tracing-Actix-Web
- **配置管理**: Config crate
- **容器化**: Docker + Docker Compose

## 项目结构

```
webserver/
├── src/
│   ├── main.rs          # 应用程序入口点
│   ├── lib.rs           # 核心库文件
│   ├── startup.rs       # 服务器启动配置
│   ├── configuration.rs # 配置管理
│   └── routes/          # 路由模块
│       ├── mod.rs       # 路由模块声明
│       ├── greet.rs     # 问候功能
│       ├── health.rs    # 健康检查
│       ├── subscribe.rs # 用户订阅
│       └── telemetry.rs # 日志追踪配置
├── migrations/          # 数据库迁移文件
│   └── 20251022202150_create_subscriptions_table.sql
├── tests/
│   └── testUseTokio     # Tokio 使用测试
├── script/
│   └── init_db.sh       # 数据库初始化脚本
├── docker-build.sh      # Docker 构建脚本
├── docker-run.sh        # Docker 运行脚本
├── docker-compose.yml   # Docker Compose 配置
├── Dockerfile           # Docker 镜像构建文件
├── configuration.yaml   # 应用配置文件
├── Cargo.toml           # 项目依赖配置
└── README.md            # 项目说明文档
```

## 快速开始

### 前置要求

- Rust 1.70+ 
- Docker & Docker Compose (用于数据库和容器化部署)
- PostgreSQL 客户端工具 (可选)

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

3. **配置环境变量**
   ```bash
   # 创建 .env 文件 (可选)
   echo "DATABASE_URL=postgres://postgres:123456@localhost:5432/newsletter" > .env
   echo "RUST_LOG=info" >> .env
   ```

4. **启动数据库**
   ```bash
   # 方式1: 使用 Docker Compose (推荐)
   docker-compose up -d postgres
   
   # 方式2: 使用提供的脚本
   chmod +x script/init_db.sh
   ./script/init_db.sh
   ```

5. **运行数据库迁移**
   ```bash
   sqlx migrate run
   ```

6. **运行服务器**
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

- `DATABASE_URL` - 数据库连接字符串 (默认: postgres://postgres:123456@localhost:5432/newsletter)
- `RUST_LOG` - 日志级别 (默认: info)
- `APPLICATION_HOST` - 应用主机地址 (默认: 127.0.0.1)
- `APPLICATION_PORT` - 应用端口 (默认: 8080)

### 配置文件

项目支持 YAML 配置文件 (`configuration.yaml`)：

```yaml
application:
  host: "127.0.0.1"
  port: 8080

database:
  username: "postgres"
  password: "123456"
  port: 5432
  host: "localhost"
  database_name: "newsletter"
```

### 日志配置

项目使用 Tracing 框架提供结构化日志：

- **格式**: Bunyan JSON 格式
- **级别**: 通过 `RUST_LOG` 环境变量控制
- **中间件**: 自动捕获 HTTP 请求和响应信息
- **追踪**: 支持分布式追踪和 span 管理

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

项目提供完整的 Docker 支持：

#### 方式1: Docker Compose (推荐)

```bash
# 启动完整应用栈
docker-compose up -d

# 查看日志
docker-compose logs -f webserver

# 停止服务
docker-compose down
```

#### 方式2: 手动 Docker 构建

```bash
# 构建镜像
./docker-build.sh

# 运行容器
./docker-run.sh
```

#### 方式3: 传统部署

```bash
# 启动数据库
./script/init_db.sh

# 运行数据库迁移
sqlx migrate run

# 构建和运行应用
cargo build --release
./target/release/webserver
```

### 生产环境注意事项

- 确保设置正确的环境变量
- 配置适当的日志级别
- 使用 HTTPS 和反向代理
- 设置数据库连接池大小
- 配置监控和告警

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

## 性能特性

- **异步处理**: 基于 Tokio 异步运行时
- **连接池**: SQLx 数据库连接池
- **结构化日志**: Bunyan JSON 格式，便于日志分析
- **中间件支持**: Tracing-Actix-Web 自动捕获请求信息
- **配置管理**: 支持环境变量和 YAML 配置文件

## 监控和调试

### 日志查看

```bash
# 设置日志级别
export RUST_LOG=debug

# 查看结构化日志
cargo run | jq '.'
```

### 健康检查

```bash
# 检查服务状态
curl http://localhost:8080/health
```

---

**注意**: 这是一个学习项目，用于演示 Actix Web 框架的现代化用法，包括日志追踪、容器化和配置管理。在生产环境中使用前，请确保进行适当的安全配置和性能优化。
