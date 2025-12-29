//! Masscan Integration for Dead-Zone Detection & Honeypot Avoidance
//! 
//! Pre-flight scanning of circuit hops to avoid compromised infrastructure

use serde::{Deserialize, Serialize};
use std::process::Command;
use std::net::IpAddr;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ScanResult {
    pub ip: IpAddr,
    pub is_honeypot: bool,
    pub is_responsive: bool,
    pub suspicious_ports: Vec<u16>,
    pub response_time_ms: u64,
}

pub struct MasscanIntegration {
    /// Path to masscan executable
    masscan_path: String,
}

impl MasscanIntegration {
    pub fn new(masscan_path: String) -> Self {
        Self { masscan_path }
    }

    /// Pre-flight scan next N circuit hops
    pub async fn scan_circuit_hops(&self, ips: &[IpAddr]) -> Result<Vec<ScanResult>, String> {
        tracing::info!("🔍 MASSCAN: Scanning {} potential circuit hops", ips.len());
        
        let mut results = Vec::new();
        
        for ip in ips {
            let result = self.scan_single_node(ip).await?;
            results.push(result);
        }
        
        tracing::info!("✅ MASSCAN: Scan complete, {} nodes analyzed", results.len());
        Ok(results)
    }

    async fn scan_single_node(&self, ip: &IpAddr) -> Result<ScanResult, String> {
        let ip_str = ip.to_string();
        
        // Scan common Tor relay ports + suspicious honeypot indicators
        let output = Command::new(&self.masscan_path)
            .args(&[
                &ip_str,
                "-p", "9001,9030,80,443,22,21,23", // Tor + suspicious ports
                "--rate", "1000",
                "--wait", "1",
            ])
            .output()
            .map_err(|e| format!("Masscan execution failed: {}", e))?;

        let scan_output = String::from_utf8_lossy(&output.stdout);
        
        // Analyze for honeypot indicators
        let is_honeypot = self.detect_honeypot(&scan_output);
        let suspicious_ports = self.extract_ports(&scan_output);
        
        Ok(ScanResult {
            ip: *ip,
            is_honeypot,
            is_responsive: !suspicious_ports.is_empty(),
            suspicious_ports,
            response_time_ms: 10, // Placeholder
        })
    }

    fn detect_honeypot(&self, scan_output: &str) -> bool {
        // Honeypot indicators:
        // - Too many open ports (>20)
        // - Suspicious port combinations
        // - Uncommon service banners
        
        let port_count = scan_output.matches("open").count();
        
        if port_count > 20 {
            tracing::warn!("🚨 HONEYPOT DETECTED: {} open ports (suspicious)", port_count);
            return true;
        }
        
        false
    }

    fn extract_ports(&self, scan_output: &str) -> Vec<u16> {
        // Parse masscan output for open ports
        scan_output.lines()
            .filter(|line| line.contains("open"))
            .filter_map(|line| {
                line.split_whitespace()
                    .find(|word| word.parse::<u16>().is_ok())
                    .and_then(|port_str| port_str.parse().ok())
            })
            .collect()
    }

    /// Filter out honeypot nodes from circuit selection
    pub fn filter_safe_nodes(&self, results: Vec<ScanResult>) -> Vec<IpAddr> {
        let safe_nodes: Vec<IpAddr> = results.iter()
            .filter(|r| !r.is_honeypot && r.is_responsive)
            .map(|r| r.ip)
            .collect();
        
        tracing::info!("✅ SAFE NODES: {}/{} nodes passed honeypot filter", 
            safe_nodes.len(), results.len());
        
        safe_nodes
    }
}

/// Installation: https://github.com/robertdavidgraham/masscan
/// Usage: masscan 192.168.0.1 -p80,443 --rate 1000
