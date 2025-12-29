//! DAO Governance Framework for Protocol Upgrades
//! 
//! On-chain voting for cipher migrations and network changes

use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Proposal {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub proposer: String,
    pub votes_for: u64,
    pub votes_against: u64,
    pub threshold: u64,
}

pub struct DaoGovernance {
    proposals: HashMap<u64, Proposal>,
    next_proposal_id: u64,
}

impl DaoGovernance {
    pub fn new() -> Self {
        Self {
            proposals: HashMap::new(),
            next_proposal_id: 1,
        }
    }

    /// Create new governance proposal
    pub fn create_proposal(&mut self, title: String, description: String, proposer: String) -> u64 {
        let id = self.next_proposal_id;
        self.next_proposal_id += 1;
        
        let proposal = Proposal {
            id,
            title: title.clone(),
            description,
            proposer,
            votes_for: 0,
            votes_against: 0,
            threshold: 66, // 66% approval required
        };
        
        self.proposals.insert(id, proposal);
        tracing::info!("📜 New proposal #{}: {}", id, title);
        id
    }

    /// Cast vote on proposal
    pub fn vote(&mut self, proposal_id: u64, in_favor: bool, voting_power: u64) -> Result<(), String> {
        let proposal = self.proposals.get_mut(&proposal_id)
            .ok_or("Proposal not found")?;
        
        if in_favor {
            proposal.votes_for += voting_power;
        } else {
            proposal.votes_against += voting_power;
        }
        
        tracing::info!("🗳️ Vote cast on proposal #{}", proposal_id);
        Ok(())
    }

    /// Execute proposal if threshold reached
    pub fn execute_if_passed(&self, proposal_id: u64) -> Result<bool, String> {
        let proposal = self.proposals.get(&proposal_id)
            .ok_or("Proposal not found")?;
        
        let total_votes = proposal.votes_for + proposal.votes_against;
        if total_votes == 0 {
            return Ok(false);
        }
        
        let approval_pct = (proposal.votes_for * 100) / total_votes;
        
        if approval_pct >= proposal.threshold {
            tracing::info!("✅ Proposal #{} PASSED ({}% approval)", proposal_id, approval_pct);
            Ok(true)
        } else {
            tracing::info!("❌ Proposal #{} failed ({}% approval, need {}%)", 
                proposal_id, approval_pct, proposal.threshold);
            Ok(false)
        }
    }
}
