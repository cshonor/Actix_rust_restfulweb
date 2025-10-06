# Actix Rust 开发环境启动脚本
# 适用于 Windows PowerShell

Write-Host "🚀 启动 Actix Rust 开发环境..." -ForegroundColor Green

# 检查是否安装了 PostgreSQL
Write-Host "📋 检查 PostgreSQL 安装..." -ForegroundColor Yellow
try {
    $pgVersion = psql --version 2>$null
    if ($LASTEXITCODE -eq 0) {
        Write-Host "✅ PostgreSQL 已安装: $pgVersion" -ForegroundColor Green
    } else {
        Write-Host "❌ PostgreSQL 未安装，请先安装 PostgreSQL" -ForegroundColor Red
        Write-Host "   下载地址: https://www.postgresql.org/download/windows/" -ForegroundColor Cyan
        exit 1
    }
} catch {
    Write-Host "❌ PostgreSQL 未安装，请先安装 PostgreSQL" -ForegroundColor Red
    Write-Host "   下载地址: https://www.postgresql.org/download/windows/" -ForegroundColor Cyan
    exit 1
}

# 检查环境变量文件
if (-not (Test-Path ".env")) {
    Write-Host "📝 创建 .env 文件..." -ForegroundColor Yellow
    Copy-Item "env.example" ".env"
    Write-Host "✅ 已创建 .env 文件，请根据需要修改数据库连接信息" -ForegroundColor Green
}

# 检查 Rust 工具链
Write-Host "🔧 检查 Rust 工具链..." -ForegroundColor Yellow
try {
    $rustVersion = rustc --version 2>$null
    if ($LASTEXITCODE -eq 0) {
        Write-Host "✅ Rust 已安装: $rustVersion" -ForegroundColor Green
    } else {
        Write-Host "❌ Rust 未安装，请先安装 Rust" -ForegroundColor Red
        Write-Host "   安装命令: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh" -ForegroundColor Cyan
        exit 1
    }
} catch {
    Write-Host "❌ Rust 未安装，请先安装 Rust" -ForegroundColor Red
    Write-Host "   安装命令: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh" -ForegroundColor Cyan
    exit 1
}

# 构建项目
Write-Host "🔨 构建项目..." -ForegroundColor Yellow
cargo build
if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ 构建失败" -ForegroundColor Red
    exit 1
}
Write-Host "✅ 构建成功" -ForegroundColor Green

# 启动应用
Write-Host "🚀 启动应用..." -ForegroundColor Yellow
Write-Host "📡 服务器将在 http://localhost:8081 启动" -ForegroundColor Cyan
Write-Host "🔗 健康检查: http://localhost:8081/health" -ForegroundColor Cyan
Write-Host "📚 API 文档: 查看 README.md" -ForegroundColor Cyan
Write-Host ""
Write-Host "按 Ctrl+C 停止服务器" -ForegroundColor Yellow
Write-Host ""

cargo run
