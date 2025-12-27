# Aether Network

**A Post-Quantum Anonymity Network for Cybersecurity Research**

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org/)

## ⚠️ RESEARCH DISCLAIMER

This project is developed for **cybersecurity research and educational purposes only**. It is designed to:
- Test and prove theories about network anonymity
- Explore post-quantum cryptography in mixnets
- Research traffic analysis resistance techniques
- Study advanced anonymity network architectures

**NOT intended for production use or illegal activities.**

## 🔬 Overview

Aether Network is an advanced mixnet implementation featuring:

- **Post-Quantum Cryptography**: Kyber-1024 KEM for quantum-resistant encryption
- **Outfox Protocol**: Modern packet format for layered mixnets
- **Stop-and-Go Mixing**: Exponential delays with Poisson distribution
- **Cover Traffic**: Entropy-based dummy packet generation
- **Multi-Path Routing**: K-disjoint paths for redundancy
- **Reputation System**: Stake-based node selection with slashing
- **Traffic Analysis Resistance**: Multiple countermeasures against correlation attacks

## 📋 Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│ Layer 5: Application Proxy │ ZK-Identity │ Quantum Random       │
├─────────────────────────────────────────────────────────────────┤
│ Layer 4: Multi-Path Routing │ Adaptive Topology │ Mesh Network │
├─────────────────────────────────────────────────────────────────┤
│ Layer 3: Outfox Protocol │ Sphinx Packets │ Mixnet Routing      │
├─────────────────────────────────────────────────────────────────┤
│ Layer 2: Quantum-Resistant Crypto │ Hybrid KEM │ Forward Secrecy│
├─────────────────────────────────────────────────────────────────┤
│ Layer 1: Hardware Roots │ TPM/HSM │ Secure Enclave             │
└─────────────────────────────────────────────────────────────────┘
```

### Core Components

- **Crypto Module**: Post-quantum KEMs, AEAD, hashing, signatures
- **Protocols**: Outfox packet format with layered encryption
- **Mixnet**: Stratified network with 5 layers
- **Routing**: Multi-path algorithms with reputation-based selection
- **Metrics**: Prometheus-compatible monitoring

### 🏃 Extreme Active Mode (Continuous Simulation)

To run the full network continuously in a single process with real-time simulations of ZK proofs, traffic routing, and system health monitoring:

```bash
# Run the continuous anonymity network with all advanced features
cargo run --features full,zkproofs
```

### 🕶️ Cloaking Mode (Hide Your IP)

The Aether Network now includes a **Local SOCKS5 Gateway**. You can route your real browser or application traffic through the post-quantum mixnet to hide your IP address.

1.  **Start the Gateway**: Run the network using the command above. It will listen on `127.0.0.1:9050`.
2.  **Configure Application**: Set your browser (Chrome/Firefox) or system proxy to `SOCKS5` with address `127.0.0.1` and port `9050`.
3.  **Use Stealth Scripts**:
    ```bash
    # Option A: Shell script (automatic browser launch)
    ./scripts/stealth_browser.sh

    # Option B: Advanced Python Stealth (Undetected Chromedriver)
    python scripts/stealth_browser.py
    ```

💡 **Tip**: Use [ipleak.net](https://ipleak.net) to verify that your real IP is no longer visible.

### Build

```bash
# Clone the repository
git clone https://github.com/yourusername/aether-network.git
cd aether-network

# Build the project
chmod +x scripts/build.sh
./scripts/build.sh

# Or use cargo directly
cargo build --release
```

### Run Tests

```bash
# Run unit tests
cargo test

# Run integration tests
cargo test --test integration

# Run simulator
python3 tests/simulator.py
```

### Configuration

Edit `config/default.toml` to customize:

```toml
[global]
network_id = "0x..."
mixnet_layers = 5
max_packet_size = 2413

[crypto]
pq_algorithm = "kyber1024"
quantum_safe = true

[timing]
poisson_lambda = 50.0  # Mean delay in ms

[traffic]
cover_traffic_ratio = 0.4  # 40% dummy traffic
```

## 📊 Testing & Simulation

### Network Simulator

The Python simulator tests anonymity properties:

```bash
python3 tests/simulator.py
```

Output includes:
- **Anonymity Set Size**: Number of indistinguishable senders
- **Traffic Entropy**: Shannon entropy of traffic distribution
- **Correlation Success Rate**: Adversary's ability to link flows
- **Average Latency**: End-to-end packet delay

### Unit Tests

```bash
# Test crypto primitives
cargo test crypto

# Test packet processing
cargo test protocols

# Test routing algorithms
cargo test routing
```

## 🔐 Security Features

### Against Passive Adversaries

- **Layered Encryption**: 5 hops with Kyber-1024
- **Fixed Packet Size**: All packets padded to 2413 bytes
- **Random Delays**: Exponential distribution breaks timing correlation
- **Cover Traffic**: Dummy packets increase anonymity set

### Against Active Adversaries

- **Integrity Verification**: BLAKE3 tags detect tampering
- **Reputation System**: Malicious nodes lose stake
- **Multi-Path Routing**: Redundancy prevents single point of failure

### Against Quantum Adversaries

- **Kyber-1024 KEM**: Post-quantum key encapsulation
- **Forward Secrecy**: Ephemeral keys for each packet
- **Quantum-Random Nonces**: Unpredictable initialization vectors

## 📈 Performance

Typical performance on modern hardware:

- **Throughput**: ~1000 packets/second per node
- **Latency**: 250-500ms (5 hops with 50ms mean delay)
- **Memory**: ~500MB per node
- **CPU**: ~10% for mixing operations

## 🛠️ Development

### Project Structure

```
aether-network/
├── src/
│   ├── crypto/         # Cryptographic primitives
│   ├── protocols/      # Outfox packet format
│   ├── mixnet/         # Mix nodes and strategies
│   ├── routing/        # Path finding algorithms
│   └── lib.rs          # Main library entry
├── config/             # Configuration files
├── tests/              # Integration tests & simulator
├── scripts/            # Build and deployment scripts
└── Cargo.toml          # Rust dependencies
```

### Contributing

This is a research project. Contributions welcome:

1. Fork the repository
2. Create a feature branch
3. Make your changes with tests
4. Submit a pull request

## 📚 References

This implementation is based on:

- **Nym**: [github.com/nymtech/nym](https://github.com/nymtech/nym)
- **Outfox**: [arXiv:2412.19937](https://arxiv.org/abs/2412.19937)
- **LARMix++**: [eprint.iacr.org/2024/1485](https://eprint.iacr.org/2024/1485)
- **Loopix**: USENIX Security 2017
- **Sphinx**: IEEE S&P 2009

## ⚖️ Legal & Ethical Use

**IMPORTANT**: This software is provided for educational and research purposes only.

✅ **Permitted Uses**:
- Academic research on anonymity networks
- Security testing in controlled environments
- Educational demonstrations
- Privacy technology development

❌ **Prohibited Uses**:
- Any illegal activities
- Circumventing lawful access restrictions
- Facilitating criminal behavior
- Production deployment without thorough security audit

## 📝 License

MIT License - see [LICENSE](LICENSE) file for details.

Copyright (c) 2025 Aether Research Team

---

**For cybersecurity research and educational purposes only.**

## 🔗 Resources

- [Documentation](docs/)
- [Research Papers](docs/papers/)
- [Security Analysis](docs/security.md)
- [FAQ](docs/faq.md)

---

Built with ❤️ for privacy research
