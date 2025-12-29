//! Tor Control Client for identity rotation
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::time::{interval, Duration};

pub struct TorController {
    control_addr: String,
}

impl TorController {
    pub fn new(control_addr: String) -> Self {
        Self { control_addr }
    }

    pub async fn start_rotation(&self, interval_secs: u64) {
        let addr = self.control_addr.clone();
        tokio::spawn(async move {
            let mut ticker = interval(Duration::from_secs(interval_secs));
            loop {
                ticker.tick().await;
                if let Err(e) = Self::send_newnym(&addr).await {
                    tracing::debug!("Tor NEWNYM failed: {}", e);
                }
            }
        });
    }

    async fn send_newnym(addr: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut stream = TcpStream::connect(addr).await?;
        
        // Authenticate (empty password by default for expert bundle if configured)
        stream.write_all(b"AUTHENTICATE \"\"\r\n").await?;
        let mut buf = [0u8; 128];
        let _ = stream.read(&mut buf).await?;
        
        // Signal NEWNYM (request new circuit)
        stream.write_all(b"SIGNAL NEWNYM\r\n").await?;
        let _ = stream.read(&mut buf).await?;
        
        tracing::info!("🔄 TOR IDENTITY JUMP: Requesting new global exit node...");
        Ok(())
    }
}
