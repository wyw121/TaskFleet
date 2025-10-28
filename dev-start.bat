@echo off
REM TaskFleet 开发环境快速启动脚本 (Windows)

echo ========================================
echo   TaskFleet 开发环境启动脚本
echo ========================================
echo.

REM 检查是否在正确的目录
if not exist "TaskFleet.code-workspace" (
    echo [错误] 请在TaskFleet项目根目录运行此脚本
    pause
    exit /b 1
)

echo [1/2] 启动后端服务器 (Rust + Axum)...
start "TaskFleet Backend" cmd /k "cd server-backend && cargo run --release"
echo       后端API: http://localhost:8000
echo.

echo [2/2] 启动前端开发服务器 (Vite + React)...
timeout /t 3 /nobreak >nul
start "TaskFleet Frontend" cmd /k "cd server-frontend && npm run dev"
echo       前端界面: http://localhost:5173
echo.

echo ========================================
echo   TaskFleet 开发环境已启动!
echo ========================================
echo.
echo 后端API:  http://localhost:8000
echo 前端界面: http://localhost:5173
echo.
echo 提示: 关闭窗口即可停止对应的服务
echo.
pause