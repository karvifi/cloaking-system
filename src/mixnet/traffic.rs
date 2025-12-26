//! Traffic shaping and cover traffic generation

use crate::protocols::OutfoxPacket;
use crate::crypto::kyber::PublicKey;
use std::collections::VecDeque;
use rand::Rng;

/// Traffic shaping engine
pub struct TrafficShaper {
    /// Target traffic rate (packets per second)
    target_rate: f64,
    
    /// Cover traffic ratio
    cover_ratio: f64,
    
    /// Recent packet history for entropy calculation
    history: VecDeque<PacketTimestamp>,
}

#[derive(Clone)]
struct PacketTimestamp {
    timestamp: u64,
    is_cover: bool,
}

impl TrafficShaper {
    pub fn new(target_rate: f64, cover_ratio: f64) -> Self {
        Self {
            target_rate,
            cover_ratio,
            history: VecDeque::new(),
        }
    }
    
    /// Shape traffic by adding cover packets
    pub fn shape(&mut self, real_packets: Vec<OutfoxPacket>) -> Vec<OutfoxPacket> {
        let mut shaped = real_packets;
        let num_real = shaped.len();
        
        // Calculate how many cover packets to add
        let num_cover = (num_real as f64 * self.cover_ratio / (1.0 - self.cover_ratio)) as usize;
        
        // Add cover packets
        for _ in 0..num_cover {
            // Cover packets would be generated here
            // For now, we just track the count
        }
        
        // Shuffle to mix real and cover packets
        use rand::seq::SliceRandom;
        let mut rng = rand::thread_rng();
        shaped.shuffle(&mut rng);
        
        shaped
    }
    
    /// Calculate Shannon entropy of traffic patterns
    pub fn calculate_entropy(&self) -> f64 {
        if self.history.is_empty() {
            return 0.0;
        }
        
        // Calculate time intervals between packets
        let mut intervals = Vec::new();
        for window in self.history.iter().collect::<Vec<_>>().windows(2) {
            let interval = window[1].timestamp - window[0].timestamp;
            intervals.push(interval);
        }
        
        // Count frequency of intervals (bucketed)
        let mut counts = std::collections::HashMap::new();
        for &interval in &intervals {
            let bucket = interval / 10; // 10ms buckets
            *counts.entry(bucket).or_insert(0) += 1;
        }
        
        // Calculate entropy
        let total = intervals.len() as f64;
        let mut entropy = 0.0;
        
        for &count in counts.values() {
            let probability = count as f64 / total;
            if probability > 0.0 {
                entropy -= probability * probability.log2();
            }
        }
        
        entropy
    }
    
    /// Record packet timestamp for analysis
    pub fn record_packet(&mut self, is_cover: bool) {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;
        
        self.history.push_back(PacketTimestamp { timestamp, is_cover });
        
        // Keep only recent history (last 1000 packets)
        while self.history.len() > 1000 {
            self.history.pop_front();
        }
    }
}

/// Cover traffic generator
pub struct CoverTrafficGenerator {
    /// Rate of cover traffic generation (packets per second)
    pub rate: f64,
    
    /// Dummy route for cover packets
    pub dummy_route: Vec<PublicKey>,
}

impl CoverTrafficGenerator {
    pub fn new(rate: f64) -> Self {
        Self {
            rate,
            dummy_route: Vec::new(),
        }
    }
    
    /// Generate a dummy/cover packet
    pub fn generate_cover_packet(&self) -> Option<OutfoxPacket> {
        if self.dummy_route.is_empty() {
            return None;
        }
        
        // Generate random dummy data
        let mut rng = rand::thread_rng();
        let size = rng.gen_range(100..2000);
        let dummy_data: Vec<u8> = (0..size).map(|_| rng.gen()).collect();
        
        // Create packet (would go back to self in a loop)
        OutfoxPacket::new(&dummy_data, &self.dummy_route).ok()
    }
    
    /// Set the dummy route (typically a loop to self)
    pub fn set_dummy_route(&mut self, route: Vec<PublicKey>) {
        self.dummy_route = route;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_traffic_shaper() {
        let mut shaper = TrafficShaper::new(100.0, 0.4);
        
        for i in 0..50 {
            shaper.record_packet(i % 5 == 0); // 20% cover traffic
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
        
        let entropy = shaper.calculate_entropy();
        assert!(entropy > 0.0);
    }
}
