@echo off
chcp 65001 >nul
title Generate Icons

set "ROOT=%~dp0"
if "%ROOT:~-1%"=="\" set "ROOT=%ROOT:~0,-1%"
set "SRC=%ROOT%\rust\src-tauri"

cd /d "%SRC%"

echo.
echo ========================================
echo    Generate Icons from source.png
echo ========================================
echo.

npx @tauri-apps/cli icon icons/source.png

echo.
echo Done. Icons generated in icons/
echo.
pause