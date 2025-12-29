//! IP Rotation Manager - Massive Global Diversity Edition
//!
//! Makes IP jump across continents every second

use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tokio::time::interval;
use rand::seq::SliceRandom;

pub struct ActiveRotationManager {
    current_ip: Arc<RwLock<String>>,
    rotation_interval: u64,
    proxy_list: Arc<RwLock<Vec<String>>>,
}

impl ActiveRotationManager {
    pub fn new(rotation_interval_secs: u64) -> Self {
        Self {
            current_ip: Arc::new(RwLock::new("initializing...".to_string())),
            rotation_interval: rotation_interval_secs,
            proxy_list: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub async fn start_rotation(&self) {
        let current_ip = Arc::clone(&self.current_ip);
        let proxy_list = Arc::clone(&self.proxy_list);
        let rotation_interval = self.rotation_interval;

        tokio::spawn(async move {
            let mut ticker = interval(Duration::from_secs(rotation_interval));
            
            loop {
                ticker.tick().await;
                
                // Refresh pool if empty
                let is_empty = {
                    let list = proxy_list.read().await;
                    list.is_empty()
                };

                if is_empty {
                    if let Ok(mut proxies) = Self::fetch_proxy_list().await {
                        {
                            let mut rng = rand::rngs::OsRng;
                            use rand::seq::SliceRandom;
                            proxies.shuffle(&mut rng);
                        }
                        let mut list = proxy_list.write().await;
                        *list = proxies;
                        tracing::info!("🌐 GLOBAL POOL LOADED: Rapid jumping armed");
                    }
                }

                // Select a RANDOM IP from the pool
                let next_ip = {
                    let list = proxy_list.read().await;
                    if !list.is_empty() {
                        let mut rng = rand::rngs::OsRng;
                        use rand::seq::SliceRandom;
                        list.choose(&mut rng).cloned()
                    } else {
                        None
                    }
                };

                if let Some(ip) = next_ip {
                    let mut current = current_ip.write().await;
                    if *current != ip {
                        *current = ip.clone();
                        tracing::info!("🔄 JUMP: New global identity selected -> {}", ip);
                    }
                }
            }
        });
    }

    async fn fetch_proxy_list() -> Result<Vec<String>, Box<dyn std::error::Error + Send + Sync>> {
        let mut all_proxies = Vec::new();

        let sources = vec![
            "https://api.proxyscrape.com/v2/?request=get&protocol=http&timeout=5000&country=all&anonymity=elite",
            "https://www.proxy-list.download/api/v1/get?type=https",
            "https://raw.githubusercontent.com/TheSpeedX/SOCKS-List/master/http.txt",
        ];

        for source in sources {
            if let Ok(resp) = reqwest::get(source).await {
                if let Ok(text) = resp.text().await {
                    let lines: Vec<String> = text.lines()
                        .filter(|l| l.contains(':') && l.len() > 7)
                        .map(|s| s.trim().to_string())
                        .collect();
                    all_proxies.extend(lines);
                }
            }
        }

        if all_proxies.len() < 10 {
            all_proxies = vec![
                "45.79.155.12:3128", "198.50.20.1:8080", "142.44.156.23:3128", "67.205.145.124:8080", "51.158.68.89:8080",
                "159.69.12.34:3128", "185.123.45.67:3128", "13.229.107.106:80", "103.92.78.12:8080", "45.248.77.90:8080",
                "51.158.111.22:8811", "212.47.234.12:3128", "163.172.189.20:3128", "103.87.169.12:8080", "128.199.89.77:3128",
                "191.252.103.45:3128", "41.216.55.12:3128", "105.235.122.10:3128", "190.103.177.22:8080", "1.10.189.54:8080",
                "103.253.146.126:8080", "159.203.111.90:3128", "52.77.218.12:3128", "139.59.252.10:3128", "51.15.166.167:3128",
            ].into_iter().map(|s| s.to_string()).collect();
        }

        Ok(all_proxies)
    }

    pub async fn get_current_ip(&self) -> String {
        self.current_ip.read().await.clone()
    }
}
