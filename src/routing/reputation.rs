//! Reputation system for node selection

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Node reputation tracking
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NodeReputation {
    /// Node ID
    pub node_id: [u8; 32],
    
    /// Current reputation score (0.0 to 1.0)
    pub score: f64,
    
    /// Number of packets successfully processed
    pub packets_processed: u64,
    
    /// Number of failures/timeouts
    pub failures: u64,
    
    /// Average latency in milliseconds
    pub avg_latency_ms: u64,
    
    /// Uptime percentage
    pub uptime: f64,
}

/// Reputation system managing all nodes
pub struct ReputationSystem {
    reputations: HashMap<[u8; 32], NodeReputation>,
    
    /// Decay factor for reputation (per hour)
    decay_factor: f64,
    
    /// Reward for successful packet processing
    success_reward: f64,
    
    /// Penalty for failures
    failure_penalty: f64,
}

impl ReputationSystem {
    pub fn new() -> Self {
        Self {
            reputations: HashMap::new(),
            decay_factor: 0.95,
            success_reward: 0.01,
            failure_penalty: 0.05,
        }
    }
    
    /// Initialize reputation for a new node
    pub fn init_node(&mut self, node_id: [u8; 32], initial_score: f64) {
        self.reputations.insert(
            node_id,
            NodeReputation {
                node_id,
                score: initial_score,
                packets_processed: 0,
                failures: 0,
                avg_latency_ms: 0,
                uptime: 1.0,
            },
        );
    }
    
    /// Record successful packet processing
    pub fn record_success(&mut self, node_id: &[u8; 32], latency_ms: u64) {
        if let Some(rep) = self.reputations.get_mut(node_id) {
            rep.packets_processed += 1;
            
            // Update average latency
            let total_latency = rep.avg_latency_ms * rep.packets_processed + latency_ms;
            rep.avg_latency_ms = total_latency / (rep.packets_processed + 1);
            
            // Increase reputation
            rep.score = (rep.score + self.success_reward).min(1.0);
        }
    }
    
    /// Record failure (timeout, invalid packet, etc.)
    pub fn record_failure(&mut self, node_id: &[u8; 32]) {
        if let Some(rep) = self.reputations.get_mut(node_id) {
            rep.failures += 1;
            
            // Decrease reputation
            rep.score = (rep.score - self.failure_penalty).max(0.0);
        }
    }
    
    /// Apply time-based decay to all reputations
    pub fn apply_decay(&mut self) {
        for rep in self.reputations.values_mut() {
            rep.score *= self.decay_factor;
        }
    }
    
    /// Get reputation for a node
    pub fn get_reputation(&self, node_id: &[u8; 32]) -> Option<&NodeReputation> {
        self.reputations.get(node_id)
    }
    
    /// Get nodes with reputation above threshold
    pub fn get_high_reputation_nodes(&self, threshold: f64) -> Vec<[u8; 32]> {
        self.reputations
            .iter()
            .filter(|(_, rep)| rep.score >= threshold)
            .map(|(&id, _)| id)
            .collect()
    }
    
    /// Slash reputation (for malicious behavior)
    pub fn slash(&mut self, node_id: &[u8; 32], percentage: f64) {
        if let Some(rep) = self.reputations.get_mut(node_id) {
            rep.score *= 1.0 - percentage.min(1.0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_reputation_system() {
        let mut system = ReputationSystem::new();
        let node_id = [1u8; 32];
        
        system.init_node(node_id, 0.5);
        
        // Record successes
        for _ in 0..10 {
            system.record_success(&node_id, 50);
        }
        
        let rep = system.get_reputation(&node_id).unwrap();
        assert!(rep.score > 0.5);
        assert_eq!(rep.packets_processed, 10);
        
        // Record failure
        system.record_failure(&node_id);
        let rep = system.get_reputation(&node_id).unwrap();
        assert_eq!(rep.failures, 1);
    }
    
    #[test]
    fn test_slashing() {
        let mut system = ReputationSystem::new();
        let node_id = [2u8; 32];
        
        system.init_node(node_id, 1.0);
        system.slash(&node_id, 0.5); // Slash 50%
        
        let rep = system.get_reputation(&node_id).unwrap();
        assert_eq!(rep.score, 0.5);
    }
}
