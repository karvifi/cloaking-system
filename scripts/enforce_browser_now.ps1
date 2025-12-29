# Immediate Browser Anonymity (User Level)
# Does NOT require admin - fixes the browser IP leak NOW

$SOCKS5_HOST = "127.0.0.1"
$SOCKS5_PORT = 9050

Write-Host "Enforcing immediate browser anonymity..." -ForegroundColor Cyan

# Set user-level proxy (for browsers)
Set-ItemProperty -Path 'HKCU:\Software\Microsoft\Windows\CurrentVersion\Internet Settings' -Name ProxyEnable -Value 1 -ErrorAction SilentlyContinue
Set-ItemProperty -Path 'HKCU:\Software\Microsoft\Windows\CurrentVersion\Internet Settings' -Name ProxyServer -Value "$SOCKS5_HOST`:$SOCKS5_PORT" -ErrorAction SilentlyContinue

Write-Host "✅ Browser proxy enabled (127.0.0.1:9050)" -ForegroundColor Green
Write-Host "Please RESTART your browser now to see the change." -ForegroundColor Yellow
