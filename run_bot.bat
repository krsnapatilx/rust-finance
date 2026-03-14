@echo off
echo Initializing Visual Studio Build Environment...
call "C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Auxiliary\Build\vcvars64.bat"

if %ERRORLEVEL% NEQ 0 (
    echo Failed to initialize Visual Studio environment.
    exit /b %ERRORLEVEL%
)

echo Setting up build environment for Windows (OneDrive mitigation)...
set CARGO_TARGET_DIR=%TEMP%\rl-trading-bot-target
set CARGO_INCREMENTAL=0
set LIB=C:\Program Files (x86)\Windows Kits\10\Lib\10.0.26100.0\um\x64;C:\Program Files (x86)\Windows Kits\10\Lib\10.0.26100.0\ucrt\x64;%LIB%

set USE_MOCK=1
set RUST_LOG=info
set CARGO_INCREMENTAL=0

REM Explicitly call cargo with single job to be safe
"C:\Users\ashut\.cargo\bin\cargo.exe" run -p daemon --jobs 1
