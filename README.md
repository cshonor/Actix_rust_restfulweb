# Actix Web RESTful Server with PostgreSQL

一个使用 Rust 和 Actix Web 框架构建的生产级 RESTful API 服务器，支持 PostgreSQL 数据库持久化存储。

## 功能特性

- 🚀 基于 Actix Web 框架
- 📡 RESTful API 设计
- 🔒 类型安全的数据处理
- 🎯 完整的 CRUD 操作
- 🗄️ PostgreSQL 数据库持久化存储
- 🔄 异步数据库连接池
- 📊 自动数据库迁移
- 🐳 Docker 容器化部署
- 🏥 健康检查端点
- 📝 完整的生产环境支持

## API 端点

### 健康检查
- `GET /health` - 服务器健康状态检查

### 用户管理
- `GET /api/users` - 获取所有用户
- `GET /api/users/{id}` - 根据 ID 获取用户
- `POST /api/users` - 创建新用户
- `PUT /api/users/{id}` - 更新用户信息
- `DELETE /api/users/{id}` - 删除用户

## 快速开始

### 方法一：使用 Docker Compose（推荐）

```bash
# 启动所有服务（数据库 + 应用）
docker-compose up -d

# 查看日志
docker-compose logs -f app
```

访问：
- API: http://localhost:8081
- 数据库管理: http://localhost:8080 (Adminer)

### 方法二：本地开发

#### 1. 安装依赖

**PostgreSQL**
```bash
# Windows (使用 Chocolatey)
choco install postgresql

# macOS
brew install postgresql

# Ubuntu/Debian
sudo apt install postgresql postgresql-contrib
```

**Rust**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

#### 2. 设置数据库

```sql
-- 创建数据库
CREATE DATABASE actix_rust_db;

-- 创建用户（可选）
CREATE USER actix_user WITH PASSWORD 'your_password';
GRANT ALL PRIVILEGES ON DATABASE actix_rust_db TO actix_user;
```

#### 3. 配置环境

```bash
# 复制环境变量文件
cp env.example .env

# 编辑数据库连接信息
# DATABASE_URL=postgresql://username:password@localhost:5432/actix_rust_db
```

#### 4. 运行应用

```bash
# Windows
.\start_dev.ps1

# Linux/macOS
cargo run
```

服务器将在 `http://localhost:8081` 启动。

### 3. 测试 API

#### 创建用户
```bash
curl -X POST http://localhost:8081/api/users \
  -H "Content-Type: application/json" \
  -d '{"name": "张三", "email": "zhangsan@example.com"}'
```

#### 获取所有用户
```bash
curl http://localhost:8081/api/users
```

#### 获取特定用户
```bash
curl http://localhost:8081/api/users/{user-id}
```

#### 更新用户
```bash
curl -X PUT http://localhost:8081/api/users/{user-id} \
  -H "Content-Type: application/json" \
  -d '{"name": "李四", "email": "lisi@example.com"}'
```

#### 删除用户
```bash
curl -X DELETE http://localhost:8081/api/users/{user-id}
```

#### 健康检查
```bash
curl http://localhost:8081/health
```

## 项目结构

```
actix_rust/
├── src/
│   ├── main.rs          # 主应用入口
│   ├── config.rs        # 配置管理
│   ├── models.rs        # 数据模型
│   └── database.rs      # 数据库访问层
├── migrations/          # 数据库迁移文件
│   └── 001_create_users_table.sql
├── docker-compose.yml   # Docker 编排文件
├── Dockerfile          # Docker 镜像构建
├── env.example         # 环境变量示例
├── start_dev.ps1       # 开发环境启动脚本
├── DATABASE_SETUP.md   # 详细数据库设置指南
├── README_DATABASE.md  # 数据库版本完整文档
└── README.md           # 项目说明
```

## 技术栈

- **Actix Web**: 高性能 Web 框架
- **PostgreSQL**: 生产级关系数据库
- **SQLx**: 异步数据库驱动
- **Serde**: 序列化和反序列化
- **UUID**: 唯一标识符生成
- **Tokio**: 异步运行时
- **Docker**: 容器化部署

## 数据库架构

### 用户表 (users)

| 字段 | 类型 | 说明 |
|------|------|------|
| id | UUID | 主键，自动生成 |
| name | VARCHAR(255) | 用户姓名 |
| email | VARCHAR(255) | 邮箱地址（唯一） |
| created_at | TIMESTAMP | 创建时间 |
| updated_at | TIMESTAMP | 更新时间 |

## 生产环境部署

### Docker 部署

```bash
# 构建镜像
docker build -t actix-rust-api .

# 运行容器
docker run -d \
  --name actix-api \
  -p 8081:8081 \
  -e DATABASE_URL="postgresql://user:pass@db:5432/actix_rust_db" \
  actix-rust-api
```

### 环境变量配置

```env
# 生产环境配置
DATABASE_URL=postgresql://user:pass@prod-db:5432/actix_rust_db
HOST=0.0.0.0
PORT=8081
RUST_LOG=warn
```

## 开发说明

这个项目演示了如何使用 Actix Web 构建一个生产级的 RESTful API，包括：

- 数据模型定义和数据库映射
- 异步数据库连接池管理
- 自动数据库迁移
- 配置管理和环境变量
- 错误处理和日志记录
- Docker 容器化部署
- 完整的生产环境支持

## 相关文档

- [DATABASE_SETUP.md](./DATABASE_SETUP.md) - 详细数据库设置指南
- [README_DATABASE.md](./README_DATABASE.md) - 数据库版本完整文档
- [Actix Web 文档](https://actix.rs/) - 框架官方文档
- [SQLx 文档](https://docs.rs/sqlx/) - 数据库驱动文档
