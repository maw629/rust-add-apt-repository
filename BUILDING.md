# Building rust-add-apt-repository

This guide provides complete instructions for building `rust-add-apt-repository` from source on Ubuntu/Debian systems.

## Table of Contents

- [Prerequisites](#prerequisites)
- [Rust Installation](#rust-installation)
- [Install System Dependencies](#install-system-dependencies)
- [Building from Source](#building-from-source)
- [Building Debian Package](#building-debian-package)
- [Testing the Build](#testing-the-build)
- [Troubleshooting](#troubleshooting)

## Prerequisites

### Supported Operating Systems

This project has been tested on:
- **Ubuntu 24.04 LTS (Noble Numbat)**
- **Ubuntu 22.04 LTS (Jammy Jellyfish)**
- **Ubuntu 20.04 LTS (Focal Fossa)**
- **Debian 12 (Bookworm)**
- **Debian 11 (Bullseye)**

Older versions may work but are not officially supported.

### Required System Packages

You'll need the following categories of packages:

1. **Build Essentials**: Basic compilation tools
   - `build-essential` - GCC, G++, make, and core build utilities
   - `pkg-config` - Tool for managing compile/link flags

2. **APT Development Libraries**: For interacting with APT system
   - `libapt-pkg-dev` - Development files for APT library

3. **GPG/Cryptography Libraries**: For key management
   - `libgpgme-dev` - GnuPG Made Easy development files
   - `libgpg-error-dev` - GPG error codes library

4. **SSL/TLS Libraries**: For secure connections
   - `libssl-dev` - OpenSSL development files

5. **Debian Packaging Tools** (optional, for building .deb):
   - `debhelper` - Helper tools for Debian packaging
   - `devscripts` - Scripts to make Debian package development easier
   - `dh-cargo` - Debhelper buildsystem for Cargo (Rust)

6. **Rust Toolchain Installation** (if using rustup method):
   - `curl` - For downloading rustup installer

### Minimum Versions

- **Rust**: 1.70.0 or newer
- **Debhelper**: 11 or newer (for Debian packaging)

## Rust Installation

You have two options for installing Rust:

### Option 1: Using rustup (Recommended)

Rustup is the official Rust toolchain manager and ensures you have the latest stable version:

```bash
# Download and install rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Follow the on-screen prompts (usually just press Enter for defaults)

# Load Rust environment in current shell
source $HOME/.cargo/env

# Verify installation
rustc --version
cargo --version
```

The rustup installer will:
- Install the latest stable Rust toolchain
- Install cargo (Rust package manager)
- Add cargo binaries to your PATH
- Set up shell integration

### Option 2: Using System Package Manager

Alternatively, install Rust via apt (may be an older version):

```bash
sudo apt update
sudo apt install rustc cargo
```

**Note**: System packages may provide older Rust versions. Use rustup if you need a specific version.

## Install System Dependencies

### Quick Install (One-liner)

Install all required build dependencies with a single command:

```bash
sudo apt update
sudo apt install -y build-essential pkg-config libapt-pkg-dev \
  libgpgme-dev libgpg-error-dev libssl-dev curl
```

### For Debian Package Building

If you also want to build Debian packages, add these tools:

```bash
sudo apt install -y debhelper devscripts dh-cargo
```

### Verify Installation

Check that all required tools are available:

```bash
# Check build tools
which gcc pkg-config

# Check library installations
pkg-config --exists apt-pkg && echo "✓ apt-pkg found" || echo "✗ apt-pkg NOT found"
pkg-config --exists gpgme && echo "✓ gpgme found" || echo "✗ gpgme NOT found"

# Check Rust installation
rustc --version
cargo --version
```

## Building from Source

### 1. Clone the Repository

```bash
# Clone the repository
git clone https://github.com/maw629/rust-add-apt-repository.git
cd rust-add-apt-repository
```

Or if you already have the source:

```bash
cd /path/to/rust-add-apt-repository
```

### 2. Build Debug Version

For development and testing:

```bash
cargo build
```

This creates a debug binary at `target/debug/rust-add-apt-repository`.

**Debug builds**:
- Include debug symbols
- Slower execution
- Larger binary size
- Faster compilation
- Useful for development

### 3. Build Release Version

For production use:

```bash
cargo build --release
```

This creates an optimized binary at `target/release/rust-add-apt-repository`.

**Release builds**:
- Optimized for performance
- Strip debug symbols
- Smaller binary size
- Slower compilation
- Ready for deployment

### 4. Run the Binary

After building, test the binary:

```bash
# Show help
./target/release/rust-add-apt-repository --help

# Show version
./target/release/rust-add-apt-repository --version
```

### 5. Install Locally (Optional)

To install the binary system-wide:

```bash
sudo cp target/release/rust-add-apt-repository /usr/local/bin/
sudo chmod 755 /usr/local/bin/rust-add-apt-repository
```

## Building Debian Package

Building a Debian package allows for easier installation and dependency management.

### 1. Ensure Packaging Tools are Installed

```bash
sudo apt install -y debhelper devscripts dh-cargo
```

### 2. Build the Package

From the project root directory:

```bash
# Build unsigned binary package (for local use)
debuild -us -uc -b
```

**Flags explained**:
- `-us` - Do not sign source package
- `-uc` - Do not sign .changes file
- `-b` - Build binary package only (no source package)

The build process will:
1. Check build dependencies
2. Compile the Rust code
3. Run tests (if configured)
4. Create .deb package

### 3. Locate the Package

After successful build, the .deb package will be in the parent directory:

```bash
ls -lh ../*.deb
```

You should see a file like: `rust-add-apt-repository_0.1.0-1_amd64.deb`

### 4. Install the Package

Install using dpkg:

```bash
sudo dpkg -i ../rust-add-apt-repository_*.deb

# If there are dependency issues, fix them with:
sudo apt install -f
```

### 5. Verify Installation

```bash
# Check that both packages are installed
dpkg -l | grep add-apt-repository

# You should see:
# - software-properties-common (Python version)
# - rust-add-apt-repository (Rust version)

# Test the command
rust-add-apt-repository --version
```

### 6. Verify Co-installation

The Rust version is designed to co-exist with the Python version:

```bash
# Check that both binaries exist
which add-apt-repository        # Python version
which rust-add-apt-repository   # Rust version

# Both should be installed without conflicts
dpkg -l | grep -E "software-properties-common|rust-add-apt-repository"
```

## Testing the Build

### Unit Tests

Run all unit tests:

```bash
cargo test
```

### Integration Tests

Run integration tests (if available):

```bash
cargo test --test '*'
```

### Run Specific Tests

```bash
# Run tests for a specific module
cargo test error

# Run tests with verbose output
cargo test -- --nocapture

# Run tests in a single thread (for debugging)
cargo test -- --test-threads=1
```

### Code Quality Checks

#### Linting with Clippy

Clippy is Rust's official linter:

```bash
# Install clippy (if not already installed)
rustup component add clippy

# Run clippy
cargo clippy

# Run with stricter checks
cargo clippy -- -D warnings
```

#### Code Formatting

Check code formatting:

```bash
# Install rustfmt (if not already installed)
rustup component add rustfmt

# Check formatting without modifying files
cargo fmt --check

# Apply formatting
cargo fmt
```

### Manual Testing Checklist

After building, manually test these features:

- [ ] `--help` displays help text
- [ ] `--version` displays version information
- [ ] Invalid arguments show error messages
- [ ] Running without sudo shows permission error (where applicable)
- [ ] Dry-run mode works without modifying system (`-n` flag)

## Troubleshooting

### Common Build Errors

#### Error: "apt-pkg.h: No such file or directory"

**Cause**: Missing APT development headers

**Solution**:
```bash
sudo apt install libapt-pkg-dev
```

#### Error: "gpgme.h: No such file or directory"

**Cause**: Missing GPGME development headers

**Solution**:
```bash
sudo apt install libgpgme-dev libgpg-error-dev
```

#### Error: "cannot find -lssl"

**Cause**: Missing OpenSSL development libraries

**Solution**:
```bash
sudo apt install libssl-dev
```

#### Error: "cargo: command not found"

**Cause**: Rust toolchain not installed or not in PATH

**Solution**:
```bash
# If using rustup:
source $HOME/.cargo/env

# Or reinstall:
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

#### Error: "debuild: command not found"

**Cause**: Debian packaging tools not installed

**Solution**:
```bash
sudo apt install devscripts
```

#### Error: "linker 'cc' not found"

**Cause**: C compiler not installed

**Solution**:
```bash
sudo apt install build-essential
```

### Build Performance Tips

#### Faster Incremental Builds

Enable incremental compilation (usually on by default):

```bash
export CARGO_INCREMENTAL=1
cargo build
```

#### Parallel Compilation

Use all CPU cores for faster builds:

```bash
cargo build -j$(nproc)
```

#### Cleaning Build Artifacts

To start fresh:

```bash
# Clean all build artifacts
cargo clean

# Clean only release artifacts
cargo clean --release
```

### Debian Package Build Issues

#### Error: "dh_auto_configure: cannot find Cargo.toml"

**Cause**: Running debuild from wrong directory

**Solution**: Ensure you're in the project root where `Cargo.toml` exists

#### Error: "dpkg-checkbuilddeps: missing dependencies"

**Cause**: Missing build dependencies declared in `debian/control`

**Solution**:
```bash
# Install missing build dependencies
sudo apt install -y $(dpkg-checkbuilddeps 2>&1 | sed 's/.*: //')

# Or use mk-build-deps:
sudo apt install devscripts equivs
sudo mk-build-deps -i -r debian/control
```

## Development Workflow

### Recommended Development Cycle

```bash
# 1. Make code changes
vim src/main.rs

# 2. Check formatting
cargo fmt

# 3. Run clippy
cargo clippy

# 4. Run tests
cargo test

# 5. Build and test
cargo build --release
./target/release/rust-add-apt-repository --help

# 6. Build Debian package (if needed)
debuild -us -uc -b
```

### Watch Mode for Development

Install and use cargo-watch for automatic rebuilds:

```bash
# Install cargo-watch
cargo install cargo-watch

# Auto-rebuild on file changes
cargo watch -x build

# Auto-test on file changes
cargo watch -x test
```

## Cross-Compilation (Advanced)

To build for different architectures:

```bash
# Install cross-compilation target
rustup target add aarch64-unknown-linux-gnu

# Install cross-compilation toolchain
sudo apt install gcc-aarch64-linux-gnu

# Build for ARM64
cargo build --release --target=aarch64-unknown-linux-gnu
```

## Next Steps

After successfully building:

1. Read [README.md](README.md) for usage instructions
2. Check [WSL.md](WSL.md) if using Windows Subsystem for Linux
3. See [DEPENDENCIES.md](DEPENDENCIES.md) for dependency details
4. Review [docs/development/plan.md](docs/development/plan.md) for project roadmap

## Getting Help

If you encounter issues not covered here:

1. Check existing issues on GitHub
2. Search the Rust users forum
3. Review the Debian packaging guide
4. Ask on project issue tracker

## Contributing

See development workflow above. All contributions should:
- Pass `cargo test`
- Pass `cargo clippy`
- Follow `cargo fmt` formatting
- Build successfully with `debuild`
