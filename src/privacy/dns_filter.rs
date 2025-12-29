use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use tracing::{info, warn};

pub struct DnsFilter {
    blocked_domains: HashSet<String>,
}

impl DnsFilter {
    /// Loads blocked domains from a file
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let mut blocked_domains = HashSet::new();
        
        if let Ok(file) = File::open(path) {
            let reader = BufReader::new(file);
            for line in reader.lines() {
                if let Ok(domain) = line {
                    let domain = domain.trim();
                    if !domain.is_empty() && !domain.starts_with('#') {
                        blocked_domains.insert(domain.to_lowercase());
                    }
                }
            }
            info!("🛡️ Layer 11: Loaded {} blocked domains from HaGeZi list", blocked_domains.len());
        } else {
            warn!("⚠️ Layer 11: Failed to load HaGeZi blocklist. Content Shielding DISABLED.");
        }

        Self { blocked_domains }
    }

    /// Checks if a domain is blocked.
    pub fn is_blocked(&self, domain: &str) -> bool {
        let domain_lower = domain.to_lowercase();
        
        // 🎭 CRITICAL EXCLUSION: Never block these essential connectivity/identity services
        let whitelist = ["api.ipify.org", "check.torproject.org", "icanhazip.com"];
        if whitelist.contains(&domain_lower.as_str()) {
            return false;
        }

        // Exact match
        if self.blocked_domains.contains(&domain_lower) {
            return true;
        }

        // Subdomain match (traverses up the tree)
        let parts: Vec<&str> = domain_lower.split('.').collect();
        for i in 0..parts.len() {
            let parent = parts[i..].join(".");
            if self.blocked_domains.contains(&parent) {
                return true;
            }
        }

        false
    }
}
