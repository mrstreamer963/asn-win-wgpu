#!/bin/bash

# ASN Web Example Build Script
# This script builds the ex_web example for web using trunk

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}🚀 Building ASN Web Example...${NC}"

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    echo -e "${RED}❌ Error: Cargo.toml not found. Please run this script from the ex_web directory.${NC}"
    exit 1
fi

# Check if wasm32 target is installed
if ! rustup target list --installed | grep -q "wasm32-unknown-unknown"; then
    echo -e "${YELLOW}⚠️  wasm32-unknown-unknown target not installed. Installing...${NC}"
    rustup target add wasm32-unknown-unknown
fi

# Clean previous builds (optional)
# Run with --clean flag to remove previous builds:
# ./build.sh --clean
if [ "$1" = "--clean" ]; then
    echo -e "${BLUE}🧹 Cleaning previous builds...${NC}"
    rm -rf target/wasm32-unknown-unknown
    rm -rf pkg
    rm -rf dist
fi

# Build with progress
if trunk build; then
    echo -e "${GREEN}✅ Build completed successfully!${NC}"
    
    # Check if dist directory was created
    if [ -d "dist" ]; then
        echo -e "${GREEN}📦 Web package created in ./dist/${NC}"
        
        # Show package size
        if command -v du &> /dev/null; then
            DIST_SIZE=$(du -sh dist 2>/dev/null | cut -f1)
            echo -e "${BLUE}📊 Package size: ${DIST_SIZE}${NC}"
        fi
    else
        echo -e "${RED}❌ Warning: dist directory was not created${NC}"
    fi
    
    echo ""
    echo -e "${GREEN}🌐 To run the web example:${NC}"
    echo -e "   ${BLUE}./run.sh${NC}"
    echo -e "   ${BLUE}Or manually: trunk serve${NC}"
    echo -e "   ${BLUE}Then open http://localhost:8091 in your browser${NC}"
    
else
    echo -e "${RED}❌ Build failed!${NC}"
    echo -e "${YELLOW}💡 Common solutions:${NC}"
    echo -e "   - Make sure all dependencies are installed"
    echo -e "   - Check that you're in the correct directory"
    echo -e "   - Try running: cargo clean && ./build.sh"
    exit 1
fi
