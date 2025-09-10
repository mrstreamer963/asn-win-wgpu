#!/bin/bash

# ASN Web Example Build Script
# This script builds the ex_web example for web using wasm-pack

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}🚀 Building ASN Web Example with wasm-pack...${NC}"

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

# Check if wasm-pack is installed
if ! command -v wasm-pack &> /dev/null; then
    echo -e "${YELLOW}⚠️  wasm-pack not installed. Installing...${NC}"
    cargo install wasm-pack
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

# Build the project with wasm-pack
echo -e "${BLUE}🏗️  Compiling project with wasm-pack...${NC}"
if wasm-pack build --target web --out-dir pkg --release; then
    echo -e "${GREEN}✅ Compilation completed successfully!${NC}"
else
    echo -e "${RED}❌ Compilation failed!${NC}"
    exit 1
fi

# Create dist directory and copy files
echo -e "${BLUE}📦 Creating distribution package...${NC}"
rm -rf dist
mkdir -p dist

# Copy generated files
cp -r pkg/* dist/

# Copy index.html
cp index.html dist/

# Copy any additional assets (if they exist)
if [ -d "assets" ]; then
    cp -r assets dist/
fi

# Check if dist directory was created
if [ -d "dist" ]; then
    echo -e "${GREEN}✅ Web package created in ./dist/${NC}"
    
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
echo -e "   ${BLUE}Or manually: python3 -m http.server 8091 -d dist${NC}"
echo -e "   ${BLUE}Then open http://localhost:8091 in your browser${NC}"

echo -e "${GREEN}✅ Build completed successfully!${NC}"
