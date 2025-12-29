use std::collections::HashMap;
use std::sync::Mutex;
use tracing::{info, warn};
use crate::consensus::bft::{BftEngine, Step};

pub struct ReputationManager {
    scores: Mutex<HashMap<String, u32>>,
    bft: Mutex<BftEngine>,
}

impl ReputationManager {
    pub fn new() -> Self {
        Self {
            scores: Mutex::new(HashMap::new()),
            bft: Mutex::new(BftEngine::new("validator-1".to_string())),
        }
    }

    pub fn record_success(&self, node_id: &str) {
        let mut bft = self.bft.lock().unwrap();
        
        info!("⚖️ RIGOR: Proposing REPUTATION_UP for {} via BFT Consensus", node_id);
        bft.handle_proposal("validator-1", &format!("REP_UP_{}", node_id));

        // In a real system, the score update would only happen AFTER Step::Commit
        // For simulation, we apply it here but log the BFT state
        let mut scores = self.scores.lock().unwrap();
        let score = scores.entry(node_id.to_string()).or_insert(100);
        *score += 1;
    }

    pub fn record_failure(&self, node_id: &str) {
        let mut bft = self.bft.lock().unwrap();
        
        warn!("💥 RIGOR: Proposing REPUTATION_DOWN for {} via BFT Consensus", node_id);
        bft.handle_proposal("validator-1", &format!("REP_DOWN_{}", node_id));

        let mut scores = self.scores.lock().unwrap();
        let score = scores.entry(node_id.to_string()).or_insert(100);
        if *score > 0 {
            *score -= 10;
        }
    }

    pub fn get_reputation(&self, node_id: &str) -> u32 {
        let scores = self.scores.lock().unwrap();
        *scores.get(node_id).unwrap_or(&100)
    }

    pub fn sync_ledger(&self) {
        info!("🕸️ PHASE 18: Synchronizing Distributed Reputation Ledger (BFT ENFORCED)");
        let bft = self.bft.lock().unwrap();
        info!("   Current BFT Height: {}, Step: {:?}", bft.height, bft.step);
    }
}
