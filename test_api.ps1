# Test script for Actix Web RESTful API
Write-Host "Testing Actix Web RESTful API..." -ForegroundColor Green

# Test health endpoint
Write-Host "`n1. Testing health endpoint..." -ForegroundColor Yellow
try {
    $health = Invoke-RestMethod -Uri "http://localhost:8081/health" -Method GET
    Write-Host "Health check response:" -ForegroundColor Cyan
    $health | ConvertTo-Json
} catch {
    Write-Host "Health check failed: $($_.Exception.Message)" -ForegroundColor Red
}

# Test get users (should return empty array initially)
Write-Host "`n2. Testing GET /api/users..." -ForegroundColor Yellow
try {
    $users = Invoke-RestMethod -Uri "http://localhost:8081/api/users" -Method GET
    Write-Host "Users response:" -ForegroundColor Cyan
    $users | ConvertTo-Json
} catch {
    Write-Host "GET users failed: $($_.Exception.Message)" -ForegroundColor Red
}

# Test create user
Write-Host "`n3. Testing POST /api/users..." -ForegroundColor Yellow
try {
    $newUser = @{
        name = "张三"
        email = "zhangsan@example.com"
    } | ConvertTo-Json

    $createdUser = Invoke-RestMethod -Uri "http://localhost:8081/api/users" -Method POST -Body $newUser -ContentType "application/json"
    Write-Host "Created user:" -ForegroundColor Cyan
    $createdUser | ConvertTo-Json
    $userId = $createdUser.id
} catch {
    Write-Host "Create user failed: $($_.Exception.Message)" -ForegroundColor Red
    $userId = $null
}

# Test get specific user
if ($userId) {
    Write-Host "`n4. Testing GET /api/users/$userId..." -ForegroundColor Yellow
    try {
        $user = Invoke-RestMethod -Uri "http://localhost:8081/api/users/$userId" -Method GET
        Write-Host "Retrieved user:" -ForegroundColor Cyan
        $user | ConvertTo-Json
    } catch {
        Write-Host "GET specific user failed: $($_.Exception.Message)" -ForegroundColor Red
    }
}

Write-Host "`nTest completed!" -ForegroundColor Green
