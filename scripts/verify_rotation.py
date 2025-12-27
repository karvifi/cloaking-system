import requests
import time
import json
from datetime import datetime

# SOCKS5 Proxy Configuration
PROXIES = {
    'http': 'socks5h://127.0.0.1:9050',
    'https': 'socks5h://127.0.0.1:9050'
}

def verify_rapid_rotation():
    """
    Checks the exit IP every second to verify that the Aether Hyper-Rotation Engine 
    is successfully shifting the global identity.
    """
    print("🌌 Aether Hyper-Rotation Verifier")
    print("🛡️  Monitoring Global IP Diversity (1s Intervals)")
    print("-" * 50)
    
    seen_ips = set()
    
    try:
        for i in range(1, 21): # Test for 20 seconds
            try:
                # Use a fast IP lookup service
                response = requests.get('https://api.ipify.org?format=json', proxies=PROXIES, timeout=5)
                ip_data = response.json()
                current_ip = ip_data.get('ip')
                
                timestamp = datetime.now().strftime("%H:%M:%S")
                status = "NEW" if current_ip not in seen_ips else "STALE"
                seen_ips.add(current_ip)
                
                print(f"[{timestamp}] Shift #{i}: {current_ip} ({status})")
                
            except Exception as e:
                print(f"[{datetime.now().strftime('%H:%M:%S')}] Shift #{i}: [CLOAKED/ROUTING]")
            
            time.sleep(1)
            
    except KeyboardInterrupt:
        print("\nVerification stopped by user.")
        
    print("-" * 50)
    print(f"✅ Rotation Test Complete. Unique Exit Points Detected: {len(seen_ips)}")
    if len(seen_ips) > 5:
        print("🚀 VERDICT: Hyper-Rotation Engine is STABLE and ACTIVE.")
    else:
        print("⚠️  VERDICT: Low entropy detected. Ensure Aether is running with 'full' features.")

if __name__ == "__main__":
    verify_rapid_rotation()
