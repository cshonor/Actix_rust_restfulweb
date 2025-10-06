# 安全框架文档

## 🔒 概述

本项目集成了完整的安全框架，提供多层安全防护，确保应用程序在生产环境中的安全性。

## 🛡️ 安全特性

### 1. 身份认证和授权

#### JWT (JSON Web Tokens)
- **算法**: HS256
- **过期时间**: 可配置（默认1小时）
- **密钥**: 环境变量 `JWT_SECRET`
- **用途**: 无状态身份认证

#### 密码安全
- **哈希算法**: Argon2id
- **盐值**: 随机生成
- **强度**: 抗彩虹表攻击
- **验证**: 密码强度检查

### 2. 输入验证和防护

#### 数据验证
```rust
// 用户注册验证
#[derive(Validate)]
pub struct CreateUserRequest {
    #[validate(length(min = 2, max = 100))]
    pub name: String,
    
    #[validate(email)]
    #[validate(length(max = 255))]
    pub email: String,
    
    #[validate(length(min = 8, max = 128))]
    pub password: String,
}
```

#### 输入清理
- XSS 防护
- SQL 注入防护（SQLx 参数化查询）
- 特殊字符过滤

### 3. 网络安全

#### CORS 配置
```rust
pub fn cors_config() -> actix_cors::Cors {
    actix_cors::Cors::default()
        .allowed_origin("http://localhost:3000")
        .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
        .allowed_headers(vec!["Content-Type", "Authorization"])
        .max_age(3600)
}
```

#### 安全头
- `X-Frame-Options: DENY` - 防止点击劫持
- `X-Content-Type-Options: nosniff` - 防止MIME嗅探
- `X-XSS-Protection: 1; mode=block` - XSS保护
- `Strict-Transport-Security` - HTTPS强制
- `Content-Security-Policy` - 内容安全策略
- `Referrer-Policy` - 引用者策略

### 4. 速率限制

#### 配置
- **请求限制**: 100 请求/分钟（可配置）
- **时间窗口**: 60 秒
- **基于IP**: 客户端IP地址
- **自动清理**: 过期条目自动删除

#### 实现
```rust
pub struct RateLimitMiddleware {
    max_requests: u32,
    window_duration: Duration,
    store: Arc<RwLock<HashMap<IpAddr, (u32, Instant)>>>,
}
```

### 5. 数据保护

#### 数据库安全
- **连接加密**: TLS/SSL
- **参数化查询**: 防止SQL注入
- **连接池**: 限制并发连接
- **权限控制**: 最小权限原则

#### 敏感数据处理
- **密码**: Argon2 哈希存储
- **JWT密钥**: 环境变量存储
- **日志**: 不记录敏感信息

## 🔧 配置

### 环境变量

```env
# JWT 配置
JWT_SECRET=your-super-secret-jwt-key-change-this-in-production
JWT_EXPIRATION=3600

# 速率限制
RATE_LIMIT_REQUESTS=100
RATE_LIMIT_WINDOW=60

# CORS 配置
ALLOWED_ORIGINS=http://localhost:3000,http://localhost:8080
```

### 生产环境建议

```env
# 生产环境安全配置
JWT_SECRET=your-256-bit-secret-key-here
JWT_EXPIRATION=1800  # 30分钟
RATE_LIMIT_REQUESTS=50
RATE_LIMIT_WINDOW=60
RUST_LOG=warn
```

## 🚀 使用示例

### 用户注册

```bash
curl -X POST http://localhost:8081/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "name": "张三",
    "email": "zhangsan@example.com",
    "password": "SecurePass123!"
  }'
```

**响应**:
```json
{
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "user": {
    "id": "123e4567-e89b-12d3-a456-426614174000",
    "name": "张三",
    "email": "zhangsan@example.com",
    "created_at": "2024-01-01T00:00:00Z",
    "updated_at": "2024-01-01T00:00:00Z"
  }
}
```

### 用户登录

```bash
curl -X POST http://localhost:8081/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "email": "zhangsan@example.com",
    "password": "SecurePass123!"
  }'
```

### 受保护的API调用

```bash
curl -X GET http://localhost:8081/api/users \
  -H "Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
```

## 🔍 安全监控

### 日志记录

```rust
// 请求日志
log::info!(
    "Request: {} {} from {} (User-Agent: {})",
    method, path, client_ip, user_agent
);

// 安全事件日志
log::warn!("Rate limit exceeded for IP: {}", client_ip);
log::error!("Authentication failed for user: {}", email);
```

### 健康检查

```bash
curl http://localhost:8081/health
```

**响应**:
```json
{
  "status": "healthy",
  "message": "Server is running",
  "security": "enabled"
}
```

## 🛠️ 安全最佳实践

### 1. 密码策略
- 最少8个字符
- 包含大小写字母
- 包含数字
- 包含特殊字符
- 定期更换

### 2. JWT 安全
- 使用强密钥（256位）
- 设置合理的过期时间
- 定期轮换密钥
- 使用 HTTPS

### 3. 数据库安全
- 使用参数化查询
- 限制数据库用户权限
- 启用连接加密
- 定期备份

### 4. 网络安全
- 使用 HTTPS
- 配置防火墙
- 限制访问来源
- 监控异常流量

### 5. 应用安全
- 输入验证
- 输出编码
- 错误处理
- 日志记录

## 🔧 故障排除

### 常见问题

1. **JWT 验证失败**
   - 检查密钥配置
   - 验证token格式
   - 确认过期时间

2. **速率限制触发**
   - 检查请求频率
   - 调整限制参数
   - 查看日志

3. **CORS 错误**
   - 检查允许的源
   - 验证请求头
   - 确认预检请求

### 调试命令

```bash
# 检查JWT token
echo "your-token" | base64 -d

# 测试速率限制
for i in {1..110}; do curl http://localhost:8081/health; done

# 验证CORS
curl -H "Origin: http://localhost:3000" \
     -H "Access-Control-Request-Method: POST" \
     -H "Access-Control-Request-Headers: Content-Type" \
     -X OPTIONS \
     http://localhost:8081/api/users
```

## 📚 相关资源

- [OWASP Top 10](https://owasp.org/www-project-top-ten/)
- [JWT.io](https://jwt.io/) - JWT 调试工具
- [Argon2](https://github.com/P-H-C/phc-winner-argon2) - 密码哈希算法
- [Actix Web Security](https://actix.rs/docs/security/) - 官方安全文档
- [Rust Security](https://cheats.rs/#security) - Rust 安全最佳实践
