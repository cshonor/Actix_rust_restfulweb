# 多阶段构建
FROM rust:1.75 as builder

# 设置工作目录
WORKDIR /app

# 复制依赖文件
COPY Cargo.toml Cargo.lock ./

# 创建虚拟依赖以利用 Docker 缓存
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# 复制源代码
COPY . .

# 构建应用
RUN cargo build --release

# 运行时镜像
FROM debian:bookworm-slim

# 安装运行时依赖
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# 创建非 root 用户
RUN useradd -m -u 1001 appuser

# 设置工作目录
WORKDIR /app

# 复制二进制文件
COPY --from=builder /app/target/release/actix_rust /app/actix_rust

# 复制迁移文件
COPY --from=builder /app/migrations /app/migrations

# 设置权限
RUN chown -R appuser:appuser /app
USER appuser

# 暴露端口
EXPOSE 8081

# 设置环境变量
ENV RUST_LOG=info
ENV HOST=0.0.0.0
ENV PORT=8081

# 启动应用
CMD ["./actix_rust"]
