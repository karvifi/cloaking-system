# 🔓 Disable Ghost Mode - Restore Normal Network Settings
# Fixed for maximum compatibility

Write-Host "🔓 Deactivating Ghost Mode..." -ForegroundColor Yellow
Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor Yellow

# Check for admin privileges
if (-NOT ([Security.Principal.WindowsPrincipal][Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole] "Administrator")) {
    Write-Host "❌ This script requires Administrator privileges!" -ForegroundColor Red
    exit 1
}

Write-Host "📡 Removing proxy settings..." -ForegroundColor Green
netsh winhttp reset proxy

# Disable user-level proxy
Set-ItemProperty -Path 'HKCU:\Software\Microsoft\Windows\CurrentVersion\Internet Settings' -Name ProxyEnable -Value 0 -ErrorAction SilentlyContinue

Write-Host "🛡️  Re-enabling IPv6..." -ForegroundColor Green
Get-NetAdapterBinding -ComponentID ms_tcpip6 | Enable-NetAdapterBinding -ComponentID ms_tcpip6 -Confirm:$false -ErrorAction SilentlyContinue

Write-Host "🔐 Resetting DNS to automatic..." -ForegroundColor Green
# Use native cmdlet for better compatibility
Get-NetAdapter | Where-Object { $_.Status -eq "Up" } | Set-DnsClientServerAddress -ResetServerAddresses -ErrorAction SilentlyContinue

Write-Host "🔥 Removing firewall rules..." -ForegroundColor Green
Remove-NetFirewallRule -DisplayName "Aether*" -ErrorAction SilentlyContinue

Write-Host ""
Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor Yellow
Write-Host "✅ Ghost Mode DEACTIVATED!" -ForegroundColor Green
Write-Host "   Your network settings have been restored." -ForegroundColor White
Write-Host ""
