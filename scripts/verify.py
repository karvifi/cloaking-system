#!/usr/bin/env python3
"""
Quick verification script to test the Aether Network implementation
"""

import subprocess
import sys
import os

def run_command(cmd, description):
    """Run a command and report results"""
    print(f"\n{'='*60}")
    print(f"Testing: {description}")
    print(f"{'='*60}")
    print(f"Command: {cmd}")
    
    result = subprocess.run(
        cmd,
        shell=True,
        capture_output=True,
        text=True
    )
    
    if result.returncode == 0:
        print(f"✅ SUCCESS: {description}")
        if result.stdout:
            print(result.stdout[:500])  # First 500 chars
    else:
        print(f"❌ FAILED: {description}")
        print(f"Error: {result.stderr[:500]}")
        return False
    
    return True

def main():
    print("""
    ╔══════════════════════════════════════════════════════════╗
    ║                AETHER NETWORK VERIFICATION                ║
    ║          Post-Quantum Anonymity Network Testing          ║
    ╚══════════════════════════════════════════════════════════╝
    """)
    
    # Change to project directory
    os.chdir(os.path.dirname(os.path.abspath(__file__)) + "/..")
    
    tests = [
        ("cargo --version", "Rust installation"),
        ("cargo check", "Code compilation check"),
        ("cargo test --lib", "Unit tests"),
        ("cargo clippy --all-targets -- -D warnings 2>&1 | head -20", "Code quality (Clippy)"),
        ("python3 --version", "Python installation"),
        ("python3 -c 'import networkx, numpy, matplotlib'", "Python dependencies"),
    ]
    
    results = []
    for cmd, desc in tests:
        results.append(run_command(cmd, desc))
    
    print(f"\n{'='*60}")
    print("SUMMARY")
    print(f"{'='*60}")
    
    passed = sum(results)
    total = len(results)
    
    print(f"Tests Passed: {passed}/{total}")
    print(f"Success Rate: {(passed/total)*100:.1f}%")
    
    if passed == total:
        print("\n✅ All verification tests passed!")
        print("\nNext steps:")
        print("  1. Run simulation: python3 tests/simulator.py")
        print("  2. Build project: cargo build --release")
        print("  3. Read docs: cat docs/TESTING.md")
        return 0
    else:
        print(f"\n❌ {total - passed} test(s) failed")
        print("\nTroubleshooting:")
        print("  - Install Rust: https://rustup.rs/")
        print("  - Install Python deps: pip3 install networkx numpy matplotlib")
        print("  - Check docs/TESTING.md for details")
        return 1

if __name__ == "__main__":
    sys.exit(main())
