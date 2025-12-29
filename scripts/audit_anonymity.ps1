# Aether Network: Anonymity Audit Suite (Powered by BBOT)
# Performs automated adversarial reconnaissance to verify zero metadata leakage.

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "   AETHER ANONYMITY AUDIT (BBOT)" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

# 1. Get Current Aether Exit IP
Write-Host "[*] Fetching current Aether Exit IP..." -ForegroundColor Cyan
$exitIp = curl.exe --proxy socks5h://127.0.0.1:9050 -s https://api.ipify.org --connect-timeout 15

if (!$exitIp) {
    Write-Host "[ERROR] Could not reach Aether Proxy. Is verified_10_layer running?" -ForegroundColor Red
    exit 1
}

Write-Host "[OK] Auditing Exit IP: $exitIp" -ForegroundColor Green
Write-Host ""

# 2. Check for BBOT
if (!(Get-Command bbot -ErrorAction SilentlyContinue)) {
    Write-Host "[!] BBOT not found. Attempting to install via pip..." -ForegroundColor Yellow
    pip install bbot
    if (!$?) {
        Write-Host "[ERROR] BBOT installation failed. Please install manually: pip install bbot" -ForegroundColor Red
        exit 1
    }
}

# 3. Perform Adversarial Scan
Write-Host "[*] Starting Adversarial Reconnaissance (BBOT)..." -ForegroundColor Cyan
Write-Host "    Scanning for open ports, headers, and OS clues on $exitIp..."
Write-Host ""

# Run a focused bbot scan against the IP
# We use moderate flags to ensure speed while maintaining depth
bbot -t $exitIp -p ip_recon -y --output-dir ./audits/ --name "aether_audit_$(Get-Date -Format 'yyyyMMdd_HHmmss')"

Write-Host ""
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "   AUDIT COMPLETE" -ForegroundColor Green
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""
Write-Host "Check the ./audits/ directory for detailed leak reports."
Write-Host "If results show NO correlation to your home IP, Layer 11/12 are SUCCESSFUL."
