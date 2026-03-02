# Phase 0 Implementation Checklist

## Project Setup Tasks

### 1. Rust Project Initialization
- [ ] Run `cargo init --name rust-add-apt-repository`
- [ ] Create `src/lib.rs` for library code
- [ ] Create module structure:
  - [ ] `src/error.rs` - Error types
  - [ ] `src/cli.rs` - CLI argument parsing
  - [ ] `src/config.rs` - Configuration constants
- [ ] Update `.gitignore` for Rust artifacts (target/, Cargo.lock for binaries)

### 2. Cargo.toml Configuration
- [ ] Set package metadata (name, version, authors, edition, description, license)
- [ ] Add initial dependencies:
  - [ ] `clap` (with derive feature) - CLI parsing
  - [ ] `anyhow` - Error handling
  - [ ] `thiserror` - Custom error types
  - [ ] Additional deps to be determined in later phases
- [ ] Set up binary target pointing to `src/main.rs`
- [ ] Configure package metadata for Debian packaging

### 3. Debian Package Infrastructure

#### debian/ Directory Structure
- [ ] Create `debian/` directory
- [ ] Create `debian/control`:
  - Source package name: `rust-add-apt-repository`
  - Binary package name: `rust-add-apt-repository`
  - Build dependencies: `debhelper (>= 11)`, `cargo`, `rustc`, `libapt-pkg-dev`, etc.
  - Runtime dependencies: `libapt-pkg6.0`, `libgpgme11`, etc.
  - Conflicts: None (should co-install with `software-properties-common`)
  - Description: Full package description
- [ ] Create `debian/rules`:
  - Use `dh` with `--buildsystem=cargo`
  - Override dh_auto_install to place binary correctly
  - Set up cross-compilation support if needed
- [ ] Create `debian/changelog`:
  - Initial entry for version 0.1.0-1
  - Use proper Debian changelog format
- [ ] Create `debian/compat`:
  - Set debhelper compatibility level (11 or higher)
- [ ] Create `debian/copyright`:
  - Use DEP-5 format
  - Specify GPL-2+ or appropriate license
  - List all copyright holders
- [ ] Create `debian/install`:
  - Map `target/release/rust-add-apt-repository` to `/usr/bin/`
- [ ] Create `debian/source/format`:
  - Set to "3.0 (native)" or "3.0 (quilt)"
- [ ] Create `debian/manpages` (placeholder for Phase 11)
- [ ] Optional: Create `debian/watch` for upstream tracking

### 4. Documentation - BUILDING.md
- [ ] Create `BUILDING.md` with sections:

#### Prerequisites Section
- [ ] List required Ubuntu/Debian versions (tested on)
- [ ] System packages needed:
  - [ ] Build essentials: `build-essential`, `pkg-config`
  - [ ] Rust toolchain: `curl` for rustup installation
  - [ ] APT development libraries: `libapt-pkg-dev`
  - [ ] GPG libraries: `libgpgme-dev`, `libgpg-error-dev`
  - [ ] SSL/TLS: `libssl-dev`
  - [ ] Debian packaging: `debhelper`, `devscripts`, `dh-cargo`
- [ ] Minimum Rust version required (e.g., 1.70+)

#### Rust Installation Section
- [ ] Instructions for rustup installation:
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  source $HOME/.cargo/env
  ```
- [ ] Verify installation: `rustc --version`
- [ ] Alternative: System Rust package (`rustc`, `cargo`)

#### Install System Dependencies
- [ ] Complete apt install command:
  ```bash
  sudo apt update
  sudo apt install build-essential pkg-config libapt-pkg-dev \
    libgpgme-dev libgpg-error-dev libssl-dev curl
  ```

#### Building from Source
- [ ] Clone repository instructions
- [ ] Build debug version: `cargo build`
- [ ] Build release version: `cargo build --release`
- [ ] Binary location: `target/release/rust-add-apt-repository`
- [ ] Run tests: `cargo test`
- [ ] Run binary: `./target/release/rust-add-apt-repository --help`

#### Building Debian Package
- [ ] Install packaging tools:
  ```bash
  sudo apt install debhelper devscripts dh-cargo
  ```
- [ ] Build package: `debuild -us -uc -b`
- [ ] Install package: `sudo dpkg -i ../rust-add-apt-repository_*.deb`
- [ ] Verify co-installation: `dpkg -l | grep add-apt-repository`

#### Testing the Build
- [ ] Unit tests: `cargo test`
- [ ] Integration tests: `cargo test --test '*'`
- [ ] Manual testing checklist
- [ ] Linting: `cargo clippy`
- [ ] Formatting: `cargo fmt --check`

### 5. Documentation - WSL.md
- [ ] Create `WSL.md` with sections:

#### WSL Environment Overview
- [ ] Explain what WSL is
- [ ] Supported WSL versions (WSL 2 recommended)
- [ ] Which Ubuntu versions work in WSL

#### WSL-Specific Prerequisites
- [ ] Ensure WSL is up-to-date: `wsl --update`
- [ ] Check Ubuntu version: `lsb_release -a`
- [ ] Install Windows Terminal (recommended)

#### Key Differences from Native Ubuntu
- [ ] **systemd**: May not be available in WSL 1
  - How to check: `ps -p 1 -o comm=`
  - Workaround if needed
- [ ] **Network configuration**: Different from native Linux
  - May need Windows firewall configuration
  - DNS resolution differences
- [ ] **File permissions**: Windows filesystem vs Linux filesystem
  - Use Linux filesystem (/home) not Windows mount (/mnt/c)
- [ ] **apt-get update**: Works the same in WSL
- [ ] **GPG/keyring**: May have different behavior
  - Desktop keyring not available
  - Use file-based credentials only

#### Installation Steps for WSL
- [ ] Same as BUILDING.md but with WSL notes
- [ ] Where to run commands (WSL terminal)
- [ ] File location recommendations

#### Known Issues in WSL
- [ ] List any systemd-dependent features that won't work
- [ ] Launchpad authentication with desktop keyring not available
- [ ] Must use `LP_CREDENTIALS_FILE` environment variable
- [ ] Any dbus-related limitations

#### Testing Limitations in WSL
- [ ] Can build and run the command
- [ ] Can test dry-run mode fully
- [ ] May not be able to test actual repository addition (needs sudo)
- [ ] Recommend testing on native Ubuntu or Ubuntu VM for full validation

#### WSL-Specific Workarounds
- [ ] How to test without breaking your WSL environment
- [ ] Using Docker inside WSL for isolated testing
- [ ] Snapshots/backup recommendations

### 6. Documentation - README.md Update
- [ ] Add project overview:
  - [ ] What is rust-add-apt-repository
  - [ ] Why create a Rust version
  - [ ] Relationship to original command
- [ ] Add feature compatibility section:
  - [ ] Table showing implemented vs planned features
  - [ ] Compatibility status per phase
- [ ] Add quick start section:
  - [ ] Prerequisites one-liner
  - [ ] Build one-liner
  - [ ] Install one-liner
- [ ] Add links to detailed docs:
  - [ ] Link to BUILDING.md
  - [ ] Link to WSL.md
  - [ ] Link to plan.md
  - [ ] Link to source-analysis.md
- [ ] Add usage examples (placeholder for now):
  - [ ] Basic PPA addition
  - [ ] Repository removal
  - [ ] List repositories
- [ ] Add development section:
  - [ ] How to contribute
  - [ ] Running tests
  - [ ] Code style (rustfmt)
- [ ] Add license information
- [ ] Add contact/issues section

### 7. Initial Code Setup

#### src/error.rs
- [ ] Define `AppError` enum with variants:
  - [ ] `Io(std::io::Error)`
  - [ ] `Permission` - Not running as root
  - [ ] `InvalidInput` - Bad user input
  - [ ] Other common errors
- [ ] Implement `Display` for `AppError`
- [ ] Implement `std::error::Error` for `AppError`
- [ ] Implement `From` conversions for common error types

#### src/config.rs
- [ ] Define constants:
  - [ ] `SOURCES_LIST_PATH` = "/etc/apt/sources.list"
  - [ ] `SOURCES_LIST_D_PATH` = "/etc/apt/sources.list.d"
  - [ ] `TRUSTED_GPG_D_PATH` = "/etc/apt/trusted.gpg.d"
  - [ ] `AUTH_CONF_D_PATH` = "/etc/apt/auth.conf.d"
- [ ] Define default values
- [ ] Add configuration struct (for future use)

#### src/cli.rs
- [ ] Define CLI structure using clap:
  - [ ] All flags from original command
  - [ ] Mutually exclusive groups
  - [ ] Help text matching original
- [ ] Implement parsing function
- [ ] Add validation logic

#### src/main.rs
- [ ] Basic skeleton:
  - [ ] Parse CLI arguments
  - [ ] Check for root (if needed)
  - [ ] Call into lib functions (placeholder)
  - [ ] Handle errors and exit codes
  - [ ] Print version info
- [ ] Implement `--version` output
- [ ] Implement `--help` output

#### src/lib.rs
- [ ] Export modules (error, cli, config)
- [ ] Add placeholder public functions:
  - [ ] `run()` - Main entry point
  - [ ] Will be expanded in later phases

### 8. Testing Setup
- [ ] Create `tests/` directory for integration tests
- [ ] Add basic unit test in each module
- [ ] Set up test fixtures directory
- [ ] Document testing approach

### 9. Verification
- [ ] Project compiles: `cargo build`
- [ ] Tests pass: `cargo test`
- [ ] Clippy has no warnings: `cargo clippy`
- [ ] Code is formatted: `cargo fmt --check`
- [ ] Help output works: `cargo run -- --help`
- [ ] Version output works: `cargo run -- --version`
- [ ] Debian package builds: `debuild -us -uc -b`
- [ ] All documentation is clear and complete

### 10. Git Commit
- [ ] Stage all files
- [ ] Commit with message: "Phase 0: Project setup and Debian packaging infrastructure"
- [ ] Include Co-authored-by trailer

## Notes
- Focus on getting infrastructure right before adding features
- Documentation should be comprehensive and beginner-friendly
- WSL.md should clearly distinguish WSL differences
- Debian package should follow Debian policy
- Test on actual WSL to verify instructions work
