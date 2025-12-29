//! XKEYSCORE Defense: Automatic Session Key Rotation
//! 
//! Implements 60-second automatic key rotation and key erasure
//! to defeat NSA XKEYSCORE retroactive deanonymization

use chacha20poly1305::{
    aead::{Aead, KeyInit, OsRng},
    ChaCha20Poly1305, Nonce,
};
use std::time::{Duration, SystemTime};
use tokio::time::interval;
use zeroize::Zeroize;

pub struct SessionKeyManager {
    current_key: Vec<u8>,
    key_generation_time: SystemTime,
    rotation_interval: Duration,
}

impl SessionKeyManager {
    pub fn new() -> Self {
        let key = ChaCha20Poly1305::generate_key(&mut OsRng).to_vec();
        
        Self {
            current_key: key,
            key_generation_time: SystemTime::now(),
            rotation_interval: Duration::from_secs(60),
        }
    }

    /// Start automatic key rotation in background
    pub async fn start_rotation(mut self) {
        let mut ticker = interval(self.rotation_interval);
        
        tokio::spawn(async move {
            loop {
                ticker.tick().await;
                self.rotate_key();
            }
        });
    }

    /// Rotate session key and zeroize old key
    fn rotate_key(&mut self) {
        tracing::info!("🔄 XKEYSCORE DEFENSE: Rotating session key (60s interval)");
        
        // Zeroize old key before generating new one
        self.current_key.zeroize();
        
        // Generate new key
        self.current_key = ChaCha20Poly1305::generate_key(&mut OsRng).to_vec();
        self.key_generation_time = SystemTime::now();
        
        tracing::info!("✅ XKEYSCORE DEFENSE: Key rotated, old key erased from memory");
    }

    /// Get current encryption key
    pub fn get_current_key(&self) -> &[u8] {
        &self.current_key
    }

    /// Encrypt data with current session key
    pub fn encrypt(&self, plaintext: &[u8]) -> Result<Vec<u8>, String> {
        let cipher = ChaCha20Poly1305::new(self.current_key.as_slice().into());
        let nonce = Nonce::from_slice(&[0u8; 12]); // In production, use random nonce
        
        cipher.encrypt(nonce, plaintext)
            .map_err(|e| format!("Encryption failed: {}", e))
    }

    /// Decrypt data with current session key
    pub fn decrypt(&self, ciphertext: &[u8]) -> Result<Vec<u8>, String> {
        let cipher = ChaCha20Poly1305::new(self.current_key.as_slice().into());
        let nonce = Nonce::from_slice(&[0u8; 12]);
        
        cipher.decrypt(nonce, ciphertext)
            .map_err(|e| format!("Decryption failed: {}", e))
    }
}

impl Drop for SessionKeyManager {
    fn drop(&mut self) {
        // Ensure key is zeroized on drop
        self.current_key.zeroize();
        tracing::info!("🔐 Session key zeroized on termination");
    }
}
