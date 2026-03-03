#!/bin/bash
# Build script for rust-add-apt-repository
# Creates a release directory with .deb package, binary, and checksums

set -e  # Exit on error

# Color output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${GREEN}=== Building rust-add-apt-repository ===${NC}"

# Get version from Cargo.toml
VERSION=$(grep '^version = ' Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/')
echo -e "${YELLOW}Version: ${VERSION}${NC}"

# Clean previous builds
echo -e "${YELLOW}Cleaning previous builds...${NC}"
cargo clean
rm -rf release/v${VERSION}
mkdir -p release/v${VERSION}

# Build release binary
echo -e "${YELLOW}Building release binary...${NC}"
cargo build --release

# Copy binary to release directory
echo -e "${YELLOW}Copying binary...${NC}"
cp target/release/rust-add-apt-repository release/v${VERSION}/rust-add-apt-repository-v${VERSION}-x86_64-linux
chmod +x release/v${VERSION}/rust-add-apt-repository-v${VERSION}-x86_64-linux

# Build Debian package
echo -e "${YELLOW}Building Debian package...${NC}"
debuild -us -uc -b

# Move .deb package to release directory
echo -e "${YELLOW}Moving .deb package...${NC}"
DEB_FILE=$(ls -1 ../rust-add-apt-repository_*.deb 2>/dev/null | head -1)
if [ -n "$DEB_FILE" ]; then
    cp "$DEB_FILE" release/v${VERSION}/
    echo -e "${GREEN}Package: $(basename $DEB_FILE)${NC}"
else
    echo -e "${RED}Error: .deb package not found${NC}"
    exit 1
fi

# Generate checksums
echo -e "${YELLOW}Generating checksums...${NC}"
cd release/v${VERSION}
sha256sum rust-add-apt-repository* > SHA256SUMS.txt
cd ../..

# Summary
echo -e "${GREEN}=== Build Complete ===${NC}"
echo -e "${GREEN}Release directory: release/v${VERSION}/${NC}"
echo ""
echo "Contents:"
ls -lh release/v${VERSION}/

echo ""
echo -e "${GREEN}Checksums:${NC}"
cat release/v${VERSION}/SHA256SUMS.txt
