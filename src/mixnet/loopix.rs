//! Loopix Cover Traffic System
//! 
//! Implements the Loopix mixing strategy (USENIX Security 2017)
//! with three types of cover traffic to hide communication patterns

use tokio::time::Duration;
use rand::Rng;
use rand_distr::{Distribution, Exp};
use crate::protocols::sphinx::SphinxBuilder;

/// Loopix cover traffic generator
pub struct LoopixCoverTraffic {
    /// Rate of loop messages (sent to yourself)
    loop_rate: f64,
    
    /// Rate of drop messages (sent to nowhere)
    drop_rate: f64,
    
    /// Your own address (for loop traffic)
    own_address: [u8; 32],
    
    /// Running state
    running: bool,
}

impl LoopixCoverTraffic {
    /// Create new Loopix cover traffic generator
    pub fn new(own_address: [u8; 32]) -> Self {
        Self {
            loop_rate: 0.5,  // λ_L = 0.5 messages/sec (from paper)
            drop_rate: 0.3,  // λ_D = 0.3 messages/sec
            own_address,
            running: false,
        }
    }

    /// Start generating cover traffic
    pub async fn start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.running = true;
        tracing::info!("🎭 Starting Loopix cover traffic");
        tracing::info!("   Loop rate: {} msg/sec", self.loop_rate);
        tracing::info!("   Drop rate: {} msg/sec", self.drop_rate);

        // Spawn loop traffic generator
        let loop_rate = self.loop_rate;
        let own_addr = self.own_address;
        tokio::task::spawn(async move {
            Self::loop_traffic_worker(loop_rate, own_addr).await;
        });

        // Spawn drop traffic generator  
        let drop_rate = self.drop_rate;
        tokio::task::spawn(async move {
            Self::drop_traffic_worker(drop_rate).await;
        });

        Ok(())
    }

    /// Worker for loop traffic (messages to yourself)
    async fn loop_traffic_worker(rate: f64, own_address: [u8; 32]) {
        let exp_dist = Exp::new(rate).unwrap();

        loop {
            // Create RNG inside loop to avoid Send issues
            let delay_secs = {
                let mut rng = rand::thread_rng();
                exp_dist.sample(&mut rng)
            };
            
            tokio::time::sleep(Duration::from_secs_f64(delay_secs)).await;

            // Create loop message
            let path = vec![
                own_address,
                Self::random_relay(),
                Self::random_relay(),
                own_address, // Back to us
            ];

            let message = Self::generate_dummy_payload();
            
            match SphinxBuilder::new(path, message).build() {
                Ok(_packet) => {
                    tracing::debug!("🔄 Generated loop cover traffic");
                    // TODO: Send packet into network
                }
                Err(e) => {
                    tracing::error!("Failed to create loop packet: {}", e);
                }
            }
        }
    }

    /// Worker for drop traffic (messages to nowhere)
    async fn drop_traffic_worker(rate: f64) {
        let exp_dist = Exp::new(rate).unwrap();

        loop {
            // Create RNG inside loop to avoid Send issues
            let delay_secs = {
                let mut rng = rand::thread_rng();
                exp_dist.sample(&mut rng)
            };
            
            tokio::time::sleep(Duration::from_secs_f64(delay_secs)).await;

            // Create drop message (goes to a fake destination that drops it)
            let path = vec![
                Self::random_relay(),
                Self::random_relay(),
                Self::random_relay(),
                [0xFFu8; 32], // Special "drop" address
            ];

            let message = Self::generate_dummy_payload();
            
            match SphinxBuilder::new(path, message).build() {
                Ok(_packet) => {
                    tracing::debug!("🗑️  Generated drop cover traffic");
                    // TODO: Send packet into network
                }
                Err(e) => {
                    tracing::error!("Failed to create drop packet: {}", e);
                }
            }
        }
    }

    /// Generate random relay address
    fn random_relay() -> [u8; 32] {
        let mut rng = rand::thread_rng();
        let mut addr = [0u8; 32];
        rng.fill(&mut addr);
        addr
    }

    /// Generate dummy payload (indistinguishable from real messages)
    fn generate_dummy_payload() -> Vec<u8> {
        let mut rng = rand::thread_rng();
        let size = rng.gen_range(100..2000);
        let mut payload = vec![0u8; size];
        rng.fill(&mut payload[..]);
        payload
    }

    /// Stop cover traffic generation
    pub fn stop(&mut self) {
        self.running = false;
        tracing::info!("⏹️  Stopped Loopix cover traffic");
    }
}

/// Statistics for cover traffic
#[derive(Default)]
pub struct CoverTrafficStats {
    /// Total loop messages sent
    pub loop_messages: u64,
    
    /// Total drop messages sent
    pub drop_messages: u64,
    
    /// Total real messages sent
    pub real_messages: u64,
    
    /// Average cover traffic ratio
    pub cover_ratio: f64,
}

impl CoverTrafficStats {
    /// Calculate the cover traffic ratio
    pub fn update_ratio(&mut self) {
        let total_cover = self.loop_messages + self.drop_messages;
        let total = total_cover + self.real_messages;
        
        if total > 0 {
            self.cover_ratio = (total_cover as f64) / (total as f64);
        }
    }

    /// Get human-readable stats
    pub fn summary(&self) -> String {
        format!(
            "Loop: {}, Drop: {}, Real: {}, Cover Ratio: {:.1}%",
            self.loop_messages,
            self.drop_messages,
            self.real_messages,
            self.cover_ratio * 100.0
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_loopix_creation() {
        let addr = [1u8; 32];
        let loopix = LoopixCoverTraffic::new(addr);
        assert_eq!(loopix.loop_rate, 0.5);
        assert_eq!(loopix.drop_rate, 0.3);
    }

    #[test]
    fn test_stats_calculation() {
        let mut stats = CoverTrafficStats::default();
        stats.loop_messages = 50;
        stats.drop_messages = 30;
        stats.real_messages = 20;
        stats.update_ratio();
        
        assert!(stats.cover_ratio > 0.7); // 80/100 = 80% cover traffic
    }
}
