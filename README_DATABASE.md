# Actix Rust API - 数据库版本

## 🎯 项目概述

这是一个基于 Actix Web 框架的 RESTful API，现已升级为使用 PostgreSQL 数据库进行持久化存储，适用于生产环境。

## 🚀 快速开始

### 方法一：使用 Docker Compose（推荐）

```bash
# 克隆项目
git clone <your-repo>
cd actix_rust

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

## 📊 数据库架构

### 用户表 (users)

| 字段 | 类型 | 说明 |
|------|------|------|
| id | UUID | 主键，自动生成 |
| name | VARCHAR(255) | 用户姓名 |
| email | VARCHAR(255) | 邮箱地址（唯一） |
| created_at | TIMESTAMP | 创建时间 |
| updated_at | TIMESTAMP | 更新时间 |

### 索引

- `idx_users_email` - 邮箱唯一索引
- `idx_users_created_at` - 创建时间索引

## 🔧 API 端点

| 方法 | 端点 | 说明 |
|------|------|------|
| GET | `/health` | 健康检查 |
| GET | `/api/users` | 获取所有用户 |
| GET | `/api/users/{id}` | 获取指定用户 |
| POST | `/api/users` | 创建新用户 |
| PUT | `/api/users/{id}` | 更新用户 |
| DELETE | `/api/users/{id}` | 删除用户 |

## 📝 API 使用示例

### 创建用户

```bash
curl -X POST http://localhost:8081/api/users \
  -H "Content-Type: application/json" \
  -d '{
    "name": "张三",
    "email": "zhangsan@example.com"
  }'
```

### 获取所有用户

```bash
curl http://localhost:8081/api/users
```

### 更新用户

```bash
curl -X PUT http://localhost:8081/api/users/{user-id} \
  -H "Content-Type: application/json" \
  -d '{
    "name": "李四",
    "email": "lisi@example.com"
  }'
```

## 🏗️ 项目结构

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
└── DATABASE_SETUP.md   # 详细设置指南
```

## 🔒 生产环境部署

### 环境变量配置

```env
# 生产环境配置
DATABASE_URL=postgresql://user:pass@prod-db:5432/actix_rust_db
HOST=0.0.0.0
PORT=8081
RUST_LOG=warn
```

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

### 性能优化

1. **连接池配置**
   - 根据负载调整连接数
   - 监控连接池使用率

2. **数据库优化**
   - 定期分析查询性能
   - 添加必要的索引
   - 考虑读写分离

3. **应用优化**
   - 启用日志级别控制
   - 监控内存使用
   - 设置合理的超时时间

## 📊 监控和维护

### 健康检查

```bash
# 应用健康检查
curl http://localhost:8081/health

# 数据库连接检查
psql -h localhost -U postgres -d actix_rust_db -c "SELECT 1;"
```

### 日志监控

```bash
# 查看应用日志
docker-compose logs -f app

# 查看数据库日志
docker-compose logs -f postgres
```

### 备份策略

```bash
# 数据库备份
pg_dump -h localhost -U postgres actix_rust_db > backup_$(date +%Y%m%d).sql

# 恢复数据库
psql -h localhost -U postgres actix_rust_db < backup_20240101.sql
```

## 🛠️ 故障排除

### 常见问题

1. **数据库连接失败**
   - 检查 PostgreSQL 是否运行
   - 验证连接字符串格式
   - 确认用户权限

2. **迁移失败**
   - 检查数据库权限
   - 手动运行迁移：`sqlx migrate run`

3. **性能问题**
   - 监控连接池使用率
   - 检查数据库索引
   - 分析慢查询

### 调试命令

```bash
# 检查数据库连接
psql $DATABASE_URL -c "SELECT version();"

# 查看表结构
psql $DATABASE_URL -c "\d users"

# 检查迁移状态
sqlx migrate info
```

## 📚 相关文档

- [DATABASE_SETUP.md](./DATABASE_SETUP.md) - 详细数据库设置指南
- [Actix Web 文档](https://actix.rs/) - 框架官方文档
- [SQLx 文档](https://docs.rs/sqlx/) - 数据库驱动文档
- [PostgreSQL 文档](https://www.postgresql.org/docs/) - 数据库文档

## 🤝 贡献指南

1. Fork 项目
2. 创建功能分支
3. 提交更改
4. 推送到分支
5. 创建 Pull Request

## 📄 许可证

MIT License
