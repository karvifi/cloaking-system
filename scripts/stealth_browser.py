import undetected_chromedriver as uc
import time
import sys

def launch_stealth_browser():
    """
    Launches an undetected Chrome browser routed through the Aether SOCKS5 Gateway.
    """
    print("🚀 Initializing Undetected Stealth Browser...")
    print("🛡️  Configuration: SOCKS5 @ 127.0.0.1:9050")
    
    options = uc.ChromeOptions()
    options.add_argument('--proxy-server=socks5://127.0.0.1:9050')
    options.add_argument('--incognito')
    
    try:
        driver = uc.Chrome(options=options)
        driver.get('https://ipleak.net')
        
        print("✅ Browser active and cloaked.")
        print("💡 Check 'Your IP Address' - it should be hidden or the proxy IP.")
        
        # Keep alive
        while True:
            time.sleep(10)
    except Exception as e:
        print(f"❌ Error: {e}")
        print("💡 Make sure Aether Network is running on :9050")

if __name__ == "__main__":
    launch_stealth_browser()
