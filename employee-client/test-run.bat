@echo off
cd /d "%~dp0src-tauri"
echo Starting TaskFleet Employee Client...
cargo run 2>&1 | findstr /V "Compiling Finished"
pause
