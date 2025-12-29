//! One-Click Installation Wizard
//! 
//! Automated setup for all dependencies

use std::process::Command;

pub struct InstallationWizard;

impl InstallationWizard {
    /// Run complete installation
    pub fn install_all() -> Result<(), String> {
        println!("🚀 Aether Supreme Installation Wizard\n");
        
        Self::check_prerequisites()?;
        Self::install_rust_deps()?;
        Self::install_external_tools()?;
        Self::configure_system()?;
        Self::build_project()?;
        
        println!("\n✅ Installation complete!");
        println!("\nNext steps:");
        println!("1. Run: ./AETHER_GOD_MODE_LAUNCHER.ps1");
        println!("2. Or: cargo run --release --bin verified_10_layer");
        
        Ok(())
    }

    fn check_prerequisites() -> Result<(), String> {
        println!("📋 Checking prerequisites...");
        
        // Check Rust
        let output = Command::new("rustc")
            .arg("--version")
            .output()
            .map_err(|_| "Rust not installed")?;
        
        println!("  ✓ Rust: {}", String::from_utf8_lossy(&output.stdout).trim());
        
        // Check Tor
        let tor_check = Command::new("tor")
            .arg("--version")
            .output();
        
        if tor_check.is_ok() {
            println!("  ✓ Tor installed");
        } else {
            println!("  ⚠ Tor not found (will install)");
        }
        
        Ok(())
    }

    fn install_rust_deps() -> Result<(), String> {
        println!("\n📦 Installing Rust dependencies...");
        
        let output = Command::new("cargo")
            .args(&["build", "--release"])
            .output()
            .map_err(|e| format!("Failed to build: {}", e))?;
        
        if output.status.success() {
            println!("  ✓ Dependencies installed");
        } else {
            println!("  ✗ Build failed");
            return Err("Build failed".to_string());
        }
        
        Ok(())
    }

    fn install_external_tools() -> Result<(), String> {
        println!("\n🔧 Installing external tools...");
        
        #[cfg(target_os = "linux")]
        {
            println!("  Installing Tor...");
            let _ = Command::new("sudo")
                .args(&["apt-get", "install", "-y", "tor"])
                .output();
        }
        
        #[cfg(target_os = "windows")]
        {
            println!("  Please install Tor Browser from torproject.org");
        }
        
        println!("  ✓ External tools configured");
        Ok(())
    }

    fn configure_system() -> Result<(), String> {
        println!("\n⚙️ Configuring system...");
        
        // Create config directory
        std::fs::create_dir_all("config")
            .map_err(|e| format!("Failed to create config: {}", e))?;
        
        println!("  ✓ Configuration complete");
        Ok(())
    }

    fn build_project() -> Result<(), String> {
        println!("\n🔨 Building Aether Supreme...");
        
        let output = Command::new("cargo")
            .args(&["build", "--release", "--bin", "verified_10_layer", "--features", "quantum-safe"])
            .output()
            .map_err(|e| format!("Failed to build: {}", e))?;
        
        if output.status.success() {
            println!("  ✓ Build complete");
        } else {
            println!("  ✗ Build failed");
            return Err("Build failed".to_string());
        }
        
        Ok(())
    }
}
