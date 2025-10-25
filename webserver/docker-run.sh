#!/bin/bash

# Docker 运行脚本
set -e

echo "🚀 启动 Rust Actix Web Server"
echo "=============================="

# 检查 Docker Compose 文件是否存在
if [ ! -f "docker-compose.yml" ]; then
    echo "❌ docker-compose.yml 文件不存在"
    exit 1
fi

# 停止现有容器（如果存在）
echo "🛑 停止现有容器..."
docker-compose down 2>/dev/null || true

# 启动服务
echo "🚀 启动服务..."
docker-compose up --build

echo ""
echo "✅ 服务已启动！"
echo "📋 可用端点："
echo "  - 健康检查: http://localhost:8080/health"
echo "  - 问候: http://localhost:8080/"
echo "  - 个性化问候: http://localhost:8080/你的名字"
echo "  - 订阅: POST http://localhost:8080/subscribe"
echo ""
echo "按 Ctrl+C 停止服务"
