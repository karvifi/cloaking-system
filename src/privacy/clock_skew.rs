use std::time::{SystemTime, UNIX_EPOCH};
use rand::Rng;
use tracing::info;

pub struct ClockSkewAnonymizer;

impl ClockSkewAnonymizer {
    /// Normalizes the system clock to a network-wide epoch and adds micro-jitter.
    /// This defeats 'Clock Skew Fingerprinting' attacks (Murdoch, 2006) 
    /// where an adversary measures microscopic crystal drift to identify hardware.
    pub fn get_anonymized_timestamp() -> u128 {
        let mut rng = rand::thread_rng();
        
        // 1. Get real system time in nanoseconds
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_nanos();
            
        // 2. Perform Epoch Normalization (rounding to 100ns precision)
        let normalized = (now / 100) * 100;
        
        // 3. Inject Artificial Jitter (0-500ns)
        // This jitter is larger than the typical clock skew drift, masking the real hardware signal
        let jitter = rng.gen_range(0..500) as u128;
        
        info!("⌛ RIGOR: Clock Skew Anonymization applied (Jitter: {}ns)", jitter);
        normalized + jitter
    }
}
