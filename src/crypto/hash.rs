//! Hash functions and key derivation

use blake3::Hasher;
use hkdf::Hkdf;
use sha3::Sha3_256;
use crate::error::Result;

/// Compute BLAKE3 hash of input data
pub fn blake3_hash(data: &[u8]) -> [u8; 32] {
    let mut hasher = Hasher::new();
    hasher.update(data);
    let hash = hasher.finalize();
    *hash.as_bytes()
}

/// Derive a key using HKDF-SHA3-256
pub fn derive_key(
    input_key_material: &[u8],
    salt: Option<&[u8]>,
    info: &[u8],
    output_length: usize,
) -> Result<Vec<u8>> {
    let hkdf = Hkdf::<Sha3_256>::new(salt, input_key_material);
    let mut output = vec![0u8; output_length];
    hkdf.expand(info, &mut output)
        .map_err(|_| crate::error::AetherError::Crypto("HKDF expansion failed".to_string()))?;
    Ok(output)
}

/// Derive multiple keys from a single input
pub fn derive_multiple_keys(
    input_key_material: &[u8],
    count: usize,
    key_length: usize,
) -> Result<Vec<Vec<u8>>> {
    let mut keys = Vec::with_capacity(count);
    
    for i in 0..count {
        let info = format!("key-{}", i);
        let key = derive_key(input_key_material, None, info.as_bytes(), key_length)?;
        keys.push(key);
    }
    
    Ok(keys)
}

/// Compute a keyed hash (MAC) using BLAKE3
pub fn keyed_hash(key: &[u8; 32], data: &[u8]) -> [u8; 32] {
    let mut hasher = blake3::Hasher::new_keyed(key);
    hasher.update(data);
    *hasher.finalize().as_bytes()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_blake3_hash() {
        let data = b"test data";
        let hash1 = blake3_hash(data);
        let hash2 = blake3_hash(data);
        assert_eq!(hash1, hash2);
        assert_eq!(hash1.len(), 32);
    }
    
    #[test]
    fn test_derive_key() {
        let ikm = b"input key material";
        let salt = b"salt";
        let info = b"context";
        let key = derive_key(ikm, Some(salt), info, 32).unwrap();
        assert_eq!(key.len(), 32);
    }
    
    #[test]
    fn test_derive_multiple_keys() {
        let ikm = b"master key";
        let keys = derive_multiple_keys(ikm, 5, 32).unwrap();
        assert_eq!(keys.len(), 5);
        
        // All keys should be different
        for i in 0..keys.len() {
            for j in (i+1)..keys.len() {
                assert_ne!(keys[i], keys[j]);
            }
        }
    }
}
