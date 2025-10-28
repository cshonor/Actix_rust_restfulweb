# DigitalOcean App Platform 部署指南

## 📋 **部署步骤**

### 1. 使用 doctl 命令行工具

```bash
# 安装 doctl
scoop install doctl

# 配置 API token
doctl auth init

# 从 GitHub 仓库创建应用
doctl apps create --spec .do/app.yaml
```

### 2. 使用 Web 界面

1. 访问 [DigitalOcean App Platform](https://cloud.digitalocean.com/apps)
2. 点击 "Create App"
3. 选择 "GitHub" 作为源
4. 选择仓库: `cshonor/Actix_rust_restfulweb`
5. 选择分支: `main`
6. 应用会自动检测 `.do/app.yaml` 配置

## 🔧 **配置文件说明**

### `.do/app.yaml` 配置项

- **name**: 应用名称
- **services**: 服务配置
  - **source_dir**: 源代码目录
  - **run_command**: 运行命令
  - **environment_slug**: Rust 环境
  - **instance_size_slug**: 实例大小 (basic-xxs = 最便宜)
  - **http_port**: HTTP 端口
  - **health_check**: 健康检查路径
- **databases**: 数据库配置
  - **engine**: PostgreSQL
  - **version**: 版本 13
  - **size**: 开发数据库大小

## 🌐 **环境变量**

- `APP_ENVIRONMENT=production` - 生产环境
- `RUST_LOG=info` - 日志级别
- `DATABASE_URL` - 数据库连接字符串 (自动注入)
- `SQLX_OFFLINE=true` - 离线模式

## 💰 **成本估算**

- **应用实例**: basic-xxs ($5/月)
- **数据库**: db-s-dev-database ($15/月)
- **总计**: 约 $20/月

## 🚀 **部署后访问**

部署成功后，DigitalOcean 会提供一个 URL，例如：
`https://actix-webserver-xxxxx.ondigitalocean.app`

### API 端点

- `GET /` - 问候页面
- `GET /{name}` - 个性化问候
- `GET /health` - 健康检查
- `POST /subscribe` - 用户订阅

## 🔍 **监控和日志**

- 在 DigitalOcean 控制台查看应用日志
- 监控应用性能和错误
- 数据库连接状态检查
