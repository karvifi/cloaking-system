//! Proxy module - SOCKS5 and HTTP proxies for complete anonymity

mod socks5;
mod http_proxy;
pub mod dns_resolver;

pub use socks5::Socks5Server;
pub use http_proxy::HttpProxy;
pub use dns_resolver::DohResolver;
