//! Crypto Module Coordinator
//! 
//! Manages all cryptographic primitives

pub mod kyber;
pub mod symmetric;
pub mod hash;
pub mod signatures;
pub mod sharding;
pub mod agility;
pub mod hybrid_pq;
pub mod constant_time;
pub mod simd_crypto;

pub use kyber::{KeyPair, PublicKey, SecretKey, encapsulate, decapsulate};
pub use symmetric::{encrypt_aead, decrypt_aead};
pub use hash::{blake3_hash, derive_key};
pub use signatures::{sign_message, verify_signature};
pub use sharding::IdentitySharder;
pub use agility::{AgilityManager, CipherSuite};
pub use hybrid_pq::HybridSigner;
pub use constant_time::ConstantTimeCrypto;
pub use simd_crypto::SimdCrypto;
