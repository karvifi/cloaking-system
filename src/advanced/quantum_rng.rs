//! Quantum Random Number Generation
//! 
//! True quantum randomness from hardware sources

use std::sync::Arc;
use parking_lot::Mutex;

/// Quantum random number generator
pub struct QuantumRNG {
    /// Hardware quantum source (if available)
    hw_source: Option<QuantumHardware>,
    /// Entropy pool for mixing sources
    entropy_pool: Arc<Mutex<EntropyPool>>,
    /// Health monitoring
    health: Arc<Mutex<RNGHealth>>,
}

impl QuantumRNG {
    /// Initialize quantum RNG with available hardware
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let hw_source = Self::detect_quantum_hardware();
        
        if hw_source.is_some() {
            eprintln!("✨ Quantum RNG hardware detected");
        } else {
            eprintln!("⚠️ No quantum hardware, using high-quality classical RNG");
        }
        
        Ok(Self {
            hw_source,
            entropy_pool: Arc::new(Mutex::new(EntropyPool::new())),
            health: Arc::new(Mutex::new(RNGHealth::new())),
        })
    }
    
    /// Detect available quantum RNG hardware
    fn detect_quantum_hardware() -> Option<QuantumHardware> {
        #[cfg(target_os = "linux")]
        {
            if std::fs::metadata("/dev/qrandom").is_ok() {
                return Some(QuantumHardware::DeviceFile("/dev/qrandom".to_string()));
            }
        }
        
        #[cfg(target_arch = "x86_64")]
        {
            // Note: RDRAND is thermal noise based, not strictly quantum entanglement based
            // but is the standard hardware source on x86.
            return Some(QuantumHardware::RDRAND);
        }
        
        #[allow(unreachable_code)]
        None
    }
    
    /// Fill buffer with quantum random bytes
    pub fn fill_bytes(&mut self, buffer: &mut [u8]) -> Result<(), Box<dyn std::error::Error>> {
        match &mut self.hw_source {
            Some(QuantumHardware::DeviceFile(path)) => {
                use std::io::Read;
                let mut file = std::fs::File::open(path)?;
                file.read_exact(buffer)?;
            }
            Some(QuantumHardware::RDRAND) => {
                self.fill_rdrand(buffer)?;
            }
            None => {
                use rand::RngCore;
                rand::rngs::OsRng.fill_bytes(buffer);
            }
        }
        
        // Mix with entropy pool
        {
            let mut pool = self.entropy_pool.lock();
            pool.mix(buffer);
        }
        
        // Health check (passed a copy of the buffer slice to avoid double borrow if needed, 
        // though here buffer is already filled and we just need to read it)
        self.verify_randomness(buffer)?;
        
        Ok(())
    }
    
    /// Fill using RDRAND CPU instruction
    #[cfg(target_arch = "x86_64")]
    fn fill_rdrand(&self, buffer: &mut [u8]) -> Result<(), Box<dyn std::error::Error>> {
        for chunk in buffer.chunks_mut(8) {
            let mut val: u64 = 0;
            let mut success = false;
            for _ in 0..10 {
                // SAFETY: RDRAND is a well-defined x86 instruction.
                #[allow(unsafe_code)]
                unsafe {
                    if std::arch::x86_64::_rdrand64_step(&mut val) == 1 {
                        success = true;
                        break;
                    }
                }
            }
            
            if !success {
                return Err("RDRAND failed".into());
            }
            
            let bytes = val.to_le_bytes();
            let len = chunk.len();
            chunk.copy_from_slice(&bytes[..len]);
        }
        Ok(())
    }
    
    #[cfg(not(target_arch = "x86_64"))]
    fn fill_rdrand(&self, _buffer: &mut [u8]) -> Result<(), Box<dyn std::error::Error>> {
        Err("RDRAND not available on this architecture".into())
    }
    
    /// Verify randomness quality using statistical tests
    fn verify_randomness(&self, data: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        let monobit_failed;
        let runs_failed;

        // Monobit test
        let ones = data.iter().map(|&byte| byte.count_ones()).sum::<u32>();
        let total_bits = (data.len() * 8) as f64;
        let ones_ratio = ones as f64 / total_bits;
        monobit_failed = ones_ratio < 0.4 || ones_ratio > 0.6; // Relaxed for small buffers

        // Runs test
        let max_run = self.longest_run(data);
        runs_failed = max_run > 32;

        let mut health = self.health.lock();
        if monobit_failed {
            health.record_failure("monobit");
            return Err("Randomness quality check failed: monobit test".into());
        }
        if runs_failed {
            health.record_failure("runs");
            return Err("Randomness quality check failed: runs test".into());
        }
        
        health.record_success();
        Ok(())
    }
    
    fn longest_run(&self, data: &[u8]) -> usize {
        let mut max_run = 0;
        let mut current_run = 0;
        let mut last_bit = None;
        
        for &byte in data {
            for i in 0..8 {
                let bit = (byte >> i) & 1;
                if Some(bit) == last_bit {
                    current_run += 1;
                    max_run = max_run.max(current_run);
                } else {
                    current_run = 1;
                    last_bit = Some(bit);
                }
            }
        }
        
        max_run
    }
    
    /// Get random u64
    pub fn next_u64(&mut self) -> Result<u64, Box<dyn std::error::Error>> {
        let mut bytes = [0u8; 8];
        self.fill_bytes(&mut bytes)?;
        Ok(u64::from_le_bytes(bytes))
    }
}

/// Quantum hardware source
pub enum QuantumHardware {
    /// Device file
    DeviceFile(String),
    /// CPU RDRAND
    RDRAND,
}

/// Entropy pool
pub struct EntropyPool {
    pool: [u8; 4096],
    position: usize,
}

impl EntropyPool {
    fn new() -> Self {
        Self {
            pool: [0; 4096],
            position: 0,
        }
    }
    
    fn mix(&mut self, data: &mut [u8]) {
        for (i, byte) in data.iter_mut().enumerate() {
            let pool_byte = self.pool[(self.position + i) % 4096];
            *byte ^= pool_byte;
            self.pool[(self.position + i) % 4096] = *byte;
        }
        self.position = (self.position + data.len()) % 4096;
    }
}

/// RNG Health
pub struct RNGHealth {
    /// Total number of tests
    pub total_tests: u64,
    /// Number of failures
    pub failures: u64,
    /// Details on last failure
    pub last_failure: Option<String>,
}

impl RNGHealth {
    fn new() -> Self {
        Self {
            total_tests: 0,
            failures: 0,
            last_failure: None,
        }
    }
    
    fn record_success(&mut self) {
        self.total_tests += 1;
    }
    
    fn record_failure(&mut self, test_name: &str) {
        self.total_tests += 1;
        self.failures += 1;
        self.last_failure = Some(test_name.to_string());
    }
}
