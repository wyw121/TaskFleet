# TaskFleet 前端权限验证测试脚本
# 测试所有角色的登录和API访问权限

$baseUrl = "http://localhost:8000"
$testResults = @()

Write-Host "=====================================" -ForegroundColor Cyan
Write-Host "TaskFleet 前端权限验证测试" -ForegroundColor Cyan
Write-Host "=====================================" -ForegroundColor Cyan
Write-Host ""

# 测试用户列表
$testUsers = @(
    @{
        Username = "admin"
        Password = "admin123"
        ExpectedRole = "platform_admin"
        RoleName = "平台管理员 (PlatformAdmin)"
    },
    @{
        Username = "company_admin_1"
        Password = "admin123"
        ExpectedRole = "project_manager"
        RoleName = "项目经理 (ProjectManager)"
    },
    @{
        Username = "employee_1"
        Password = "admin123"
        ExpectedRole = "task_executor"
        RoleName = "任务执行者 (TaskExecutor)"
    }
)

function Test-UserLogin {
    param(
        [string]$Username,
        [string]$Password,
        [string]$ExpectedRole,
        [string]$RoleName
    )

    Write-Host "----------------------------------------" -ForegroundColor Yellow
    Write-Host "测试用户: $Username ($RoleName)" -ForegroundColor Yellow
    Write-Host "----------------------------------------" -ForegroundColor Yellow

    $result = @{
        Username = $Username
        RoleName = $RoleName
        LoginSuccess = $false
        TokenValid = $false
        MeEndpointSuccess = $false
        ActualRole = ""
        Errors = @()
    }

    try {
        # 1. 测试登录
        Write-Host "  [1/3] 测试登录..." -NoNewline
        $loginBody = @{
            username = $Username
            password = $Password
        } | ConvertTo-Json

        $loginResponse = Invoke-RestMethod -Uri "$baseUrl/api/v1/auth/login" `
            -Method POST `
            -ContentType "application/json" `
            -Body $loginBody `
            -ErrorAction Stop

        if ($loginResponse.success -and $loginResponse.data.token) {
            $result.LoginSuccess = $true
            $result.ActualRole = $loginResponse.data.user.role
            $token = $loginResponse.data.token
            Write-Host " ✅ 成功" -ForegroundColor Green
            Write-Host "     Token: $($token.Substring(0, 50))..." -ForegroundColor Gray
            Write-Host "     角色: $($loginResponse.data.user.role)" -ForegroundColor Gray
        } else {
            Write-Host " ❌ 失败 - 响应格式错误" -ForegroundColor Red
            $result.Errors += "登录响应格式错误"
            return $result
        }

        # 2. 验证角色
        Write-Host "  [2/3] 验证角色..." -NoNewline
        if ($result.ActualRole -eq $ExpectedRole) {
            Write-Host " ✅ 正确 ($ExpectedRole)" -ForegroundColor Green
            $result.TokenValid = $true
        } else {
            Write-Host " ❌ 错误 (期望: $ExpectedRole, 实际: $($result.ActualRole))" -ForegroundColor Red
            $result.Errors += "角色不匹配"
        }

        # 3. 测试 /api/v1/auth/me 端点
        Write-Host "  [3/3] 测试 /me 端点..." -NoNewline
        $headers = @{
            "Authorization" = "Bearer $token"
        }

        $meResponse = Invoke-RestMethod -Uri "$baseUrl/api/v1/auth/me" `
            -Method GET `
            -Headers $headers `
            -ErrorAction Stop

        if ($meResponse.success -and $meResponse.data) {
            $result.MeEndpointSuccess = $true
            Write-Host " ✅ 成功" -ForegroundColor Green
            Write-Host "     用户ID: $($meResponse.data.id)" -ForegroundColor Gray
            Write-Host "     用户名: $($meResponse.data.username)" -ForegroundColor Gray
            Write-Host "     角色: $($meResponse.data.role)" -ForegroundColor Gray
            
            # 验证角色一致性
            if ($meResponse.data.role -ne $result.ActualRole) {
                Write-Host "     ⚠️  警告: /me端点返回的角色与登录时不一致!" -ForegroundColor Yellow
                $result.Errors += "/me端点角色不一致"
            }
        } else {
            Write-Host " ❌ 失败 - 响应格式错误" -ForegroundColor Red
            $result.Errors += "/me端点响应格式错误"
        }

    } catch {
        Write-Host " ❌ 失败" -ForegroundColor Red
        Write-Host "     错误: $($_.Exception.Message)" -ForegroundColor Red
        if ($_.ErrorDetails) {
            Write-Host "     详情: $($_.ErrorDetails.Message)" -ForegroundColor Red
        }
        $result.Errors += $_.Exception.Message
    }

    Write-Host ""
    return $result
}

# 执行测试
foreach ($user in $testUsers) {
    $result = Test-UserLogin -Username $user.Username `
                              -Password $user.Password `
                              -ExpectedRole $user.ExpectedRole `
                              -RoleName $user.RoleName
    $testResults += $result
}

# 生成测试报告
Write-Host "=====================================" -ForegroundColor Cyan
Write-Host "测试结果汇总" -ForegroundColor Cyan
Write-Host "=====================================" -ForegroundColor Cyan
Write-Host ""

$totalTests = $testResults.Count
$passedLogins = ($testResults | Where-Object { $_.LoginSuccess }).Count
$passedRoles = ($testResults | Where-Object { $_.TokenValid }).Count
$passedMe = ($testResults | Where-Object { $_.MeEndpointSuccess }).Count

Write-Host "总测试用户数: $totalTests" -ForegroundColor White
Write-Host "登录成功: $passedLogins / $totalTests" -ForegroundColor $(if ($passedLogins -eq $totalTests) { "Green" } else { "Red" })
Write-Host "角色验证通过: $passedRoles / $totalTests" -ForegroundColor $(if ($passedRoles -eq $totalTests) { "Green" } else { "Red" })
Write-Host "/me端点成功: $passedMe / $totalTests" -ForegroundColor $(if ($passedMe -eq $totalTests) { "Green" } else { "Red" })
Write-Host ""

# 详细结果表
Write-Host "详细结果:" -ForegroundColor Cyan
Write-Host ""
Write-Host "用户名            角色              登录    角色验证  /me端点  错误" -ForegroundColor White
Write-Host "----------------  ----------------  ------  --------  -------  ----" -ForegroundColor White

foreach ($result in $testResults) {
    $loginMark = if ($result.LoginSuccess) { "✅" } else { "❌" }
    $roleMark = if ($result.TokenValid) { "✅" } else { "❌" }
    $meMark = if ($result.MeEndpointSuccess) { "✅" } else { "❌" }
    $errors = if ($result.Errors.Count -gt 0) { $result.Errors -join ", " } else { "-" }
    
    $color = if ($result.LoginSuccess -and $result.TokenValid -and $result.MeEndpointSuccess) {
        "Green"
    } else {
        "Red"
    }
    
    Write-Host ("{0,-16}  {1,-16}  {2,-6}  {3,-8}  {4,-7}  {5}" -f `
        $result.Username, `
        $result.ActualRole, `
        $loginMark, `
        $roleMark, `
        $meMark, `
        $errors) -ForegroundColor $color
}

Write-Host ""

# 最终结论
if ($passedLogins -eq $totalTests -and $passedRoles -eq $totalTests -and $passedMe -eq $totalTests) {
    Write-Host "=====================================" -ForegroundColor Green
    Write-Host "✅ 所有测试通过！系统权限正常" -ForegroundColor Green
    Write-Host "=====================================" -ForegroundColor Green
    exit 0
} else {
    Write-Host "=====================================" -ForegroundColor Red
    Write-Host "❌ 部分测试失败，请检查错误详情" -ForegroundColor Red
    Write-Host "=====================================" -ForegroundColor Red
    exit 1
}
