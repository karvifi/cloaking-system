# AETHER SUPREME - ADMIN LAUNCHER (Auto-Elevated)
# Automatically requests administrator privileges

$ScriptPath = $PSScriptRoot
$LauncherScript = Join-Path $ScriptPath "AETHER_GOD_MODE_LAUNCHER.ps1"

# Check if running as administrator
$currentPrincipal = New-Object Security.Principal.WindowsPrincipal([Security.Principal.WindowsIdentity]::GetCurrent())
$isAdmin = $currentPrincipal.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)

if (-not $isAdmin) {
    Write-Host "[ADMIN REQUIRED] Requesting administrator privileges..." -ForegroundColor Yellow
    Write-Host "[ELEVATION] This is required for WinHTTP system-level services" -ForegroundColor Cyan
    
    # Relaunch as administrator
    $arguments = "-NoProfile -ExecutionPolicy Bypass -File `"$LauncherScript`""
    Start-Process powershell.exe -Verb RunAs -ArgumentList $arguments
    
    Write-Host "[OK] Relaunching with admin privileges..." -ForegroundColor Green
    exit
}

# Already running as admin
Write-Host "========================================" -ForegroundColor Green
Write-Host "   AETHER SUPREME - ADMIN MODE ACTIVE" -ForegroundColor Green
Write-Host "========================================" -ForegroundColor Green
Write-Host ""
Write-Host "[ADMIN] Running with administrator privileges" -ForegroundColor Green
Write-Host "[ADMIN] WinHTTP system-level services accessible" -ForegroundColor Green
Write-Host ""

# Enable WinHTTP system proxy settings
Write-Host "[WINHTTP] Configuring system-level proxy..." -ForegroundColor Cyan
try {
    # Set system proxy to use Tor SOCKS5
    netsh winhttp set proxy proxy-server="socks=127.0.0.1:9050" bypass-list="<local>"
    Write-Host "[OK] WinHTTP proxy configured: 127.0.0.1:9050" -ForegroundColor Green
}
catch {
    Write-Host "[WARNING] Could not configure WinHTTP proxy: $_" -ForegroundColor Yellow
}

Write-Host ""

# Launch the main Aether system
& $LauncherScript
