# Installation Guide for rust-add-apt-repository

This guide covers installation methods for `rust-add-apt-repository` on Debian and Ubuntu systems.

## Table of Contents
- [Quick Install (Binary)](#quick-install-binary)
- [Build from Source](#build-from-source)
- [Debian Package Installation](#debian-package-installation)
- [Co-installation with Python Version](#co-installation-with-python-version)
- [Verification](#verification)
- [Uninstallation](#uninstallation)

## Quick Install (Binary)

### From Release Binary

Download and install the pre-built binary:

```bash
# Download the latest release
wget https://github.com/maw629/rust-add-apt-repository/releases/download/v0.1.0/rust-add-apt-repository

# Make executable
chmod +x rust-add-apt-repository

# Install to system
sudo mv rust-add-apt-repository /usr/local/bin/

# Verify installation
rust-add-apt-repository --version
```

### Install Man Page (Optional)

```bash
# Download man page
wget https://github.com/maw629/rust-add-apt-repository/releases/download/v0.1.0/rust-add-apt-repository.1

# Install man page
sudo mkdir -p /usr/local/share/man/man1
sudo cp rust-add-apt-repository.1 /usr/local/share/man/man1/
sudo gzip /usr/local/share/man/man1/rust-add-apt-repository.1

# Update man database
sudo mandb

# View man page
man rust-add-apt-repository
```

## Build from Source

### Prerequisites

Install build dependencies:

```bash
# Ubuntu/Debian
sudo apt update
sudo apt install -y build-essential pkg-config rustc cargo \
  libapt-pkg-dev libgpgme-dev libgpg-error-dev libssl-dev
```

See [DEPENDENCIES.md](DEPENDENCIES.md) for detailed dependency information.

### Clone and Build

```bash
# Clone repository
git clone https://github.com/maw629/rust-add-apt-repository.git
cd rust-add-apt-repository

# Build release binary
cargo build --release

# Binary will be at: target/release/rust-add-apt-repository
```

### Install Built Binary

```bash
# Install binary
sudo install -D -m 755 target/release/rust-add-apt-repository \
  /usr/local/bin/rust-add-apt-repository

# Install man page
sudo install -D -m 644 man/rust-add-apt-repository.1 \
  /usr/local/share/man/man1/rust-add-apt-repository.1
sudo gzip /usr/local/share/man/man1/rust-add-apt-repository.1

# Update man database
sudo mandb

# Verify installation
rust-add-apt-repository --version
man rust-add-apt-repository
```

## Debian Package Installation

### Build Debian Package

#### Install Build Dependencies

```bash
# Install packaging tools
sudo apt install -y debhelper devscripts build-essential \
  cargo rustc pkg-config
```

#### Build Package

```bash
# Clone repository (if not already done)
git clone https://github.com/maw629/rust-add-apt-repository.git
cd rust-add-apt-repository

# Build unsigned package (for local use)
debuild -us -uc -b

# Package will be created in parent directory
ls -lh ../rust-add-apt-repository_*.deb
```

#### Build Signed Package (for Distribution)

```bash
# Build signed package (requires GPG key configured)
debuild -b

# Or specify key
debuild -k<KEY_ID> -b
```

### Install Debian Package

```bash
# Install the package
sudo dpkg -i ../rust-add-apt-repository_0.1.0-1_amd64.deb

# Install any missing dependencies
sudo apt-get install -f

# Verify installation
rust-add-apt-repository --version
dpkg -L rust-add-apt-repository
```

### Package Contents

The Debian package includes:

- **Binary**: `/usr/bin/rust-add-apt-repository`
- **Man page**: `/usr/share/man/man1/rust-add-apt-repository.1.gz`
- **Documentation**: `/usr/share/doc/rust-add-apt-repository/`
  - README.md
  - EXAMPLES.md
  - copyright
  - changelog.Debian.gz

## Co-installation with Python Version

`rust-add-apt-repository` is designed to coexist with the Python version from `software-properties-common`.

### Installation Order

Either order works:

**Option 1: Install Python version first**
```bash
sudo apt install software-properties-common
sudo dpkg -i rust-add-apt-repository_0.1.0-1_amd64.deb
```

**Option 2: Install Rust version first**
```bash
sudo dpkg -i rust-add-apt-repository_0.1.0-1_amd64.deb
sudo apt install software-properties-common
```

### Command Names

Both versions can be installed simultaneously:

- **Python version**: `/usr/bin/add-apt-repository`
- **Rust version**: `/usr/bin/rust-add-apt-repository`

### Using the Rust Version

```bash
# Use Rust version explicitly
sudo rust-add-apt-repository ppa:test/ppa

# Python version (if installed)
sudo add-apt-repository ppa:test/ppa
```

### Creating an Alias (Optional)

If you want `add-apt-repository` to use the Rust version:

```bash
# Add to ~/.bashrc or ~/.bash_aliases
alias add-apt-repository='rust-add-apt-repository'

# Reload shell
source ~/.bashrc
```

### Using update-alternatives (System-wide)

Set up alternatives to switch between versions:

```bash
# Register both versions
sudo update-alternatives --install /usr/local/bin/add-apt-repository \
  add-apt-repository /usr/bin/rust-add-apt-repository 50

sudo update-alternatives --install /usr/local/bin/add-apt-repository \
  add-apt-repository /usr/bin/add-apt-repository 40

# Choose which version to use
sudo update-alternatives --config add-apt-repository

# Now 'add-apt-repository' will use the selected version
```

## Verification

### Verify Installation

```bash
# Check command is available
which rust-add-apt-repository

# Check version
rust-add-apt-repository --version

# Check man page
man rust-add-apt-repository

# Test with help
rust-add-apt-repository --help
```

### Test Basic Functionality

```bash
# List repositories (no root needed)
rust-add-apt-repository --list

# Dry-run mode (safe test)
sudo rust-add-apt-repository --dry-run ppa:test/ppa

# Debug mode test
sudo rust-add-apt-repository --debug --dry-run ppa:test/ppa
```

### Run Tests (From Source)

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_help_flag
```

## Uninstallation

### Remove Binary Installation

```bash
# Remove binary
sudo rm /usr/local/bin/rust-add-apt-repository

# Remove man page
sudo rm /usr/local/share/man/man1/rust-add-apt-repository.1.gz
sudo mandb
```

### Remove Debian Package

```bash
# Remove package (keep config)
sudo apt remove rust-add-apt-repository

# Remove package and config
sudo apt purge rust-add-apt-repository

# Check removal
dpkg -l | grep rust-add-apt-repository
```

### Clean up update-alternatives (if used)

```bash
sudo update-alternatives --remove add-apt-repository /usr/bin/rust-add-apt-repository
```

## Troubleshooting

### Missing Dependencies

If you get dependency errors:

```bash
# Install missing dependencies
sudo apt-get install -f

# Or manually install
sudo apt install apt gnupg ca-certificates
```

### Permission Errors

Most operations require root:

```bash
# Use sudo
sudo rust-add-apt-repository ppa:test/ppa

# Operations that don't need root:
rust-add-apt-repository --list
rust-add-apt-repository --help
```

### Build Failures

If build fails:

```bash
# Clean and rebuild
cargo clean
cargo build --release

# Check Rust version
rustc --version  # Should be 1.70+
```

### Package Build Failures

```bash
# Clean Debian build artifacts
debuild clean
fakeroot debian/rules clean

# Rebuild
debuild -us -uc -b
```

## Distribution-Specific Notes

### Ubuntu 24.04 (Noble)

Fully supported with all features.

```bash
# Standard installation works
sudo dpkg -i rust-add-apt-repository_0.1.0-1_amd64.deb
```

### Ubuntu 22.04 (Jammy)

Fully supported. May need newer Rust:

```bash
# If rustc too old, install from rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### Ubuntu 20.04 (Focal)

Supported but requires Rust 1.70+:

```bash
# Install rustup for newer Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Debian 12 (Bookworm)

Fully supported.

```bash
# Standard installation works
sudo dpkg -i rust-add-apt-repository_0.1.0-1_amd64.deb
```

### Debian 11 (Bullseye)

May need backports for newer Rust:

```bash
# Enable bullseye-backports
sudo apt install -t bullseye-backports rustc cargo
```

### WSL (Windows Subsystem for Linux)

Fully supported. See [WSL.md](WSL.md) for specific notes.

```bash
# Works the same as native Ubuntu
sudo dpkg -i rust-add-apt-repository_0.1.0-1_amd64.deb
```

## Upgrading

### Upgrade Binary Installation

```bash
# Download new version
wget https://github.com/maw629/rust-add-apt-repository/releases/download/v0.2.0/rust-add-apt-repository

# Replace existing
sudo install -m 755 rust-add-apt-repository /usr/local/bin/
```

### Upgrade Debian Package

```bash
# Download new package
wget https://github.com/maw629/rust-add-apt-repository/releases/download/v0.2.0/rust-add-apt-repository_0.2.0-1_amd64.deb

# Upgrade
sudo dpkg -i rust-add-apt-repository_0.2.0-1_amd64.deb
```

## Getting Help

- **Man page**: `man rust-add-apt-repository`
- **Help text**: `rust-add-apt-repository --help`
- **Examples**: `/usr/share/doc/rust-add-apt-repository/EXAMPLES.md`
- **GitHub Issues**: https://github.com/maw629/rust-add-apt-repository/issues

## See Also

- [README.md](README.md) - Project overview and features
- [EXAMPLES.md](EXAMPLES.md) - Comprehensive usage examples
- [BUILDING.md](BUILDING.md) - Detailed build instructions
- [DEPENDENCIES.md](DEPENDENCIES.md) - Dependency information
- [WSL.md](WSL.md) - WSL-specific notes
