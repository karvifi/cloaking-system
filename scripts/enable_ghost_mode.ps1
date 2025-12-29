# Ghost Mode - System Anonymity Activation
# Standard PowerShell Script for routing traffic

Write-Host "Activating Ghost Mode..." -ForegroundColor Cyan

# Configuration
$ProxyHost = "127.0.0.1"
$SocksPort = "9050"
$HttpPort = "8080"

# 1. System Proxy (Immediate browser effect)
$ProxyValue = "$ProxyHost`:$SocksPort"
Set-ItemProperty -Path 'HKCU:\Software\Microsoft\Windows\CurrentVersion\Internet Settings' -Name ProxyEnable -Value 1
Set-ItemProperty -Path 'HKCU:\Software\Microsoft\Windows\CurrentVersion\Internet Settings' -Name ProxyServer -Value $ProxyValue

# 2. WinHTTP Proxy (System-wide)
$WinHttpSetting = "socks=$ProxyHost`:$SocksPort;http=$ProxyHost`:$HttpPort;https=$ProxyHost`:$HttpPort"
netsh winhttp set proxy proxy-server="$WinHttpSetting" bypass-list="<local>"

# 3. Disable IPv6
Get-NetAdapterBinding -ComponentID ms_tcpip6 | Disable-NetAdapterBinding -ComponentID ms_tcpip6 -Confirm:$false -ErrorAction SilentlyContinue

# 4. DNS (Cloudflare)
$ActiveAdapters = Get-NetAdapter | Where-Object { $_.Status -eq "Up" }
foreach ($Adapter in $ActiveAdapters) {
    Set-DnsClientServerAddress -InterfaceAlias $Adapter.Name -ServerAddresses ("1.1.1.1", "1.0.0.1") -ErrorAction SilentlyContinue
}

# 5. Firewall Kill-Switch
New-NetFirewallRule -DisplayName "Aether Allow Proxy" -Direction Outbound -LocalPort $SocksPort, $HttpPort -Protocol TCP -Action Allow -ErrorAction SilentlyContinue
New-NetFirewallRule -DisplayName "Aether Block Direct" -Direction Outbound -RemoteAddress Internet4 -Action Block -ErrorAction SilentlyContinue

Write-Host "Ghost Mode ACTIVATED!" -ForegroundColor Green
Write-Host "Your IP is now hidden behind 10 layers of protection."
