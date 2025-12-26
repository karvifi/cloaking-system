//! Symmetric encryption using XChaCha20-Poly1305

use chacha20poly1305::{
    aead::{Aead, KeyInit},
    XChaCha20Poly1305, XNonce
};
use crate::error::{AetherError, Result};

/// Encrypt data using XChaCha20-Poly1305 AEAD
pub fn encrypt_aead(
    key: &[u8; 32],
    nonce: &[u8; 24],
    plaintext: &[u8],
    associated_data: &[u8],
) -> Result<Vec<u8>> {
    let cipher = XChaCha20Poly1305::new(key.into());
    let nonce = XNonce::from_slice(nonce);
    
    cipher
        .encrypt(nonce, chacha20poly1305::aead::Payload {
            msg: plaintext,
            aad: associated_data,
        })
        .map_err(|_| AetherError::Crypto("Encryption failed".to_string()))
}

/// Decrypt data using XChaCha20-Poly1305 AEAD
pub fn decrypt_aead(
    key: &[u8; 32],
    nonce: &[u8; 24],
    ciphertext: &[u8],
    associated_data: &[u8],
) -> Result<Vec<u8>> {
    let cipher = XChaCha20Poly1305::new(key.into());
    let nonce = XNonce::from_slice(nonce);
    
    cipher
        .decrypt(nonce, chacha20poly1305::aead::Payload {
            msg: ciphertext,
            aad: associated_data,
        })
        .map_err(|_| AetherError::Crypto("Decryption failed".to_string()))
}

/// Generate a random nonce for XChaCha20-Poly1305
pub fn generate_nonce() -> [u8; 24] {
    let mut nonce = [0u8; 24];
    use ring::rand::{SecureRandom, SystemRandom};
    let rng = SystemRandom::new();
    rng.fill(&mut nonce).expect("Failed to generate nonce");
    nonce
}

/// Generate a random symmetric key
pub fn generate_key() -> [u8; 32] {
    let mut key = [0u8; 32];
    use ring::rand::{SecureRandom, SystemRandom};
    let rng = SystemRandom::new();
    rng.fill(&mut key).expect("Failed to generate key");
    key
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_encrypt_decrypt() {
        let key = generate_key();
        let nonce = generate_nonce();
        let plaintext = b"Hello, Aether!";
        let aad = b"additional data";
        
        let ciphertext = encrypt_aead(&key, &nonce, plaintext, aad).unwrap();
        let decrypted = decrypt_aead(&key, &nonce, &ciphertext, aad).unwrap();
        
        assert_eq!(plaintext, &decrypted[..]);
    }
    
    #[test]
    fn test_wrong_key_fails() {
        let key1 = generate_key();
        let key2 = generate_key();
        let nonce = generate_nonce();
        let plaintext = b"secret";
        
        let ciphertext = encrypt_aead(&key1, &nonce, plaintext, b"").unwrap();
        let result = decrypt_aead(&key2, &nonce, &ciphertext, b"");
        
        assert!(result.is_err());
    }
}
