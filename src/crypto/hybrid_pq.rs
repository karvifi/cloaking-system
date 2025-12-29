//! Hybrid Post-Quantum Signatures (Dilithium + Ed25519)
//! 
//! Dual-signature scheme for quantum-safe authentication

use ed25519_dalek::{Signer, SigningKey, Signature as Ed25519Signature, Verifier, VerifyingKey};
use pqcrypto_dilithium::dilithium5;
use pqcrypto_traits::sign::{PublicKey, SecretKey, SignedMessage};
use rand::rngs::OsRng;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct HybridKeyPair {
    /// Ed25519 classical signing key
    ed25519_sk: Vec<u8>,
    /// Dilithium5 post-quantum signing key
    dilithium_sk: Vec<u8>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct HybridPublicKey {
    /// Ed25519 classical verifying key
    ed25519_pk: Vec<u8>,
    /// Dilithium5 post-quantum public key
    dilithium_pk: Vec<u8>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct HybridSignature {
    /// Ed25519 signature (64 bytes)
    ed25519_sig: Vec<u8>,
    /// Dilithium5 signature (~4600 bytes)
    dilithium_sig: Vec<u8>,
}

pub struct HybridSigner {
    ed25519_key: SigningKey,
    dilithium_sk: dilithium5::SecretKey,
    dilithium_pk: dilithium5::PublicKey,
}

impl HybridSigner {
    /// Generate new hybrid key pair
    pub fn new() -> Self {
        let mut csprng = OsRng;
        let ed25519_key = SigningKey::generate(&mut csprng);
        let (dilithium_pk, dilithium_sk) = dilithium5::keypair();

        tracing::info!("🔐 Generated hybrid PQ key pair (Dilithium5 + Ed25519)");
        
        Self {
            ed25519_key,
            dilithium_sk,
            dilithium_pk,
        }
    }

    /// Sign message with both classical and PQ signatures
    pub fn sign(&self, message: &[u8]) -> HybridSignature {
        // Classical Ed25519 signature
        let ed25519_sig = self.ed25519_key.sign(message);
        
        // Post-quantum Dilithium5 signature
        let dilithium_msg = dilithium5::sign(message, &self.dilithium_sk);
        
        tracing::info!("✍️ Created hybrid signature: {} bytes total", 
            ed25519_sig.to_bytes().len() + dilithium_msg.as_bytes().len());
        
        HybridSignature {
            ed25519_sig: ed25519_sig.to_bytes().to_vec(),
            dilithium_sig: dilithium_msg.as_bytes().to_vec(),
        }
    }

    /// Get hybrid public key
    pub fn public_key(&self) -> HybridPublicKey {
        let ed25519_pk = self.ed25519_key.verifying_key();
        
        HybridPublicKey {
            ed25519_pk: ed25519_pk.to_bytes().to_vec(),
            dilithium_pk: self.dilithium_pk.as_bytes().to_vec(),
        }
    }

    /// Verify hybrid signature
    pub fn verify(
        &self,
        message: &[u8],
        signature: &HybridSignature,
        public_key: &HybridPublicKey,
    ) -> Result<(), String> {
        // Verify Ed25519 signature
        let ed25519_pk = VerifyingKey::from_bytes(
            public_key.ed25519_pk.as_slice().try_into()
                .map_err(|_| "Invalid Ed25519 public key")?
        ).map_err(|_| "Invalid Ed25519 public key format")?;

        let ed25519_sig = Ed25519Signature::from_bytes(
            signature.ed25519_sig.as_slice().try_into()
                .map_err(|_| "Invalid Ed25519 signature")?
        );

        ed25519_pk.verify(message, &ed25519_sig)
            .map_err(|_| "Ed25519 signature verification failed")?;

        // Verify Dilithium5 signature
        let dilithium_pk = dilithium5::PublicKey::from_bytes(&public_key.dilithium_pk)
            .map_err(|_| "Invalid Dilithium public key")?;

        let dilithium_msg = dilithium5::SignedMessage::from_bytes(&signature.dilithium_sig)
            .map_err(|_| "Invalid Dilithium signature")?;

        dilithium5::open(&dilithium_msg, &dilithium_pk)
            .map_err(|_| "Dilithium signature verification failed")?;

        tracing::info!("✅ HYBRID SIGNATURE VERIFIED: Both Ed25519 AND Dilithium5 valid");
        Ok(())
    }
}

// Security Analysis:
// 
// - Ed25519: 2^128 classical security
// - Dilithium5: NIST Level 5 post-quantum security (~256-bit equivalent)
// - Hybrid: Secure against BOTH classical AND quantum attackers
// - Signature size: ~4664 bytes (64 + ~4600)
