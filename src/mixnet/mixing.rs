//! Mixing strategies for anonymity
//! 
//! Defines how packets are delayed and batched to resist traffic analysis.

use std::time::Duration;
use rand_distr::{Distribution, Exp};

/// Trait for mixing strategies
pub trait MixingStrategy: Send + Sync {
    /// Calculate the delay for a packet based on its size and current node load
    fn calculate_delay(&self, packet_size: usize, current_load: f64) -> Duration;
    
    /// Get the human-readable name of the mixing strategy
    fn name(&self) -> &str;
}

/// Stop-and-Go mixing with exponential delays
/// This is the standard mixing strategy for resisting timing attacks.
pub struct StopAndGoMixing {
    /// Mean delay in milliseconds for the exponential distribution
    pub mean_delay_ms: f64,
    
    /// Minimum delay bound in milliseconds
    pub min_delay_ms: u64,
    
    /// Maximum delay bound in milliseconds
    pub max_delay_ms: u64,
}

impl StopAndGoMixing {
    /// Create a new StopAndGoMixing strategy with a given mean delay
    pub fn new(mean_delay_ms: f64) -> Self {
        Self {
            mean_delay_ms,
            min_delay_ms: 10,
            max_delay_ms: 5000,
        }
    }
}

impl MixingStrategy for StopAndGoMixing {
    fn calculate_delay(&self, _packet_size: usize, current_load: f64) -> Duration {
        let mut rng = rand::thread_rng();
        
        let exp = Exp::new(1.0 / self.mean_delay_ms).unwrap();
        let base_delay = exp.sample(&mut rng);
        
        let load_factor = 1.0 + (current_load * 0.5);
        let adjusted_delay = base_delay * load_factor;
        
        let delay_ms = adjusted_delay
            .max(self.min_delay_ms as f64)
            .min(self.max_delay_ms as f64);
        
        Duration::from_millis(delay_ms as u64)
    }
    
    fn name(&self) -> &str {
        "Stop-and-Go"
    }
}

/// Timed mixing strategy that uses a fixed delay
pub struct TimedMixing {
    /// Fixed interval to wait before releasing a batch
    pub batch_interval_ms: u64,
}

impl MixingStrategy for TimedMixing {
    fn calculate_delay(&self, _packet_size: usize, _current_load: f64) -> Duration {
        Duration::from_millis(self.batch_interval_ms)
    }
    
    fn name(&self) -> &str {
        "Timed"
    }
}
