# Aether Network: Ghost Mode Activator (Non-Admin / Current User)
# Configures proxy settings for the current user only.

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "   AETHER GHOST MODE (NON-ADMIN)" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

$SocksProxy = "127.0.0.1:9050"
$HttpProxy = "127.0.0.1:8080"
$BypassList = "<local>;*.local;127.0.0.1;localhost"

Write-Host "[*] Applying proxy settings to HKCU registry..." -ForegroundColor Cyan

try {
    # Internet Settings
    $registryPath = "HKCU:\Software\Microsoft\Windows\CurrentVersion\Internet Settings"
    
    Set-ItemProperty -Path $registryPath -Name ProxyEnable -Value 1 -ErrorAction Stop
    # Format: http=127.0.0.1:8080;https=127.0.0.1:8080;socks=127.0.0.1:9050
    $proxyServer = "http=$HttpProxy;https=$HttpProxy;socks=$SocksProxy"
    Set-ItemProperty -Path $registryPath -Name ProxyServer -Value $proxyServer -ErrorAction Stop
    Set-ItemProperty -Path $registryPath -Name ProxyOverride -Value $BypassList -ErrorAction Stop
    
    Write-Host "[OK] Current user proxy configured." -ForegroundColor Green
    Write-Host "     SOCKS: $SocksProxy"
    Write-Host "     HTTP:  $HttpProxy"
}
catch {
    Write-Host "[ERROR] Failed to set registry keys: $_" -ForegroundColor Red
    exit 1
}

Write-Host ""
Write-Host "[!] NOTE: Browsers (Chrome/Edge/Brave) will now route through Aether." -ForegroundColor Yellow
Write-Host "[!] NOTE: This does NOT apply to system-level WinHTTP services (requires admin)." -ForegroundColor Yellow
Write-Host ""
Write-Host "AETHER GHOST SHIELD: [ACTIVE]" -ForegroundColor Green
Write-Host "========================================" -ForegroundColor Cyan
