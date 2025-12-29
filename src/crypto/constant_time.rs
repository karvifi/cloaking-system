//! Constant-Time Cryptographic Implementations
//! 
//! Timing-attack resistant operations

use subtle::ConstantTimeEq;

pub struct ConstantTimeCrypto;

impl ConstantTimeCrypto {
    /// Constant-time byte array comparison
    pub fn constant_time_compare(a: &[u8], b: &[u8]) -> bool {
        if a.len() != b.len() {
            return false;
        }
        
        a.ct_eq(b).into()
    }

    /// Constant-time conditional selection
    pub fn constant_time_select(condition: bool, true_val: u64, false_val: u64) -> u64 {
        let mask = if condition { u64::MAX } else { 0 };
        (true_val & mask) | (false_val & !mask)
    }

    /// Verify without timing leaks
    pub fn verify_signature_constant_time(sig1: &[u8], sig2: &[u8]) -> bool {
        Self::constant_time_compare(sig1, sig2)
    }
}
