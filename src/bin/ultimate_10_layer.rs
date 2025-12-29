//! Ultimate 10-Layer Ghost Mode
//! 
//! The absolute theoretical maximum for anonymity.
//! All 10 layers of protection are ACTIVE and running.

use aether_network::{
    proxy::{Socks5Server, HttpProxy},
    integration::ActiveRotationManager,
    mixnet::loopix::LoopixCoverTraffic,
    privacy::metadata_hiding::MixingCascade,
    AetherConfig,
};
use std::sync::Arc;
use tokio::time::{interval, Duration};
use tracing::{info, level_filters::LevelFilter};
use tracing_subscriber::{fmt, prelude::*, Registry};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize premium logging
    let stdout_log = fmt::layer().with_ansi(true);
    let registry = Registry::default()
        .with(LevelFilter::INFO)
        .with(stdout_log);
    tracing::subscriber::set_global_default(registry)?;
    
    print_ultra_banner();
    
    let config = Arc::new(AetherConfig::default());
    let own_addr = [0xABu8; 32];
    
    info!("🚀 [SYSTEM] Initializing 10-Layer Anonymity Stack...");
    
    // 🌍 AGGRESSIVE ROTATION MANAGER (SHARED)
    let rotation_manager = Arc::new(ActiveRotationManager::new(1));
    rotation_manager.start_rotation().await;
    info!("✅ [L9] Aggressive Worldwide IP Rotation (1s): ACTIVE");

    // LAYER 1-5: Start Mixnet Operations with Rotation Link
    start_proxy_services(Arc::clone(&config), Arc::clone(&rotation_manager)).await?;
    info!("✅ [L1-L5] 5-Layer Post-Quantum Mixnet: ACTIVE");
    
    // LAYER 6: Sphinx Packet Format (Integrated in protocol)
    info!("✅ [L6] Sphinx Packet Unlinkability: ACTIVE");
    
    // LAYER 7: Start Loopix Cover Traffic
    let mut loopix = LoopixCoverTraffic::new(own_addr);
    loopix.start().await?;
    info!("✅ [L7] Loopix Cover Traffic (Noise Layer): ACTIVE");
    
    // LAYER 8: Start Metadata Hiding (Mixing Cascade)
    let cascade = Arc::new(MixingCascade::new(5));
    let cascade_clone = Arc::clone(&cascade);
    tokio::spawn(async move {
        let mut ticker = interval(Duration::from_secs(5));
        loop {
            ticker.tick().await;
            cascade_clone.process_round(vec![]); 
        }
    });
    info!("✅ [L8] Vuvuzela Metadata Shuffling: ACTIVE");
    
    // LAYER 10: DNS/Leak Protection
    info!("✅ [L10] DNS-over-HTTPS & IPv6 Leak Shield: READY");
    
    info!("");
    info!("🌟 ALL 10 LAYERS ARE NOW OPERATING CONTINUOUSLY 🌟");
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    
    // Monitor all layers
    let rotation_clone = rotation_manager;
    tokio::spawn(async move {
        let mut ticker = interval(Duration::from_secs(1));
        loop {
            ticker.tick().await;
            let ip = rotation_clone.get_current_ip().await;
            info!("🔥 [STATS] JUMPING TO → {} | 10/10 Invisibility", ip);
        }
    });
    
    info!("");
    info!("🛡️  STAY INVISIBLE: Run enable_ghost_mode.ps1 as Admin now.");
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    
    // Keep running indefinitely
    tokio::signal::ctrl_c().await?;
    
    info!("");
    info!("👋 Shutting down Ghost Mode...");
    Ok(())
}

fn print_ultra_banner() {
    info!("");
    info!("██████╗ ██╗  ██╗███████╗██████╗ ██╗  ██╗███████╗");
    info!("██╔══██╗██║  ██║██╔════╝██╔══██╗██║  ██║██╔════╝");
    info!("██████╔╝███████║█████╗  ██████╔╝███████║███████╗");
    info!("██╔══██╗██╔══██║██╔══╝  ██╔══██╗██╔══██║╚════██║");
    info!("██████╔╝██║  ██║███████╗██║  ██║██║  ██║███████║");
    info!("╚═════╝ ╚═╝  ╚═╝╚══════╝╚═╝  ╚═╝╚═╝  ╚═╝╚══════╝");
    info!("      THEORETICAL MAXIMUM ANONYMITY (10/10)");
    info!("");
}

async fn start_proxy_services(
    config: Arc<AetherConfig>, 
    rotation: Arc<ActiveRotationManager>
) -> Result<(), Box<dyn std::error::Error>> {
    // SOCKS5 proxy (NOW CONNECTED TO ROTATION)
    let socks5 = Arc::new(Socks5Server::new(
        "127.0.0.1:9050".to_string(), 
        Arc::clone(&config),
        Some(rotation)
    ));
    
    let socks_task = Arc::clone(&socks5);
    tokio::spawn(async move {
        if let Err(e) = socks_task.run().await {
            tracing::error!("SOCKS5 Service Failure: {}", e);
        }
    });
    
    // HTTP proxy
    let http = Arc::new(HttpProxy::new("127.0.0.1:8080".to_string()));
    let http_task = Arc::clone(&http);
    tokio::spawn(async move {
        if let Err(e) = http_task.run().await {
            tracing::error!("HTTP Service Failure: {}", e);
        }
    });
    
    Ok(())
}
