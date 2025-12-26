# Testing Guide

## Overview

This guide covers testing strategies for the Aether Network research project.

## Unit Tests

### Running Unit Tests

```bash
# Run all tests
cargo test

# Run tests for specific module
cargo test crypto
cargo test protocols
cargo test mixnet
cargo test routing

# Run with output
cargo test -- --nocapture

# Run in release mode (faster)
cargo test --release
```

### Test Coverage

```bash
# Install tarpaulin for coverage
cargo install cargo-tarpaulin

# Generate coverage report
cargo tarpaulin --out Html --output-dir coverage

# View report
open coverage/index.html
```

## Integration Tests

Integration tests verify end-to-end functionality:

```bash
# Run integration tests
cargo test --test integration_test

# Run specific integration test
cargo test --test integration_test test_end_to_end_packet_flow
```

## Network Simulation

The Python simulator tests anonymity properties against adversaries.

### Basic Simulation

```bash
cd tests
python3 simulator.py
```

### Custom Simulation

```python
from simulator import AetherSimulator, SimulationConfig

# Configure simulation
config = SimulationConfig(
    num_nodes=100,          # Network size
    num_layers=5,           # Mixnet layers
    num_packets=1000,       # Packets to simulate
    adversary_coverage=0.2,  # 20% of nodes monitored
)

# Run simulation
sim = AetherSimulator(config)
results = sim.run_simulation()

# Visualize network
sim.plot_network()

# Analyze results
print(f"Anonymity Set: {results['anonymity_set']}")
print(f"Entropy: {results['entropy']:.2f} bits")
print(f"Correlation: {results['correlation_rate']:.1%}")
```

### Expected Results

For a 100-node network with 20% adversary coverage:

| Metric | Good | Excellent |
|--------|------|-----------|
| Anonymity Set | >50 | >80 |
| Entropy | >5.0 bits | >6.5 bits |
| Correlation Rate | <30% | <15% |
| Avg Latency | <500ms | <300ms |

## Performance Benchmarks

### 1. Crypto Benchmarks

```bash
cargo bench crypto_operations
```

Expected throughput:
- Kyber-1024 encapsulation: ~50,000 ops/sec
- Kyber-1024 decapsulation: ~40,000 ops/sec
- XChaCha20-Poly1305: ~500 MB/sec
- BLAKE3 hashing: >1 GB/sec

### 2. Packet Processing

```bash
cargo bench packet_processing
```

Expected throughput:
- Packet creation: ~10,000 packets/sec
- Layer processing: ~8,000 packets/sec
- Serialization: ~20,000 packets/sec

## Security Testing

### 1. Timing Attack Resistance

Test that mixing delays prevent timing correlation:

```rust
#[test]
fn test_timing_variance() {
    let strategy = StopAndGoMixing::new(50.0);
    let mut delays = Vec::new();
    
    for _ in 0..1000 {
        let delay = strategy.calculate_delay(1000, 0.5);
        delays.push(delay.as_millis());
    }
    
    // Calculate coefficient of variation
    let mean = delays.iter().sum::<u128>() as f64 / delays.len() as f64;
    let variance = delays.iter()
        .map(|&d| {
            let diff = d as f64 - mean;
            diff * diff
        })
        .sum::<f64>() / delays.len() as f64;
    let std_dev = variance.sqrt();
    let cv = std_dev / mean;
    
    // High variance is good for anonymity
    assert!(cv > 0.5);
}
```

### 2. Entropy Analysis

Verify traffic entropy is sufficiently high:

```python
import numpy as np
from tests.simulator import AetherSimulator, SimulationConfig

config = SimulationConfig(num_packets=10000)
sim = AetherSimulator(config)
results = sim.run_simulation()

# Entropy should be high (>6 bits for 100 nodes)
assert results['entropy'] > 6.0, f"Low entropy: {results['entropy']}"
```

### 3. Correlation Resistance

Test resistance to flow-correlation attacks:

```python
def test_correlation_resistance():
    config = SimulationConfig(adversary_coverage=0.3)  # 30% monitored
    sim = AetherSimulator(config)
    results = sim.run_simulation()
    
    # Should resist correlation even with 30% monitoring
    assert results['correlation_rate'] < 0.25, "Vulnerable to correlation"
```

## Load Testing

### Stress Test

Test network behavior under high load:

```rust
#[tokio::test]
async fn test_high_load() {
    let config = Arc::new(AetherConfig::default());
    let node = MixNode::new(2, NodeRole::MixNode, 1000, "127.0.0.1:9090", config).unwrap();
    
    // Spam with packets
    for _ in 0..10000 {
        let route = vec![KeyPair::generate().public_key; 5];
        let packet = OutfoxPacket::new(b"test", &route).unwrap();
        node.receive_packet(packet).await;
    }
    
    // Check queue doesn't explode
    let stats = node.get_stats().await;
    assert!(stats.queue_size < 1000);
}
```

## Fuzzing

### Crypto Fuzzing

```bash
# Install cargo-fuzz
cargo install cargo-fuzz

# Create fuzz targets
cargo fuzz init

# Fuzz packet parser
cargo fuzz run packet_parsing -- -max_total_time=300

# Fuzz crypto operations
cargo fuzz run crypto_ops -- -max_total_time=300
```

### Input Fuzzing

```rust
// fuzz/fuzz_targets/packet_parsing.rs
#![no_main]
use libfuzzer_sys::fuzz_target;
use aether_network::protocols::OutfoxPacket;

fuzz_target!(|data: &[u8]| {
    let _ = OutfoxPacket::from_bytes(data);
});
```

## Continuous Integration

### GitHub Actions Workflow

```yaml
name: Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - name: Run tests
        run: cargo test --all-features
      - name: Run clippy
        run: cargo clippy -- -D warnings
      - name: Check formatting
        run: cargo fmt -- --check
```

## Test Checklist

Before committing:

- [ ] All unit tests pass (`cargo test`)
- [ ] Integration tests pass
- [ ] Clippy has no warnings
- [ ] Code is formatted (`cargo fmt`)
- [ ] Simulation shows good anonymity metrics
- [ ] No timing side-channels detected
- [ ] Documentation is updated

## Troubleshooting

### Tests Fail to Compile

```bash
# Update dependencies
cargo update

# Clean and rebuild
cargo clean
cargo build
```

### Simulator Errors

```bash
# Install dependencies
pip3 install networkx numpy matplotlib

# Run with verbose output
python3 -v simulator.py
```

### Performance Issues

```bash
# Build in release mode
cargo build --release
cargo test --release

# Profile with perf
perf record --call-graph dwarf target/release/aether-network
perf report
```

## Next Steps

After testing:

1. **Review Results**: Analyze simulation output
2. **Tune Parameters**: Adjust config for better anonymity/latency
3. **Document Findings**: Record test results
4. **Iterate**: Improve based on test feedback

---

For questions or issues, please open a GitHub issue with test output.
