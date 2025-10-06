# 数据库设置指南

## 概述

本项目已从内存存储升级为 PostgreSQL 数据库持久化存储，适用于生产环境。

## 技术栈

- **数据库**: PostgreSQL 12+
- **ORM**: SQLx (异步数据库驱动)
- **连接池**: SQLx 内置连接池
- **迁移**: SQLx Migrate

## 环境要求

### 1. PostgreSQL 安装

#### Windows
```bash
# 使用 Chocolatey
choco install postgresql

# 或下载官方安装包
# https://www.postgresql.org/download/windows/
```

#### macOS
```bash
# 使用 Homebrew
brew install postgresql
brew services start postgresql
```

#### Linux (Ubuntu/Debian)
```bash
sudo apt update
sudo apt install postgresql postgresql-contrib
sudo systemctl start postgresql
sudo systemctl enable postgresql
```

### 2. 创建数据库

```sql
-- 连接到 PostgreSQL
psql -U postgres

-- 创建数据库
CREATE DATABASE actix_rust_db;

-- 创建用户（可选）
CREATE USER actix_user WITH PASSWORD 'your_password';
GRANT ALL PRIVILEGES ON DATABASE actix_rust_db TO actix_user;
```

## 配置

### 1. 环境变量

复制 `env.example` 到 `.env` 并修改配置：

```bash
cp env.example .env
```

编辑 `.env` 文件：

```env
# 数据库配置
DATABASE_URL=postgresql://username:password@localhost:5432/actix_rust_db

# 服务器配置
HOST=127.0.0.1
PORT=8081

# 日志级别
RUST_LOG=info
```

### 2. 数据库连接字符串格式

```
postgresql://[user[:password]@][host][:port][/database][?param1=value1&...]
```

示例：
- 本地开发: `postgresql://postgres:password@localhost:5432/actix_rust_db`
- 生产环境: `postgresql://user:pass@prod-server:5432/actix_rust_db`

## 运行应用

### 1. 安装依赖

```bash
cargo build
```

### 2. 运行迁移

应用启动时会自动运行数据库迁移，创建必要的表和索引。

### 3. 启动应用

```bash
cargo run
```

## 生产环境部署

### 1. Docker 部署

创建 `docker-compose.yml`:

```yaml
version: '3.8'
services:
  postgres:
    image: postgres:15
    environment:
      POSTGRES_DB: actix_rust_db
      POSTGRES_USER: actix_user
      POSTGRES_PASSWORD: your_secure_password
    volumes:
      - postgres_data:/var/lib/postgresql/data
    ports:
      - "5432:5432"

  app:
    build: .
    environment:
      DATABASE_URL: postgresql://actix_user:your_secure_password@postgres:5432/actix_rust_db
      HOST: 0.0.0.0
      PORT: 8081
    ports:
      - "8081:8081"
    depends_on:
      - postgres

volumes:
  postgres_data:
```

### 2. 环境变量配置

生产环境建议使用环境变量或密钥管理服务：

```bash
# 生产环境变量
export DATABASE_URL="postgresql://user:pass@prod-db:5432/actix_rust_db"
export HOST="0.0.0.0"
export PORT="8081"
export RUST_LOG="warn"
```

### 3. 数据库优化

#### 连接池配置
```rust
// 在 main.rs 中可以配置连接池
let pool = PgPool::builder()
    .max_connections(20)
    .min_connections(5)
    .connect(&config.database_url)
    .await?;
```

#### 数据库索引
已自动创建以下索引：
- `idx_users_email` - 邮箱唯一索引
- `idx_users_created_at` - 创建时间索引

## 监控和维护

### 1. 健康检查

应用提供健康检查端点：
```bash
curl http://localhost:8081/health
```

### 2. 数据库监控

```sql
-- 查看连接数
SELECT count(*) FROM pg_stat_activity WHERE datname = 'actix_rust_db';

-- 查看表大小
SELECT schemaname,tablename,pg_size_pretty(size) as size
FROM (
    SELECT schemaname,tablename,pg_total_relation_size(schemaname||'.'||tablename) as size
    FROM pg_tables WHERE schemaname = 'public'
) t ORDER BY size DESC;
```

### 3. 备份

```bash
# 备份数据库
pg_dump -h localhost -U postgres actix_rust_db > backup.sql

# 恢复数据库
psql -h localhost -U postgres actix_rust_db < backup.sql
```

## 故障排除

### 1. 连接问题

检查数据库是否运行：
```bash
# 检查 PostgreSQL 状态
sudo systemctl status postgresql

# 测试连接
psql -h localhost -U postgres -d actix_rust_db
```

### 2. 权限问题

```sql
-- 授予权限
GRANT ALL PRIVILEGES ON DATABASE actix_rust_db TO your_user;
GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA public TO your_user;
```

### 3. 迁移问题

如果迁移失败，可以手动运行：
```bash
# 查看迁移状态
sqlx migrate info

# 手动运行迁移
sqlx migrate run
```

## 性能优化

### 1. 连接池调优

根据负载调整连接池大小：
- 轻量级应用: 5-10 连接
- 中等负载: 10-20 连接  
- 高负载: 20-50 连接

### 2. 查询优化

- 使用适当的索引
- 避免 N+1 查询
- 使用连接池复用连接

### 3. 监控指标

- 连接池使用率
- 查询响应时间
- 数据库连接数
- 内存使用情况
