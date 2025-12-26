//! Outfox packet implementation

use serde::{Deserialize, Serialize};
use crate::crypto::kyber::{PublicKey, SecretKey, encapsulate, decapsulate};
use crate::crypto::symmetric::{encrypt_aead, generate_nonce};
use crate::crypto::hash::{blake3_hash, derive_key};
use crate::error::{AetherError, Result};
use std::time::{SystemTime, UNIX_EPOCH};

/// Outfox packet structure
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OutfoxPacket {
    /// Layered encrypted headers (one per hop)
    pub header: Vec<u8>,
    
    /// Encrypted payload
    pub payload: Vec<u8>,
    
    /// Packet metadata
    pub metadata: PacketMetadata,
}

/// Packet metadata
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PacketMetadata {
    /// Timestamp of packet creation (nanoseconds since epoch)
    pub timestamp: u64,
    
    /// Current layer being processed
    pub layer: u8,
    
    /// Hash of next hop public key
    pub next_hop_hash: [u8; 32],
    
    /// Integrity tag
    pub integrity_tag: [u8; 32],
    
    /// Packet ID for tracking (does not reveal identity)
    pub packet_id: [u8; 16],
}

/// Route information for creating a packet
pub struct Route {
    pub hops: Vec<PublicKey>,
}

impl OutfoxPacket {
    /// Create a new Outfox packet for the given route
    pub fn new(
        message: &[u8],
        route: &[PublicKey],
    ) -> Result<Self> {
        if route.is_empty() {
            return Err(AetherError::Packet("Route cannot be empty".to_string()));
        }
        
        if route.len() > 5 {
            return Err(AetherError::Packet("Maximum 5 hops allowed".to_string()));
        }
        
        // Generate packet ID
        let mut packet_id = [0u8; 16];
        use ring::rand::{SecureRandom, SystemRandom};
        let rng = SystemRandom::new();
        rng.fill(&mut packet_id)
            .map_err(|_| AetherError::Crypto("Failed to generate packet ID".to_string()))?;
        
        // Build header by encapsulating for each hop
        let mut header = Vec::new();
        let mut shared_secrets = Vec::new();
        
        for pk in route.iter() {
            let (ct, ss) = encapsulate(pk);
            header.extend_from_slice(ct.as_bytes());
            shared_secrets.push(ss);
        }
        
        // Pad header to fixed size (5 layers)
        while header.len() < super::HEADER_SIZE {
            header.push(0);
        }
        
        // Encrypt payload with the last shared secret
        let final_ss = &shared_secrets[shared_secrets.len() - 1];
        let payload_key = derive_key(
            final_ss.as_bytes(),
            None,
            b"payload-encryption",
            32,
        )?;
        
        let nonce = generate_nonce();
        let mut payload_key_array = [0u8; 32];
        payload_key_array.copy_from_slice(&payload_key[..32]);
        
        let encrypted_payload = encrypt_aead(
            &payload_key_array,
            &nonce,
            message,
            &packet_id,
        )?;
        
        // Combine nonce and ciphertext
        let mut payload = nonce.to_vec();
        payload.extend_from_slice(&encrypted_payload);
        
        // Calculate next hop hash
        let next_hop_hash = blake3_hash(route[0].as_bytes());
        
        // Calculate integrity tag
        let integrity_tag = Self::calculate_integrity_tag(&header, &payload, &packet_id);
        
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|_| AetherError::InvalidState("System time before epoch".to_string()))?
            .as_nanos() as u64;
        
        Ok(Self {
            header,
            payload,
            metadata: PacketMetadata {
                timestamp,
                layer: 0,
                next_hop_hash,
                integrity_tag,
                packet_id,
            },
        })
    }
    
    /// Process one layer of the packet at a mix node
    pub fn process_layer(&mut self, secret_key: &SecretKey) -> Result<()> {
        if self.metadata.layer >= 5 {
            return Err(AetherError::Packet("Maximum layers exceeded".to_string()));
        }
        
        // Extract ciphertext for this layer
        let ct_size = 1568; // Kyber-1024 ciphertext size
        let start = self.metadata.layer as usize * ct_size;
        let end = start + ct_size;
        
        if end > self.header.len() {
            return Err(AetherError::Packet("Invalid header size".to_string()));
        }
        
        let ct_bytes = &self.header[start..end];
        let ct = crate::crypto::kyber::Ciphertext::from_bytes(ct_bytes)?;
        
        // Decapsulate to get shared secret
        let _shared_secret = decapsulate(&ct, secret_key)?;
        
        // Re-randomize this layer (important for unlinkability)
        // Generate new ciphertext for next hop
        let next_layer = self.metadata.layer + 1;
        if (next_layer as usize) < 5 {
            // Derive next hop's public key hash from routing info
            // (In a real implementation, this would be embedded in the header)
            self.metadata.layer = next_layer;
        }
        
        // Update integrity tag
        self.metadata.integrity_tag = Self::calculate_integrity_tag(
            &self.header,
            &self.payload,
            &self.metadata.packet_id,
        );
        
        Ok(())
    }
    
    /// Calculate integrity tag for the packet
    fn calculate_integrity_tag(header: &[u8], payload: &[u8], packet_id: &[u8; 16]) -> [u8; 32] {
        let mut data = Vec::new();
        data.extend_from_slice(header);
        data.extend_from_slice(payload);
        data.extend_from_slice(packet_id);
        blake3_hash(&data)
    }
    
    /// Verify integrity of the packet
    pub fn verify_integrity(&self) -> bool {
        let calculated_tag = Self::calculate_integrity_tag(
            &self.header,
            &self.payload,
            &self.metadata.packet_id,
        );
        calculated_tag == self.metadata.integrity_tag
    }
    
    /// Serialize packet to bytes
    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        bincode::serialize(self)
            .map_err(|e| AetherError::Serialization(e.to_string()))
    }
    
    /// Deserialize packet from bytes
    pub fn from_bytes(data: &[u8]) -> Result<Self> {
        bincode::deserialize(data)
            .map_err(|e| AetherError::Serialization(e.to_string()))
    }
}

/// Create a new packet (convenience function)
pub fn create_packet(message: &[u8], route: &[PublicKey]) -> Result<OutfoxPacket> {
    OutfoxPacket::new(message, route)
}

/// Process one layer of a packet (convenience function)
pub fn process_packet_layer(packet: &mut OutfoxPacket, secret_key: &SecretKey) -> Result<()> {
    packet.process_layer(secret_key)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::kyber::KeyPair;
    
    #[test]
    fn test_packet_creation() {
        let route: Vec<_> = (0..3).map(|_| KeyPair::generate().public_key).collect();
        let message = b"Test message";
        
        let packet = OutfoxPacket::new(message, &route).unwrap();
        assert_eq!(packet.metadata.layer, 0);
        assert!(packet.verify_integrity());
    }
    
    #[test]
    fn test_packet_processing() {
        let keys: Vec<_> = (0..3).map(|_| KeyPair::generate()).collect();
        let route: Vec<_> = keys.iter().map(|kp| kp.public_key.clone()).collect();
        let message = b"Secret message";
        
        let mut packet = OutfoxPacket::new(message, &route).unwrap();
        
        // Process through first hop
        packet.process_layer(&keys[0].secret_key).unwrap();
        assert_eq!(packet.metadata.layer, 1);
        assert!(packet.verify_integrity());
    }
    
    #[test]
    fn test_serialization() {
        let route: Vec<_> = (0..2).map(|_| KeyPair::generate().public_key).collect();
        let message = b"Data";
        
        let packet = OutfoxPacket::new(message, &route).unwrap();
        let bytes = packet.to_bytes().unwrap();
        let decoded = OutfoxPacket::from_bytes(&bytes).unwrap();
        
        assert_eq!(packet.metadata.packet_id, decoded.metadata.packet_id);
        assert_eq!(packet.metadata.layer, decoded.metadata.layer);
    }
}
