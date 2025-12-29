pub mod metadata_hiding;
pub mod traffic_shaping;
pub mod obfuscation;
pub mod zk_auth;
pub mod dns_filter;
pub mod sculptor;
pub mod sharder;
pub mod cloaker;
pub mod rep_ledger;
pub mod entropy_infusion;
pub mod persona;
pub mod subliminal;
pub mod temporal;
pub mod coercion;
pub mod geo_ghost;
pub mod osint_pollution;
pub mod persistence;
pub mod enclave;
pub mod entropy_sync;
pub mod sovereignty;
pub mod clock_skew;

pub use metadata_hiding::{
    MetadataProtectedMessage,
    MetadataMixer,
    MixingCascade,
    TimingNormalizer,
    ShuffleProof,
};
pub use traffic_shaping::MimicryShaper;
pub use obfuscation::SteganoWrapper;
pub use zk_auth::ZKAuthorization;
pub use dns_filter::DnsFilter;
pub use sculptor::{TrafficSculptor, TrafficProfile};
pub use sharder::ChainSharder;
pub use cloaker::HardwareCloaker;
pub use rep_ledger::ReputationManager;
pub use entropy_infusion::EntropyInfusion;
pub use persona::PersonaEngine;
pub use subliminal::SubliminalEncoder;
pub use temporal::TemporalGhost;
pub use coercion::{CoercionShield, SessionType};
pub use geo_ghost::LocationGhost;
pub use osint_pollution::OSINTPolluter;
pub use persistence::PersistenceShield;
pub use enclave::EnclaveShield;
pub use entropy_sync::GlobalEntropy;
pub use sovereignty::SovereignOrchestrator;
pub use clock_skew::ClockSkewAnonymizer;
pub mod mimicry;
pub use mimicry::AdvancedMimicry;
pub mod opsec;
pub use opsec::{OpsecManager, CoverStory};
pub mod expiration;
pub use expiration::DataExpirationManager;
