//! Network Module Coordinator
//! 
//! Manages distributed infrastructure

pub mod bootstrap;
pub mod dht_discovery;
pub mod proof_of_bandwidth;
pub mod dao_governance;

pub use bootstrap::BootstrapManager;
pub use dht_discovery::DhtNodeDiscovery;
pub use proof_of_bandwidth::ProofOfBandwidth;
pub use dao_governance::DaoGovernance;
