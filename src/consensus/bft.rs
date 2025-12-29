use std::collections::HashMap;
use tracing::{info, warn};

#[derive(Debug, Clone, PartialEq)]
pub enum Step {
    Propose,
    Prevote,
    Precommit,
    Commit,
}

#[derive(Debug, Clone)]
pub struct Vote {
    pub validator: String,
    pub height: u64,
    pub round: u32,
    pub step: Step,
    pub block_hash: String,
}

pub struct BftEngine {
    pub validator_id: String,
    pub height: u64,
    pub round: u32,
    pub step: Step,
    pub locked_round: Option<u32>,
    pub locked_block: Option<String>,
    pub votes: HashMap<u32, Vec<Vote>>, // round -> votes
}

impl BftEngine {
    pub fn new(id: String) -> Self {
        Self {
            validator_id: id,
            height: 0,
            round: 0,
            step: Step::Propose,
            locked_round: None,
            locked_block: None,
            votes: HashMap::new(),
        }
    }

    /// Handles a proposal from the leader.
    /// This is a simplified Tendermint-style BFT.
    pub fn handle_proposal(&mut self, proposer: &str, block_hash: &str) {
        info!("🤝 BFT [H:{} R:{}]: Received Proposal from {} for block {}", 
            self.height, self.round, proposer, block_hash);
        
        // Transition to Prevote step
        self.step = Step::Prevote;
        self.broadcast_vote(Step::Prevote, block_hash);
    }

    pub fn handle_vote(&mut self, vote: Vote) {
        if vote.height != self.height { return; }
        
        let round_votes = self.votes.entry(vote.round).or_insert_with(Vec::new);
        round_votes.push(vote.clone());

        // Check for +2/3 majority (simplified majority logic)
        let total_validators = 4; // Mocking a 4-node network
        let threshold = (total_validators * 2 / 3) + 1;

        let matching_votes = round_votes.iter()
            .filter(|v| v.step == vote.step && v.block_hash == vote.block_hash)
            .count();

        if matching_votes >= threshold {
            match vote.step {
                Step::Prevote => {
                    info!("✅ BFT: Reached +2/3 Prevotes for {}", vote.block_hash);
                    self.step = Step::Precommit;
                    self.broadcast_vote(Step::Precommit, &vote.block_hash);
                }
                Step::Precommit => {
                    info!("💎 BFT: Reached +2/3 Precommits. COMMITTING block {}", vote.block_hash);
                    self.step = Step::Commit;
                    self.finalize_commit(&vote.block_hash);
                }
                _ => {}
            }
        }
    }

    fn broadcast_vote(&self, step: Step, block_hash: &str) {
        info!("🗳️ BFT: Sending {:?} vote for block {}", step, block_hash);
        // In a real implementation, this would send a network packet
    }

    fn finalize_commit(&mut self, block_hash: &str) {
        info!("🏛️ BFT: Block {} committed at height {}. Moving to next height.", block_hash, self.height);
        self.height += 1;
        self.round = 0;
        self.step = Step::Propose;
        self.locked_round = None;
        self.locked_block = None;
        self.votes.clear();
    }
}
