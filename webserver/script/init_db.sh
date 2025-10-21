#!/usr/bin/env bash
set -x
set -eo pipefail

# 检查是否已设置自定义用户名。如果未设置，则默认是“postgres”
DB_USER=${POSTGRES_USER:=postgres}
# 检查是否已设置自定义密码。如果未设置，则默认是“password”
DB_PASSWORD="${POSTGRES_PASSWORD:=password}"
# 检查是否已设置自定义数据库名称。如果未设置，则默认是“newsletter”
DB_NAME="${POSTGRES_DB:=newsletter}"
# 检查是否已设置自定义数据库端口。如果未设置，则默认是“5432”
DB_PORT="${POSTGRES_PORT:=5432}"

# 使用Docker启动Postgres
docker run \
  -e POSTGRES_USER=${DB_USER} \
  -e POSTGRES_PASSWORD=${DB_PASSWORD} \
  -e POSTGRES_DB=${DB_NAME} \
  -p "${DB_PORT}":5432 \
  -d postgres \
  postgres -N 1000
# ^ 为了进行测试，增加了最大连接数