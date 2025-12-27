#!/bin/bash
# Aether Network - Stealth Browser Launcher
# 
# This script launches a browser configured to use the Aether SOCKS5 Gateway.
# Requires: Google Chrome or Firefox

SOCKS_PROXY="127.0.0.1:9050"

echo "🌌 Launching Stealth Browser via Aether Network..."
echo "🛡️  Traffic will be routed through the 5-layer Post-Quantum Mixnet."

# For Chrome
if command -v google-chrome &> /dev/null; then
    google-chrome --proxy-server="socks5://$SOCKS_PROXY" --incognito "https://ifconfig.me"
# For Firefox
elif command -v firefox &> /dev/null; then
    firefox --private-window -proxy-type socks -proxy-host 127.0.0.1 -proxy-port 9050 "https://ifconfig.me"
else
    echo "❌ No supported browser found. Please manually configure your proxy to $SOCKS_PROXY"
fi
