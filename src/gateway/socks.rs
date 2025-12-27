//! SOCKS5 Gateway for Aether Network
//! 
//! Bridges local TCP traffic into the Aether Mixnet for IP obfuscation.

use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use crate::client::advanced::AetherClient;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{info, warn, debug};

/// SOCKS5 Proxy Gateway
pub struct SocksGateway {
    /// Port to listen on (default 9050)
    pub port: u16,
    /// Reference to the Aether client
    pub client: Arc<Mutex<AetherClient>>,
}

impl SocksGateway {
    /// Create a new SOCKS5 gateway
    pub fn new(port: u16, client: Arc<Mutex<AetherClient>>) -> Self {
        Self { port, client }
    }

    /// Start the SOCKS5 server
    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let addr = format!("127.0.0.1:{}", self.port);
        let listener = TcpListener::bind(&addr).await?;
        info!("🔌 SOCKS5 Gateway listening on {}", addr);
        info!("🛡️  Configure your browser to use SOCKS5 at {}", addr);

        loop {
            let (stream, peer) = listener.accept().await?;
            debug!("Accepted connection from {}", peer);
            let client_ref = Arc::clone(&self.client);
            
            tokio::spawn(async move {
                if let Err(e) = handle_socks_connection(stream, client_ref).await {
                    warn!("SOCKS5 connection error: {}", e);
                }
            });
        }
    }
}

/// Handle a single SOCKS5 connection
async fn handle_socks_connection(
    mut stream: TcpStream,
    client: Arc<Mutex<AetherClient>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut buf = [0u8; 512];

    // 1. Handshake
    // [VERSION, NMETHODS, METHODS...]
    stream.read_exact(&mut buf[0..2]).await?;
    if buf[0] != 0x05 {
        return Err("Unsupported SOCKS version".into());
    }
    let nmethods = buf[1] as usize;
    stream.read_exact(&mut buf[0..nmethods]).await?;

    // Respond with No Authentication Required (0x00)
    stream.write_all(&[0x05, 0x00]).await?;

    // 2. Request
    // [VERSION, CMD, RSV, ATYP, DST.ADDR, DST.PORT]
    stream.read_exact(&mut buf[0..4]).await?;
    if buf[1] != 0x01 { // Only CONNECT is supported
        return Err("Only CONNECT command is supported".into());
    }

    let _atyp = buf[3];
    let mut dest_addr = String::new();
    
    match _atyp {
        0x01 => { // IPv4
            let mut ipv4 = [0u8; 4];
            stream.read_exact(&mut ipv4).await?;
            dest_addr = format!("{}.{}.{}.{}", ipv4[0], ipv4[1], ipv4[2], ipv4[3]);
        }
        0x03 => { // Domain Name
            let len = stream.read_u8().await? as usize;
            let mut domain = vec![0u8; len];
            stream.read_exact(&mut domain).await?;
            dest_addr = String::from_utf8_lossy(&domain).to_string();
        }
        _ => return Err("Unsupported address type".into()),
    }

    let dest_port = stream.read_u16().await?;
    info!("🕶️  Anonymizing request to {}:{}", dest_addr, dest_port);

    // 3. Respond to client
    // Version 5, Success 0, RSV 0, ATYP IPv4, Addr 0.0.0.0, Port 0
    stream.write_all(&[0x05, 0x00, 0x00, 0x01, 0, 0, 0, 0, 0, 0]).await?;

    // 4. Data Transfer Loop
    // In a real system, this would be sharded into Aether packets and sent to an exit node.
    // For this demonstration, we bridge to the Aether client's packet injector.
    
    let mut data_buf = [0u8; 1024];
    loop {
        let n = stream.read(&mut data_buf).await?;
        if n == 0 { break; }

        let payload = &data_buf[0..n];
        let mut client_lock = client.lock().await;
        
        // Wrap in post-quantum anonymity packet
        let mut recipient = [0u8; 32]; // In a real system, this would be an exit node ID
        recipient[0] = 0xFF; // Symbolic Exit Node
        
        client_lock.send_anonymous(payload, &recipient).await?;
        
        debug!("Encrypted {} bytes into Aether Mixnet", n);
        
        // Mock response to keep browser happy
        // In reality, the AetherClient would receive shards back and reassemble them.
        stream.write_all(b"HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n\r\n[Aether Network] Traffic successfully cloaked and routed via Mixnet.").await?;
        break; // Close after mock response for simulation
    }

    Ok(())
}
