# 多阶段构建 Dockerfile for Rust Actix Web Server

# 第一阶段：构建阶段
FROM rust:1.75-slim as builder

# 安装系统依赖
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libpq-dev \
    curl \
    && rm -rf /var/lib/apt/lists/*

# 设置工作目录
WORKDIR /app

# 复制 Cargo.toml 和 Cargo.lock（利用 Docker 缓存）
COPY Cargo.toml Cargo.lock ./

# 创建虚拟的 src/main.rs 来预编译依赖
RUN mkdir src && echo "fn main() {}" > src/main.rs

# 预编译依赖（利用 Docker 缓存层）
RUN cargo build --release && rm -rf src

# 复制源代码
COPY src ./src
COPY configuration.yaml ./
COPY migrations ./migrations

# 构建应用程序
RUN cargo build --release

# 第二阶段：运行阶段
FROM debian:bookworm-slim

# 安装运行时依赖
RUN apt-get update && apt-get install -y \
    libssl3 \
    libpq5 \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

# 创建非 root 用户
RUN groupadd -r appuser && useradd -r -g appuser appuser

# 设置工作目录
WORKDIR /app

# 从构建阶段复制二进制文件
COPY --from=builder /app/target/release/webserver /app/webserver

# 复制配置文件
COPY --from=builder /app/configuration.yaml /app/configuration.yaml

# 复制数据库迁移文件
COPY --from=builder /app/migrations /app/migrations

# 更改文件所有者
RUN chown -R appuser:appuser /app

# 切换到非 root 用户
USER appuser

# 暴露端口
EXPOSE 8080

# 设置环境变量
ENV RUST_LOG=info
ENV RUST_BACKTRACE=1

# 健康检查
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8080/health || exit 1

# 启动应用程序
CMD ["./webserver"]
