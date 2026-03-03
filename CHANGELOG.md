# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.1] - 2026-03-03

### Changed
- Simplified GitHub release workflow trigger to use tag selector UI instead of manual input
- Switched to tilde version requirements (`~X.Y`) for all Cargo dependencies
- Updated dependency versions:
  - `tempfile`: 3.8.0 → 3.26.0 (dev-dependency)
  - `actions/cache`: v4 → v5 (GitHub Actions)
  - `actions/checkout`: v4 → v6 (GitHub Actions)

### Fixed
- Removed redundant CI verification step from release workflow
- Added write permissions to release workflow for creating GitHub releases
- Added safety check to prevent releasing from branches (tags only)

### Documentation
- Updated DEPENDENCIES.md with tilde versioning strategy
- Enhanced RELEASE-NOTES.md with dependency management section
- Updated .github/copilot-instructions.md with current dependency versions

## [0.2.0] - 2026-03-03

### Added
- **CI/CD Automation**:
  - GitHub Actions workflow for automated testing and linting
  - Automated release workflow with GitHub Release creation
  - Build automation script (`build-package.sh`)
- **Development Workflow**:
  - Trunk-Based Development workflow
  - Dependabot configuration for automated dependency updates
  - Security policy (SECURITY.md)
  - Pull request template
  - GitHub Copilot instructions for AI-assisted development

### Fixed
- **Critical Bug Fixes**:
  - Only backup files that are actually being modified (not all files)
  - Properly backup files before deletion when removing repository
  - Preserve DEB822 format when saving .sources files
  - Use `.save` extension for backup files instead of timestamps

### Changed
- Reorganized documentation structure:
  - Moved 16 development files to `docs/development/`
  - Created comprehensive phase summaries (Phases 0-12)
  - Improved documentation organization
- Updated to Rust 1.93.0 as minimum version
- Removed unused dependencies (`chrono`, `anyhow`, `thiserror`)

### Documentation
- Added BUILDING.md with detailed build instructions
- Added TESTING.md with comprehensive E2E testing guide
- Added EXAMPLES.md with common usage workflows
- Added DEPENDENCIES.md with dependency reference
- Added WSL.md with Windows Subsystem for Linux notes
- Added LICENSE file (GPL-2.0-or-later)
- Updated README.md with badges and improved structure

## [0.1.0] - 2024-12-24

### Added
- **Core Features** (All 13 phases completed):
  - Repository management (add, remove, list)
  - Full PPA support with Launchpad API integration
  - Ubuntu Cloud Archive support (OpenStack releases)
  - GPG key management (automatic import and verification)
  - Private PPA authentication
  - Global operations (components, pockets, source code)
  - DEB822 format support (.sources files)
  - Advanced features (debug mode, validation, dry-run)
  - Proper exit codes and error handling

- **Testing**:
  - 95 comprehensive tests (74 unit + 21 integration)
  - All tests passing
  - Coverage of all major features

- **Documentation**:
  - Complete implementation plan (docs/development/plan.md)
  - Python source code analysis (docs/development/source-analysis.md)
  - Phase-by-phase summaries
  - Man pages for command-line usage

- **Packaging**:
  - Debian packaging infrastructure
  - Binary installable on Ubuntu/Debian systems
  - Co-installable with original Python version

### Notable Implementation Details
- Behavioral compatibility with Python version from `software-properties-common`
- Selective backup strategy (only modified files)
- Support for both one-line and DEB822 repository formats
- Comprehensive validation and error reporting
- Root permission checks
- Launchpad API integration for PPA metadata

---

## Version History Summary

- **v0.2.1** (2026-03-03): Release workflow improvements and dependency management
- **v0.2.0** (2026-03-03): CI/CD automation, bug fixes, documentation improvements
- **v0.1.0** (2024-12-24): Initial release with all core features

[Unreleased]: https://github.com/maw629/rust-add-apt-repository/compare/v0.2.1...HEAD
[0.2.1]: https://github.com/maw629/rust-add-apt-repository/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/maw629/rust-add-apt-repository/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/maw629/rust-add-apt-repository/releases/tag/v0.1.0
