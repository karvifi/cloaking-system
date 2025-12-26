//! Kyber-1024 post-quantum key encapsulation mechanism

use pqcrypto_kyber::kyber1024::*;
use pqcrypto_traits::kem::{PublicKey as PQPublicKey, SecretKey as PQSecretKey, SharedSecret as PQSharedSecret, Ciphertext as PQCiphertext};
use crate::error::{AetherError, Result};

/// Wrapper for Kyber-1024 public key
#[derive(Clone)]
pub struct PublicKey(pqcrypto_kyber::kyber1024::PublicKey);

/// Wrapper for Kyber-1024 secret key
#[derive(Clone)]
pub struct SecretKey(pqcrypto_kyber::kyber1024::SecretKey);

///Wrapper for Kyber-1024 ciphertext
#[derive(Clone)]
pub struct Ciphertext(pqcrypto_kyber::kyber1024::Ciphertext);

/// Wrapper for shared secret
#[derive(Clone)]
pub struct SharedSecret(pqcrypto_kyber::kyber1024::SharedSecret);

/// Key pair structure
#[derive(Clone)]
pub struct KeyPair {
    pub public_key: PublicKey,
    pub secret_key: SecretKey,
}

impl KeyPair {
    /// Generate a new Kyber-1024 key pair
    pub fn generate() -> Self {
        let (pk, sk) = keypair();
        Self {
            public_key: PublicKey(pk),
            secret_key: SecretKey(sk),
        }
    }
}

impl PublicKey {
    /// Get the raw bytes of the public key
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }
    
    /// Create from bytes
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        pqcrypto_kyber::kyber1024::PublicKey::from_bytes(bytes)
            .map(PublicKey)
            .map_err(|_| AetherError::Crypto("Invalid public key".to_string()))
    }
}

impl SecretKey {
    /// Get the raw bytes of the secret key
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }
    
    /// Create from bytes
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        pqcrypto_kyber::kyber1024::SecretKey::from_bytes(bytes)
            .map(SecretKey)
            .map_err(|_| AetherError::Crypto("Invalid secret key".to_string()))
    }
}

impl SharedSecret {
    /// Get the raw bytes of the shared secret
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

impl Ciphertext {
    /// Get the raw bytes of the ciphertext
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }
    
    /// Create from bytes
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        pqcrypto_kyber::kyber1024::Ciphertext::from_bytes(bytes)
            .map(Ciphertext)
            .map_err(|_| AetherError::Crypto("Invalid ciphertext".to_string()))
    }
}

/// Encapsulate: generate a ciphertext and shared secret for the given public key
pub fn encapsulate(public_key: &PublicKey) -> (Ciphertext, SharedSecret) {
    let (ss, ct) = pqcrypto_kyber::kyber1024::encapsulate(&public_key.0);
    (Ciphertext(ct), SharedSecret(ss))
}

/// Decapsulate: recover the shared secret from ciphertext using secret key
pub fn decapsulate(ciphertext: &Ciphertext, secret_key: &SecretKey) -> Result<SharedSecret> {
    let ss = pqcrypto_kyber::kyber1024::decapsulate(&ciphertext.0, &secret_key.0);
    Ok(SharedSecret(ss))
}

/// Get the size of a Kyber-1024 ciphertext in bytes
pub fn ciphertext_size() -> usize {
    1568 // Kyber-1024 ciphertext size
}

/// Get the size of a Kyber-1024 public key in bytes
pub fn public_key_size() -> usize {
    1568 // Kyber-1024 public key size
}

/// Get the size of a Kyber-1024 secret key in bytes
pub fn secret_key_size() -> usize {
    3168 // Kyber-1024 secret key size
}

/// Get the size of a shared secret in bytes
pub fn shared_secret_size() -> usize {
    32 // 256 bits
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_keypair_generation() {
        let kp = KeyPair::generate();
        assert_eq!(kp.public_key.as_bytes().len(), public_key_size());
        assert_eq!(kp.secret_key.as_bytes().len(), secret_key_size());
    }
    
    #[test]
    fn test_encapsulation_decapsulation() {
        let kp = KeyPair::generate();
        let (ct, ss1) = encapsulate(&kp.public_key);
        let ss2 = decapsulate(&ct, &kp.secret_key).unwrap();
        assert_eq!(ss1.as_bytes(), ss2.as_bytes());
        assert_eq!(ss1.as_bytes().len(), shared_secret_size());
    }
    
    #[test]
    fn test_serialization() {
        let kp = KeyPair::generate();
        let pk_bytes = kp.public_key.as_bytes();
        let pk2 = PublicKey::from_bytes(pk_bytes).unwrap();
        assert_eq!(pk2.as_bytes(), pk_bytes);
    }
}
