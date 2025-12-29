# Tor Setup Script for Windows
# Downloads and configures Tor for the Aether Network

$ErrorActionPreference = "Continue"

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "   TOR SETUP FOR AETHER NETWORK" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

# Check if Tor is already installed
$torPaths = @(
    "C:\Tor Browser\Browser\TorBrowser\Tor\tor.exe",
    "C:\Program Files\Tor Browser\Browser\TorBrowser\Tor\tor.exe",
    "C:\Tor\Tor\tor.exe"
)

$torFound = $false
foreach ($path in $torPaths) {
    if (Test-Path $path) {
        Write-Host "[OK] Tor found at: $path" -ForegroundColor Green
        $torFound = $true
        break
    }
}

if (!$torFound) {
    Write-Host "[!] Tor not found. Installing..." -ForegroundColor Yellow
    Write-Host ""
    
    # Download Tor Expert Bundle
    $torUrl = "https://archive.torproject.org/tor-package-archive/torbrowser/13.0.8/tor-expert-bundle-windows-x86_64-13.0.8.tar.gz"
    $downloadPath = "$env:TEMP\tor.tar.gz"
    
    Write-Host "Downloading Tor from: $torUrl"
    try {
        Invoke-WebRequest -Uri $torUrl -OutFile $downloadPath -UseBasicParsing
        Write-Host "[OK] Downloaded" -ForegroundColor Green
    }
    catch {
        Write-Host "[ERROR] Download failed: $_" -ForegroundColor Red
        Write-Host ""
        Write-Host "ALTERNATIVE: Install Tor Browser manually" -ForegroundColor Yellow
        Write-Host "Download from: https://www.torproject.org/download/"
        exit 1
    }
    
    # Extract
    Write-Host "Extracting..."
    New-Item -ItemType Directory -Path "C:\Tor" -Force | Out-Null
    
    # Use 7zip or tar if available
    if (Get-Command tar -ErrorAction SilentlyContinue) {
        tar -xzf $downloadPath -C "C:\Tor"
        Write-Host "[OK] Extracted" -ForegroundColor Green
    }
    else {
        Write-Host "[!] Please extract $downloadPath to C:\Tor manually" -ForegroundColor Yellow
        Write-Host "    Then run this script again"
        exit 1
    }
}

# Create Tor configuration
Write-Host ""
Write-Host "Configuring Tor..." -ForegroundColor Cyan
$torrcPath = "C:\Tor\torrc"
$torrcContent = @"
# Aether Network Tor Configuration
SocksPort 9150
ControlPort 9151
DataDirectory C:\Tor\Data
Log notice file C:\Tor\tor.log

# Performance
NumEntryGuards 8
CircuitBuildTimeout 30

# Anonymity
EnforceDistinctSubnets 1
"@

Set-Content -Path $torrcPath -Value $torrcContent -Force
Write-Host "[OK] Configuration created" -ForegroundColor Green

# Start Tor
Write-Host ""
Write-Host "Starting Tor..." -ForegroundColor Cyan

$torExe = "C:\Tor\Tor\tor.exe"
if (!(Test-Path $torExe)) {
    $torExe = "C:\Tor\tor.exe"
}

if (Test-Path $torExe) {
    # Kill any existing Tor processes
    Get-Process tor -ErrorAction SilentlyContinue | Stop-Process -Force
    
    # Start Tor in background
    Start-Process -FilePath $torExe -ArgumentList "-f `"$torrcPath`"" -WindowStyle Hidden
    
    Write-Host "[OK] Tor starting..." -ForegroundColor Green
    Write-Host "    Waiting for circuit to establish (15s)..."
    Start-Sleep -Seconds 15
    
    # Test connection
    try {
        $testConnection = Test-NetConnection -ComputerName 127.0.0.1 -Port 9150 -WarningAction SilentlyContinue
        if ($testConnection.TcpTestSucceeded) {
            Write-Host "[OK] Tor is running on 127.0.0.1:9150" -ForegroundColor Green
        }
        else {
            Write-Host "[!] Tor may not be ready yet. Check C:\Tor\tor.log" -ForegroundColor Yellow
        }
    }
    catch {
        Write-Host "[!] Could not verify Tor status" -ForegroundColor Yellow
    }
}
else {
    Write-Host "[ERROR] Tor executable not found at $torExe" -ForegroundColor Red
    exit 1
}

Write-Host ""
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "   TOR SETUP COMPLETE" -ForegroundColor Green
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""
Write-Host "Next steps:"
Write-Host "1. Run: cargo run --release --bin verified_10_layer"
Write-Host "2. Test: curl --proxy socks5h://127.0.0.1:9050 https://check.torproject.org"
Write-Host ""
