//! Deterministic Builds & Supply Chain Security
//! 
//! Reproducible build verification and cryptographic attestation

use sha2::{Sha256, Digest};
use std::process::Command;
use std::path::Path;

pub struct BuildAttestation {
    /// SHA-256 hash of the compiled binary
    pub binary_hash: String,
    /// Rust compiler version
    pub rustc_version: String,
    /// Build timestamp
    pub build_timestamp: u64,
    /// Git commit hash
    pub git_commit: String,
    /// Cargo dependencies hash
    pub dependencies_hash: String,
}

impl BuildAttestation {
    /// Generate attestation for current build
    pub fn generate(binary_path: &Path) -> Result<Self, String> {
        // Hash the binary
        let binary_hash = Self::hash_file(binary_path)?;
        
        // Get rustc version
        let rustc_version = Self::get_rustc_version()?;
        
        // Get git commit
        let git_commit = Self::get_git_commit()?;
        
        // Hash Cargo.lock for dependency verification
        let dependencies_hash = Self::hash_file(Path::new("Cargo.lock"))?;
        
        let build_timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        Ok(Self {
            binary_hash,
            rustc_version,
            build_timestamp,
            git_commit,
            dependencies_hash,
        })
    }

    fn hash_file(path: &Path) -> Result<String, String> {
        let bytes = std::fs::read(path)
            .map_err(|e| format!("Failed to read file: {}", e))?;
        
        let mut hasher = Sha256::new();
        hasher.update(&bytes);
        Ok(format!("{:x}", hasher.finalize()))
    }

    fn get_rustc_version() -> Result<String, String> {
        let output = Command::new("rustc")
            .arg("--version")
            .output()
            .map_err(|e| format!("Failed to  get rustc version: {}", e))?;
        
        String::from_utf8(output.stdout)
            .map_err(|e| format!("Invalid UTF-8: {}", e))
            .map(|s| s.trim().to_string())
    }

    fn get_git_commit() -> Result<String, String> {
        let output = Command::new("git")
            .args(&["rev-parse", "HEAD"])
            .output()
            .map_err(|e| format!("Failed to get git commit: {}", e))?;
        
        String::from_utf8(output.stdout)
            .map_err(|e| format!("Invalid UTF-8: {}", e))
            .map(|s| s.trim().to_string())
    }

    /// Verify build matches attestation
    pub fn verify(&self, binary_path: &Path) -> Result<(), String> {
        let current_hash = Self::hash_file(binary_path)?;
        
        if current_hash != self.binary_hash {
            return Err(format!(
                "🚨 SUPPLY CHAIN ATTACK DETECTED!\nExpected: {}\nGot: {}",
                self.binary_hash, current_hash
            ));
        }
        
        tracing::info!("✅ Build attestation verified: Binary unmodified");
        Ok(())
    }

    /// Serialize attestation for distribution
    pub fn to_json(&self) -> String {
        format!(
            r#"{{
  "binary_hash": "{}",
  "rustc_version": "{}",
  "build_timestamp": {},
  "git_commit": "{}",
  "dependencies_hash": "{}"
}}"#,
            self.binary_hash,
            self.rustc_version,
            self.build_timestamp,
            self.git_commit,
            self.dependencies_hash
        )
    }
}

/// Usage: Generate attestation after build
/// ```
/// cargo build --release
/// let attestation = BuildAttestation::generate(Path::new("target/release/aether-network"))?;
/// std::fs::write("attestation.json", attestation.to_json())?;
/// ```
