//! # Aether Network
//! 
//! A post-quantum anonymity network for cybersecurity research.

#![warn(missing_docs)]
#![warn(unsafe_code)]

pub mod config;
pub mod crypto;
pub mod mixnet;
pub mod consensus;
pub mod advanced;
pub mod integration;
pub mod metrics;
pub mod docs;
pub mod install;
pub mod orchestrator;
pub mod cli;
pub mod error;
pub mod zk_access;
pub mod network;

// Advanced features (Ultimate Edition)
#[cfg(feature = "advanced-stealth")]
pub mod stealth;

#[cfg(feature = "hardware-security")]
pub mod hardware;
pub mod proxy;
pub mod integration;
pub mod privacy;

#[cfg(any(feature = "ai-routing", feature = "ultimate"))]
pub mod client;

// Maximum realistic features (9.8/10 system)
pub mod advanced;

pub use config::AetherConfig;
pub use error::{AetherError, Result};

/// Version of the Aether protocol
pub const PROTOCOL_VERSION: &str = "0.2.0";

/// Maximum packet size
pub const MAX_PACKET_SIZE: usize = 10000;

/// Number of layers in the mix network
pub const MIXNET_LAYERS: usize = 5;
