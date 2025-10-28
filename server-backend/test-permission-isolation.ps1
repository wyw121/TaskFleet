# 权限隔离测试脚本
# 测试不同角色用户看到的数据是否正确隔离

Write-Host "=== TaskFleet 权限隔离测试 ===" -ForegroundColor Cyan
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
Write-Host "✅ admin 登录成功，Token: $($adminToken.Substring(0, 20))..." -ForegroundColor Green

# 获取用户列表
$adminUsers = Invoke-RestMethod -Uri "$baseUrl/api/v1/users" -Method Get -Headers @{ Authorization = "Bearer $adminToken" }
Write-Host "admin 看到的用户数: $($adminUsers.data.Count)" -ForegroundColor Cyan
$adminUsers.data | ForEach-Object {
    Write-Host "  - $($_.username) (role: $($_.role), parent_id: $($_.parent_id))" -ForegroundColor Gray
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
Write-Host "✅ company_admin_1 登录成功，Token: $($companyAdmin1Token.Substring(0, 20))..." -ForegroundColor Green

# 获取用户列表
$companyAdmin1Users = Invoke-RestMethod -Uri "$baseUrl/api/v1/users" -Method Get -Headers @{ Authorization = "Bearer $companyAdmin1Token" }
Write-Host "company_admin_1 看到的用户数: $($companyAdmin1Users.data.Count)" -ForegroundColor Cyan
$companyAdmin1Users.data | ForEach-Object {
    Write-Host "  - $($_.username) (role: $($_.role), parent_id: $($_.parent_id))" -ForegroundColor Gray
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
Write-Host "✅ company_admin_2 登录成功，Token: $($companyAdmin2Token.Substring(0, 20))..." -ForegroundColor Green

# 获取用户列表
$companyAdmin2Users = Invoke-RestMethod -Uri "$baseUrl/api/v1/users" -Method Get -Headers @{ Authorization = "Bearer $companyAdmin2Token" }
Write-Host "company_admin_2 看到的用户数: $($companyAdmin2Users.data.Count)" -ForegroundColor Cyan
$companyAdmin2Users.data | ForEach-Object {
    Write-Host "  - $($_.username) (role: $($_.role), parent_id: $($_.parent_id))" -ForegroundColor Gray
}
Write-Host ""

# 测试4: employee_1 登录 (普通员工)
Write-Host "测试4: employee_1 (普通员工) 登录" -ForegroundColor Yellow
$employee1Login = @{
    username = "employee_1"
    password = "employee123"
} | ConvertTo-Json

$employee1Response = Invoke-RestMethod -Uri "$baseUrl/api/v1/auth/login" -Method Post -Body $employee1Login -ContentType "application/json"
$employee1Token = $employee1Response.data.token
Write-Host "✅ employee_1 登录成功，Token: $($employee1Token.Substring(0, 20))..." -ForegroundColor Green

# 尝试获取用户列表(应该被拒绝)
try {
    $employee1Users = Invoke-RestMethod -Uri "$baseUrl/api/v1/users" -Method Get -Headers @{ Authorization = "Bearer $employee1Token" }
    Write-Host "❌ 错误: employee_1 不应该能访问用户列表!" -ForegroundColor Red
} catch {
    Write-Host "✅ employee_1 被正确拒绝访问用户列表" -ForegroundColor Green
    Write-Host "   错误: $($_.Exception.Message)" -ForegroundColor Gray
}
Write-Host ""

# 总结
Write-Host "=== 测试结果总结 ===" -ForegroundColor Cyan
Write-Host "✅ admin 看到 $($adminUsers.data.Count) 个用户 (应该是 6)" -ForegroundColor $(if ($adminUsers.data.Count -eq 6) { "Green" } else { "Red" })
Write-Host "✅ company_admin_1 看到 $($companyAdmin1Users.data.Count) 个用户 (应该是 3: 自己+2员工)" -ForegroundColor $(if ($companyAdmin1Users.data.Count -eq 3) { "Green" } else { "Red" })
Write-Host "✅ company_admin_2 看到 $($companyAdmin2Users.data.Count) 个用户 (应该是 2: 自己+1员工)" -ForegroundColor $(if ($companyAdmin2Users.data.Count -eq 2) { "Green" } else { "Red" })
Write-Host "✅ employee_1 无法访问用户列表 (正确)" -ForegroundColor Green
Write-Host ""

if ($adminUsers.data.Count -eq 6 -and $companyAdmin1Users.data.Count -eq 3 -and $companyAdmin2Users.data.Count -eq 2) {
    Write-Host "🎉 权限隔离测试全部通过!" -ForegroundColor Green
} else {
    Write-Host "⚠️ 权限隔离测试存在问题，请检查!" -ForegroundColor Red
}
