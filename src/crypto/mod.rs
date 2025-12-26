//! Cryptographic primitives for Aether Network
//! 
//! This module implements:
//! - Post-quantum key encapsulation (Kyber-1024)
//! - Hybrid encryption schemes
//! - Hash functions and derivatives
//! - Digital signatures

pub mod kyber;
pub mod symmetric;
pub mod hash;
pub mod signatures;

pub use kyber::{KeyPair, PublicKey, SecretKey, encapsulate, decapsulate};
pub use symmetric::{encrypt_aead, decrypt_aead};
pub use hash::{blake3_hash, derive_key};
pub use signatures::{sign_message, verify_signature};

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_kyber_roundtrip() {
        let keypair = KeyPair::generate();
        let (ciphertext, shared_secret_1) = encapsulate(&keypair.public_key);
        let shared_secret_2 = decapsulate(&ciphertext, &keypair.secret_key).unwrap();
        assert_eq!(shared_secret_1, shared_secret_2);
    }
}
