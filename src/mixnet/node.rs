//! Mix node implementation

use crate::config::AetherConfig;
use crate::crypto::kyber::{KeyPair, SecretKey};
use crate::protocols::OutfoxPacket;
use crate::error::{AetherError, Result};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Role of a node in the network
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum NodeRole {
    /// Entry gateway (first hop)
    EntryGateway,
    
    /// Mix node (intermediate hop)
    MixNode,
    
    /// Exit gateway (last hop)
    ExitGateway,
    
    /// Validator (consensus and reputation)
    Validator,
}

/// Information about a mix node
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NodeInfo {
    /// Unique node identifier
    pub id: [u8; 32],
    
    /// Layer in the stratified topology (1-5)
    pub layer: usize,
    
    /// Node role
    pub role: NodeRole,
    
    /// Reputation score (0.0 to 1.0)
    pub reputation: f64,
    
    /// Staked tokens
    pub stake: u64,
    
    /// Network address
    pub address: String,
    
    /// Public key for key encapsulation
    pub public_key_bytes: Vec<u8>,
}

/// Mix node structure
pub struct MixNode {
    /// Node information
    pub info: NodeInfo,
    
    /// Secret key for decryption
    secret_key: SecretKey,
    
    /// Configuration
    config: Arc<AetherConfig>,
    
    /// Packet queues
    incoming_queue: Arc<RwLock<VecDeque<OutfoxPacket>>>,
    outgoing_queue: Arc<RwLock<VecDeque<OutfoxPacket>>>,
    
    /// Statistics
    packets_processed: Arc<RwLock<u64>>,
    total_latency_ms: Arc<RwLock<u64>>,
}

impl MixNode {
    /// Create a new mix node
    pub fn new(
        layer: usize,
        role: NodeRole,
        stake: u64,
        address: String,
        config: Arc<AetherConfig>,
    ) -> Result<Self> {
        if layer == 0 || layer > config.mixnet_layers {
            return Err(AetherError::Config(
                format!("Invalid layer {}. Must be 1-{}", layer, config.mixnet_layers)
            ));
        }
        
        // Generate node identity
        let key_pair = KeyPair::generate();
        let mut id = [0u8; 32];
        id.copy_from_slice(&crate::crypto::hash::blake3_hash(
            key_pair.public_key.as_bytes()
        ));
        
        let info = NodeInfo {
            id,
            layer,
            role,
            reputation: 1.0, // Start with full reputation
            stake,
            address,
            public_key_bytes: key_pair.public_key.as_bytes().to_vec(),
        };
        
        Ok(Self {
            info,
            secret_key: key_pair.secret_key,
            config,
            incoming_queue: Arc::new(RwLock::new(VecDeque::new())),
            outgoing_queue: Arc::new(RwLock::new(VecDeque::new())),
            packets_processed: Arc::new(RwLock::new(0)),
            total_latency_ms: Arc::new(RwLock::new(0)),
        })
    }
    
    /// Start the mix node
    pub async fn run(&self) -> Result<()> {
        tracing::info!(
            "Starting mix node {} on layer {} as {:?}",
            hex::encode(&self.info.id[..8]),
            self.info.layer,
            self.info.role
        );
        
        // Spawn packet processing task
        let self_clone = self.clone_arc_fields();
        tokio::spawn(async move {
            loop {
                if let Err(e) = self_clone.process_packets().await {
                    tracing::error!("Packet processing error: {}", e);
                }
                tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
            }
        });
        
        // Spawn cover traffic generator
        let self_clone = self.clone_arc_fields();
        tokio::spawn(async move {
            loop {
                self_clone.generate_cover_traffic().await;
                let delay = rand::random::<u64>() % 1000;
                tokio::time::sleep(tokio::time::Duration::from_millis(delay)).await;
            }
        });
        
        Ok(())
    }
    
    /// Process packets from the incoming queue
    async fn process_packets(&self) -> Result<()> {
        let mut incoming = self.incoming_queue.write().await;
        
        if let Some(mut packet) = incoming.pop_front() {
            drop(incoming); // Release lock
            
            // Verify integrity
            if !packet.verify_integrity() {
                tracing::warn!("Dropping packet with invalid integrity");
                return Ok(());
            }
            
            // Process the packet layer
            packet.process_layer(&self.secret_key)?;
            
            // Apply mixing delay (stop-and-go)
            let delay_ms = self.calculate_mixing_delay();
            tokio::time::sleep(tokio::time::Duration::from_millis(delay_ms)).await;
            
            // Add to outgoing queue
            let mut outgoing = self.outgoing_queue.write().await;
            outgoing.push_back(packet);
            
            // Update statistics
            let mut processed = self.packets_processed.write().await;
            *processed += 1;
            
            let mut latency = self.total_latency_ms.write().await;
            *latency += delay_ms;
        }
        
        Ok(())
    }
    
    /// Calculate mixing delay using Poisson distribution
    fn calculate_mixing_delay(&self) -> u64 {
        use rand_distr::{Distribution, Exp};
        let mut rng = rand::thread_rng();
        let exp = Exp::new(1.0 / self.config.poisson_lambda).unwrap();
        let delay = exp.sample(&mut rng);
        delay as u64
    }
    
    /// Generate cover traffic
    async fn generate_cover_traffic(&self) {
        if rand::random::<f64>() < self.config.cover_traffic_ratio {
            // Create a dummy packet (implementation simplified)
            tracing::debug!("Generating cover traffic");
            // In a real implementation, dummy packets would be created here
        }
    }
    
    /// Helper to clone Arc fields for spawning tasks
    fn clone_arc_fields(&self) -> Self {
        Self {
            info: self.info.clone(),
            secret_key: self.secret_key.clone(),
            config: Arc::clone(&self.config),
            incoming_queue: Arc::clone(&self.incoming_queue),
            outgoing_queue: Arc::clone(&self.outgoing_queue),
            packets_processed: Arc::clone(&self.packets_processed),
            total_latency_ms: Arc::clone(&self.total_latency_ms),
        }
    }
    
    /// Add a packet to the incoming queue
    pub async fn receive_packet(&self, packet: OutfoxPacket) {
        let mut queue = self.incoming_queue.write().await;
        queue.push_back(packet);
    }
    
    /// Get a packet from the outgoing queue
    pub async fn send_packet(&self) -> Option<OutfoxPacket> {
        let mut queue = self.outgoing_queue.write().await;
        queue.pop_front()
    }
    
    /// Get node statistics
    pub async fn get_stats(&self) -> NodeStats {
        let processed = *self.packets_processed.read().await;
        let total_latency = *self.total_latency_ms.read().await;
        
        NodeStats {
            packets_processed: processed,
            average_latency_ms: if processed > 0 {
                total_latency / processed
            } else {
                0
            },
            reputation: self.info.reputation,
            queue_size: self.incoming_queue.read().await.len(),
        }
    }
}

/// Node statistics
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NodeStats {
    pub packets_processed: u64,
    pub average_latency_ms: u64,
    pub reputation: f64,
    pub queue_size: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_mix_node_creation() {
        let config = Arc::new(AetherConfig::default());
        let node = MixNode::new(
            2,
            NodeRole::MixNode,
            1000,
            "127.0.0.1:9090".to_string(),
            config,
        ).unwrap();
        
        assert_eq!(node.info.layer, 2);
        assert_eq!(node.info.stake, 1000);
        assert_eq!(node.info.reputation, 1.0);
    }
}
