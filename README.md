# Actix Web RESTful Server

一个使用 Rust 和 Actix Web 框架构建的 RESTful API 服务器。

## 功能特性

- 🚀 基于 Actix Web 框架
- 📡 RESTful API 设计
- 🔒 类型安全的数据处理
- 🎯 完整的 CRUD 操作
- 💾 内存数据存储
- 🏥 健康检查端点

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

### 1. 安装依赖
```bash
cargo build
```

### 2. 运行服务器
```bash
cargo run
```

服务器将在 `http://localhost:8080` 启动。

### 3. 测试 API

#### 创建用户
```bash
curl -X POST http://localhost:8080/api/users \
  -H "Content-Type: application/json" \
  -d '{"name": "张三", "email": "zhangsan@example.com"}'
```

#### 获取所有用户
```bash
curl http://localhost:8080/api/users
```

#### 获取特定用户
```bash
curl http://localhost:8080/api/users/{user-id}
```

#### 更新用户
```bash
curl -X PUT http://localhost:8080/api/users/{user-id} \
  -H "Content-Type: application/json" \
  -d '{"name": "李四", "email": "lisi@example.com"}'
```

#### 删除用户
```bash
curl -X DELETE http://localhost:8080/api/users/{user-id}
```

#### 健康检查
```bash
curl http://localhost:8080/health
```

## 项目结构

```
actix_rust/
├── Cargo.toml          # 项目配置和依赖
├── src/
│   └── main.rs         # 主程序文件
└── README.md           # 项目说明
```

## 技术栈

- **Actix Web**: 高性能 Web 框架
- **Serde**: 序列化和反序列化
- **UUID**: 唯一标识符生成
- **Tokio**: 异步运行时

## 开发说明

这个项目演示了如何使用 Actix Web 构建一个完整的 RESTful API，包括：

- 数据模型定义
- 路由配置
- 请求处理
- 错误处理
- 状态管理

数据存储在内存中，重启服务器后数据会丢失。在生产环境中，建议使用数据库进行持久化存储。
