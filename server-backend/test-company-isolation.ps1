# 第二阶段权限隔离测试脚本 - 使用 company_id
# 测试基于公司ID的数据隔离

Write-Host "=== TaskFleet 第二阶段权限隔离测试 (company_id) ===" -ForegroundColor Cyan
Write-Host ""

$baseUrl = "http://localhost:8000"

# 测试1: admin 登录 (系统管理员)
Write-Host "测试1: admin (系统管理员) 登录" -ForegroundColor Yellow
$adminLogin = @{
    username = "admin"
    password = "admin123"
} | ConvertTo-Json

$adminResponse = Invoke-RestMethod -Uri "$baseUrl/api/v1/auth/login" -Method Post -Body $adminLogin -ContentType "application/json"
$adminToken = $adminResponse.data.token
Write-Host "✅ admin 登录成功" -ForegroundColor Green

# 获取用户列表
$adminUsers = Invoke-RestMethod -Uri "$baseUrl/api/v1/users" -Method Get -Headers @{ Authorization = "Bearer $adminToken" }
Write-Host "admin 看到的用户数: $($adminUsers.data.Count)" -ForegroundColor Cyan
$adminUsers.data | ForEach-Object {
    $companyId = if ($_.company_id) { $_.company_id } else { "NULL" }
    Write-Host "  - $($_.username) (role: $($_.role), company_id: $companyId)" -ForegroundColor Gray
}
Write-Host ""

# 测试2: company_admin_1 登录 (公司A管理员)
Write-Host "测试2: company_admin_1 (公司A管理员) 登录" -ForegroundColor Yellow
$companyAdmin1Login = @{
    username = "company_admin_1"
    password = "admin123"
} | ConvertTo-Json

$companyAdmin1Response = Invoke-RestMethod -Uri "$baseUrl/api/v1/auth/login" -Method Post -Body $companyAdmin1Login -ContentType "application/json"
$companyAdmin1Token = $companyAdmin1Response.data.token
$companyAdmin1Info = $companyAdmin1Response.data.user
Write-Host "✅ company_admin_1 登录成功 (company_id: $($companyAdmin1Info.company_id))" -ForegroundColor Green

# 获取用户列表
$companyAdmin1Users = Invoke-RestMethod -Uri "$baseUrl/api/v1/users" -Method Get -Headers @{ Authorization = "Bearer $companyAdmin1Token" }
Write-Host "company_admin_1 看到的用户数: $($companyAdmin1Users.data.Count)" -ForegroundColor Cyan
$companyAdmin1Users.data | ForEach-Object {
    $companyId = if ($_.company_id) { $_.company_id } else { "NULL" }
    Write-Host "  - $($_.username) (role: $($_.role), company_id: $companyId)" -ForegroundColor Gray
}
Write-Host ""

# 测试3: company_admin_2 登录 (公司B管理员)
Write-Host "测试3: company_admin_2 (公司B管理员) 登录" -ForegroundColor Yellow
$companyAdmin2Login = @{
    username = "company_admin_2"
    password = "admin123"
} | ConvertTo-Json

$companyAdmin2Response = Invoke-RestMethod -Uri "$baseUrl/api/v1/auth/login" -Method Post -Body $companyAdmin2Login -ContentType "application/json"
$companyAdmin2Token = $companyAdmin2Response.data.token
$companyAdmin2Info = $companyAdmin2Response.data.user
Write-Host "✅ company_admin_2 登录成功 (company_id: $($companyAdmin2Info.company_id))" -ForegroundColor Green

# 获取用户列表
$companyAdmin2Users = Invoke-RestMethod -Uri "$baseUrl/api/v1/users" -Method Get -Headers @{ Authorization = "Bearer $companyAdmin2Token" }
Write-Host "company_admin_2 看到的用户数: $($companyAdmin2Users.data.Count)" -ForegroundColor Cyan
$companyAdmin2Users.data | ForEach-Object {
    $companyId = if ($_.company_id) { $_.company_id } else { "NULL" }
    Write-Host "  - $($_.username) (role: $($_.role), company_id: $companyId)" -ForegroundColor Gray
}
Write-Host ""

# 测试4: 验证数据隔离
Write-Host "=== 数据隔离验证 ===" -ForegroundColor Cyan

# 验证公司A的用户都有company_id=1
$companyAUsers = $companyAdmin1Users.data | Where-Object { $_.company_id -eq 1 }
$companyACount = $companyAUsers.Count
Write-Host "公司A用户 (company_id=1): $companyACount 个" -ForegroundColor $(if ($companyACount -eq 3) { "Green" } else { "Red" })

# 验证公司B的用户都有company_id=2
$companyBUsers = $companyAdmin2Users.data | Where-Object { $_.company_id -eq 2 }
$companyBCount = $companyBUsers.Count
Write-Host "公司B用户 (company_id=2): $companyBCount 个" -ForegroundColor $(if ($companyBCount -eq 2) { "Green" } else { "Red" })

# 验证没有跨公司数据泄露
$leakA = $companyAdmin1Users.data | Where-Object { $_.company_id -eq 2 }
$leakB = $companyAdmin2Users.data | Where-Object { $_.company_id -eq 1 }

if ($leakA.Count -eq 0 -and $leakB.Count -eq 0) {
    Write-Host "✅ 没有跨公司数据泄露" -ForegroundColor Green
} else {
    Write-Host "❌ 发现跨公司数据泄露!" -ForegroundColor Red
}
Write-Host ""

# 总结
Write-Host "=== 测试结果总结 ===" -ForegroundColor Cyan
Write-Host "✅ admin 看到 $($adminUsers.data.Count) 个用户 (应该是 6)" -ForegroundColor $(if ($adminUsers.data.Count -eq 6) { "Green" } else { "Red" })
Write-Host "✅ company_admin_1 看到 $($companyAdmin1Users.data.Count) 个用户 (应该是 3: 全部company_id=1)" -ForegroundColor $(if ($companyAdmin1Users.data.Count -eq 3) { "Green" } else { "Red" })
Write-Host "✅ company_admin_2 看到 $($companyAdmin2Users.data.Count) 个用户 (应该是 2: 全部company_id=2)" -ForegroundColor $(if ($companyAdmin2Users.data.Count -eq 2) { "Green" } else { "Red" })
Write-Host ""

if ($adminUsers.data.Count -eq 6 -and $companyAdmin1Users.data.Count -eq 3 -and $companyAdmin2Users.data.Count -eq 2 -and $leakA.Count -eq 0 -and $leakB.Count -eq 0) {
    Write-Host "🎉 第二阶段权限隔离测试全部通过! (基于company_id)" -ForegroundColor Green
    Write-Host "✅ 完整的多租户数据隔离已实现" -ForegroundColor Green
} else {
    Write-Host "⚠️ 权限隔离测试存在问题，请检查!" -ForegroundColor Red
}
