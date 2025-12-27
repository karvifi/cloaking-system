//! Gateway Module - Local Interfacing for Anonymity
//! 
//! Provides SOCKS5 and other local proxy interfaces to the Aether Network.

pub mod socks;

pub use socks::SocksGateway;
