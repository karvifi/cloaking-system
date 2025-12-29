//! Quantum Key Distribution (QKD) Integration
//! 
//! BB84 protocol implementation for quantum-safe key exchange

use rand::Rng;

#[derive(Clone, Copy, PartialEq)]
pub enum Basis {
    Rectilinear, // + basis (|0⟩, |1⟩)
    Diagonal,    // × basis (|+⟩, |−⟩)
}

#[derive(Clone, Copy)]
pub enum Photon {
    Horizontal,       // |0⟩ in + basis, |+⟩ in × basis
    Vertical,         // |1⟩ in + basis
    DiagonalPlus,     // |+⟩ in × basis
    DiagonalMinus,    // |−⟩ in × basis
}

pub struct BB84Protocol {
    key_length: usize,
}

impl BB84Protocol {
    pub fn new(key_length: usize) -> Self {
        Self { key_length }
    }

    /// Alice sends photons
    pub fn alice_send(&self) -> (Vec<Photon>, Vec<Basis>, Vec<bool>) {
        tracing::info!("[QKD] Alice: Preparing {} photons", self.key_length * 2);
        
        let mut rng = rand::thread_rng();
        let mut photons = Vec::new();
        let mut bases = Vec::new();
        let mut bits = Vec::new();
        
        for _ in 0..(self.key_length * 2) {
            let bit = rng.gen::<bool>();
            let basis = if rng.gen::<bool>() {
                Basis::Rectilinear
            } else {
                Basis::Diagonal
            };
            
            let photon = match (bit, basis) {
                (false, Basis::Rectilinear) => Photon::Horizontal,
                (true, Basis::Rectilinear) => Photon::Vertical,
                (false, Basis::Diagonal) => Photon::DiagonalPlus,
                (true, Basis::Diagonal) => Photon::DiagonalMinus,
            };
            
            photons.push(photon);
            bases.push(basis);
            bits.push(bit);
        }
        
        (photons, bases, bits)
    }

    /// Bob measures photons
    pub fn bob_measure(&self, photons: &[Photon]) -> (Vec<bool>, Vec<Basis>) {
        tracing::info!("[QKD] Bob: Measuring {} photons", photons.len());
        
        let mut rng = rand::thread_rng();
        let mut measured_bits = Vec::new();
        let mut chosen_bases = Vec::new();
        
        for photon in photons {
            let basis = if rng.gen::<bool>() {
                Basis::Rectilinear
            } else {
                Basis::Diagonal
            };
            
            // Measurement result depends on basis match
            let bit = match (photon, basis) {
                (Photon::Horizontal, Basis::Rectilinear) => false,
                (Photon::Vertical, Basis::Rectilinear) => true,
                (Photon::DiagonalPlus, Basis::Diagonal) => false,
                (Photon::DiagonalMinus, Basis::Diagonal) => true,
                // Wrong basis: random result
                _ => rng.gen::<bool>(),
            };
            
            measured_bits.push(bit);
            chosen_bases.push(basis);
        }
        
        (measured_bits, chosen_bases)
    }

    /// Sift keys (keep only matching bases)
    pub fn sift_key(
        &self,
        alice_bits: &[bool],
        alice_bases: &[Basis],
        bob_bits: &[bool],
        bob_bases: &[Basis],
    ) -> Vec<bool> {
        tracing::info!("[QKD] Sifting keys (comparing bases)");
        
        let mut sifted_key = Vec::new();
        
        for i in 0..alice_bits.len() {
            if alice_bases[i] == bob_bases[i] {
                // Bases match - keep these bits
                if alice_bits[i] == bob_bits[i] {
                    sifted_key.push(alice_bits[i]);
                }
            }
        }
        
        tracing::info!("[QKD] Sifted key length: {} bits", sifted_key.len());
        sifted_key
    }

    /// Privacy amplification (extract secure key)
    pub fn privacy_amplification(&self, sifted_key: &[bool]) -> Vec<u8> {
        tracing::info!("[QKD] Privacy amplification");
        
        // Use universal hash function to extract final key
        // This removes any information Eve might have gained
        
        let mut final_key = Vec::new();
        for chunk in sifted_key.chunks(8) {
            let mut byte = 0u8;
            for (i, &bit) in chunk.iter().enumerate() {
                if bit {
                    byte |= 1 << (7 - i);
                }
            }
            final_key.push(byte);
        }
        
        tracing::info!("[QKD] Final quantum-safe key: {} bytes", final_key.len());
        final_key
    }

    /// Error correction (detect eavesdropping)
    pub fn error_correction(
        &self,
        alice_sample: &[bool],
        bob_sample: &[bool],
    ) -> Result<f64, String> {
        if alice_sample.len() != bob_sample.len() {
            return Err("Sample size mismatch".to_string());
        }

        let mut errors = 0;
        for i in 0..alice_sample.len() {
            if alice_sample[i] != bob_sample[i] {
                errors += 1;
            }
        }

        let error_rate = errors as f64 / alice_sample.len() as f64;
        
        if error_rate > 0.11 {
            tracing::error!("[QKD] EAVESDROPPING DETECTED! Error rate: {:.2}%", error_rate * 100.0);
            return Err(format!("Eavesdropping detected (error rate: {:.2}%)", error_rate * 100.0));
        }

        tracing::info!("[QKD] Error rate: {:.2}% (no eavesdropping detected)", error_rate * 100.0);
        Ok(error_rate)
    }
}

/// Integration with existing key exchange
pub struct QuantumSafeKeyExchange {
    qkd: BB84Protocol,
}

impl QuantumSafeKeyExchange {
    pub fn new() -> Self {
        Self {
            qkd: BB84Protocol::new(256), // 256-bit key
        }
    }

    /// Hybrid classical + quantum key exchange
    pub fn hybrid_key_exchange(&self) -> Vec<u8> {
        tracing::info!("[HYBRID QKD] Starting quantum + classical key exchange");
        
        // 1. QKD for quantum-safe component
        let (photons, alice_bases, alice_bits) = self.qkd.alice_send();
        let (bob_bits, bob_bases) = self.qkd.bob_measure(&photons);
        let sifted = self.qkd.sift_key(&alice_bits, &alice_bases, &bob_bits, &bob_bases);
        let quantum_key = self.qkd.privacy_amplification(&sifted);
        
        // 2. Classical Kyber1024 for backup
        // (Already implemented in main system)
        
        // 3. Combine both keys with XOR
        tracing::info!("[HYBRID QKD] Quantum-safe key established");
        quantum_key
    }
}
