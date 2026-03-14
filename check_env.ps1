Write-Host "Checking RL Trading Bot Prerequisites..." -ForegroundColor Cyan

# 1. Check Rust
if (Get-Command cargo -ErrorAction SilentlyContinue) {
    Write-Host "✅ Rust/Cargo detected" -ForegroundColor Green
    cargo --version
} else {
    Write-Host "❌ Rust not found. Install from rustup.rs" -ForegroundColor Red
}

# 2. Check VS Build Tools
$vcvars = "C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Auxiliary\Build\vcvars64.bat"
if (Test-Path $vcvars) {
    Write-Host "✅ VS Build Tools detected" -ForegroundColor Green
} else {
    Write-Host "❌ VS Build Tools NOT found at standard path." -ForegroundColor Red
    Write-Host "   Path checked: $vcvars" -ForegroundColor Gray
}

# 3. Check Windows SDK (kernel32.lib)
$kits10 = "C:\Program Files (x86)\Windows Kits\10\Lib"
if (Test-Path $kits10) {
    Write-Host "✅ Windows SDK Libs folder detected" -ForegroundColor Green
    
    $kernel32 = Get-ChildItem -Path $kits10 -Filter "kernel32.lib" -Recurse -ErrorAction SilentlyContinue | Select-Object -First 1
    if ($kernel32) {
        Write-Host "✅ kernel32.lib found at: $($kernel32.FullName)" -ForegroundColor Green
    } else {
        Write-Host "❌ kernel32.lib NOT found in Windows Kits folder." -ForegroundColor Red
        Write-Host "   Please Modify VS Build Tools -> Individual Components -> Windows 10 SDK" -ForegroundColor Yellow
    }
} else {
    Write-Host "❌ Windows Kits\10\Lib folder NOT found." -ForegroundColor Red
    Write-Host "   Is the Windows 10/11 SDK installed?" -ForegroundColor Yellow
}

Write-Host "`nTo fix missing components, run the Visual Studio Installer and modify 'Visual Studio Build Tools 2022'." -ForegroundColor Cyan
