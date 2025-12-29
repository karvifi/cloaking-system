//! IP Rotation JSON Parsing with Health Validation
//!
//! Structured parsing of proxy API responses with health checking

use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProxyCandidate {
    pub ip: String,
    pub port: u16,
    pub country: Option<String>,
    pub protocol: String,
    pub anonymity: Option<String>,
    pub last_checked: Option<u64>,
    pub response_time_ms: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProxyApiResponse {
    pub proxies: Vec<ProxyCandidate>,
    pub source: String,
    pub timestamp: u64,
}

pub struct ProxyHealthChecker {
    timeout: Duration,
}

impl ProxyHealthChecker {
    pub fn new() -> Self {
        Self {
            timeout: Duration::from_secs(5),
        }
    }

    /// Parse JSON response from proxy APIs
    pub fn parse_json_response(&self, json: &str, source: &str) -> Result<Vec<ProxyCandidate>, String> {
        // Try structured JSON first
        if let Ok(response) = serde_json::from_str::<ProxyApiResponse>(json) {
            tracing::info!("Parsed {} proxies from {}", response.proxies.len(), source);
            return Ok(response.proxies);
        }

        // Try array format
        if let Ok(proxies) = serde_json::from_str::<Vec<ProxyCandidate>>(json) {
            tracing::info!("Parsed {} proxies from {}", proxies.len(), source);
            return Ok(proxies);
        }

        // Fallback: parse line-by-line (ip:port format)
        self.parse_plain_text(json, source)
    }

    /// Parse plain text proxy lists (ip:port format)
    fn parse_plain_text(&self, text: &str, source: &str) -> Result<Vec<ProxyCandidate>, String> {
        let mut proxies = Vec::new();
        
        for line in text.lines() {
            let line = line.trim();
            if line.is_empty() || !line.contains(':') {
                continue;
            }

            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() != 2 {
                continue;
            }

            if let Ok(port) = parts[1].parse::<u16>() {
                proxies.push(ProxyCandidate {
                    ip: parts[0].to_string(),
                    port,
                    country: None,
                    protocol: "http".to_string(),
                    anonymity: None,
                    last_checked: None,
                    response_time_ms: None,
                });
            }
        }

        if proxies.is_empty() {
            return Err(format!("Failed to parse any proxies from {}", source));
        }

        tracing::info!("Parsed {} proxies from {} (plain text)", proxies.len(), source);
        Ok(proxies)
    }

    /// Health check a proxy candidate
    pub async fn health_check(&self, proxy: &ProxyCandidate) -> bool {
        let addr = format!("{}:{}", proxy.ip, proxy.port);
        
        match tokio::time::timeout(
            self.timeout,
            tokio::net::TcpStream::connect(&addr)
        ).await {
            Ok(Ok(_)) => {
                tracing::info!("Proxy {} is responsive", addr);
                true
            }
            Ok(Err(e)) => {
                tracing::warn!("Proxy {} failed: {}", addr, e);
                false
            }
            Err(_) => {
                tracing::warn!("Proxy {} timed out", addr);
                false
            }
        }
    }

    /// Filter proxies by health and quality
    pub async fn filter_healthy(&self, proxies: Vec<ProxyCandidate>) -> Vec<ProxyCandidate> {
        let mut healthy = Vec::new();
        
        for proxy in proxies {
            if self.health_check(&proxy).await {
                healthy.push(proxy);
            }
            
            // Limit health checks to avoid overwhelming the system
            if healthy.len() >= 50 {
                break;
            }
        }
        
        tracing::info!("Health check complete: {}/{} proxies healthy", 
            healthy.len(), healthy.len());
        
        healthy
    }
}
