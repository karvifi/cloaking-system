//! Configuration module for Aether Network

use serde::{Deserialize, Serialize};

/// Main configuration structure for Aether Network
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AetherConfig {
    /// Unique network identifier (32 bytes)
    pub network_id: [u8; 32],
    
    /// Maximum packet size in bytes (fixed at 2413 for Sphinx)
    pub max_packet_size: usize,
    
    /// Number of layers in the mixnet (typically 5)
    pub mixnet_layers: usize,
    
    /// Lambda parameter for Poisson process timing (in milliseconds)
    pub poisson_lambda: f64,
    
    /// Enable post-quantum cryptography
    pub quantum_safe: bool,
    
    /// Ratio of cover traffic to real traffic (0.0 to 1.0)
    pub cover_traffic_ratio: f64,
    
    /// Enable zero-knowledge proofs for access control
    pub use_zkp: bool,
    
    /// Minimum reputation threshold for node selection
    pub reputation_threshold: f64,
    
    /// Minimum stake requirement in tokens
    pub stake_requirement: u64,
    
    /// Network topology configuration
    pub topology: TopologyConfig,
    
    /// Crypto configuration
    pub crypto: CryptoConfig,
    
    /// Economic parameters
    pub economics: EconomicsConfig,
}

/// Network topology configuration
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TopologyConfig {
    /// Topology type: "stratified" or "free-routes"
    pub topology_type: String,
    
    /// Maximum number of nodes
    pub max_nodes: usize,
    
    /// Topology update interval in seconds
    pub update_interval: u64,
}

/// Cryptographic configuration
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CryptoConfig {
    /// Post-quantum algorithm: "kyber1024", "mceliece", "xwing"
    pub pq_algorithm: String,
    
    /// Symmetric encryption: "xchacha20poly1305", "aes256gcm"
    pub symmetric_cipher: String,
    
    /// Hash function: "blake3", "sha3-256"
    pub hash_function: String,
    
    /// Enable hardware security module
    pub use_hsm: bool,
}

/// Economic and incentive configuration
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EconomicsConfig {
    /// Stake required to run a node
    pub stake_required: u64,
    
    /// Reward per packet processed
    pub reward_per_packet: f64,
    
    /// Percentage slashed for misbehavior
    pub slash_percentage: f64,
    
    /// Minimum reputation for rewards
    pub min_reputation_for_rewards: f64,
}

impl Default for AetherConfig {
    fn default() -> Self {
        use ring::rand::{SecureRandom, SystemRandom};
        
        let mut network_id = [0u8; 32];
        let rng = SystemRandom::new();
        rng.fill(&mut network_id).expect("Failed to generate network ID");
        
        Self {
            network_id,
            max_packet_size: 2413,
            mixnet_layers: 5,
            poisson_lambda: 50.0,
            quantum_safe: true,
            cover_traffic_ratio: 0.4,
            use_zkp: true,
            reputation_threshold: 0.7,
            stake_requirement: 1000,
            topology: TopologyConfig {
                topology_type: "stratified".to_string(),
                max_nodes: 1000,
                update_interval: 3600,
            },
            crypto: CryptoConfig {
                pq_algorithm: "kyber1024".to_string(),
                symmetric_cipher: "xchacha20poly1305".to_string(),
                hash_function: "blake3".to_string(),
                use_hsm: false,
            },
            economics: EconomicsConfig {
                stake_required: 1000,
                reward_per_packet: 0.001,
                slash_percentage: 0.1,
                min_reputation_for_rewards: 0.5,
            },
        }
    }
}

impl AetherConfig {
    /// Load configuration from TOML file
    pub fn from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let config: AetherConfig = toml::from_str(&content)?;
        Ok(config)
    }
    
    /// Save configuration to TOML file
    pub fn to_file(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let content = toml::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }
    
    /// Validate configuration parameters
    pub fn validate(&self) -> Result<(), String> {
        if self.mixnet_layers < 3 {
            return Err("Mixnet must have at least 3 layers".to_string());
        }
        
        if self.cover_traffic_ratio < 0.0 || self.cover_traffic_ratio > 1.0 {
            return Err("Cover traffic ratio must be between 0.0 and 1.0".to_string());
        }
        
        if self.reputation_threshold < 0.0 || self.reputation_threshold > 1.0 {
            return Err("Reputation threshold must be between 0.0 and 1.0".to_string());
        }
        
        if self.poisson_lambda <= 0.0 {
            return Err("Poisson lambda must be positive".to_string());
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_default_config() {
        let config = AetherConfig::default();
        assert_eq!(config.mixnet_layers, 5);
        assert_eq!(config.max_packet_size, 2413);
        assert!(config.validate().is_ok());
    }
    
    #[test]
    fn test_invalid_config() {
        let mut config = AetherConfig::default();
        config.mixnet_layers = 2;
        assert!(config.validate().is_err());
    }
}
