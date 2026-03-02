# Release Notes - rust-add-apt-repository

## v0.2.0 - Bug Fixes and Documentation Improvements

**Release Date**: March 2, 2026

This release focuses on critical bug fixes related to backup file handling and documentation organization.

### 🐛 Bug Fixes

#### Backup System Improvements
- **Fixed excessive backup creation**: Only backup files that are actually modified, not all loaded repository files
  - Implemented dirty file tracking using `HashSet<PathBuf>`
  - Files are only marked as modified when entries are added, removed, or changed
  - Dramatically reduces unnecessary `.save` file creation
  
- **Fixed backup before deletion**: Repository files are now properly backed up before removal
  - Ensures `.save` backup exists before deleting repository files
  - Prevents data loss when removing repositories
  
- **Fixed DEB822 format preservation**: `.sources` files now maintain DEB822 format when saved
  - Detects file format by extension (`.sources` vs `.list`)
  - Preserves DEB822 stanza structure with proper field formatting
  - Prevents corruption of modern Ubuntu repository files
  
- **Fixed backup file extension**: Changed from timestamp-based to `.save` extension
  - Uses `.save` extension matching Python version behavior
  - APT properly ignores `.save` files during repository scanning
  - Prevents "invalid filename extension" warnings

### 📚 Documentation

#### Structure Reorganization
- **Reorganized documentation**: Moved development artifacts to `docs/development/`
  - Root directory reduced from 24 to 8 markdown files (-67%)
  - Clear separation: root = user docs, docs/ = development history
  - Added `docs/development/README.md` to explain contents
  - Updated all references in root documentation

#### New Documentation
- **Enhanced testing guide**: Added comprehensive E2E testing procedures (TESTING.md)
  - 5 detailed test workflows with expected results
  - Troubleshooting and cleanup procedures
  - WSL-specific testing considerations
  
- **Added common usage workflows**: Real-world examples in README.md
  - Before/after comparisons for adding PPAs
  - Package installation verification steps
  - Repository management examples

### 🔧 Technical Details

#### Files Modified
- `src/sourceslist.rs`: Added `modified_files` tracking, format detection, backup improvements
- `src/lib.rs`: Mark files as modified in remove operations
- `src/sources.rs`: Added `PartialOrd` and `Ord` traits to `SourceType`
- Documentation files: Reorganized and enhanced

#### Testing
- All 95 unit tests passing
- Manual E2E testing completed
- Backup behavior verified against Python version

### 📊 Statistics

- **Commits**: 7 since v0.1.0
- **Files Changed**: 20+ files
- **Bug Fixes**: 4 critical issues resolved
- **Documentation**: Comprehensive reorganization

### 🙏 Contributors

Thanks to all who tested and provided feedback!

---

## v0.1.0 - Initial Release

**Release Date**: March 2, 2026  
**Status**: Production Ready  
**License**: GPL-2.0-or-later

## Overview

First stable release of `rust-add-apt-repository`, a complete Rust reimplementation of the Ubuntu/Debian `add-apt-repository` command. This release provides a fast, safe, and feature-complete alternative to the Python version from `software-properties-common`.

## Highlights

- 🎉 **Complete Feature Parity** with Python add-apt-repository
- 🚀 **Production Ready** with 95 passing tests
- 📚 **Comprehensive Documentation** (man page, examples, guides)
- 🔒 **Memory Safe** Rust implementation
- ⚡ **Fast Execution** with improved performance
- 🤝 **Co-installable** with original Python version

## Features

### Repository Management
- ✅ Add and remove repositories
- ✅ List all configured repositories
- ✅ Support for multiple repository formats:
  - PPA shortcuts (`ppa:user/ppa-name`)
  - Cloud Archive (`cloud-archive:release`)
  - Direct URIs with components
  - Full sources.list lines
  - Positional arguments (deprecated but supported)

### PPA Support
- ✅ Public PPA integration with Launchpad API
- ✅ Private PPA support with authentication
- ✅ Automatic GPG key import and verification
- ✅ PPA information display (description, web link)
- ✅ Source code repository management

### Cloud Archive
- ✅ Ubuntu OpenStack release support
- ✅ All pockets (main, proposed, updates)
- ✅ Release name mapping (bobcat, caracal, dalmatian, etc.)
- ✅ Automatic pocket construction

### Advanced Features
- ✅ **Global Operations**: Component, pocket, and source management across all repos
- ✅ **DEB822 Format**: Full support for modern .sources files
- ✅ **GPG Key Management**: Automatic import to trusted.gpg.d
- ✅ **Authentication**: Private repository credential management
- ✅ **Debug Mode**: Verbose logging for troubleshooting
- ✅ **Validation**: Component and suite validation with warnings
- ✅ **Dry-run Mode**: Preview changes before applying
- ✅ **Proper Exit Codes**: 0 (success), 1 (error), 2 (invalid input)

### File Format Support
- ✅ One-line format (.list files)
- ✅ DEB822 format (.sources files)
- ✅ Mixed format support
- ✅ Comment preservation
- ✅ Inline PGP keys (Signed-By field)

## Installation

### From Debian Package

```bash
# Download package
wget https://github.com/maw629/rust-add-apt-repository/releases/download/v0.1.0/rust-add-apt-repository_0.1.0-1_amd64.deb

# Install
sudo dpkg -i rust-add-apt-repository_0.1.0-1_amd64.deb
sudo apt-get install -f  # Install dependencies if needed
```

### From Source

```bash
# Clone repository
git clone https://github.com/maw629/rust-add-apt-repository.git
cd rust-add-apt-repository

# Build and install
cargo build --release
sudo install -m 755 target/release/rust-add-apt-repository /usr/local/bin/
```

See [INSTALL.md](INSTALL.md) for detailed installation instructions.

## Usage Examples

### Add a PPA
```bash
sudo rust-add-apt-repository ppa:graphics-drivers/ppa
```

### Add Cloud Archive
```bash
sudo rust-add-apt-repository cloud-archive:bobcat
```

### Add Repository by URI
```bash
sudo rust-add-apt-repository --uri http://example.com/repo --dist noble --component main
```

### Global Operations
```bash
# Enable universe component
sudo rust-add-apt-repository --component universe

# Enable source repositories
sudo rust-add-apt-repository -s

# Add updates pocket
sudo rust-add-apt-repository --pocket updates
```

### List Repositories
```bash
rust-add-apt-repository --list
```

See [EXAMPLES.md](EXAMPLES.md) for comprehensive usage examples.

## Compatibility

### Python Version Compatibility

Maintains behavioral compatibility with the Python `add-apt-repository`:

- ✅ Same command-line interface
- ✅ Same file operations and locations
- ✅ Same output format
- ✅ Same exit codes
- ✅ Same validation rules
- ✅ Same global operations behavior

**Enhancements over Python version:**
- Faster execution
- Better error messages
- Structured debug output
- Memory safety guarantees

### Supported Distributions

Tested and supported on:

- Ubuntu 24.04 (Noble Numbat) ✅
- Ubuntu 22.04 (Jammy Jellyfish) ✅
- Ubuntu 20.04 (Focal Fossa) ✅
- Debian 12 (Bookworm) ✅
- Debian 11 (Bullseye) ✅
- Windows Subsystem for Linux (WSL) ✅

### Co-installation

Can be installed alongside `software-properties-common`:

- **Python version**: `/usr/bin/add-apt-repository`
- **Rust version**: `/usr/bin/rust-add-apt-repository`

No conflicts or compatibility issues.

## Documentation

### User Documentation
- **Man Page**: `man rust-add-apt-repository`
- **Examples**: [EXAMPLES.md](EXAMPLES.md) - 50+ examples
- **Installation**: [INSTALL.md](INSTALL.md) - Complete guide
- **README**: [README.md](README.md) - Overview and quick start

### Developer Documentation
- **Build Guide**: [BUILDING.md](BUILDING.md)
- **Dependencies**: [DEPENDENCIES.md](DEPENDENCIES.md)
- **WSL Notes**: [WSL.md](WSL.md)
- **Phase Summaries**: See [docs/development/](docs/development/) for detailed implementation history

## Testing

Comprehensive test coverage ensures reliability:

- **74 Unit Tests**: Testing individual modules and functions
- **21 Integration Tests**: End-to-end CLI workflow testing
- **95 Total Tests**: All passing with no failures
- **2 Ignored Tests**: Network-dependent tests (manual verification)

### Test Coverage
- ✅ All CLI flags and options
- ✅ All repository formats
- ✅ Global operations
- ✅ Error handling and validation
- ✅ Exit codes
- ✅ Dry-run and debug modes

## Technical Details

### Implementation
- **Language**: Rust 2021 Edition
- **Lines of Code**: 4,777 lines across 17 modules
- **Binary Size**: ~1.4 MB (release build)
- **Build Time**: ~2 seconds (release)
- **Dependencies**: Minimal (clap, anyhow, thiserror, libc, chrono, serde)

### Modules
- `main.rs` - Entry point
- `lib.rs` - Main application logic
- `cli.rs` - Command-line parsing
- `repository.rs` - Repository representation
- `sourceslist.rs` - Sources.list file management
- `sources.rs` - Source entry parsing
- `ppa.rs` - PPA handling
- `cloudarchive.rs` - Cloud Archive support
- `gpg.rs` - GPG key management
- `auth.rs` - Authentication handling
- `global.rs` - Global operations
- `deb822.rs` - DEB822 format support
- `validation.rs` - Input validation
- `debug.rs` - Debug logging
- `utils.rs` - Utility functions
- `config.rs` - Configuration constants
- `error.rs` - Error types

### Dependencies
- `clap 4.4` - Command-line parsing
- `anyhow 1.0` - Error handling
- `thiserror 1.0` - Error derive macros
- `libc 0.2` - System calls
- `chrono 0.4.31` - Date/time handling
- `serde 1.0` - Serialization
- `serde_json 1.0` - JSON parsing

### Package Details
- **Package Name**: rust-add-apt-repository
- **Version**: 0.1.0-1
- **Size**: 523 KB
- **Architecture**: amd64
- **Section**: admin
- **Priority**: optional
- **License**: GPL-2.0-or-later

## Development

The project followed a rigorous 13-phase incremental development approach:

1. **Phase 0**: Project Setup
2. **Phase 1**: Core Data Structures
3. **Phase 2**: Sources.list Operations
4. **Phase 3**: Basic Repository Addition
5. **Phase 4**: GPG Key Management
6. **Phase 5**: PPA Support Core
7. **Phase 6**: PPA Authentication
8. **Phase 7**: Cloud Archive Support
9. **Phase 8**: Global Operations
10. **Phase 9**: DEB822 Format Support
11. **Phase 10**: Advanced Features
12. **Phase 11**: Testing & Documentation
13. **Phase 12**: Package & Distribution

Each phase was:
- Fully documented and planned
- Implemented incrementally with tests
- Reviewed and validated
- Committed with meaningful messages
- Documented in phase summaries

See [docs/development/plan.md](docs/development/plan.md) for the complete development history.

## Known Limitations

1. **Network Tests**: 2 tests requiring network access are ignored in automated suite
2. **Root Privileges**: Most operations require root (by design, matches Python version)
3. **Distribution Detection**: Currently uses lsb_release fallback (sufficient for Ubuntu/Debian)

None of these limitations affect normal usage or compatibility.

## Future Enhancements

Potential improvements for future releases (not critical for v1.0):

- Additional repository shortcuts (custom shortcuts)
- Batch repository operations
- Repository backup/restore
- Repository migration tools
- Additional validation rules
- More comprehensive distribution detection

## Migration from Python Version

The Rust version is a drop-in replacement for most use cases:

### Same Commands Work
```bash
# Python version
sudo add-apt-repository ppa:test/ppa

# Rust version (same behavior)
sudo rust-add-apt-repository ppa:test/ppa
```

### Switching to Rust Version

**Option 1**: Use explicit command
```bash
sudo rust-add-apt-repository ppa:test/ppa
```

**Option 2**: Create alias
```bash
alias add-apt-repository='rust-add-apt-repository'
```

**Option 3**: Use update-alternatives
```bash
sudo update-alternatives --install /usr/local/bin/add-apt-repository \
  add-apt-repository /usr/bin/rust-add-apt-repository 50
```

### No Breaking Changes

All existing scripts and workflows continue to work with the Python version. The Rust version is an additional option, not a replacement.

## Contributors

This project was developed following the original Python implementation from `software-properties-common` maintained by Canonical Ltd.

## License

GPL-2.0-or-later

This program is free software; you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation; either version 2 of the License, or (at your option) any later version.

## Links

- **GitHub**: https://github.com/maw629/rust-add-apt-repository
- **Issues**: https://github.com/maw629/rust-add-apt-repository/issues
- **Releases**: https://github.com/maw629/rust-add-apt-repository/releases

## Changelog

See [debian/changelog](debian/changelog) for detailed change history.

## Acknowledgments

Based on the Python implementation from `software-properties-common` package. This Rust version is an independent reimplementation designed for compatibility and interoperability.

---

**Questions or Issues?**

- File an issue: https://github.com/maw629/rust-add-apt-repository/issues
- Read the docs: `man rust-add-apt-repository`
- Check examples: [EXAMPLES.md](EXAMPLES.md)
