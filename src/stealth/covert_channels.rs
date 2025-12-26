//! Covert Channels - Hidden communication methods
//! 
//! ⚠️ FOR AUTHORIZED SECURITY RESEARCH ONLY

use std::time::{Duration, Instant};
use std::collections::VecDeque;

/// Timing-based covert channel
pub struct TimingChannel {
    /// Base delay for bit '0'
    zero_delay_ms: u64,
    
    /// Base delay for bit '1'
    one_delay_ms: u64,
    
    /// Jitter tolerance
    jitter_tolerance_ms: u64,
    
    /// Last transmission time
    last_tx: Option<Instant>,
}

impl TimingChannel {
    /// Create new timing channel
    pub fn new(zero_delay_ms: u64, one_delay_ms: u64) -> Self {
        Self {
            zero_delay_ms,
            one_delay_ms,
            jitter_tolerance_ms: 10,
            last_tx: None,
        }
    }
    
    /// Encode a bit using timing
    pub async fn send_bit(&mut self, bit: bool) {
        let delay = if bit {
            Duration::from_millis(self.one_delay_ms)
        } else {
            Duration::from_millis(self.zero_delay_ms)
        };
        
        tokio::time::sleep(delay).await;
        self.last_tx = Some(Instant::now());
    }
    
    /// Decode a bit from timing
    pub fn receive_bit(&mut self, arrival_time: Instant) -> Option<bool> {
        if let Some(last) = self.last_tx {
            let interval = arrival_time.duration_since(last).as_millis() as u64;
            
            // Check which delay it's closer to
            let zero_diff = interval.abs_diff(self.zero_delay_ms);
            let one_diff = interval.abs_diff(self.one_delay_ms);
            
            self.last_tx = Some(arrival_time);
            
            if zero_diff <= self.jitter_tolerance_ms {
                Some(false)
            } else if one_diff <= self.jitter_tolerance_ms {
                Some(true)
            } else {
                None // Too much jitter
            }
        } else {
            self.last_tx = Some(arrival_time);
            None
        }
    }
    
    /// Send a byte using timing
    pub async fn send_byte(&mut self, byte: u8) {
        for i in 0..8 {
            let bit = (byte >> i) & 1 == 1;
            self.send_bit(bit).await;
        }
    }
}

/// DNS-based covert channel
pub struct DnsChannel {
    /// Base domain for queries
    base_domain: String,
    
    /// Maximum label length (63 chars per DNS spec)
    max_label_len: usize,
}

impl DnsChannel {
    /// Create new DNS channel
    pub fn new(base_domain: String) -> Self {
        Self {
            base_domain,
            max_label_len: 63,
        }
    }
    
    /// Encode data as DNS query
    pub fn encode_query(&self, data: &[u8]) -> String {
        // Base32 encode data (DNS-safe)
        let encoded = base32::encode(base32::Alphabet::RFC4648 { padding: false }, data);
        
        // Split into labels (max 63 chars each)
        let mut labels = Vec::new();
        for chunk in encoded.as_bytes().chunks(self.max_label_len) {
            labels.push(String::from_utf8_lossy(chunk).to_string());
        }
        
        // Build DNS query: data1.data2.data3.base_domain.com
        let mut query = labels.join(".");
        query.push('.');
        query.push_str(&self.base_domain);
        
        query
    }
    
    /// Decode data from DNS query
    pub fn decode_query(&self, query: &str) -> Option<Vec<u8>> {
        // Remove base domain
        let data_part = query.strip_suffix(&format!(".{}", self.base_domain))?;
        
        // Rejoin labels
        let encoded = data_part.replace('.', "");
        
        // Base32 decode
        base32::decode(base32::Alphabet::RFC4648 { padding: false }, &encoded)
    }
}

/// HTTP parameter-based covert channel
pub struct HttpChannel {
    /// Parameter name to use
    param_name: String,
}

impl HttpChannel {
    pub fn new(param_name: String) -> Self {
        Self { param_name }
    }
    
    /// Encode data in URL parameter
    pub fn encode_url(&self, base_url: &str, data: &[u8]) -> String {
        let encoded = base64::encode(data);
        format!("{}?{}={}", base_url, self.param_name, encoded)
    }
    
    /// Decode data from URL parameter
    pub fn decode_url(&self, url: &str) -> Option<Vec<u8>> {
        // Parse URL and extract parameter
        // Simplified - in practice use proper URL parsing
        let parts: Vec<&str> = url.split('?').collect();
        if parts.len() < 2 {
            return None;
        }
        
        for param in parts[1].split('&') {
            let kv: Vec<&str> = param.split('=').collect();
            if kv.len() == 2 && kv[0] == self.param_name {
                return base64::decode(kv[1]).ok();
            }
        }
        
        None
    }
}

/// Detection utilities for defenders
pub struct CovertChannelDetector {
    /// Recent packet timings
    timing_history: VecDeque<(Instant, usize)>,
    
    /// Maximum history size
    max_history: usize,
}

impl CovertChannelDetector {
    pub fn new() -> Self {
        Self {
            timing_history: VecDeque::new(),
            max_history: 1000,
        }
    }
    
    /// Record a packet arrival
    pub fn record_packet(&mut self, size: usize) {
        let now = Instant::now();
        self.timing_history.push_back((now, size));
        
        if self.timing_history.len() > self.max_history {
            self.timing_history.pop_front();
        }
    }
    
    /// Detect suspicious timing patterns
    pub fn detect_timing_channel(&self) -> f64 {
        if self.timing_history.len() < 10 {
            return 0.0;
        }
        
        // Calculate inter-arrival times
        let mut intervals = Vec::new();
        for window in self.timing_history.iter().collect::<Vec<_>>().windows(2) {
            let interval = window[1].0.duration_since(window[0].0).as_millis() as f64;
            intervals.push(interval);
        }
        
        // Calculate coefficient of variation
        let mean = intervals.iter().sum::<f64>() / intervals.len() as f64;
        let variance = intervals
            .iter()
            .map(|&x| {
                let diff = x - mean;
                diff * diff
            })
            .sum::<f64>()
            / intervals.len() as f64;
        
        let std_dev = variance.sqrt();
        let cv = std_dev / mean;
        
        // Natural traffic has high variance (cv > 0.5)
        // Covert timing channels have low variance (cv < 0.3)
        if cv < 0.3 {
            1.0 - cv // Suspicion score
        } else {
            0.0
        }
    }
    
    /// Detect DNS tunneling
    pub fn detect_dns_tunnel(&self, query: &str) -> bool {
        // Heuristics for DNS tunneling:
        // 1. Very long subdomain labels
        // 2. High entropy in labels
        // 3. Unusual character distribution
        
        let labels: Vec<&str> = query.split('.').collect();
        
        // Check for suspiciously long labels
        for label in &labels {
            if label.len() > 40 {
                return true;
            }
        }
        
        // Check entropy of first label
        if let Some(first_label) = labels.first() {
            let entropy = calculate_entropy(first_label.as_bytes());
            if entropy > 4.5 {
                // High entropy suggests encoded data
                return true;
            }
        }
        
        false
    }
}

/// Calculate Shannon entropy of data
fn calculate_entropy(data: &[u8]) -> f64 {
    let mut counts = [0usize; 256];
    
    for &byte in data {
        counts[byte as usize] += 1;
    }
    
    let total = data.len() as f64;
    let mut entropy = 0.0;
    
    for &count in &counts {
        if count > 0 {
            let p = count as f64 / total;
            entropy -= p * p.log2();
        }
    }
    
    entropy
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_timing_channel() {
        let mut channel = TimingChannel::new(100, 200);
        
        // Send byte 0b10101010
        channel.send_byte(0b10101010).await;
        
        // In practice, receiver would decode from packet arrivals
    }
    
    #[test]
    fn test_dns_channel() {
        let channel = DnsChannel::new("evil.com".to_string());
        
        let data = b"SECRET";
        let query = channel.encode_query(data);
        
        println!("DNS Query: {}", query);
        assert!(query.ends_with(".evil.com"));
        
        let decoded = channel.decode_query(&query).unwrap();
        assert_eq!(decoded, data);
    }
    
    #[test]
    fn test_http_channel() {
        let channel = HttpChannel::new("id".to_string());
        
        let data = b"hidden message";
        let url = channel.encode_url("https://example.com/page", data);
        
        println!("URL: {}", url);
        
        let decoded = channel.decode_url(&url).unwrap();
        assert_eq!(decoded, data);
    }
    
    #[test]
    fn test_detection() {
        let mut detector = CovertChannelDetector::new();
        
        // Simulate regular timing (high variance)
        let mut now = Instant::now();
        for _ in 0..50 {
            detector.record_packet(1000);
            now += Duration::from_millis(rand::random::<u64>() % 200);
        }
        
        let suspicion = detector.detect_timing_channel();
        assert!(suspicion < 0.5, "Should not detect regular traffic");
        
        // Simulate covert channel (regular timing)
        detector = CovertChannelDetector::new();
        now = Instant::now();
        for _ in 0..50 {
            detector.record_packet(1000);
            now += Duration::from_millis(100); // Very regular!
        }
        
        let suspicion = detector.detect_timing_channel();
        assert!(suspicion > 0.5, "Should detect covert timing channel");
    }
    
    #[test]
    fn test_dns_tunnel_detection() {
        let detector = CovertChannelDetector::new();
        
        // Normal DNS query
        assert!(!detector.detect_dns_tunnel("www.google.com"));
        
        // Suspicious - very long subdomain
        assert!(detector.detect_dns_tunnel(
            "JVQW4Y3UNFXW4ZZAMFZXGLLBNNSXQZLBMRQXIZLSEBUW4IDCMFZQ.evil.com"
        ));
    }
}
