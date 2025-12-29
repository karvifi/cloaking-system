//! Advanced Module Coordinator
//! 
//! Manages all advanced security modules

pub mod zkproofs;
pub mod homomorphic;
pub mod supply_chain;
pub mod container_security;
pub mod hsm;
pub mod recursive_snarks;
pub mod ml_mimicry;
pub mod memory_safety;
pub mod adversarial_testing;
pub mod fuzzing;
pub mod zerocopy_io;
pub mod ai_traffic_analyzer;
pub mod sgx_enclave;
pub mod steganography;
pub mod qkd;
pub mod garlic_routing;
pub mod oqs_integration;
pub mod dpi_evasion_live;
pub mod i2p_garlic_live;
pub mod all_integrations_live;
pub mod katzenpost_nym;
pub mod gan_traffic;
pub mod qiskit_qkd;

pub use supply_chain::BuildAttestation;
pub use container_security::ContainerSecurityManager;
pub use hsm::HsmManager;
pub use recursive_snarks::RecursiveProofSystem;
pub use ml_mimicry::MlTrafficGenerator;
pub use adversarial_testing::AdversarialTester;
