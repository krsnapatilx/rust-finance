@echo off
echo Initializing Visual Studio Build Environment...
call "C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Auxiliary\Build\vcvars64.bat"

if %ERRORLEVEL% NEQ 0 (
    echo Failed to initialize Visual Studio environment.
    exit /b %ERRORLEVEL%
)

echo Forcing LIB environment variable to specific x64 SDK paths...
set "LIB=C:\Program Files (x86)\Windows Kits\10\Lib\10.0.26100.0\um\x64;C:\Program Files (x86)\Windows Kits\10\Lib\10.0.26100.0\ucrt\x64;%LIB%"

set CARGO_TARGET_DIR=%TEMP%\rl-trading-bot-target
set CARGO_INCREMENTAL=0

echo Killing existing tui instances...
taskkill /F /IM tui.exe /T 2>nul

echo Starting RL Trading Bot Dashboard (TUI)...
"C:\Users\ashut\.cargo\bin\cargo.exe" run -p tui
