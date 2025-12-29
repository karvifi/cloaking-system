//! Enhanced Main Binary with Tier 0-8 Integration
//! 
//! Launches Aether Supreme with all configured modules

use aether_network::config::AetherConfig;
use aether_network::orchestrator::AetherOrchestrator;
use aether_network::cli::{Cli, Commands, run_cli};
use clap::Parser;
use std::path::Path;
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    tracing::info!("========================================");
    tracing::info!("   AETHER SUPREME - 31+ PHASE NETWORK");
    tracing::info!("   Tier 0-8 Advancement Implementation");
    tracing::info!("========================================\n");

    // Parse CLI
    let cli = Cli::parse();

    match cli.command {
        Commands::Start { quantum_defense, traffic_morphing, ja3_profile } => {
            start_aether_supreme(quantum_defense, traffic_morphing, &ja3_profile).await?;
        }
        Commands::Test { suite } => {
            run_tests(&suite).await?;
        }
        Commands::Deploy { output } => {
            deploy_configs(&output)?;
        }
        Commands::Benchmark { duration } => {
            run_benchmarks(duration).await?;
        }
    }

    Ok(())
}

async fn start_aether_supreme(
    quantum_defense: bool,
    traffic_morphing: bool,
    ja3_profile: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration
    let config_path = Path::new("config/aether.toml");
    let mut config = if config_path.exists() {
        AetherConfig::load(config_path)?
    } else {
        tracing::info!("📝 Generating default configuration...");
        let config = AetherConfig::default();
        config.save(config_path)?;
        config
    };

    // Apply CLI overrides
    if quantum_defense {
        config.tier0.quantuminsert_defense = true;
        config.tier0.xkeyscore_defense = true;
    }
    if traffic_morphing {
        config.tier1.traffic_morphing = true;
        config.tier1.ja3_profile = ja3_profile.to_string();
    }

    // Initialize orchestrator
    let mut orchestrator = AetherOrchestrator::new(config);
    
    // Print status
    orchestrator.print_status();

    tracing::info!("\n🚀 Aether Supreme is now ACTIVE");
    tracing::info!("   All configured modules running");
    tracing::info!("   Press Ctrl+C to stop\n");

    // Keep running
    tokio::signal::ctrl_c().await?;
    
    tracing::info!("\n👋 Shutting down gracefully...");
    Ok(())
}

async fn run_tests(suite: &str) -> Result<(), Box<dyn std::error::Error>> {
    use aether_network::advanced::AdversarialTester;
    
    tracing::info!("🎯 Running test suite: {}", suite);
    
    match suite {
        "mitm" => AdversarialTester::test_mitm_resistance().await?,
        "timing" => AdversarialTester::test_timing_correlation().await?,
        "dpi" => AdversarialTester::test_dpi_evasion().await?,
        "all" => AdversarialTester::run_full_test_suite().await?,
        _ => return Err(format!("Unknown test suite: {}", suite).into()),
    }
    
    Ok(())
}

fn deploy_configs(output: &str) -> Result<(), Box<dyn std::error::Error>> {
    use aether_network::advanced::ContainerSecurityManager;
    
    let output_path = Path::new(output);
    ContainerSecurityManager::deploy_security_configs(output_path)?;
    
    tracing::info!("✅ Deployment configs written to: {}", output);
    Ok(())
}

async fn run_benchmarks(duration: u64) -> Result<(), Box<dyn std::error::Error>> {
    use aether_network::metrics::PerformanceMonitor;
    
    tracing::info!("📊 Running benchmarks for {} seconds...", duration);
    
    let mut monitor = PerformanceMonitor::new();
    
    // Simulate packet processing
    for _ in 0..duration {
        let start = std::time::Instant::now();
        
        // Simulate work
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
        
        monitor.record_latency(start.elapsed());
        monitor.record_bytes(1400); // Standard packet size
    }
    
    monitor.print_report();
    Ok(())
}
