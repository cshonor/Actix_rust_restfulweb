FROM rust:1.59-slim-bullseye 
WORKDIR /app
RUN apt update && apt install lld clang -y
COPY . .
ENV SQLX_OFFLINE=true
RUN cargo build --release
ENV APP_ENVIRONMENT=production
CMD ["./target/release/webserver"]