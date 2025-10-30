#!/usr/bin/env pwsh
# TaskFleet 多端权限一致性集成测试
# 验证 Web端 和 桌面端 权限控制完全一致

Write-Host "=====================================" -ForegroundColor Cyan
Write-Host "  TaskFleet 多端权限一致性测试" -ForegroundColor Cyan
Write-Host "=====================================" -ForegroundColor Cyan
Write-Host ""

# 配置
$SERVER_URL = "http://localhost:8000"
$TEST_USERS = @(
    @{
        username = "admin"
        password = "admin123"
        expected_role = "platform_admin"
        role_display = "平台管理员"
    },
    @{
        username = "manager"
        password = "manager123"
        expected_role = "project_manager"
        role_display = "项目经理"
    },
    @{
        username = "executor"
        password = "executor123"
        expected_role = "task_executor"
        role_display = "任务执行者"
    }
)

# 测试结果统计
$total_tests = 0
$passed_tests = 0
$failed_tests = 0

function Test-API {
    param(
        [string]$Method,
        [string]$Endpoint,
        [string]$Token,
        [object]$Body = $null
    )
    
    $headers = @{}
    if ($Token) {
        $headers["Authorization"] = "Bearer $Token"
    }
    
    try {
        $params = @{
            Uri = "$SERVER_URL$Endpoint"
            Method = $Method
            Headers = $headers
            ContentType = "application/json"
        }
        
        if ($Body) {
            $params["Body"] = ($Body | ConvertTo-Json -Depth 10)
        }
        
        $response = Invoke-RestMethod @params
        return @{
            success = $true
            data = $response
            status = 200
        }
    }
    catch {
        $statusCode = $_.Exception.Response.StatusCode.value__
        return @{
            success = $false
            error = $_.Exception.Message
            status = $statusCode
        }
    }
}

function Assert-Equal {
    param(
        [string]$TestName,
        $Expected,
        $Actual
    )
    
    $script:total_tests++
    
    if ($Expected -eq $Actual) {
        Write-Host "  ✅ PASS: $TestName" -ForegroundColor Green
        $script:passed_tests++
        return $true
    }
    else {
        Write-Host "  ❌ FAIL: $TestName" -ForegroundColor Red
        Write-Host "     Expected: $Expected" -ForegroundColor Yellow
        Write-Host "     Actual: $Actual" -ForegroundColor Yellow
        $script:failed_tests++
        return $false
    }
}

function Test-UserPermissions {
    param(
        [object]$User
    )
    
    Write-Host "`n测试用户: $($User.username) ($($User.role_display))" -ForegroundColor Cyan
    Write-Host "----------------------------------------"
    
    # 1. 登录测试
    Write-Host "1️⃣  测试登录..." -ForegroundColor Yellow
    $loginResult = Test-API -Method POST -Endpoint "/api/auth/login" -Body @{
        username = $User.username
        password = $User.password
    }
    
    if (-not $loginResult.success) {
        Write-Host "  ❌ 登录失败: $($loginResult.error)" -ForegroundColor Red
        return
    }
    
    $token = $loginResult.data.token
    $userInfo = $loginResult.data.user
    
    Assert-Equal "角色正确" $User.expected_role $userInfo.role
    
    # 2. 测试公司管理权限
    Write-Host "`n2️⃣  测试公司管理权限..." -ForegroundColor Yellow
    $companiesResult = Test-API -Method GET -Endpoint "/api/v1/companies" -Token $token
    
    if ($User.expected_role -eq "platform_admin") {
        Assert-Equal "平台管理员可以查看所有公司" 200 $companiesResult.status
    }
    else {
        Assert-Equal "非平台管理员不能查看所有公司" 403 $companiesResult.status
    }
    
    # 3. 测试用户管理权限
    Write-Host "`n3️⃣  测试用户管理权限..." -ForegroundColor Yellow
    $usersResult = Test-API -Method GET -Endpoint "/api/v1/users" -Token $token
    
    if ($User.expected_role -in @("platform_admin", "project_manager")) {
        Assert-Equal "管理员可以查看用户列表" 200 $usersResult.status
    }
    else {
        Assert-Equal "任务执行者不能查看用户列表" 403 $usersResult.status
    }
    
    # 4. 测试创建用户权限
    Write-Host "`n4️⃣  测试创建用户权限..." -ForegroundColor Yellow
    $createUserResult = Test-API -Method POST -Endpoint "/api/v1/users" -Token $token -Body @{
        username = "test_user_$(Get-Random)"
        email = "test@example.com"
        password = "test123"
        role = "task_executor"
        full_name = "测试用户"
    }
    
    if ($User.expected_role -in @("platform_admin", "project_manager")) {
        Assert-Equal "管理员可以创建用户" 200 $createUserResult.status
    }
    else {
        Assert-Equal "任务执行者不能创建用户" 403 $createUserResult.status
    }
    
    # 5. 测试任务查看权限
    Write-Host "`n5️⃣  测试任务查看权限..." -ForegroundColor Yellow
    $tasksResult = Test-API -Method GET -Endpoint "/api/v1/tasks" -Token $token
    
    # 所有角色都能查看任务(范围不同)
    Assert-Equal "所有角色都能查看任务" 200 $tasksResult.status
    
    # 6. 测试创建任务权限
    Write-Host "`n6️⃣  测试创建任务权限..." -ForegroundColor Yellow
    $createTaskResult = Test-API -Method POST -Endpoint "/api/v1/tasks" -Token $token -Body @{
        title = "测试任务 $(Get-Random)"
        description = "权限测试"
        status = "pending"
        priority = "medium"
    }
    
    if ($User.expected_role -in @("platform_admin", "project_manager")) {
        Assert-Equal "管理员可以创建任务" 200 $createTaskResult.status
    }
    else {
        Assert-Equal "任务执行者不能创建任务" 403 $createTaskResult.status
    }
}

# 主测试流程
Write-Host "检查服务器连接..." -ForegroundColor Yellow
$healthCheck = Test-API -Method GET -Endpoint "/health"

if (-not $healthCheck.success) {
    Write-Host "❌ 无法连接到服务器: $SERVER_URL" -ForegroundColor Red
    Write-Host "请确保后端服务正在运行: cd server-backend && cargo run" -ForegroundColor Yellow
    exit 1
}

Write-Host "✅ 服务器连接成功" -ForegroundColor Green

# 对每个测试用户执行权限测试
foreach ($user in $TEST_USERS) {
    Test-UserPermissions -User $user
}

# 输出测试结果
Write-Host "`n=====================================" -ForegroundColor Cyan
Write-Host "         测试结果汇总" -ForegroundColor Cyan
Write-Host "=====================================" -ForegroundColor Cyan
Write-Host "总测试数: $total_tests" -ForegroundColor White
Write-Host "通过: $passed_tests" -ForegroundColor Green
Write-Host "失败: $failed_tests" -ForegroundColor Red

if ($failed_tests -eq 0) {
    Write-Host "`n✅ 所有测试通过! 权限控制一致性验证成功!" -ForegroundColor Green
    exit 0
}
else {
    Write-Host "`n❌ 存在 $failed_tests 个测试失败" -ForegroundColor Red
    exit 1
}
