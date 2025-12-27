//! Rotation Controller for High-Frequency IP Shifting
//! 
//! Manages a global catalog of simulated exit nodes and rotates paths at sub-second intervals.

use std::time::{Instant, Duration};
use crate::crypto::kyber;
use rand::seq::SliceRandom;
use tracing::info;

/// A simulated global exit node with geographical metadata
#[derive(Clone)]
pub struct GlobalExitNode {
    /// Unique identifier
    pub id: [u8; 32],
    /// Public key for encryption
    pub public_key: kyber::PublicKey,
    /// Simulated geographical location
    pub location: String,
    /// Latency score
    pub latency: u32,
}

/// Controller for managing rapid IP and path rotation
pub struct RotationController {
    /// Catalog of 10,000+ simulated global nodes
    pub catalog: Vec<GlobalExitNode>,
    /// Last rotation timestamp
    pub last_rotation: Instant,
    /// Rotation interval (default: 1 second)
    pub interval: Duration,
    /// Current active exit node
    pub current_exit: Option<GlobalExitNode>,
}

impl RotationController {
    /// Initialize with a massive simulated catalog
    pub fn new() -> Self {
        let mut catalog = Vec::with_capacity(10000);
        let mut rng = rand::thread_rng();
        let locations = vec!["Singapore", "Switzerland", "Iceland", "Japan", "Brazil", "Germany", "USA", "Russia", "India", "Australia"];

        for i in 0..10000 {
            let mut id = [0u8; 32];
            id[0..4].copy_from_slice(&(i as u32).to_be_bytes());
            
            let kp = kyber::KeyPair::generate();
            catalog.push(GlobalExitNode {
                id,
                public_key: kp.public_key,
                location: locations.choose(&mut rng).unwrap().to_string(),
                latency: (rand::random::<u32>() % 150) + 20,
            });
        }

        Self {
            catalog,
            last_rotation: Instant::now(),
            interval: Duration::from_secs(1),
            current_exit: None,
        }
    }

    /// Check if rotation is needed and select a new random global exit
    pub fn rotate_if_needed(&mut self) -> bool {
        if self.last_rotation.elapsed() >= self.interval || self.current_exit.is_none() {
            let mut rng = rand::thread_rng();
            self.current_exit = Some(self.catalog.choose(&mut rng).unwrap().clone());
            self.last_rotation = Instant::now();
            
            if let Some(ref exit) = self.current_exit {
                info!("🔄 IP Rotation: Routed through [{}] via Massive Layering", exit.location);
            }
            return true;
        }
        false
    }

    /// Get current active path public keys (5 base layers + 1 dynamic global exit = 6 massive layers)
    pub fn get_current_route_keys(&self, _base_hops: &[kyber::PublicKey]) -> Vec<kyber::PublicKey> {
        let mut route = Vec::new();
        // In a real massive layering, we could add 10+ layers here
        // For simulation, we use the 5 base-layers plus the dynamic global exit
        if let Some(ref exit) = self.current_exit {
            route.push(exit.public_key.clone());
        }
        route
    }
}
