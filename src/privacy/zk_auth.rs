//! Phase 13: Zero-Knowledge Authorization (Schnorr Proofs)
//!
//! Implements a ZK handshake that allows the proxy to verify the user
//! possesses a valid "Ghost Key" without the user ever revealing it.

use curve25519_dalek::scalar::Scalar;
use curve25519_dalek::constants::RISTRETTO_BASEPOINT_TABLE;
use curve25519_dalek::ristretto::RistrettoPoint;
use sha3::{Sha3_512, Digest};
use rand::rngs::OsRng;

pub struct ZKAuthorization;

impl ZKAuthorization {
    /// Generates a random scalar for identity or proof
    pub fn random_scalar() -> Scalar {
        let mut bytes = [0u8; 64];
        rand::RngCore::fill_bytes(&mut OsRng, &mut bytes);
        Scalar::from_bytes_mod_order_wide(&bytes)
    }

    /// Generates a proof of knowledge for the secret key 'x'
    pub fn generate_proof(secret_x: &Scalar) -> (RistrettoPoint, Scalar) {
        // 1. Commitment: r <- Zq, T = g^r
        let r = Self::random_scalar();
        let t = &r * RISTRETTO_BASEPOINT_TABLE;
        
        // 2. Challenge: c = H(g, y, T)
        let y = secret_x * RISTRETTO_BASEPOINT_TABLE;
        let mut hasher = Sha3_512::new();
        hasher.update(RISTRETTO_BASEPOINT_TABLE.basepoint().compress().as_bytes());
        hasher.update(y.compress().as_bytes());
        hasher.update(t.compress().as_bytes());
        let challenge_hash = hasher.finalize();
        
        // Use the wide hash for a robust scalar
        let mut wide_hash = [0u8; 64];
        wide_hash.copy_from_slice(&challenge_hash);
        let c = Scalar::from_bytes_mod_order_wide(&wide_hash);
        
        // 3. Response: s = r + cx
        let s = r + (c * secret_x);
        
        (t, s)
    }

    /// Verifies the ZK proof against the public key 'y'
    pub fn verify_proof(public_y: &RistrettoPoint, t: &RistrettoPoint, s: &Scalar) -> bool {
        // Compute c = H(g, y, T)
        let mut hasher = Sha3_512::new();
        hasher.update(RISTRETTO_BASEPOINT_TABLE.basepoint().compress().as_bytes());
        hasher.update(public_y.compress().as_bytes());
        hasher.update(t.compress().as_bytes());
        let challenge_hash = hasher.finalize();
        
        let mut wide_hash = [0u8; 64];
        wide_hash.copy_from_slice(&challenge_hash);
        let c = Scalar::from_bytes_mod_order_wide(&wide_hash);
        
        // Check if g^s = T * y^c
        let lhs = s * RISTRETTO_BASEPOINT_TABLE;
        let rhs = t + (c * public_y);
        
        lhs == rhs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zk_handshake() {
        let secret_x = ZKAuthorization::random_scalar();
        let public_y = &secret_x * RISTRETTO_BASEPOINT_TABLE;
        
        let (t, s) = ZKAuthorization::generate_proof(&secret_x);
        assert!(ZKAuthorization::verify_proof(&public_y, &t, &s));
    }
}
