//! Centralized Configuration Management
//! 
//! TOML-based configuration for all Tier 0-8 features

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AetherConfig {
    pub tier0: Tier0Config,
    pub tier1: Tier1Config,
    pub tier2: Tier2Config,
    pub tier3: Tier3Config,
    pub tier4: Tier4Config,
    pub tier5: Tier5Config,
    pub tier6: Tier6Config,
    pub tier7: Tier7Config,
    pub tier8: Tier8Config,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tier0Config {
    pub quantuminsert_defense: bool,
    pub xkeyscore_defense: bool,
    pub key_rotation_seconds: u64,
    pub metadata_stripping: bool,
    pub certificate_pinning: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tier1Config {
    pub proxy_health_check: bool,
    pub ja3_profile: String,
    pub traffic_morphing: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tier2Config {
    pub bbot_enabled: bool,
    pub masscan_enabled: bool,
    pub smartdns_enabled: bool,
    pub frp_enabled: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tier3Config {
    pub hybrid_pq_signatures: bool,
    pub recursive_snarks: bool,
    pub homomorphic_encryption: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tier4Config {
    pub supply_chain_verification: bool,
    pub container_isolation: bool,
    pub hsm_enabled: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tier5Config {
    pub dht_discovery: bool,
    pub proof_of_bandwidth: bool,
    pub dao_governance: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tier6Config {
    pub constant_time_crypto: bool,
    pub ml_mimicry: bool,
    pub memory_safety_hardening: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tier7Config {
    pub adversarial_testing: bool,
    pub performance_monitoring: bool,
    pub fuzzing: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tier8Config {
    pub cli_enabled: bool,
    pub api_docs: bool,
}

impl Default for AetherConfig {
    fn default() -> Self {
        Self {
            tier0: Tier0Config {
                quantuminsert_defense: true,
                xkeyscore_defense: true,
                key_rotation_seconds: 60,
                metadata_stripping: true,
                certificate_pinning: true,
            },
            tier1: Tier1Config {
                proxy_health_check: true,
                ja3_profile: "chrome_128_win11".to_string(),
                traffic_morphing: true,
            },
            tier2: Tier2Config {
                bbot_enabled: false,
                masscan_enabled: false,
                smartdns_enabled: true,
                frp_enabled: false,
            },
            tier3: Tier3Config {
                hybrid_pq_signatures: true,
                recursive_snarks: false,
                homomorphic_encryption: false,
            },
            tier4: Tier4Config {
                supply_chain_verification: true,
                container_isolation: false,
                hsm_enabled: false,
            },
            tier5: Tier5Config {
                dht_discovery: false,
                proof_of_bandwidth: false,
                dao_governance: false,
            },
            tier6: Tier6Config {
                constant_time_crypto: true,
                ml_mimicry: false,
                memory_safety_hardening: true,
            },
            tier7: Tier7Config {
                adversarial_testing: false,
                performance_monitoring: true,
                fuzzing: false,
            },
            tier8: Tier8Config {
                cli_enabled: true,
                api_docs: true,
            },
        }
    }
}

impl AetherConfig {
    /// Load configuration from TOML file
    pub fn load(path: &Path) -> Result<Self, String> {
        let contents = fs::read_to_string(path)
            .map_err(|e| format!("Failed to read config: {}", e))?;
        
        toml::from_str(&contents)
            .map_err(|e| format!("Failed to parse config: {}", e))
    }

    /// Save configuration to TOML file
    pub fn save(&self, path: &Path) -> Result<(), String> {
        let toml = toml::to_string_pretty(self)
            .map_err(|e| format!("Failed to serialize config: {}", e))?;
        
        fs::write(path, toml)
            .map_err(|e| format!("Failed to write config: {}", e))
    }

    /// Generate default config file
    pub fn generate_default_config() -> String {
        let config = Self::default();
        toml::to_string_pretty(&config).unwrap()
    }
}
