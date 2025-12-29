"""
Proxy Pool Manager - Manages Multiple IP Rotation
Integrates with jhao104/proxy_pool for massive IP diversity
"""

import sys
import os
import json
import requests
import random
from typing import List, Dict, Optional

# Add proxy_pool to path
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '../../proxy_pool'))

class ProxyPoolManager:
    """Manages a pool of proxies for IP rotation"""
    
    def __init__(self, pool_url: str = "http://127.0.0.1:5010"):
        """Initialize proxy pool manager
        
        Args:
            pool_url: URL of the proxy_pool API
        """
        self.pool_url = pool_url
        self.current_proxy = None
        self.proxy_history = []
        
    def get_proxy(self) -> Optional[Dict[str, str]]:
        """Get a random proxy from the pool"""
        try:
            response = requests.get(f"{self.pool_url}/get/")
            if response.status_code == 200:
                proxy_data = response.json()
                proxy = proxy_data.get('proxy')
                if proxy:
                    self.current_proxy = {
                        'http': f'http://{proxy}',
                        'https': f'http://{proxy}',
                    }
                    self.proxy_history.append(proxy)
                    return self.current_proxy
        except Exception as e:
            print(f"Error getting proxy: {e}")
        return None
    
    def delete_proxy(self, proxy: str):
        """Delete a proxy from the pool (if it's dead)"""
        try:
            requests.get(f"{self.pool_url}/delete/?proxy={proxy}")
        except Exception:
            pass
    
    def get_all_proxies(self) -> List[str]:
        """Get all available proxies"""
        try:
            response = requests.get(f"{self.pool_url}/all/")
            if response.status_code == 200:
                return response.json()
        except Exception as e:
            print(f"Error getting all proxies: {e}")
        return []
    
    def rotate_proxy(self) -> Optional[Dict[str, str]]:
        """Rotate to a new proxy"""
        return self.get_proxy()
    
    def get_pool_count(self) -> int:
        """Get count of available proxies"""
        try:
            response = requests.get(f"{self.pool_url}/count/")
            if response.status_code == 200:
                data = response.json()
                return data.get('count', 0)
        except Exception:
            return 0


class MultiLayerProxyChain:
    """Chain multiple proxy layers for maximum anonymity"""
    
    def __init__(self):
        self.layers = []
        self.pool_manager = ProxyPoolManager()
        
    def add_layer(self, layer_type: str, config: Dict):
        """Add a proxy layer
        
        Args:
            layer_type: Type of proxy ('socks5', 'http', 'v2ray', 'aether')
            config: Configuration for the layer
        """
        self.layers.append({
            'type': layer_type,
            'config': config
        })
    
    def build_chain(self) -> List[Dict]:
        """Build the complete proxy chain"""
        # Layer 1: Random public proxy from pool
        public_proxy = self.pool_manager.get_proxy()
        
        # Layer 2: Aether Network SOCKS5
        aether_proxy = {
            'type': 'socks5',
            'host': '127.0.0.1',
            'port': 9050
        }
        
        chain = []
        if public_proxy:
            chain.append({
                'type': 'http',
                'proxy': public_proxy
            })
        chain.append(aether_proxy)
        
        return chain
    
    def get_current_chain_info(self) -> str:
        """Get info about the current proxy chain"""
        pool_count = self.pool_manager.get_pool_count()
        return f"Proxy Pool: {pool_count} IPs available | Aether: 5-layer mixnet"


def start_proxy_pool():
    """Start the proxy pool service"""
    print("🌐 Starting Proxy Pool Service...")
    proxy_pool_dir = os.path.join(os.path.dirname(__file__), '../../proxy_pool')
    
    if os.path.exists(proxy_pool_dir):
        print(f"   Proxy pool directory: {proxy_pool_dir}")
        print("   Run: python proxy_pool/run.py")
        print("   This will start HTTP API on http://127.0.0.1:5010")
    else:
        print("   ⚠️  proxy_pool not found. Clone it first.")


if __name__ == "__main__":
    # Test the proxy pool
    print("🧪 Testing Proxy Pool Integration")
    print("=" * 50)
    
    manager = ProxyPoolManager()
    
    # Get pool count
    count = manager.get_pool_count()
    print(f"📊 Available proxies: {count}")
    
    if count > 0:
        # Get a proxy
        proxy = manager.get_proxy()
        if proxy:
            print(f"✅ Got proxy: {proxy}")
            
        # Test multi-layer chain
        chain = MultiLayerProxyChain()
        chain_info = chain.build_chain()
        print(f"🔗 Proxy chain: {len(chain_info)} layers")
        for i, layer in enumerate(chain_info):
            print(f"   Layer {i+1}: {layer}")
    else:
        print("⚠️  Proxy pool is empty. Start the proxy_pool service:")
        start_proxy_pool()
