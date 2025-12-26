//! Outfox Protocol implementation - Post-quantum packet format for mixnets

pub mod packet;
pub mod sphinx_compat;

pub use packet::{OutfoxPacket, PacketMetadata, create_packet, process_packet_layer};

/// Maximum packet size for Outfox (larger than Sphinx to accommodate post-quantum)
pub const MAX_PACKET_SIZE: usize = 10000;

/// Header size for Outfox packets
pub const HEADER_SIZE: usize = 1568 * 5; // 5 layers of Kyber-1024 ciphertexts (7840 bytes)

/// Metadata size
pub const METADATA_SIZE: usize = 128;

/// Payload size
pub const PAYLOAD_SIZE: usize = MAX_PACKET_SIZE - HEADER_SIZE - METADATA_SIZE; // 2032 bytes
