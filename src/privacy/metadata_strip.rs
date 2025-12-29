//! Metadata Stripping Engine
//! 
//! Comprehensive HTTP header sanitization and randomization
//! to defeat metadata-based deanonymization

use rand::seq::SliceRandom;
use std::collections::HashMap;

pub struct MetadataStripper {
    synthetic_headers: HashMap<String, Vec<String>>,
}

impl MetadataStripper {
    pub fn new() -> Self {
        let mut synthetic_headers = HashMap::new();
        
        // Generic User-Agent pool (no identifying info)
        synthetic_headers.insert("User-Agent".to_string(), vec![
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36".to_string(),
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36".to_string(),
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36".to_string(),
        ]);
        
        // Randomized Accept-Language (avoids timezone leaks)
        synthetic_headers.insert("Accept-Language".to_string(), vec![
            "en-US,en;q=0.9".to_string(),
            "en-GB,en;q=0.9".to_string(),
        ]);
        
        Self { synthetic_headers }
    }

    /// Strip identifiable metadata from HTTP headers
    pub fn strip_headers(&self, mut headers: HashMap<String, String>) -> HashMap<String, String> {
        // Remove ALL potentially identifying headers
        let dangerous_headers = vec![
            "X-Forwarded-For", "X-Real-IP", "Via", "From", "Referer",
            "DNT", "Authorization", "Cookie", "Set-Cookie",
            "X-Requested-With", "Origin", "Host", 
        ];
        
        for header in &dangerous_headers {
            headers.remove(*header);
        }
        
        tracing::info!("🧹 METADATA STRIPPED: Removed {} identifying headers", dangerous_headers.len());
        
        // Replace with synthetic generic headers
        self.randomize_headers(headers)
    }

    /// Randomize header ordering and values
    fn randomize_headers(&self, mut headers: HashMap<String, String>) -> HashMap<String, String> {
        let mut rng = rand::thread_rng();
        
        // Replace User-Agent with random generic one
        if let Some(ua_pool) = self.synthetic_headers.get("User-Agent") {
            if let Some(ua) = ua_pool.choose(&mut rng) {
                headers.insert("User-Agent".to_string(), ua.clone());
            }
        }
        
        // Replace Accept-Language
        if let Some(lang_pool) = self.synthetic_headers.get("Accept-Language") {
            if let Some(lang) = lang_pool.choose(&mut rng) {
                headers.insert("Accept-Language".to_string(), lang.clone());
            }
        }
        
        tracing::info!("🎭 METADATA RANDOMIZED: Synthetic headers applied");
        headers
    }

    /// Create completely generic HTTP request headers
    pub fn create_generic_headers(&self) -> HashMap<String, String> {
        let mut headers = HashMap::new();
        let mut rng = rand::thread_rng();
        
        if let Some(ua_pool) = self.synthetic_headers.get("User-Agent") {
            if let Some(ua) = ua_pool.choose(&mut rng) {
                headers.insert("User-Agent".to_string(), ua.clone());
            }
        }
        
        headers.insert("Accept".to_string(), "*/*".to_string());
        headers.insert("Accept-Encoding".to_string(), "gzip, deflate".to_string());
        headers.insert("Connection".to_string(), "keep-alive".to_string());
        
        headers
    }
}
