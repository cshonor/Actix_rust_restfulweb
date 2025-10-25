#!/bin/bash

# Docker 构建和运行脚本
set -e

echo "🐳 Rust Actix Web Server Docker 构建脚本"
echo "========================================"

# 检查 Docker 是否运行
if ! docker info >/dev/null 2>&1; then
    echo "❌ Docker 未运行，请启动 Docker"
    exit 1
fi

# 构建镜像
echo "📦 构建 Docker 镜像..."
docker build -t actix-webserver:latest .

if [ $? -eq 0 ]; then
    echo "✅ 镜像构建成功！"
else
    echo "❌ 镜像构建失败"
    exit 1
fi

# 显示镜像信息
echo ""
echo "📋 镜像信息："
docker images actix-webserver:latest

echo ""
echo "🚀 使用方法："
echo "1. 单独运行应用: docker run -p 8080:8080 actix-webserver:latest"
echo "2. 使用 Docker Compose: docker-compose up"
echo "3. 后台运行: docker-compose up -d"
