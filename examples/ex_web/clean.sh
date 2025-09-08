#!/bin/bash

# ASN Web Example Clean Script
# This script cleans build artifacts and cache files

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}🧹 Cleaning ASN Web Example build artifacts...${NC}"

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    echo -e "${RED}❌ Error: Cargo.toml not found. Please run this script from the ex_web directory.${NC}"
    exit 1
fi

# Clean Rust target directory
if [ -d "target" ]; then
    echo -e "${YELLOW}🗑️  Removing target directory...${NC}"
    rm -rf target
    echo -e "${GREEN}✅ Target directory removed${NC}"
else
    echo -e "${YELLOW}ℹ️  Target directory not found${NC}"
fi

# Clean pkg directory (WASM build output from wasm-pack)
if [ -d "pkg" ]; then
    echo -e "${YELLOW}🗑️  Removing pkg directory...${NC}"
    rm -rf pkg
    echo -e "${GREEN}✅ Pkg directory removed${NC}"
else
    echo -e "${YELLOW}ℹ️  Pkg directory not found${NC}"
fi

# Clean dist directory (Web build output from trunk)
if [ -d "dist" ]; then
    echo -e "${YELLOW}🗑️  Removing dist directory...${NC}"
    rm -rf dist
    echo -e "${GREEN}✅ Dist directory removed${NC}"
else
    echo -e "${YELLOW}ℹ️  Dist directory not found${NC}"
fi

# Clean cargo cache for this project
if command -v cargo &> /dev/null; then
    echo -e "${YELLOW}🗑️  Cleaning cargo cache for this project...${NC}"
    cargo clean
    echo -e "${GREEN}✅ Cargo cache cleaned${NC}"
fi

# Remove any temporary files
if [ -f ".tmp" ]; then
    echo -e "${YELLOW}🗑️  Removing temporary files...${NC}"
    rm -f .tmp
    echo -e "${GREEN}✅ Temporary files removed${NC}"
fi

# Remove any log files (if they exist)
LOG_FILES=$(find . -name "*.log" -type f 2>/dev/null)
if [ -n "$LOG_FILES" ]; then
    echo -e "${YELLOW}🗑️  Removing log files...${NC}"
    rm -f *.log
    echo -e "${GREEN}✅ Log files removed${NC}"
fi

echo -e "${GREEN}✅ Cleaning completed successfully!${NC}"
echo -e "${BLUE}📊 Summary:${NC}"
echo -e "  - Removed target directory (if existed)"
echo -e "  - Removed pkg directory (if existed)"
echo -e "  - Removed dist directory (if existed)"
echo -e "  - Cleaned cargo cache"
echo -e "  - Removed temporary files"
echo -e "  - Removed log files (if existed)"

echo ""
echo -e "${BLUE}💡 Usage tips:${NC}"
echo -e "  - Run ${GREEN}./build.sh${BLUE} to rebuild the project"
echo -e "  - Run ${GREEN}./run.sh${BLUE} to rebuild and start the server"