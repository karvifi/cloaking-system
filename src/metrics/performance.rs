//! Performance Monitoring & Optimization
//! 
//! Real-time latency and throughput metrics

use std::time::{Duration, Instant};
use std::collections::VecDeque;

pub struct PerformanceMonitor {
    /// Recent latency samples
    latency_samples: VecDeque<Duration>,
    /// Throughput counter
    bytes_processed: u64,
    start_time: Instant,
}

impl PerformanceMonitor {
    pub fn new() -> Self {
        Self {
            latency_samples: VecDeque::with_capacity(1000),
            bytes_processed: 0,
            start_time: Instant::now(),
        }
    }

    /// Record packet processing latency
    pub fn record_latency(&mut self, latency: Duration) {
        if self.latency_samples.len() >= 1000 {
            self.latency_samples.pop_front();
        }
        self.latency_samples.push_back(latency);
    }

    /// Record bytes processed
    pub fn record_bytes(&mut self, bytes: u64) {
        self.bytes_processed += bytes;
    }

    /// Get average latency
    pub fn avg_latency(&self) -> Duration {
        if self.latency_samples.is_empty() {
            return Duration::from_secs(0);
        }
        
        let total: Duration = self.latency_samples.iter().sum();
        total / self.latency_samples.len() as u32
    }

    /// Get current throughput (bytes/sec)
    pub fn throughput(&self) -> u64 {
        let elapsed = self.start_time.elapsed().as_secs();
        if elapsed == 0 {
            return 0;
        }
        self.bytes_processed / elapsed
    }

    /// Print performance report
    pub fn print_report(&self) {
        tracing::info!("📊 PERFORMANCE REPORT");
        tracing::info!("   Avg Latency: {:?}", self.avg_latency());
        tracing::info!("   Throughput: {} MB/s", self.throughput() / 1_000_000);
        tracing::info!("   Total Bytes: {} MB", self.bytes_processed / 1_000_000);
    }
}
