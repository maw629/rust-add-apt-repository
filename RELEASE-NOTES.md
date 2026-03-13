# Release Notes - rust-add-apt-repository

## v0.2.1 - Release Workflow Fix

**Release Date**: March 4, 2026

This patch release fixes the GitHub release workflow for easier operation. All features from v0.2.0 remain unchanged.

### 🔧 Workflow Improvements

#### Release Workflow Fixes
- **Simplified workflow trigger**: Use GitHub's tag selector UI instead of manual tag input
  - Select tag directly from "Use workflow from" dropdown
  - Eliminates need to type tag name twice
  - Reduces user error

- **Removed CI verification step**: 
  - CI already runs automatically when tag is pushed
  - Removed redundant and failing CI status check
  - Workflow now starts immediately

- **Added safety check**: Prevents releasing from branches
  - Validates that selected ref is actually a tag
  - Fails fast with clear error message if branch selected
  - Provides step-by-step instructions for correct usage

### 📦 Dependency Management Improvements

#### Tilde Version Requirements
- **Switched to tilde requirements** (`~X.Y`) for all Cargo dependencies
  - Makes dependency updates visible in Cargo.toml (not just Cargo.lock)
  - Locks to minor versions while allowing patch updates
  - Improves Dependabot PR clarity and reviewability
  
#### Dependency Updates via Dependabot
- **tempfile**: 3.8.0 → 3.26.0 (dev-dependency)
  - 18 minor versions of bug fixes and platform improvements
  - Only affects test code
- **GitHub Actions updates**:
  - `actions/cache`: v4 → v5 (Node.js 24 runtime)
  - `actions/checkout`: v4 → v6 (improved credential handling)

### 📊 Changes

- **Files Modified**: 6 files
  - `.github/workflows/release.yml` - Simplified workflow
  - `.github/workflows/ci.yml` - Updated actions versions
  - `.github/copilot-instructions.md` - Updated release instructions
  - `Cargo.toml` - Switched to tilde version requirements
  - `Cargo.lock` - Updated locked versions
  - `DEPENDENCIES.md` - Documented versioning strategy

### 🎯 Impact

This release improves the development workflow through better dependency management visibility and simplified release process. All code features, bug fixes, and functionality from v0.2.0 remain unchanged.

---

## v0.2.0 - Bug Fixes, CI/CD, and Documentation Improvements

**Release Date**: March 4, 2026

This release focuses on critical bug fixes related to backup file handling, introduces CI/CD automation, and includes comprehensive documentation reorganization.

### 🚀 CI/CD & Automation

#### GitHub Actions Workflows
- **Continuous Integration**: Automated testing and linting on every push/PR
  - Runs all 95 tests (74 unit + 21 integration)
  - Enforces code formatting with `cargo fmt`
  - Runs clippy linter to catch common issues
  - Uses Rust 1.93.0 for consistency
  - Triggers on branches, PRs, and version tags
  
- **Release Automation**: Streamlined release process with GitHub Actions
  - Manual trigger via workflow_dispatch for controlled releases
  - Verifies CI passed before creating release
  - Builds both binary and .deb package
  - Generates SHA256 checksums automatically
  - Extracts changelog from git tag annotations
  - Uploads artifacts to GitHub releases
  
- **Build Script**: Automated build process (`build-package.sh`)
  - Single command to build complete release
  - Creates versioned artifacts in `release/v{VERSION}/`
  - Handles cargo build, debuild, and checksum generation
  - Color-coded output for easy debugging

#### Rust Version Management
- **Pinned to Rust 1.93.0**: Consistent builds across local and CI environments
  - Uses `rust-toolchain.toml` for version pinning
  - Prevents "works on my machine" issues
  - Updated codebase to use Rust 1.93.0 features (e.g., `is_none_or`)
  - `debian/rules` supports both rustup and system Rust

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
- `.github/workflows/ci.yml`: Added CI workflow for automated testing
- `.github/workflows/release.yml`: Added release workflow for automated releases
- `build-package.sh`: New automated build script
- `debian/rules`: Updated to support rustup cargo
- `rust-toolchain.toml`: Pinned Rust version to 1.93.0
- Documentation files: Reorganized and enhanced

#### Testing
- All 95 unit tests passing
- CI runs automatically on all branches and PRs
- Manual E2E testing completed
- Backup behavior verified against Python version

### 📊 Statistics

- **Commits**: All development work since project inception
- **Pull Requests**: 5 merged (CI setup, dependency cleanup, Rust upgrade, release workflow)
- **Files Changed**: 25+ files
- **Bug Fixes**: 4 critical issues resolved
- **CI/CD**: Complete automation infrastructure
- **Documentation**: Comprehensive reorganization

### 🙏 Contributors

Thanks to all who tested and provided feedback!

---

**Note**: v0.2.0 is the initial public release. All development work including the 13 implementation phases was completed before this first tagged release.
