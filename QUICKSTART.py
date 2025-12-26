#!/usr/bin/env python3
"""
Quick start script for first-time users
"""

import os
import sys

def print_header():
    print("""
╔════════════════════════════════════════════════════════════════════╗
║                                                                    ║
║          🔐 AETHER NETWORK - QUICK START GUIDE 🔐                 ║
║                                                                    ║
║       Post-Quantum Anonymity Network Research Project             ║
║                                                                    ║
╚════════════════════════════== ═══════════════════════════════════╝
    """)

def print_section(title):
    print(f"\n{'='*70}")
    print(f"  {title}")
    print(f"{'='*70}\n")

def main():
    print_header()
    
    print_section("📚 WHAT IS AETHER?")
    print("""
Aether is a research implementation of an advanced anonymity network featuring:

  ✓ Post-Quantum Cryptography (Kyber-1024)
  ✓ 5-Layer Mixnet with Stop-and-Go Mixing
  ✓ Cover Traffic (40% dummy packets)
  ✓ Multi-Path Routing
  ✓ Reputation System
  ✓ Traffic Analysis Resistance

⚠️  FOR RESEARCH AND EDUCATIONAL PURPOSES ONLY
    """)
    
    print_section("🎯 YOUR RESEARCH GOAL")
    print("""
You want to prove that:
  1. Post-quantum crypto is practical in mixnets
  2. Proper mixing defeats traffic analysis
  3. Cover traffic increases anonymity
  4. Multi-path routing provides redundancy

This implementation lets you test these theories!
    """)
    
    print_section("🚀 QUICK START")
    print("""
Step 1: Verify Environment
  Run: python scripts/verify.py

Step 2: Build the Project
  Run: cargo build --release

Step 3: Run Tests
  Run: cargo test

Step 4: Run Simulation
  Run: python tests/simulator.py

Step 5: Analyze Results
  Check: Anonymity set, entropy, correlation rate
    """)
    
    print_section("📊 EXPECTED SIMULATION RESULTS")
    print("""
For a 100-node network with 20% adversary coverage:

  Metric               | Good      | Excellent
  ---------------------|-----------|------------
  Anonymity Set       | >50       | >80
  Traffic Entropy     | >5 bits   | >6.5 bits
  Correlation Rate    | <30%      | <15%
  Average Latency     | <500ms    | <300ms

If you see "Excellent" results, your theory is validated! ✅
    """)
    
    print_section("🔬 TESTING YOUR THEORY")
    print("""
Experiment 1: Vary Cover Traffic
  Edit config/default.toml:
    cover_traffic_ratio = 0.0   # 0% cover
    cover_traffic_ratio = 0.2   # 20% cover
    cover_traffic_ratio = 0.4   # 40% cover (default)
    cover_traffic_ratio = 0.6   # 60% cover
  
  Run simulation for each, compare anonymity metrics.

Experiment 2: Test Against Stronger Adversaries
  In tests/simulator.py, change:
    adversary_coverage = 0.1   # 10% monitored
    adversary_coverage = 0.2   # 20% monitored (default)
    adversary_coverage = 0.3   # 30% monitored
    adversary_coverage = 0.5   # 50% monitored (very strong)
  
  Measure correlation success rate.

Experiment 3: Compare Mixing Strategies
  In config/default.toml:
    poisson_lambda = 25.0   # Fast (higher risk)
    poisson_lambda = 50.0   # Balanced (default)
    poisson_lambda = 100.0  # Slow (more anonymous)
  
  Trade-off: latency vs. anonymity.
    """)
    
    print_section("📖 DOCUMENTATION")
    print("""
  README.md              - Project overview
  docs/TESTING.md        - Testing procedures
  docs/SECURITY.md       - Security analysis
  docs/RESEARCH_PAPER.md - Academic template
  PROJECT_STATUS.md      - Current status & next steps
    """)
    
    print_section("⚠️  IMPORTANT DISCLAIMERS")
    print("""
1. This is a RESEARCH PROTOTYPE
   - Not production-ready
   - Not security-audited
   - For controlled testing only

2. LEGAL & ETHICAL USE
   ✅ Academic research
   ✅ Security testing in lab
   ✅ Educational demonstrations
   ❌ Illegal activities
   ❌ Real-world anonymous communication (without audit)

3. YOUR RESPONSIBILITY
   - Comply with all applicable laws
   - Follow ethical research practices
   - Cite prior work properly
    """)
    
    print_section("🏁 NEXT ACTIONS")
    print("""
 1. [ ] Read README.md
    2. [ ] Run: python scripts/verify.py
 3. [ ] Run: cargo test
 4. [ ] Run: python tests/simulator.py
 5. [ ] Review results
 6. [ ] Experiment with parameters
 7. [ ] Document findings

Ready to validate your theory? Let's go! 🚀
    """)
    
    print(f"{'='*70}\n")
    
    # Check if Rust is installed
    try:
        import subprocess
        result = subprocess.run(['cargo', '--version'], capture_output=True, text=True)
        if result.returncode == 0:
            print("✅ Rust is installed:", result.stdout.strip())
        else:
            print("❌ Rust not found. Install from: https://rustup.rs/")
    except FileNotFoundError:
        print("❌ Rust not found. Install from: https://rustup.rs/")
    
    # Check if Python deps are available
    try:
        import networkx
        import numpy
        print("✅ Python dependencies installed")
    except ImportError:
        print("❌ Missing Python dependencies")
        print("   Install: pip install networkx numpy matplotlib")
    
    print(f"\n{'='*70}\n")
    print("For detailed instructions, run: python scripts/verify.py")
    print(f"{'='*70}\n")

if __name__ == "__main__":
    main()
