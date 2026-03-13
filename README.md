# rust-add-apt-repository

[![License: GPL v2+](https://img.shields.io/badge/License-GPL%20v2+-blue.svg)](https://www.gnu.org/licenses/gpl-2.0)
[![Version](https://img.shields.io/badge/version-0.2.1-green.svg)](https://github.com/maw629/rust-add-apt-repository/releases)
[![CI](https://github.com/maw629/rust-add-apt-repository/workflows/CI/badge.svg)](https://github.com/maw629/rust-add-apt-repository/actions/workflows/ci.yml)
[![Tests](https://img.shields.io/badge/tests-105%20passing-success.svg)](https://github.com/maw629/rust-add-apt-repository/actions/workflows/ci.yml)

A Rust implementation of the Debian/Ubuntu `add-apt-repository` command, designed for behavioral compatibility with the Python version from `software-properties-common` while being installable alongside the original.

## Project Status

✅ **100% Complete** - All 13 phases implemented and tested

**Production Ready:**
- ✅ All core features implemented
- ✅ 105 tests passing (74 unit + 31 integration)
- ✅ CI/CD automation with GitHub Actions
- ✅ Automated release workflow
- ✅ Comprehensive documentation
- ✅ Debian package available
- ✅ Behavioral compatibility with Python version

See [docs/development/plan.md](docs/development/plan.md) for the complete implementation roadmap.

## Features

- ✅ **Repository Management**: Add, remove, list repositories
- ✅ **PPA Support**: Full Ubuntu PPA integration with Launchpad API
- ✅ **Cloud Archive**: Ubuntu OpenStack releases (bobcat, caracal, etc.)
- ✅ **GPG Key Management**: Automatic key import and verification (with --key, --keyring, --key-id flags)
- ✅ **Authentication**: Private PPA support with credential management
- ✅ **Global Operations**: Component, pocket, and source code management
- ✅ **DEB822 Format**: Support for modern .sources files
- ✅ **Advanced Features**: Debug mode, validation, proper exit codes
- ✅ **Dry-run Mode**: Preview changes before applying
- ✅ **Behavioral Compatibility**: Drop-in replacement for Python version

Legend: ✅ Complete | 🚧 In Progress | ⏳ Planned

## Quick Start

### Prerequisites

```bash
# Ubuntu/Debian - Install dependencies
sudo apt install -y build-essential pkg-config rustc cargo \
  libapt-pkg-dev libgpgme-dev libgpg-error-dev libssl-dev
```

See [DEPENDENCIES.md](DEPENDENCIES.md) for detailed information.

### Building from Source

```bash
# Clone the repository
git clone https://github.com/maw629/rust-add-apt-repository.git
cd rust-add-apt-repository

# Build release binary
cargo build --release

# Test the binary
./target/release/rust-add-apt-repository --help
```

### Installation

```bash
# Option 1: Install binary directly
sudo cp target/release/rust-add-apt-repository /usr/local/bin/

# Option 2: Build and install Debian package
debuild -us -uc -b
sudo dpkg -i ../rust-add-apt-repository_*.deb
```

## Common Usage Workflows

This section shows complete, real-world workflows to help you understand how to use the command effectively.

### Workflow 1: Add a PPA and Install Software

```bash
# Check current repositories
ls /etc/apt/sources.list.d/

# Add graphics drivers PPA
sudo rust-add-apt-repository ppa:graphics-drivers/ppa

# Update package lists
sudo apt update

# Search for packages from the new PPA
apt search nvidia-driver

# Install a package
sudo apt install nvidia-driver-535

# Later, to remove the PPA:
sudo rust-add-apt-repository --remove ppa:graphics-drivers/ppa
sudo apt update
```

### Workflow 2: Enable Universe Component

```bash
# Check what components are currently enabled
grep -h "^deb " /etc/apt/sources.list /etc/apt/sources.list.d/*.list 2>/dev/null | grep -v "#"

# Enable universe component globally
sudo rust-add-apt-repository --component universe

# Update and install from universe
sudo apt update
apt search some-universe-package
sudo apt install some-universe-package
```

### Workflow 3: Add Cloud Archive for OpenStack

```bash
# Add Ubuntu Cloud Archive for OpenStack Bobcat
sudo rust-add-apt-repository cloud-archive:bobcat

# Update package lists
sudo apt update

# Install OpenStack packages
sudo apt install nova-compute neutron-linuxbridge-agent
```

### Workflow 4: Add Repository with GPG Key

```bash
# Option 1: Download key from URL
sudo rust-add-apt-repository \
  --uri https://cli.github.com/packages \
  --dist stable \
  --component main \
  --key https://cli.github.com/packages/githubcli-archive-keyring.gpg

# Option 2: Import key from local file
sudo rust-add-apt-repository \
  --uri https://repo.example.com/ubuntu \
  --dist noble \
  --component main \
  --keyring /usr/share/keyrings/example.gpg

# Option 3: Fetch key by ID from keyserver
sudo rust-add-apt-repository \
  --uri https://example.com/repo \
  --dist noble \
  --component main \
  --key-id 23F3D4EA75716059

# Update and verify
sudo apt update
apt-cache policy
```

### Workflow 5: Preview Changes with Dry-Run

```bash
# Preview adding a repository with a key
rust-add-apt-repository --dry-run \
  --uri https://example.com/repo \
  --dist noble \
  --component main \
  --key https://example.com/key.gpg

# Shows what would be added without making changes
# No root required for --dry-run mode
```

### Quick Reference

| Task | Command |
|------|---------|
| Add PPA | `sudo rust-add-apt-repository ppa:user/ppa-name` |
| Remove PPA | `sudo rust-add-apt-repository --remove ppa:user/ppa-name` |
| Add Cloud Archive | `sudo rust-add-apt-repository cloud-archive:release` |
| Add repo with key URL | `sudo rust-add-apt-repository --uri <URI> --dist <DIST> --component <COMP> --key <URL>` |
| Add repo with keyring | `sudo rust-add-apt-repository --uri <URI> --dist <DIST> --component <COMP> --keyring <FILE>` |
| Add repo with key ID | `sudo rust-add-apt-repository --uri <URI> --dist <DIST> --component <COMP> --key-id <ID>` |
| Enable component | `sudo rust-add-apt-repository --component universe` |
| Enable sources | `sudo rust-add-apt-repository -s` |
| List repositories | `rust-add-apt-repository --list` |
| Preview changes | `rust-add-apt-repository --dry-run ppa:test/ppa` |

### More Examples

For comprehensive examples covering authentication, DEB822 format, advanced scenarios, and troubleshooting, see **[EXAMPLES.md](EXAMPLES.md)**.

## Documentation

### User Documentation
- **[EXAMPLES.md](EXAMPLES.md)** - Comprehensive usage examples and workflows
- **[TESTING.md](TESTING.md)** - Testing guide (automated and manual E2E tests)
- **[man page](man/rust-add-apt-repository.1)** - Complete manual page (troff format)
- **README.md** (this file) - Quick start and overview

### Developer Documentation
- **[docs/development/](docs/development/)** - Development history, phase summaries, and implementation plans
- **[BUILDING.md](BUILDING.md)** - Comprehensive build instructions
- **[DEPENDENCIES.md](DEPENDENCIES.md)** - Quick reference for all required packages
- **[WSL.md](WSL.md)** - WSL-specific instructions and differences
- **[INSTALL.md](INSTALL.md)** - Installation guide (binary, source, package)
- **[RELEASE-NOTES.md](RELEASE-NOTES.md)** - Release notes and migration guide

### Phase Summaries
- **[docs/development/PHASE-0-SUMMARY.md](docs/development/PHASE-0-SUMMARY.md)** through **[docs/development/PHASE-12-SUMMARY.md](docs/development/PHASE-12-SUMMARY.md)**
  - Detailed documentation of each completed phase
  - Implementation details and design decisions
  - Testing results and validation

## Testing

The project includes multiple levels of testing:

### Automated Tests (No Root Required)

- **74 unit tests** - Testing individual modules and functions
- **21 integration tests** - CLI interface testing with `--dry-run` mode
- **2 ignored tests** - Network-dependent tests (manual verification)

These tests run safely without requiring root privileges or modifying your system:

```bash
# Run all automated tests
cargo test

# Run specific test suite
cargo test --lib                    # Unit tests only
cargo test --test integration_test  # Integration tests

# Run with output
cargo test -- --nocapture

# Run ignored tests (requires network)
cargo test -- --ignored
```

**Important**: Automated tests use `--dry-run` mode to verify logic without making system changes.

### Manual End-to-End Testing (Requires Root)

For testing actual system integration (real repository additions, apt operations), see **[TESTING.md](TESTING.md)** for:
- Complete E2E test workflows
- Before/after verification steps
- Safe test repositories
- Cleanup procedures

**Example E2E test**:
```bash
# 1. Verify initial state
apt search some-package  # Should not exist

# 2. Add test repository
sudo rust-add-apt-repository ppa:test/ppa
sudo apt update

# 3. Verify package available
apt search some-package  # Should be found
apt install some-package # Should install

# 4. Cleanup
sudo rust-add-apt-repository --remove ppa:test/ppa
sudo apt update
```

## WSL Users

If you're building on Windows Subsystem for Linux (WSL), see [WSL.md](WSL.md) for important notes:

- WSL 2 recommended for best compatibility
- Use Linux filesystem (`/home`), not Windows mounts (`/mnt/c`)
- Desktop keyring not available (use file-based authentication)
- Some systemd features may not work in WSL 1

## Development

### Code Structure

```
rust-add-apt-repository/
├── src/
│   ├── main.rs           # Entry point
│   ├── lib.rs            # Main application logic
│   ├── cli.rs            # Command-line argument parsing
│   ├── repository.rs     # Repository representation
│   ├── sourceslist.rs    # Sources.list file management
│   ├── sources.rs        # Source entry parsing
│   ├── ppa.rs            # PPA handling and Launchpad API
│   ├── cloudarchive.rs   # Cloud Archive support
│   ├── gpg.rs            # GPG key management
│   ├── auth.rs           # Authentication handling
│   ├── global.rs         # Global operations (components, pockets)
│   ├── deb822.rs         # DEB822 format support
│   ├── validation.rs     # Input validation and template matching
│   ├── debug.rs          # Debug logging utilities
│   ├── utils.rs          # Utility functions
│   ├── config.rs         # Configuration constants
│   └── error.rs          # Error types
├── tests/
│   └── integration_test.rs  # Integration tests
├── man/
│   └── rust-add-apt-repository.1  # Man page (troff)
└── debian/               # Debian packaging files

Total: ~4,650 lines of Rust code across 17 modules
```

### Code Quality

```bash
# Format code
cargo fmt

# Check formatting
cargo fmt --check

# Run linter
cargo clippy

# Check for issues
cargo clippy -- -D warnings
```

### Development Workflow

Each phase follows a strict workflow:
1. Review Python implementation for behavior
2. Plan and design Rust implementation
3. Implement incrementally with tests
4. Review and validate
5. Commit with meaningful messages
6. Document in phase summary

See [docs/development/plan.md](docs/development/plan.md) for the complete development strategy.

## Behavioral Compatibility

This implementation maintains compatibility with the Python `add-apt-repository`:

- **Same command-line interface**: All flags and options work identically
- **Same file operations**: Modifies the same files in the same way
- **Same output format**: User-facing messages match closely
- **Same exit codes**: 0 (success), 1 (error), 2 (invalid input)
- **Same global operations**: Component/pocket/source management behavior
- **Same validation**: Component warnings, suite checks

**Enhancements over Python version:**
- Faster execution (Rust performance)
- Better error messages with context
- Structured debug output
- More comprehensive validation
- Memory safety guarantees

## Project Timeline

- **Phase 0** (Setup): Complete ✅
- **Phase 1-4** (Core Features): Complete ✅
- **Phase 5-7** (PPA, Auth, Cloud): Complete ✅
- **Phase 8-10** (Advanced Features): Complete ✅
- **Phase 11** (Testing & Docs): Complete ✅
- **Phase 12** (Package & Distribution): Complete ✅

**Current Status**: 100% complete - All 13 phases implemented and tested

## Contributing

We welcome contributions! This project uses **Trunk-Based Development** for a simple, streamlined workflow.

**Quick start:**
1. Fork the repository
2. Create a feature branch from `trunk`
3. Make your changes (ensure tests pass: `cargo test`)
4. Create a Pull Request to `trunk`

See **[CONTRIBUTING.md](CONTRIBUTING.md)** for complete guidelines including:
- Development workflow and branch conventions
- Code quality requirements
- Testing procedures
- PR guidelines and review process

**Key principles:**
- All changes flow through `trunk` branch via Pull Requests
- Each PR must pass all tests and linting checks
- Maintain behavioral compatibility with Python version
- Keep changes focused and well-tested

**Code of Conduct**: This project adheres to the [Contributor Covenant Code of Conduct](CODE_OF_CONDUCT.md). By participating, you are expected to uphold this code.

## License

This project is licensed under the GNU General Public License v2.0 or later (GPL-2.0-or-later).

See the [LICENSE](LICENSE) file for the full license text.

This license matches the original Python implementation from `software-properties-common` to ensure legal compatibility.

## Acknowledgments

Based on the Python implementation from the `software-properties-common` package maintained by Canonical Ltd. This Rust version is an independent reimplementation designed for compatibility and interoperability.

## Links

- Original Python implementation: `software-properties-common` package
- Ubuntu Launchpad: https://launchpad.net/
- Ubuntu Cloud Archive: https://wiki.ubuntu.com/OpenStack/CloudArchive
- APT sources.list format: `man sources.list`
