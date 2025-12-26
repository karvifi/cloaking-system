//! Metrics collection and monitoring
//! 
//! Handles Prometheus-compatible metrics for network monitoring

use prometheus::{
    Counter, Gauge, Histogram, HistogramOpts, Registry, Encoder, TextEncoder,
};

/// Metrics collector for Aether Network
pub struct MetricsCollector {
    /// Internal Prometheus registry
    pub registry: Registry,
    
    /// Total packets successfully processed
    pub packets_processed: Counter,
    /// Total packets dropped due to errors or load
    pub packets_dropped: Counter,
    /// Total count of cryptographic operations
    pub crypto_operations: Counter,
    
    /// Current node reputation score
    pub node_reputation: Gauge,
    /// Current size of the packet processing queue
    pub queue_size: Gauge,
    /// Estimated Shannon entropy of local traffic
    pub traffic_entropy: Gauge,
    /// Number of perceived active nodes in the network
    pub active_nodes: Gauge,
    
    /// Histogram of packet processing latencies
    pub packet_latency: Histogram,
    /// Histogram of delays introduced by the mixnet
    pub mixing_delay: Histogram,
}

impl MetricsCollector {
    /// Initialize a new metrics collector with a fresh registry
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let registry = Registry::new();
        
        let packets_processed = Counter::new("aether_packets_processed_total", "Total packets processed")?;
        let packets_dropped = Counter::new("aether_packets_dropped_total", "Total packets dropped")?;
        let crypto_operations = Counter::new("aether_crypto_operations_total", "Total crypto operations")?;
        
        let node_reputation = Gauge::new("aether_node_reputation", "Current node reputation")?;
        let queue_size = Gauge::new("aether_queue_size", "Current packet queue size")?;
        let traffic_entropy = Gauge::new("aether_traffic_entropy", "Shannon entropy of traffic")?;
        let active_nodes = Gauge::new("aether_active_nodes", "Number of active nodes")?;
        
        let packet_latency = Histogram::with_opts(
            HistogramOpts::new("aether_packet_latency_ms", "Packet processing latency")
        )?;
        let mixing_delay = Histogram::with_opts(
            HistogramOpts::new("aether_mixing_delay_ms", "Mixing delay applied")
        )?;
        
        registry.register(Box::new(packets_processed.clone()))?;
        registry.register(Box::new(packets_dropped.clone()))?;
        registry.register(Box::new(crypto_operations.clone()))?;
        registry.register(Box::new(node_reputation.clone()))?;
        registry.register(Box::new(queue_size.clone()))?;
        registry.register(Box::new(traffic_entropy.clone()))?;
        registry.register(Box::new(active_nodes.clone()))?;
        registry.register(Box::new(packet_latency.clone()))?;
        registry.register(Box::new(mixing_delay.clone()))?;
        
        Ok(Self {
            registry,
            packets_processed,
            packets_dropped,
            crypto_operations,
            node_reputation,
            queue_size,
            traffic_entropy,
            active_nodes,
            packet_latency,
            mixing_delay,
        })
    }
    
    /// Get metrics as Prometheus text format for scapers
    pub fn export_metrics(&self) -> Result<String, Box<dyn std::error::Error>> {
        let encoder = TextEncoder::new();
        let metric_families = self.registry.gather();
        let mut buffer = Vec::new();
        encoder.encode(&metric_families, &mut buffer)?;
        Ok(String::from_utf8(buffer)?)
    }
}

impl Default for MetricsCollector {
    fn default() -> Self {
        Self::new().expect("Failed to create metrics collector")
    }
}
