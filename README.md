# rust-add-apt-repository

A Rust implementation of the Debian/Ubuntu `add-apt-repository` command, designed for behavioral compatibility with the Python version from `software-properties-common` while being installable alongside the original.

## Project Status

🚧 **In Development** - Currently in Phase 0 (Project Setup)

See [plan.md](plan.md) for the complete implementation roadmap.

## Documentation

- **[PHASE-0-CHECKLIST.md](PHASE-0-CHECKLIST.md)** - Detailed checklist for Phase 0 setup
- **[DEPENDENCIES.md](DEPENDENCIES.md)** - Quick reference for all required packages and libraries
- **[plan.md](plan.md)** - Complete 12-phase implementation plan
- **[source-analysis.md](source-analysis.md)** - Analysis of the original Python implementation
- **[BUILDING.md](BUILDING.md)** - Comprehensive build instructions (to be created)
- **[WSL.md](WSL.md)** - WSL-specific instructions and differences (to be created)

## Quick Start (After Phase 0 Completion)

### Prerequisites

```bash
# Install dependencies (Ubuntu/Debian)
sudo apt install -y build-essential pkg-config rustc cargo \
  libapt-pkg-dev libgpgme-dev libgpg-error-dev libssl-dev
```

See [DEPENDENCIES.md](DEPENDENCIES.md) for detailed information.

### Building from Source

```bash
# Clone the repository
git clone <repository-url>
cd rust-add-apt-repository

# Build release binary
cargo build --release

# Binary will be at: target/release/rust-add-apt-repository
./target/release/rust-add-apt-repository --help
```

### Building Debian Package

```bash
# Install packaging tools
sudo apt install debhelper devscripts dh-cargo

# Build package
debuild -us -uc -b

# Install
sudo dpkg -i ../rust-add-apt-repository_*.deb
```

## WSL Users

If you're building on Windows Subsystem for Linux (WSL), see [WSL.md](WSL.md) for important differences and considerations.

Key points:
- WSL 2 recommended
- Use Linux filesystem (/home), not Windows mounts (/mnt/c)
- Desktop keyring not available (use file-based auth)
- Some systemd-dependent features may not work in WSL 1

## Features (Planned)

- ✅ Behavioral compatibility with original `add-apt-repository`
- ✅ Co-installable with `software-properties-common`
- 🚧 PPA support with Launchpad integration
- 🚧 Cloud Archive support
- 🚧 GPG key management
- 🚧 DEB822 format support
- 🚧 Global operations (components, pockets, sources)

Legend: ✅ Complete | 🚧 Planned | ❌ Not Implemented

## Development

See [plan.md](plan.md) for the complete development strategy. Each phase is:
- Fully documented and planned
- Implemented incrementally
- Reviewed before proceeding
- Tested thoroughly
- Committed with meaningful messages

### Running Tests

```bash
cargo test
```

### Code Quality

```bash
# Format code
cargo fmt

# Check formatting
cargo fmt --check

# Run linter
cargo clippy
```

## License

[To be determined - likely GPL-2+ to match original]

## Contributing

This project follows an incremental, phase-based development approach. See [plan.md](plan.md) for current status and upcoming work.

## Acknowledgments

Based on the Python implementation from the `software-properties-common` package maintained by Canonical Ltd.
