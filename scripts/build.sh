#!/usr/bin/env bash
# Build script for Aether Network

set -e

echo "======================================"
echo "   Aether Network Build Script"
echo "======================================"

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Check for Rust
if ! command -v cargo &> /dev/null; then
    echo -e "${RED}Error: Rust is not installed${NC}"
    echo "Please install Rust from https://rustup.rs/"
    exit 1
fi

echo -e "${BLUE}Rust version:${NC}"
rustc --version
cargo --version

# Clean previous builds
echo -e "\n${BLUE}Cleaning previous builds...${NC}"
cargo clean

# Generate cryptographic keys
echo -e "\n${BLUE}Generating cryptographic keys...${NC}"
mkdir -p keys
if [ ! -f keys/network_id.bin ]; then
    head -c 32 /dev/urandom > keys/network_id.bin
    echo -e "${GREEN}✓ Generated network ID${NC}"
fi

# Build in release mode
echo -e "\n${BLUE}Building Aether Network (release mode)...${NC}"
cargo build --release

# Run tests
echo -e "\n${BLUE}Running tests...${NC}"
cargo test --release

# Check formatting
echo -e "\n${BLUE}Checking code formatting...${NC}"
cargo fmt -- --check || {
    echo -e "${RED}Warning: Code formatting check failed${NC}"
    echo "Run 'cargo fmt' to fix formatting"
}

# Run Clippy linter
echo -e "\n${BLUE}Running Clippy linter...${NC}"
cargo clippy --release -- -D warnings || {
    echo -e "${RED}Warning: Clippy found issues${NC}"
}

# Build documentation
echo -e "\n${BLUE}Building documentation...${NC}"
cargo doc --no-deps

# Create distribution
echo -e "\n${BLUE}Creating distribution...${NC}"
mkdir -p dist
cp target/release/aether-network dist/
cp config/default.toml dist/config.toml
cp README.md dist/

# Display build info
NETWORK_ID=$(xxd -p keys/network_id.bin | tr -d '\n')

echo -e "\n======================================"
echo -e "${GREEN}   Build Successful!${NC}"
echo "======================================"
echo -e "Network ID: ${BLUE}0x${NETWORK_ID}${NC}"
echo -e "Binary: ${BLUE}dist/aether-network${NC}"
echo -e "Documentation: ${BLUE}target/doc/aether_network/index.html${NC}"
echo ""
echo "Next steps:"
echo "  1. Review configuration: dist/config.toml"
echo "  2. Run tests: cargo test"
echo "  3. Run simulator: python3 tests/simulator.py"
echo "======================================"
