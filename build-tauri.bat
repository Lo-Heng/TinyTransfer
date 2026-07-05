@echo off
chcp 65001 >nul
title Slim Transfer - Tauri Build

set "ROOT=%~dp0"
if "%ROOT:~-1%"=="\" set "ROOT=%ROOT:~0,-1%"
set "SRC=%ROOT%\rust\src-tauri"
set "OUT=%ROOT%\output"

:: 使用 GNU 工具链 + 项目自带 MinGW
set RUSTUP_TOOLCHAIN=stable-x86_64-pc-windows-gnu
set PATH=%ROOT%\tools\upx\mingw64\bin;%PATH%

cd /d "%SRC%"

echo.
echo ========================================
echo    Slim Transfer - Tauri Build
echo    Toolchain: GNU (MinGW built-in)
echo ========================================
echo.

:build
echo [1/5] Check TinyTransfer process...
tasklist /fi "imagename eq TinyTransfer.exe" 2>nul | find /i "TinyTransfer.exe" >nul
if %errorlevel%==0 (
    echo       Found, killing...
    taskkill /f /im TinyTransfer.exe >nul 2>&1
    timeout /t 2 >nul
    echo       Done
) else (
    echo       Not running, skip
)
echo.

echo [2/5] Create output dir...
if not exist "%OUT%" mkdir "%OUT%"
echo       OK
echo.

echo [3/5] Building Release...
echo       Please wait...
echo.

call npx @tauri-apps/cli build
if errorlevel 1 goto :build_failed
goto :build_ok

:build_failed
echo.
echo ========================================
echo   [ERROR] Build failed!
echo ========================================
echo.
echo Press any key to retry (Ctrl+C to exit)...
pause >nul
cls
goto :build

:build_ok
echo.
echo [4/5] Copy files to output...
set "TARGET=%ROOT%\rust\target\release"
copy /Y "%TARGET%\TinyTransfer.exe" "%OUT%\TinyTransfer.exe" >nul 2>&1
copy /Y "%TARGET%\WebView2Loader.dll" "%OUT%\WebView2Loader.dll" >nul 2>&1
echo       output\TinyTransfer.exe
echo       output\WebView2Loader.dll
echo.

echo [5/5] Launch app...
if not exist "%OUT%\TinyTransfer.exe" goto :no_exe
powershell -NoProfile -Command "Start-Process -FilePath '%OUT%\TinyTransfer.exe'"
echo       Launched
goto :done

:no_exe
echo       [WARN] TinyTransfer.exe not found
echo.

:done
echo.
echo ========================================
echo   Build Complete!
echo.
echo   Output: %OUT%\
echo   - TinyTransfer.exe
echo ========================================
echo.
echo Press any key to rebuild (Ctrl+C to exit)...
pause >nul
cls
goto :build
