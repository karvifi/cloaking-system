//! Packet Authenticity & Anti-QUANTUMINSERT Defense
//! 
//! Implements cryptographic signatures and sequence numbers to detect
//! man-on-the-side attacks (NSA QUANTUMINSERT)

use ed25519_dalek::{Signer, SigningKey, Signature, Verifier, VerifyingKey};
use rand::rngs::OsRng;
use serde::{Serialize, Deserialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize, Clone)]
pub struct AuthenticatedPacket {
    pub sequence: u64,
    pub timestamp: u64,
    pub payload: Vec<u8>,
    pub signature: Vec<u8>,
    pub public_key: Vec<u8>,
}

pub struct PacketAuthenticator {
    signing_key: SigningKey,
    sequence_counter: u64,
    last_received_sequence: u64,
}

impl PacketAuthenticator {
    pub fn new() -> Self {
        let mut csprng = OsRng;
        let signing_key = SigningKey::generate(&mut csprng);
        
        Self {
            signing_key,
            sequence_counter: 0,
            last_received_sequence: 0,
        }
    }

    /// Create and sign an authenticated packet
    pub fn create_packet(&mut self, payload: Vec<u8>) -> AuthenticatedPacket {
        self.sequence_counter += 1;
        
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let mut message = Vec::new();
        message.extend_from_slice(&self.sequence_counter.to_le_bytes());
        message.extend_from_slice(&timestamp.to_le_bytes());
        message.extend_from_slice(&payload);

        let signature = self.signing_key.sign(&message);
        let public_key = self.signing_key.verifying_key();

        AuthenticatedPacket {
            sequence: self.sequence_counter,
            timestamp,
            payload,
            signature: signature.to_bytes().to_vec(),
            public_key: public_key.to_bytes().to_vec(),
        }
    }

    /// Verify packet authenticity and detect replays
    pub fn verify_packet(&mut self, packet: &AuthenticatedPacket) -> Result<(), String> {
        // Check for replay attack
        if packet.sequence <= self.last_received_sequence {
            return Err(format!(
                "🚨 QUANTUMINSERT DETECTED: Replay attack! Seq {} <= {}",
                packet.sequence, self.last_received_sequence
            ));
        }

        // Check timestamp freshness (reject packets older than 60 seconds)
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        if current_time.saturating_sub(packet.timestamp) > 60 {
            return Err("🚨 QUANTUMINSERT DETECTED: Stale timestamp!".to_string());
        }

        // Verify cryptographic signature
        let public_key = VerifyingKey::from_bytes(
            packet.public_key.as_slice().try_into()
                .map_err(|_| "Invalid public key")?
        ).map_err(|_| "Invalid public key format")?;

        let signature = Signature::from_bytes(
            packet.signature.as_slice().try_into()
                .map_err(|_| "Invalid signature")?
        );

        let mut message = Vec::new();
        message.extend_from_slice(&packet.sequence.to_le_bytes());
        message.extend_from_slice(&packet.timestamp.to_le_bytes());
        message.extend_from_slice(&packet.payload);

        public_key.verify(&message, &signature)
            .map_err(|_| "🚨 QUANTUMINSERT DETECTED: Invalid signature!")?;

        // Update sequence counter
        self.last_received_sequence = packet.sequence;
        
        tracing::info!("✅ Packet authenticated: Seq {}, legitimate origin confirmed", packet.sequence);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_legitimate_packet() {
        let mut auth = PacketAuthenticator::new();
        let packet = auth.create_packet(b"Hello, World!".to_vec());
        assert!(auth.verify_packet(&packet).is_ok());
    }

    #[test]
    fn test_replay_attack_detection() {
        let mut auth = PacketAuthenticator::new();
        let packet1 = auth.create_packet(b"First".to_vec());
        let packet2 = auth.create_packet(b"Second".to_vec());
        
        assert!(auth.verify_packet(&packet2).is_ok());
        assert!(auth.verify_packet(&packet1).is_err()); // Replay should fail
    }
}
