//! Digital signatures using Ed25519

use ring::{
    signature::{Ed25519KeyPair, KeyPair as RingKeyPair, UnparsedPublicKey, ED25519},
    rand::SystemRandom,
};
use crate::error::{AetherError, Result};

/// Ed25519 signing key pair
pub struct SigningKeyPair {
    key_pair: Ed25519KeyPair,
}

impl SigningKeyPair {
    /// Generate a new Ed25519 key pair
    pub fn generate() -> Result<Self> {
        let rng = SystemRandom::new();
        let pkcs8_bytes = Ed25519KeyPair::generate_pkcs8(&rng)
            .map_err(|_| AetherError::Crypto("Failed to generate key pair".to_string()))?;
        
        let key_pair = Ed25519KeyPair::from_pkcs8(pkcs8_bytes.as_ref())
            .map_err(|_| AetherError::Crypto("Failed to parse key pair".to_string()))?;
        
        Ok(Self { key_pair })
    }
    
    /// Get the public key bytes
    pub fn public_key_bytes(&self) -> &[u8] {
        self.key_pair.public_key().as_ref()
    }
}

/// Sign a message using Ed25519
pub fn sign_message(key_pair: &SigningKeyPair, message: &[u8]) -> Vec<u8> {
    key_pair.key_pair.sign(message).as_ref().to_vec()
}

/// Verify an Ed25519 signature
pub fn verify_signature(
    public_key: &[u8],
    message: &[u8],
    signature: &[u8],
) -> Result<()> {
    let public_key = UnparsedPublicKey::new(&ED25519, public_key);
    public_key
        .verify(message, signature)
        .map_err(|_| AetherError::Crypto("Signature verification failed".to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_sign_verify() {
        let kp = SigningKeyPair::generate().unwrap();
        let message = b"Important message";
        let signature = sign_message(&kp, message);
        
        assert!(verify_signature(kp.public_key_bytes(), message, &signature).is_ok());
    }
    
    #[test]
    fn test_wrong_message_fails() {
        let kp = SigningKeyPair::generate().unwrap();
        let message = b"Original message";
        let signature = sign_message(&kp, message);
        
        let wrong_message = b"Tampered message";
        assert!(verify_signature(kp.public_key_bytes(), wrong_message, &signature).is_err());
    }
}
