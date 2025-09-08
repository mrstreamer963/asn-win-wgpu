#!/bin/bash

# ASN Web Example Quick Start Script

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}🚀 Starting ASN Web Example with Trunk...${NC}"

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    echo -e "${RED}❌ Error: Cargo.toml not found. Please run this script from the ex_web directory.${NC}"
    exit 1
fi

# Function to check if a command exists
command_exists() {
    command -v "$1" &> /dev/null
}

# Check if trunk is installed
if ! command_exists trunk; then
    echo -e "${YELLOW}⚠️  trunk is not installed. Installing...${NC}"
    cargo install trunk
    if [ $? -ne 0 ]; then
        echo -e "${RED}❌ Failed to install trunk${NC}"
        exit 1
    fi
fi

# Check if wasm32 target is installed
if ! rustup target list --installed | grep -q "wasm32-unknown-unknown"; then
    echo -e "${YELLOW}⚠️  wasm32-unknown-unknown target not installed. Installing...${NC}"
    rustup target add wasm32-unknown-unknown
fi

# Check if web build exists or if dist directory is empty
if [ ! -d "dist" ] || [ -z "$(ls -A dist)" ]; then
    echo -e "${YELLOW}⚠️  Web build not found or dist directory is empty. Building first...${NC}"
    ./build.sh
    if [ $? -ne 0 ]; then
        echo -e "${RED}❌ Build failed. Cannot start server.${NC}"
        exit 1
    fi
fi

echo -e "${GREEN}✅ Build verified successfully!${NC}"
echo ""
echo -e "${BLUE}🌐 Starting HTTP server with Trunk...${NC}"
echo -e "${BLUE}📱 Open your browser and navigate to: ${GREEN}http://localhost:8091${NC}"
echo -e "${YELLOW}🛑 Press Ctrl+C to stop the server${NC}"
echo ""

# Start HTTP server with trunk
trunk serve