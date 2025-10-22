#!/usr/bin/env bash
set -x
set -eo pipefail

# 检查是否已设置自定义用户名。如果未设置，则默认是"postgres"
DB_USER=${POSTGRES_USER:=postgres}
# 检查是否已设置自定义密码。如果未设置，则默认是"123456"
DB_PASSWORD="${POSTGRES_PASSWORD:=123456}"
# 检查是否已设置自定义数据库名称。如果未设置，则默认是"newsletter"
DB_NAME="${POSTGRES_DB:=newsletter}"
# 检查是否已设置自定义数据库端口。如果未设置，则默认是"5432"
DB_PORT="${POSTGRES_PORT:=5432}"

echo "正在检查本地 PostgreSQL 服务..."

# 检查本地 PostgreSQL 是否运行
if pg_isready -h localhost -p $DB_PORT -U $DB_USER >/dev/null 2>&1; then
    echo "✅ 本地 PostgreSQL 服务正在运行"
    echo "使用本地 PostgreSQL 数据库"
    echo "连接信息: postgres://$DB_USER:$DB_PASSWORD@localhost:$DB_PORT/$DB_NAME"
    
    # 检查数据库是否存在，如果不存在则创建
    echo "检查数据库 $DB_NAME 是否存在..."
    if ! psql -h localhost -p $DB_PORT -U $DB_USER -d $DB_NAME -c "SELECT 1;" >/dev/null 2>&1; then
        echo "数据库 $DB_NAME 不存在，正在创建..."
        createdb -h localhost -p $DB_PORT -U $DB_USER $DB_NAME
        echo "✅ 数据库 $DB_NAME 创建成功"
    else
        echo "✅ 数据库 $DB_NAME 已存在"
    fi
else
    echo "❌ 本地 PostgreSQL 服务未运行，使用 Docker 启动 PostgreSQL"
    
    # 检查 Docker 是否运行
    if ! docker info >/dev/null 2>&1; then
        echo "❌ Docker 未运行，请启动 Docker 或本地 PostgreSQL 服务"
        exit 1
    fi
    
    # 使用Docker启动Postgres
    echo "使用 Docker 启动 PostgreSQL..."
    docker run \
      -e POSTGRES_USER=${DB_USER} \
      -e POSTGRES_PASSWORD=${DB_PASSWORD} \
      -e POSTGRES_DB=${DB_NAME} \
      -p "${DB_PORT}":5432 \
      -d postgres \
      postgres -N 1000
    
    echo "✅ Docker PostgreSQL 容器启动成功"
    echo "连接信息: postgres://$DB_USER:$DB_PASSWORD@localhost:$DB_PORT/$DB_NAME"
fi

echo ""
echo "🎉 数据库初始化完成！"
echo "DATABASE_URL=postgres://$DB_USER:$DB_PASSWORD@localhost:$DB_PORT/$DB_NAME"