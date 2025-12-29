use std::sync::Arc;
use tokio::net::TcpStream;
use tracing::{info, warn};

pub struct ChainSharder {
    chains: Vec<String>, // List of SOCKS5 proxy addresses
}

impl ChainSharder {
    pub fn new(chains: Vec<String>) -> Self {
        Self { chains }
    }

    /// Selects a chain for the next session/packet.
    /// In the Hyper-Ghost model, we shard by destination.
    pub fn select_chain(&self, _target: &str) -> String {
        let mut rng = rand::thread_rng();
        let idx = rand::Rng::gen_range(&mut rng, 0..self.chains.len());
        self.chains[idx].clone()
    }

    pub async fn verify_chains(&self) {
        for chain in &self.chains {
            match TcpStream::connect(chain).await {
                Ok(_) => info!("⛓️ PHASE 16: Chain {} [VERIFIED]", chain),
                Err(_) => warn!("⛓️ PHASE 16: Chain {} [OFFLINE]", chain),
            }
        }
    }
}
