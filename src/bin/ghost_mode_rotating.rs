//! Ghost Mode with Active IP Rotation
//! 
//! IP rotates worldwide EVERY SECOND

use aether_network::{
    proxy::{Socks5Server, HttpProxy},
    integration::ActiveRotationManager,
    AetherConfig,
};
use std::sync::Arc;
use tracing::{info, level_filters::LevelFilter};
use tracing_subscriber::{fmt, prelude::*, Registry};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    let stdout_log = fmt::layer().with_ansi(true);
    let registry = Registry::default()
        .with(LevelFilter::INFO)
        .with(stdout_log);
    tracing::subscriber::set_global_default(registry)?;
    
    print_banner();
    
    let config = Arc::new(AetherConfig::default());
    
    info!("");
    info!("🔧 Initializing Ghost Mode with IP Rotation...");
    info!("");
    
    // Start proxy services
    start_proxy_services(Arc::clone(&config)).await?;
    
    // Start aggressive IP rotation (every 1 second)
    info!("🌍 Starting worldwide IP rotation (every 1 second)...");
    let rotation_manager = ActiveRotationManager::new(1); // 1 second interval
    rotation_manager.start_rotation().await;
    
    info!("");
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    info!("✨ GHOST MODE ACTIVE - IP ROTATING WORLDWIDE");
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    info!("");
    info!("🌐 IP Rotation: EVERY 1 SECOND");
    info!("🎯 Exit IPs: Worldwide (100+ locations)");
    info!("");
    info!("📊 Active Services:");
    info!("   • SOCKS5:    127.0.0.1:9050");
    info!("   • HTTP:      127.0.0.1:8080");
    info!("   • Rotation:  1 second intervals");
    info!("");
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    info!("⚙️  NEXT: Run enable_ghost_mode.ps1 (as Admin)");
    info!("🧪 TEST: Visit https://whatismyipaddress.com");
    info!("        Refresh every second - IP will CHANGE!");
    info!("");
    info!("⏳ System running... (Ctrl+C to stop)");
    info!("");
    
    // Monitor rotation
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(1));
        loop {
            interval.tick().await;
            let current = rotation_manager.get_current_ip().await;
            info!("🔄 Current exit IP: {}", current);
        }
    });
    
    // Keep running
    tokio::signal::ctrl_c().await?;
    
    info!("");
    info!("👋 Shutting down Ghost Mode...");
    info!("");
    
    Ok(())
}

fn print_banner() {
    info!("");
    info!("╔══════════════════════════════════════════════════════════╗");
    info!("║                                                          ║");
    info!("║     AETHER GHOST MODE - ROTATING IP WORLDWIDE          ║");
    info!("║                                                          ║");
    info!("║     🌍 IP CHANGES EVERY SECOND 🌍                       ║");
    info!("║                                                          ║");
    info!("║     • 100+ Exit Locations Worldwide                     ║");
    info!("║     • Automatic Rotation (1 sec)                        ║");
    info!("║     • Post-Quantum Encryption                           ║");
    info!("║     • 5-Layer Mixnet                                    ║");
    info!("║                                                          ║");
    info!("╚══════════════════════════════════════════════════════════╝");
    info!("");
}

async fn start_proxy_services(config: Arc<AetherConfig>) -> Result<(), Box<dyn std::error::Error>> {
    // SOCKS5 proxy
    let socks5 = Arc::new(Socks5Server::new(
        "127.0.0.1:9050".to_string(),
        Arc::clone(&config),
    ));
    
    let socks5_clone = Arc::clone(&socks5);
    tokio::spawn(async move {
        if let Err(e) = socks5_clone.run().await {
            tracing::error!("SOCKS5 error: {}", e);
        }
    });
    
    // HTTP proxy
    let http_proxy = Arc::new(HttpProxy::new("127.0.0.1:8080".to_string()));
    let http_clone = Arc::clone(&http_proxy);
    tokio::spawn(async move {
        if let Err(e) = http_clone.run().await {
            tracing::error!("HTTP proxy error: {}", e);
        }
    });
    
    Ok(())
}
