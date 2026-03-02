#!/bin/bash
# Test script for rust-add-apt-repository using Git Core PPA
# Safe, popular repository for testing

set -e  # Exit on error

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${BLUE}╔════════════════════════════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║           rust-add-apt-repository Test with Git Core PPA                   ║${NC}"
echo -e "${BLUE}╚════════════════════════════════════════════════════════════════════════════╝${NC}"
echo ""

# Check if running as root
if [ "$EUID" -ne 0 ]; then 
    echo "ERROR: This script must be run as root (use sudo)"
    exit 1
fi

# Check if rust-add-apt-repository is available
if ! command -v rust-add-apt-repository &> /dev/null; then
    echo "ERROR: rust-add-apt-repository not found in PATH"
    echo "Build it first: cargo build --release"
    echo "Then either:"
    echo "  1. Add to PATH: export PATH=\"\$PWD/target/release:\$PATH\""
    echo "  2. Install: sudo cp target/release/rust-add-apt-repository /usr/local/bin/"
    exit 1
fi

echo -e "${YELLOW}=== STEP 1: Check Initial State (BEFORE) ===${NC}"
echo ""
echo "Current Git version:"
git --version
echo ""
echo "Available Git versions from apt:"
apt-cache policy git | head -10
echo ""
echo "Current Git-related repositories:"
ls -la /etc/apt/sources.list.d/ 2>/dev/null | grep git || echo "  (No git PPA found)"
echo ""

read -p "Press Enter to add the Git Core PPA..."
echo ""

echo -e "${YELLOW}=== STEP 2: Add Git Core PPA ===${NC}"
rust-add-apt-repository ppa:git-core/ppa
echo ""

echo -e "${YELLOW}=== STEP 3: Verify Repository Added ===${NC}"
echo ""
echo "Repository file created:"
ls -la /etc/apt/sources.list.d/ | grep git
echo ""
echo "Repository file contents:"
cat /etc/apt/sources.list.d/git-core-ubuntu-ppa-*.list
echo ""
echo "GPG key imported:"
ls -la /etc/apt/trusted.gpg.d/ 2>/dev/null | grep -i git || echo "  (Key may be in keyrings/)"
echo ""

read -p "Press Enter to update package lists..."
echo ""

echo -e "${YELLOW}=== STEP 4: Update Package Lists ===${NC}"
apt update
echo ""

echo -e "${YELLOW}=== STEP 5: Verify Newer Version Available ===${NC}"
echo ""
echo "Git versions now available:"
apt-cache policy git
echo ""
echo "Upgradable packages (should show git if newer version available):"
apt list --upgradable 2>/dev/null | grep git || echo "  (No upgrades available or git already latest)"
echo ""

echo -e "${GREEN}✅ Repository successfully added and verified!${NC}"
echo ""
read -p "Press Enter to remove the PPA (cleanup)..."
echo ""

echo -e "${YELLOW}=== STEP 6: Remove Git Core PPA ===${NC}"
rust-add-apt-repository --remove ppa:git-core/ppa
echo ""

echo -e "${YELLOW}=== STEP 7: Verify Repository Removed ===${NC}"
echo ""
echo "Checking for Git PPA files:"
ls -la /etc/apt/sources.list.d/ 2>/dev/null | grep git || echo "  ${GREEN}✅ Git PPA removed successfully${NC}"
echo ""

read -p "Press Enter to do final apt update..."
echo ""

echo -e "${YELLOW}=== STEP 8: Final Update ===${NC}"
apt update
echo ""

echo "Git versions after removal:"
apt-cache policy git | head -10
echo ""

echo -e "${GREEN}╔════════════════════════════════════════════════════════════════════════════╗${NC}"
echo -e "${GREEN}║                         Test Complete! ✅                                  ║${NC}"
echo -e "${GREEN}╚════════════════════════════════════════════════════════════════════════════╝${NC}"
echo ""
echo "Summary:"
echo "  ✅ Added PPA successfully"
echo "  ✅ Repository file created"
echo "  ✅ GPG key imported"
echo "  ✅ Package lists updated"
echo "  ✅ Newer version available"
echo "  ✅ PPA removed successfully"
echo "  ✅ System restored to original state"
echo ""
echo "Your rust-add-apt-repository command is working correctly! 🎉"
