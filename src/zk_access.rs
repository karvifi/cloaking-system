//! Zero-knowledge access control
//! 
//! Handles anonymous credentials and membership verification

use serde::{Deserialize, Serialize};

/// An anonymous credential obtained from a service provider
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AnonymousCredential {
    /// Opaque credential data
    pub data: Vec<u8>,
    /// Credential signature or proof
    pub proof: Vec<u8>,
}

impl AnonymousCredential {
    /// Create a dummy credential
    pub fn dummy() -> Self {
        Self {
            data: vec![],
            proof: vec![],
        }
    }
}
