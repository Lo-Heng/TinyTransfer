@echo off
chcp 65001 >nul
title Slim Transfer - Tauri Build

cd /d "%~dp0rust\src-tauri"

echo.
echo ╔═══════════════════════════════════════════════════════════╗
echo ║           Slim Transfer - Tauri 打包                        ║
echo ╚═══════════════════════════════════════════════════════════╝
echo.

:: 设置输出目录为项目根目录下的 output/
set TAURI_BUNDLE_OUTPUT_DIR=..\..\output

echo [*] 开始构建 Release 版本...
echo [*] 输出目录: %TAURI_BUNDLE_OUTPUT_DIR%
echo.

cargo tauri build

if errorlevel 1 (
    echo.
    echo [错误] 构建失败！
    pause
    exit /b 1
)

echo.
echo ╔═══════════════════════════════════════════════════════════╗
echo ║  构建成功！                                                ║
echo ║                                                            ║
echo ║  安装包位置: ..\output\                                    ║
echo ╚═══════════════════════════════════════════════════════════╝
echo.
pause
