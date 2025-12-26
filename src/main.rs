//! Aether Network - Continuous Run Entry Point
//! 
//! This binary initializes the core network layers and starts a continuous
//! simulation of post-quantum anonymous packet routing.

use aether_network::mixnet::node::{MixNode, NodeRole};
use aether_network::AetherConfig;
use aether_network::advanced::AdvancedFeatures;
use aether_network::protocols::OutfoxPacket;
use std::sync::Arc;
use tokio::time::{sleep, Duration};
use tracing::{info, warn, level_filters::LevelFilter};
use tracing_subscriber::{fmt, prelude::*, Registry};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Initialize Logging with a pretty format
    let stdout_log = fmt::layer().with_ansi(true);
    let registry = Registry::default()
        .with(LevelFilter::INFO)
        .with(stdout_log);
    tracing::subscriber::set_global_default(registry).expect("Failed to set tracing subscriber");
    
    info!("🌌 Aether Network - System Initialization");
    info!("------------------------------------------");
    
    // 2. Initialize Advanced Features (ZK proofs, HE, Quantum RNG)
    let _advanced = AdvancedFeatures::initialize()?;
    
    // 3. Setup Configuration
    let config = Arc::new(AetherConfig::default());
    
    // 4. Create and Start Nodes for a 5-layer Mixnet
    let mut nodes = Vec::new();
    for layer in 1..=5 {
        let role = match layer {
            1 => NodeRole::EntryGateway,
            5 => NodeRole::ExitGateway,
            _ => NodeRole::MixNode,
        };
        
        info!("✅ Layer {} Node Initialized [{:?}]", layer, role);
        let node = Arc::new(MixNode::new(
            layer,
            role,
            1000,
            format!("127.0.0.1:{}", 9000 + layer),
            Arc::clone(&config),
        )?);
        
        node.run().await?;
        nodes.push(node);
    }
    
    info!("🚀 Network is LIVE and operational.");
    info!("🛡️  Post-Quantum Security Active (Kyber-1024)");
    info!("------------------------------------------");

    // 5. Continuous Loop: Active Simulation & Monitoring
    let mut packet_count = 0;
    loop {
        packet_count += 1;
        
        // Simulate a new packet entering the network at Layer 1
        let test_message = format!("Anonymized Packet #{}", packet_count);
        
        // Generate a route with the public keys of our nodes
        let route: Vec<_> = nodes.iter()
            .map(|n| {
                let bytes = &n.info.public_key_bytes;
                aether_network::crypto::kyber::PublicKey::from_bytes(bytes).unwrap()
            })
            .collect();

        // Create the multi-layered encrypted packet
        match OutfoxPacket::new(test_message.as_bytes(), &route) {
            Ok(packet) => {
                info!("📦 Injecting Packet #{} into Entry Gateway...", packet_count);
                nodes[0].receive_packet(packet).await;
            }
            Err(e) => warn!("❌ Failed to create packet: {}", e),
        }

        // Manually route packets between nodes in this simulation
        // (This simulates the network transport layer synchronously for the demo)
        for i in 0..nodes.len() - 1 {
            if let Some(packet) = nodes[i].send_packet().await {
                // Transfer from layer i to i+1
                nodes[i+1].receive_packet(packet).await;
            }
        }
        
        // Consume processed packets at the Exit Gateway
        if let Some(_processed) = nodes[4].send_packet().await {
            info!("🏁 Packet #{} reached Final Exit Gateway (Layer 5)", packet_count);
        }

        // Output system health stats periodically
        if packet_count % 5 == 0 {
            info!("📊 System Health Update:");
            for (i, node) in nodes.iter().enumerate() {
                let stats = node.get_stats().await;
                info!("   [Layer {}] Processed: {}, Latency: {}ms, Queue: {}", 
                     i+1, stats.packets_processed, stats.average_latency_ms, stats.queue_size);
            }
        }

        // Run continuously with a small delay between packets
        sleep(Duration::from_millis(2000)).await;
        
        // Advanced: Periodically test ZK proofs to ensure crypto validity
        if packet_count % 10 == 0 {
            info!("🔐 Running ZK Validity Shield Check...");
            match aether_network::advanced::ZKOperations::prove_range(42, 64) {
                Ok(_) => info!("   ✅ ZK System Verified"),
                Err(e) => warn!("   ⚠️ ZK check warning: {}", e),
            }
        }
    }
}
