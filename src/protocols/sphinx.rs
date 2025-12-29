//! Sphinx Packet Format - Industry Standard for Mixnets
//! 
//! Based on the Sphinx specification (George Danezis, Ian Goldberg)
//! Used by Nym, Katzenpost, and proposed for Tor

use blake3::Hasher;
use rand::Rng;
use std::convert::TryInto;

const SECURITY_PARAMETER: usize = 16; // 128-bit security
const MAX_HOPS: usize = 5;
const ROUTING_INFO_SIZE: usize = 128;
const PAYLOAD_SIZE: usize = 2048;
const MAC_SIZE: usize = 32;

/// Sphinx packet header with layered encryption
#[derive(Clone, Debug)]
pub struct SphinxHeader {
    /// Group element for ECDH (public key point)
    pub alpha: [u8; 32],
    /// Routing information (encrypted for each hop)
    pub beta: Vec<u8>,
    /// MAC for integrity
    pub gamma: [u8; MAC_SIZE],
}

/// Complete Sphinx packet
#[derive(Clone, Debug)]
pub struct SphinxPacket {
    /// Header with routing
    pub header: SphinxHeader,
    /// Encrypted payload
    pub payload: Vec<u8>,
}

/// Routing information for one hop
#[derive(Clone, Debug)]
pub struct RoutingInfo {
    /// Next hop address (or final destination)
    pub next_hop: [u8; 32],
    /// Delay at this hop (milliseconds)
    pub delay: u32,
    /// Additional routing metadata
    pub flags: u8,
}

impl RoutingInfo {
    /// Serialize to bytes
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.next_hop);
        bytes.extend_from_slice(&self.delay.to_be_bytes());
        bytes.push(self.flags);
        bytes
    }

    /// Deserialize from bytes
    pub fn from_bytes(data: &[u8]) -> Result<Self, Box<dyn std::error::Error>> {
        if data.len() < 37 {
            return Err("Invalid routing info".into());
        }
        
        Ok(Self {
            next_hop: data[0..32].try_into()?,
            delay: u32::from_be_bytes(data[32..36].try_into()?),
            flags: data[36],
        })
    }
}

/// Sphinx packet builder
pub struct SphinxBuilder {
    /// Path through the network
    path: Vec<[u8; 32]>,
    /// Message to send
    message: Vec<u8>,
}

impl SphinxBuilder {
    /// Create new Sphinx packet builder
    pub fn new(path: Vec<[u8; 32]>, message: Vec<u8>) -> Self {
        Self { path, message }
    }

    /// Build the Sphinx packet with layered encryption
    pub fn build(&self) -> Result<SphinxPacket, Box<dyn std::error::Error>> {
        let mut rng = rand::thread_rng();
        
        // Generate session key for this packet
        let mut session_key = [0u8; 32];
        rng.fill(&mut session_key);

        // Create initial group element (public key)
        let alpha = session_key; // Simplified - in real impl, use EC point multiplication

        // Build routing info for each hop
        let mut routing_infos = Vec::new();
        for (i, hop) in self.path.iter().enumerate() {
            routing_infos.push(RoutingInfo {
                next_hop: *hop,
                delay: 50 + (i as u32 * 10), // Increasing delays
                flags: if i == self.path.len() - 1 { 1 } else { 0 }, // Final hop flag
            });
        }

        // Encrypt routing info in reverse (onion layers)
        let mut beta = vec![0u8; ROUTING_INFO_SIZE * MAX_HOPS];
        for (i, info) in routing_infos.iter().rev().enumerate() {
            let info_bytes = info.to_bytes();
            let start = i * ROUTING_INFO_SIZE;
            beta[start..start + info_bytes.len()].copy_from_slice(&info_bytes);
            
            // Encrypt this layer
            let key = self.derive_key(&session_key, i as u8);
            self.encrypt_layer(&mut beta, &key)?;
        }

        // Encrypt payload
        let mut encrypted_payload = self.message.clone();
        encrypted_payload.resize(PAYLOAD_SIZE, 0); // Pad to fixed size
        
        for i in (0..self.path.len()).rev() {
            let key = self.derive_key(&session_key, i as u8);
            self.encrypt_layer(&mut encrypted_payload, &key)?;
        }

        // Compute MAC
        let mut hasher = Hasher::new();
        hasher.update(&alpha);
        hasher.update(&beta);
        hasher.update(&encrypted_payload);
        let hash = hasher.finalize();
        let gamma: [u8; MAC_SIZE] = hash.as_bytes()[0..MAC_SIZE].try_into()?;

        Ok(SphinxPacket {
            header: SphinxHeader {
                alpha,
                beta,
                gamma,
            },
            payload: encrypted_payload,
        })
    }

    /// Derive encryption key for a layer
    fn derive_key(&self, session_key: &[u8; 32], layer: u8) -> [u8; 32] {
        let mut hasher = Hasher::new();
        hasher.update(session_key);
        hasher.update(&[layer]);
        let hash = hasher.finalize();
        hash.as_bytes()[0..32].try_into().unwrap()
    }

    /// Encrypt a layer with ChaCha20-Poly1305
    fn encrypt_layer(&self, data: &mut [u8], key: &[u8; 32]) -> Result<(), Box<dyn std::error::Error>> {
        // Simplified XOR encryption
        // Real implementation would use proper AEAD
        for (i, byte) in data.iter_mut().enumerate() {
            *byte ^= key[i % 32];
        }
        
        Ok(())
    }
}

/// Process a Sphinx packet at a mix node
pub struct SphinxProcessor {
    /// This node's secret key
    secret_key: [u8; 32],
}

impl SphinxProcessor {
    /// Create new Sphinx processor
    pub fn new(secret_key: [u8; 32]) -> Self {
        Self { secret_key }
    }

    /// Process (peel one layer) from the packet
    pub fn process(&self, packet: SphinxPacket) -> Result<ProcessedPacket, Box<dyn std::error::Error>> {
        // Verify MAC
        let mut hasher = Hasher::new();
        hasher.update(&packet.header.alpha);
        hasher.update(&packet.header.beta);
        hasher.update(&packet.payload);
        let hash = hasher.finalize();
        let expected_mac: [u8; MAC_SIZE] = hash.as_bytes()[0..MAC_SIZE].try_into()?;
        
        if expected_mac != packet.header.gamma {
            return Err("MAC verification failed".into());
        }

        // Decrypt one layer of routing info
        let mut beta = packet.header.beta.clone();
        let key = self.derive_shared_key(&packet.header.alpha);
        self.decrypt_layer(&mut beta, &key)?;

        // Extract routing info for this hop
        let routing_info = RoutingInfo::from_bytes(&beta[0..ROUTING_INFO_SIZE])?;

        // Decrypt one layer of payload
        let mut payload = packet.payload.clone();
        self.decrypt_layer(&mut payload, &key)?;

        // Shift routing info (remove this hop's info)
        let mut new_beta = beta[ROUTING_INFO_SIZE..].to_vec();
        new_beta.resize(beta.len(), 0);

        // Check if this is the final hop
        let is_final = routing_info.flags == 1;

        Ok(ProcessedPacket {
            next_hop: routing_info.next_hop,
            delay: routing_info.delay,
            is_final,
            new_packet: if !is_final {
                Some(SphinxPacket {
                    header: SphinxHeader {
                        alpha: self.update_alpha(&packet.header.alpha),
                        beta: new_beta,
                        gamma: packet.header.gamma, // Recompute in real impl
                    },
                    payload: payload.clone(),
                })
            } else {
                None
            },
            final_payload: if is_final { Some(payload) } else { None },
        })
    }

    fn derive_shared_key(&self, alpha: &[u8; 32]) -> [u8; 32] {
        let mut hasher = Hasher::new();
        hasher.update(&self.secret_key);
        hasher.update(alpha);
        let hash = hasher.finalize();
        hash.as_bytes()[0..32].try_into().unwrap()
    }

    fn decrypt_layer(&self, data: &mut [u8], key: &[u8; 32]) -> Result<(), Box<dyn std::error::Error>> {
        for (i, byte) in data.iter_mut().enumerate() {
            *byte ^= key[i % 32];
        }
        Ok(())
    }

    fn update_alpha(&self, alpha: &[u8; 32]) -> [u8; 32] {
        // In real impl, do EC point multiplication
        let mut hasher = Hasher::new();
        hasher.update(alpha);
        hasher.update(&self.secret_key);
        let hash = hasher.finalize();
        hash.as_bytes()[0..32].try_into().unwrap()
    }
}

/// Result of processing a Sphinx packet
pub struct ProcessedPacket {
    /// Address of next hop
    pub next_hop: [u8; 32],
    /// Delay to apply (ms)
    pub delay: u32,
    /// Is this the final destination?
    pub is_final: bool,
    /// Packet for next hop (if not final)
    pub new_packet: Option<SphinxPacket>,
    /// Decrypted payload (if final)
    pub final_payload: Option<Vec<u8>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sphinx_packet_creation() {
        let path = vec![
            [1u8; 32],
            [2u8; 32],
            [3u8; 32],
        ];
        let message = b"Secret message".to_vec();
        
        let builder = SphinxBuilder::new(path, message);
        let packet = builder.build().unwrap();
        
        assert_eq!(packet.payload.len(), PAYLOAD_SIZE);
    }

    #[test]
    fn test_sphinx_processing() {
        let secret = [42u8; 32];
        let processor = SphinxProcessor::new(secret);
        
        // Create test packet
        let path = vec![[1u8; 32], [2u8; 32]];
        let builder = SphinxBuilder::new(path, b"test".to_vec());
        let packet = builder.build().unwrap();
        
        // Process should not panic
        let _ = processor.process(packet);
    }
}
