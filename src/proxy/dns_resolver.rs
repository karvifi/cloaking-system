//! DNS-over-HTTPS Resolver
//! 
//! Prevents DNS leaks by routing all DNS queries through HTTPS

use std::net::IpAddr;
use serde::{Deserialize, Serialize};

/// DoH resolver using Cloudflare's 1.1.1.1
pub struct DohResolver {
    /// DoH endpoint
    pub endpoint: String,
}

#[derive(Serialize, Deserialize)]
struct DohResponse {
    #[serde(rename = "Answer")]
    answer: Option<Vec<DohAnswer>>,
}

#[derive(Serialize, Deserialize)]
struct DohAnswer {
    data: String,
}

impl DohResolver {
    /// Create new DoH resolver
    pub fn new() -> Self {
        Self {
            endpoint: "https://1.1.1.1/dns-query".to_string(),
        }
    }

    /// Resolve domain name using DoH
    pub async fn resolve(&self, domain: &str) -> Result<Vec<IpAddr>, Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let url = format!("{}?name={}&type=A", self.endpoint, domain);
        
        let response = client
            .get(&url)
            .header("accept", "application/dns-json")
            .send()
            .await?;

        let doh_resp: DohResponse = response.json().await?;
        
        let mut ips = Vec::new();
        if let Some(answers) = doh_resp.answer {
            for answer in answers {
                if let Ok(ip) = answer.data.parse::<IpAddr>() {
                    ips.push(ip);
                }
            }
        }

        Ok(ips)
    }
}

impl Default for DohResolver {
    fn default() -> Self {
        Self::new()
    }
}
