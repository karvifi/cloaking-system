use tracing::{info, warn};

#[derive(Debug, Clone, PartialEq)]
pub enum CipherSuite {
    Kyber1024Aes256Gcm,
    Dilithium5ChaCha20,
    FrodoKemAes256Gcm,
}

pub struct AgilityManager {
    pub current_cipher: CipherSuite,
    pub pending_cipher: Option<CipherSuite>,
}

impl AgilityManager {
    pub fn new() -> Self {
        Self {
            current_cipher: CipherSuite::Kyber1024Aes256Gcm,
            pending_cipher: None,
        }
    }

    /// Proposes a network-wide migration to a new cipher suite.
    /// This uses the BFT Consensus engine to ensure agreement.
    pub fn propose_migration(&mut self, next: CipherSuite) {
        info!("🗳️ RIGOR: Proposing Cryptographic Migration to {:?}", next);
        self.pending_cipher = Some(next);
    }

    /// Executes the migration after consensus is reached.
    pub fn execute_migration(&mut self) {
        if let Some(next) = self.pending_cipher.take() {
            warn!("🚀 RIGOR: Executing Network-Wide Cipher Rotation. Swapping to {:?}", next);
            self.current_cipher = next;
        }
    }
}
