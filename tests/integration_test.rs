//! Integration tests for Aether Network

use aether_network::{
    AetherConfig,
    crypto::kyber::KeyPair,
    protocols::{OutfoxPacket},
    mixnet::{MixNode, NodeRole},
};
use std::sync::Arc;

#[tokio::test]
async fn test_end_to_end_packet_flow() {
    // Create configuration
    let config = Arc::new(AetherConfig::default());
    
    // Create 5 mix nodes (one per layer)
    let mut nodes = Vec::new();
    for layer in 1..=5 {
        let node = MixNode::new(
            layer,
            NodeRole::MixNode,
            1000,
            format!("127.0.0.1:{}", 9000 + layer),
            Arc::clone(&config),
        ).unwrap();
        nodes.push(node);
    }
    
    // Create route (public keys of nodes)
    let route: Vec<_> = nodes.iter()
        .map(|n| {
            let kp = KeyPair::generate();
            kp.public_key
        })
        .collect();
    
    // Create a test packet
    let message = b"Test message for anonymity network";
    let packet = OutfoxPacket::new(message, &route).unwrap();
    
    // Verify packet properties
    assert_eq!(packet.metadata.layer, 0);
    assert!(packet.verify_integrity());
    
    // Serialize and deserialize
    let bytes = packet.to_bytes().unwrap();
    let decoded = OutfoxPacket::from_bytes(&bytes).unwrap();
    
    assert_eq!(packet.metadata.packet_id, decoded.metadata.packet_id);
}

#[test]
fn test_crypto_operations() {
    // Test Kyber key generation and encapsulation
    use aether_network::crypto::kyber::{KeyPair, encapsulate, decapsulate};
    
    let kp = KeyPair::generate();
    let (ct, ss1) = encapsulate(&kp.public_key);
    let ss2 = decapsulate(&ct, &kp.secret_key).unwrap();
    
    assert_eq!(ss1.as_bytes(), ss2.as_bytes());
}

#[test]
fn test_symmetric_encryption() {
    use aether_network::crypto::symmetric::{encrypt_aead, decrypt_aead, generate_key, generate_nonce};
    
    let key = generate_key();
    let nonce = generate_nonce();
    let plaintext = b"Sensitive data";
    let aad = b"metadata";
    
    let ciphertext = encrypt_aead(&key, &nonce, plaintext, aad).unwrap();
    let decrypted = decrypt_aead(&key, &nonce, &ciphertext, aad).unwrap();
    
    assert_eq!(plaintext, &decrypted[..]);
}

#[test]
fn test_reputation_system() {
    use aether_network::routing::reputation::ReputationSystem;
    
    let mut system = ReputationSystem::new();
    let node_id = [42u8; 32];
    
    system.init_node(node_id, 0.5);
    
    // Simulate successful operations
    for _ in 0..100 {
        system.record_success(&node_id, 50);
    }
    
    let rep = system.get_reputation(&node_id).unwrap();
    assert!(rep.score > 0.5);
    assert!(rep.packets_processed == 100);
}

#[test]
fn test_traffic_shaping() {
    use aether_network::mixnet::traffic::TrafficShaper;
    
    let mut shaper = TrafficShaper::new(100.0, 0.4);
    
    // Record packets
    for i in 0..100 {
        shaper.record_packet(i % 10 == 0);
        std::thread::sleep(std::time::Duration::from_millis(5));
    }
    
    // Check entropy
    let entropy = shaper.calculate_entropy();
    assert!(entropy > 0.0);
}
