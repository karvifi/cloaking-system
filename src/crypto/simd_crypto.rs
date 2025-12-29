//! SIMD-Optimized Cryptographic Operations
//! 
//! Vectorized crypto for performance

#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

pub struct SimdCrypto;

impl SimdCrypto {
    /// SIMD-accelerated XOR operation
    #[cfg(target_arch = "x86_64")]
    pub unsafe fn xor_blocks_simd(a: &mut [u8], b: &[u8]) {
        assert_eq!(a.len(), b.len());
        
        let chunks = a.len() / 16;
        for i in 0..chunks {
            let offset = i * 16;
            let a_block = _mm_loadu_si128(a[offset..].as_ptr() as *const __m128i);
            let b_block = _mm_loadu_si128(b[offset..].as_ptr() as *const __m128i);
            let result = _mm_xor_si128(a_block, b_block);
            _mm_storeu_si128(a[offset..].as_mut_ptr() as *mut __m128i, result);
        }
    }

    /// Fallback non-SIMD XOR
    pub fn xor_blocks_scalar(a: &mut [u8], b: &[u8]) {
        for (x, y) in a.iter_mut().zip(b.iter()) {
            *x ^= y;
        }
    }

    /// Auto-dispatch to SIMD or scalar
    pub fn xor_blocks(a: &mut [u8], b: &[u8]) {
        #[cfg(target_arch = "x86_64")]
        unsafe {
            if is_x86_feature_detected!("sse2") {
                Self::xor_blocks_simd(a, b);
                return;
            }
        }
        
        Self::xor_blocks_scalar(a, b);
    }
}
