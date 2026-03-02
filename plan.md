# rust-add-apt-repository: Migration Plan

## Problem Statement
Create a Rust-based implementation of the Debian `add-apt-repository` command that is behaviorally compatible with the existing Python implementation from the `software-properties-common` package. The implementation should be installable alongside the original command.

## Source Code Analysis

### Main Command Structure (add-apt-repository)
- **Location**: `/usr/bin/add-apt-repository` (452 lines)
- **Package**: `software-properties-common`
- **Language**: Python 3
- **Architecture**: Object-oriented with class `AddAptRepository`

### Core Dependencies
1. **softwareproperties** library modules:
   - `shortcuthandler.py` - Base class for all shortcut handlers (~730 lines)
   - `ppa.py` - PPA handling with Launchpad integration (~240 lines)
   - `cloudarchive.py` - Ubuntu Cloud Archive support (~180 lines)
   - `sourceslist.py` - Sources.list line handler
   - `uri.py` - URI handler
   - `shortcuts.py` - Handler routing logic
   - `sourceutils.py` - Utility functions for source entry manipulation

2. **aptsources** library:
   - `sourceslist.SourcesList` - Sources list file management
   - `sourceslist.SourceEntry` - Individual source entry representation
   - `sourceslist.Deb822SourceEntry` - DEB822 format support
   - `distro.get_distro()` - Distribution information

3. **External dependencies**:
   - `apt_pkg` - APT package management library
   - `launchpadlib` - Launchpad API for PPA access
   - `gpg` - GPG key management

### Key Features Identified

1. **Multiple Input Formats**:
   - PPA shortcuts: `ppa:user/ppa-name`
   - Cloud Archive: `cloud-archive:release` or `uca:release`
   - URI: Direct repository URI
   - Sources.list line: Full apt source line
   - Legacy positional argument (deprecated)

2. **Repository Operations**:
   - Add repositories (default)
   - Remove repositories (`-r/--remove`)
   - List repositories (`-L/--list`)

3. **Global Operations** (without repository):
   - Add/remove components (`-c/--component`)
   - Add/remove pockets (`-p/--pocket`)
   - Enable/disable source packages (`-s/--enable-source`)

4. **File Management**:
   - `/etc/apt/sources.list` - Main sources file
   - `/etc/apt/sources.list.d/*.list` - Additional sources (one-line format)
   - `/etc/apt/sources.list.d/*.sources` - DEB822 format sources
   - `/etc/apt/trusted.gpg.d/*.gpg` - GPG keyring files
   - `/etc/apt/auth.conf.d/*.conf` - Authentication credentials

5. **Key Features**:
   - GPG key import from PPA/Cloud Archive
   - Authentication handling for private PPAs
   - Interactive confirmation prompts
   - Automatic `apt-get update` after changes
   - Dry-run mode
   - Source (deb-src) line management
   - Component validation
   - Template matching for known repositories

6. **Error Handling**:
   - Root permission check
   - Invalid shortcut format detection
   - Network error handling (Launchpad API)
   - File permission handling

## Implementation Approach

### Phase-Based Development Strategy
Each phase builds on the previous one, creating a working subset of functionality that can be tested and committed independently.

### Technical Stack (Rust)
- **CLI parsing**: `clap` (feature-rich argument parser)
- **APT interaction**: `rust-apt` or direct libapt-pkg bindings
- **HTTP/API**: `reqwest` (for Launchpad API, key fetching)
- **GPG operations**: `gpgme` or `sequoia-openpgp`
- **File I/O**: Standard library with proper permission handling
- **Error handling**: `anyhow` or `thiserror`
- **Serialization**: `serde` (for JSON/YAML parsing if needed)

## Implementation Phases

### Phase 0: Project Setup & Debian Packaging Infrastructure

#### Rust Project Setup
- Initialize Rust project with Cargo
- Set up project structure (src/main.rs, lib modules)
- Configure dependencies in Cargo.toml
- Set up basic error types
- Create placeholder for main CLI structure

#### Debian Package Build Infrastructure
- Create `debian/` directory structure with all required files:
  - `debian/control` - Package metadata and dependencies
  - `debian/rules` - Build instructions (using dh with cargo)
  - `debian/changelog` - Version history
  - `debian/compat` - Debhelper compatibility level
  - `debian/copyright` - License information
  - `debian/install` - File installation mappings
  - `debian/source/format` - Source package format
- Configure package to install binary as `rust-add-apt-repository`
- Ensure co-installability with `software-properties-common`
- Set up proper dependencies (libapt-pkg-dev, libgpg-error-dev, etc.)

#### Documentation
- Create comprehensive `BUILDING.md` with:
  - Prerequisites for Ubuntu/Debian systems
  - Rust installation instructions (rustup)
  - Required system libraries and dev packages
  - Step-by-step build instructions
  - Debian package building instructions
  - Testing guidelines
- Create `WSL.md` with WSL-specific instructions:
  - WSL-specific prerequisites
  - Differences from native Ubuntu
  - Known issues and workarounds
  - Special considerations for systemd/dbus
  - Testing limitations in WSL environment
- Update `README.md` with:
  - Project overview and goals
  - Quick start guide
  - Links to detailed build instructions
  - Installation methods (build from source, .deb package)
  - Feature compatibility matrix with original command

### Phase 1: Core Data Structures
- Define SourceEntry struct (represents apt source line)
- Define Repository configuration structs
- Implement basic file path constants
- Create utility functions for file operations
- Implement permission checking (root user validation)
- Add basic CLI argument parsing structure

### Phase 2: Sources.list File Operations
- Implement sources.list parser (one-line format)
- Implement sources.list.d directory scanning
- Add functionality to read existing sources
- Implement source entry comparison (for duplicate detection)
- Add source entry modification (enable/disable)
- Implement sources.list writer with proper permissions
- Handle file locking for concurrent access

### Phase 3: Basic Repository Addition (URI/Line Format)
- Implement URI shortcut handler
- Implement sources.list line parser
- Add basic repository addition to sources.list.d
- Implement dry-run mode
- Add user confirmation prompts
- Implement basic repository removal

### Phase 4: GPG Key Management
- Implement GPG key fingerprint extraction
- Add GPG key import to trusted.gpg.d
- Implement key verification
- Add key removal functionality
- Handle keyring file creation with correct permissions

### Phase 5: PPA Support (Core)
- Implement PPA shortcut parsing (ppa:user/ppa-name)
- Add Launchpad API client (anonymous access)
- Fetch PPA information (description, web link)
- Retrieve PPA signing keys
- Construct PPA repository URIs
- Add PPA-specific repository entries

### Phase 6: PPA Support (Authentication)
- Implement Launchpad authentication for private PPAs
- Add authentication token storage
- Handle auth.conf.d file creation
- Add credential management
- Implement --login flag functionality

### Phase 7: Cloud Archive Support
- Implement cloud-archive shortcut parsing
- Add release name to codename mapping
- Validate cloud archive pocket
- Add cloud archive repositories
- Handle ubuntu-cloud-keyring package installation

### Phase 8: Global Operations
- Implement component addition/removal across all repos
- Add pocket management for all repositories
- Implement source (deb-src) enable/disable globally
- Add repository listing functionality (-L/--list)

### Phase 9: DEB822 Format Support
- Implement DEB822 sources format parser
- Add DEB822 format writer
- Handle mixed format repositories (one-line + DEB822)
- Implement inline key signing (Signed-By field)

### Phase 10: Advanced Features & Compatibility
- Add template matching for known repositories
- Implement component validation against distro
- Add source/binary entry pairing logic
- Handle edge cases from Python implementation
- Add debug mode output
- Implement exit codes matching original

### Phase 11: Testing & Documentation
- Create integration tests
- Add unit tests for all modules
- Write man page
- Add command-line help text
- Create usage examples
- Test against various Ubuntu/Debian versions

### Phase 12: Package & Distribution
- Create Debian package (rust-add-apt-repository)
- Ensure co-installability with original package
- Add package metadata
- Create installation instructions
- Add uninstallation handling

## Key Design Decisions

1. **Binary Name**: Use `rust-add-apt-repository` to avoid conflicts
2. **Alternative Path**: Could use update-alternatives for `add-apt-repository` command
3. **Rust Crate Structure**: Modular library design for reusability
4. **Error Handling**: Use Result types throughout, convert to appropriate exit codes
5. **Compatibility Priority**: Behavioral compatibility over code structure similarity
6. **Incremental Review**: Each phase must be committed before starting the next

## Dependencies to Research

1. Rust crates for APT package management
2. Rust bindings for libapt-pkg
3. GPG/PGP libraries in Rust
4. Launchpad API - may need to implement HTTP client
5. Distribution detection libraries

## Notes

- The Python implementation is ~450 lines but depends heavily on large library modules
- The shortcuthandler.py base class alone is ~730 lines
- Full implementation will likely be 2000-3000 lines of Rust
- Focus on commonly-used features first (PPA, URI, basic operations)
- Cloud Archive and advanced features can be later priorities
- Testing on actual Ubuntu/Debian systems is essential
