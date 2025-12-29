//! Integration Module Coordinator
//! 
//! Manages all external tool integrations

pub mod ip_rotation;
pub mod tor_control;
pub mod active_rotation;
pub mod proxy_parsing;
pub mod bbot;
pub mod masscan;
pub mod smartdns;
pub mod frp;

pub use bbot::BbotIntegration;
pub use masscan::MasscanIntegration;
pub use smartdns::SmartDnsResolver;
pub use frp::FrpTunnelManager;
pub use proxy_parsing::{ProxyHealthChecker, ProxyCandidate};
