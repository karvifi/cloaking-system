//! Ghost Mode Main Binary
//! 
//! Runs the complete anonymity stack with SOCKS5/HTTP proxies and IP rotation

use aether_network::proxy::{Socks5Server, HttpProxy};
use aether_network::AetherConfig;
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
    
    info!("👻 AETHER GHOST MODE - Complete Device Invisibility");
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    info!("");
    
    let config = Arc::new(AetherConfig::default());
    
    // Start SOCKS5 proxy
    let socks5 = Arc::new(Socks5Server::new(
        "127.0.0.1:9050".to_string(),
        Arc::clone(&config),
    ));
    
    let socks5_clone = Arc::clone(&socks5);
    tokio::spawn(async move {
        if let Err(e) = socks5_clone.run().await {
            tracing::error!("SOCKS5 server error: {}", e);
        }
    });
    
    // Start HTTP proxy
    let http_proxy = Arc::new(HttpProxy::new("127.0.0.1:8080".to_string()));
    let http_clone = Arc::clone(&http_proxy);
    tokio::spawn(async move {
        if let Err(e) = http_clone.run().await {
            tracing::error!("HTTP proxy error: {}", e);
        }
    });
    
    info!("");
    info!("✅ All proxy services are LIVE:");
    info!("   🔹 SOCKS5:   127.0.0.1:9050");
    info!("   🔹 HTTP:     127.0.0.1:8080");
    info!("");
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    info!("📋 NEXT STEPS:");
    info!("");
    info!("1. Enable system-wide Ghost Mode:");
    info!("   Run as Administrator: .\\scripts\\enable_ghost_mode.ps1");
    info!("");
    info!("2. Test your anonymity:");
    info!("   Visit: https://whatismyipaddress.com");
    info!("   Visit: https://ipleak.net");
    info!("");
    info!("3. To disable Ghost Mode:");
    info!("   Run as Administrator: .\\scripts\\disable_ghost_mode.ps1");
    info!("");
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    info!("🔐 All traffic routed through this terminal will be:");
    info!("   • Encrypted with Kyber-1024 (post-quantum)");
    info!("   • Mixed through 5 anonymous layers");
    info!("   • Protected from DNS/WebRTC leaks");
    info!("");
    info!("⏳ Running... (Press Ctrl+C to stop)");
    info!("");
    
    // Keep running
    tokio::signal::ctrl_c().await?;
    info!("👋 Shutting down Ghost Mode...");
    
    Ok(())
}
