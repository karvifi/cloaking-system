//! HTTP CONNECT and Transparent Proxy that tunnels through SOCKS5
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub struct HttpProxy {
    pub bind_addr: String,
    pub socks5_addr: String,
}

impl HttpProxy {
    pub fn new(bind_addr: String, socks5_addr: String) -> Self {
        Self { bind_addr, socks5_addr }
    }

    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let listener = TcpListener::bind(&self.bind_addr).await?;
        tracing::info!("🔌 HTTP ↔ SOCKS5 Bridge listening on {}", self.bind_addr);

        loop {
            let (socket, addr) = listener.accept().await?;
            let socks5_addr = self.socks5_addr.clone();
            
            tokio::spawn(async move {
                if let Err(e) = Self::handle_client(socket, socks5_addr).await {
                    tracing::debug!("HTTP proxy client closed: {}", e);
                }
            });
        }
    }

    async fn handle_client(mut client: TcpStream, socks5_addr: String) -> Result<(), Box<dyn std::error::Error>> {
        let mut buf = vec![0u8; 8192];
        let n = client.read(&mut buf).await?;
        if n == 0 { return Ok(()); }
        
        let request_str = String::from_utf8_lossy(&buf[..n]);
        let first_line = request_str.lines().next().unwrap_or("");
        let parts: Vec<&str> = first_line.split_whitespace().collect();
        
        if parts.len() < 2 {
            client.write_all(b"HTTP/1.1 400 Bad Request\r\n\r\n").await?;
            return Ok(());
        }

        let method = parts[0];
        let url = parts[1];
        
        let (dest_host, dest_port, is_connect) = if method == "CONNECT" {
            let h_parts: Vec<&str> = url.split(':').collect();
            let host = h_parts[0];
            let port = if h_parts.len() > 1 { h_parts[1].parse::<u16>().unwrap_or(443) } else { 443 };
            (host.to_string(), port, true)
        } else {
            // Standard GET/POST with absolute URL
            // Format: http://host[:port]/path
            let url_stripped = if url.starts_with("http://") { &url[7..] } else { url };
            let (host_port, _) = url_stripped.split_once('/').unwrap_or((url_stripped, ""));
            let h_parts: Vec<&str> = host_port.split(':').collect();
            let host = h_parts[0];
            let port = if h_parts.len() > 1 { h_parts[1].parse::<u16>().unwrap_or(80) } else { 80 };
            (host.to_string(), port, false)
        };

        tracing::info!("🌐 HTTP BRIDGE ({}) → {}:{}", method, dest_host, dest_port);

        // Connect to our SOCKS5 proxy
        let mut socks_stream = TcpStream::connect(&socks5_addr).await?;
        
        // SOCKS5 greeting
        socks_stream.write_all(&[5, 1, 0]).await?;
        let mut greeting_resp = [0u8; 2];
        socks_stream.read_exact(&mut greeting_resp).await?;
        
        // SOCKS5 request (DOMAIN NAME)
        let mut req = vec![5, 1, 0, 3];
        req.push(dest_host.len() as u8);
        req.extend_from_slice(dest_host.as_bytes());
        req.extend_from_slice(&dest_port.to_be_bytes());
        
        socks_stream.write_all(&req).await?;
        
        let mut req_resp = vec![0u8; 10];
        socks_stream.read_exact(&mut req_resp).await?;
        
        if req_resp[1] == 0 {
            if is_connect {
                // Success! Tell HTTP client
                client.write_all(b"HTTP/1.1 200 Connection Established\r\n\r\n").await?;
            } else {
                // For non-CONNECT, we must send the initial request buffer to destination
                socks_stream.write_all(&buf[..n]).await?;
            }
            tokio::io::copy_bidirectional(&mut client, &mut socks_stream).await?;
        } else {
            client.write_all(b"HTTP/1.1 502 Bad Gateway\r\n\r\n").await?;
        }

        Ok(())
    }
}
