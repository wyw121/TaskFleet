@echo off
cd /d "%~dp0"
echo 🚀 启动TaskFleet后端服务器...
echo.
cargo run --release
pause
