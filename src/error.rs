//! Error handling for Aether Network
//! 
//! Provides a unified error type and result alias for the entire library.

use std::fmt;

/// Result type alias for Aether operations
pub type Result<T> = std::result::Result<T, AetherError>;

/// Main error type for Aether Network
#[derive(Debug)]
pub enum AetherError {
    /// Configuration error (e.g., missing parameters or invalid paths)
    Config(String),
    
    /// Cryptographic operation failed (e.g., decryption or key generation)
    Crypto(String),
    
    /// Network-related error (e.g., connection timed out or peer disconnected)
    Network(String),
    
    /// Packet processing error (e.g., invalid header or malformed payload)
    Packet(String),
    
    /// Routing error (e.g., no path identified for the destination)
    Routing(String),
    
    /// Zero-knowledge proof error (e.g., proof verification failed)
    ZKProof(String),
    
    /// Standard IO error wrapper
    Io(std::io::Error),
    
    /// Serialization/Deserialization failure
    Serialization(String),
    
    /// Internal component in an invalid or inconsistent state
    InvalidState(String),
}

impl fmt::Display for AetherError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AetherError::Config(msg) => write!(f, "Configuration error: {}", msg),
            AetherError::Crypto(msg) => write!(f, "Cryptographic error: {}", msg),
            AetherError::Network(msg) => write!(f, "Network error: {}", msg),
            AetherError::Packet(msg) => write!(f, "Packet error: {}", msg),
            AetherError::Routing(msg) => write!(f, "Routing error: {}", msg),
            AetherError::ZKProof(msg) => write!(f, "ZK proof error: {}", msg),
            AetherError::Io(err) => write!(f, "IO error: {}", err),
            AetherError::Serialization(msg) => write!(f, "Serialization error: {}", msg),
            AetherError::InvalidState(msg) => write!(f, "Invalid state: {}", msg),
        }
    }
}

impl std::error::Error for AetherError {}

impl From<std::io::Error> for AetherError {
    fn from(err: std::io::Error) -> Self {
        AetherError::Io(err)
    }
}

impl From<serde_json::Error> for AetherError {
    fn from(err: serde_json::Error) -> Self {
        AetherError::Serialization(err.to_string())
    }
}

impl From<toml::de::Error> for AetherError {
    fn from(err: toml::de::Error) -> Self {
        AetherError::Serialization(err.to_string())
    }
}

impl From<ark_relations::r1cs::SynthesisError> for AetherError {
    fn from(err: ark_relations::r1cs::SynthesisError) -> Self {
        AetherError::ZKProof(err.to_string())
    }
}

impl From<ark_serialize::SerializationError> for AetherError {
    fn from(err: ark_serialize::SerializationError) -> Self {
        AetherError::Serialization(err.to_string())
    }
}
