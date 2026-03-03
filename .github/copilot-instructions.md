# Copilot Instructions for rust-add-apt-repository

A Rust implementation of Debian/Ubuntu's `add-apt-repository` command, designed for behavioral compatibility with the Python version from `software-properties-common` while being installable alongside it.

**Project Status:** Version 0.2.0 - 100% complete (all 13 phases implemented), 95/95 tests passing, production ready. CI/CD automated with GitHub Actions.

**Development Workflow:** This project uses **Trunk-Based Development** (TBD) with `trunk` as the default branch. All changes flow through Pull Requests to `trunk`. CI runs automatically on all PRs. See [CONTRIBUTING.md](../CONTRIBUTING.md) for complete workflow guidelines.

## CI/CD & Release Process

### GitHub Actions Workflows

**Continuous Integration (.github/workflows/ci.yml):**
- Runs on all pushes, PRs, and version tags
- Pinned to Rust 1.93.0 for consistency
- Executes all 95 tests (74 unit + 21 integration)
- Enforces code formatting (`cargo fmt --check`)
- Runs clippy linter (`cargo clippy -- -D warnings`)
- Ensures code quality before merging

**Release Workflow (.github/workflows/release.yml):**
- Manual trigger via workflow_dispatch
- Verifies CI passed on target commit
- Builds release binary and .deb package
- Generates SHA256 checksums
- Extracts changelog from git tag annotation
- Creates GitHub release with artifacts
- Artifacts: binary, .deb package, SHA256SUMS.txt

### Creating a Release

1. Update documentation (RELEASE-NOTES.md, README.md, etc.)
2. Merge documentation PR to trunk
3. Create annotated tag with changelog in description:
   ```bash
   git tag -a v0.X.Y -m "Release v0.X.Y

   ### New Features
   - Feature 1
   - Feature 2

   ### Bug Fixes
   - Fix 1"
   ```
4. Push tag: `git push origin v0.X.Y`
5. Wait for CI to pass on the tag
6. Manually trigger release workflow from GitHub Actions UI
7. Verify release artifacts uploaded successfully

## Build, Test, and Lint

### Building

```bash
# Debug build (for development)
cargo build

# Release build (optimized)
cargo build --release

# Build Debian package
debuild -us -uc -b

# Or use the release script (if available)
./build-package.sh  # Creates .deb in release/
```

Binary outputs:
- Debug: `target/debug/rust-add-apt-repository`
- Release: `target/release/rust-add-apt-repository`
- Package: `release/rust-add-apt-repository_*.deb` (via build script)

### Testing

**Automated Tests (95 tests, no root required):**
```bash
# Run all tests (74 unit + 21 integration)
cargo test

# Run specific test suites
cargo test --lib                    # Unit tests only
cargo test --test integration_test  # Integration tests only

# Run single test
cargo test test_ppa_parsing

# Run with output
cargo test -- --nocapture

# Run network-dependent tests (2 ignored tests)
cargo test -- --ignored
```

**Integration tests use `--dry-run` mode** to safely test logic without system changes.

**Manual E2E Tests (require root):**
See [TESTING.md](../TESTING.md) for complete E2E workflows that actually modify `/etc/apt/` files.

**Key test files:**
- `src/sourceslist.rs` - 641 lines, most critical module
- `src/ppa.rs` - 380 lines, PPA/Launchpad integration
- `tests/integration_test.rs` - 963 lines of CLI tests

### Linting

```bash
# Format code
cargo fmt

# Check formatting (CI-friendly)
cargo fmt --check

# Run linter
cargo clippy

# Fail on warnings
cargo clippy -- -D warnings
```

## Architecture

### Module Structure (~4,500 lines across 17 modules)

**Entry Points:**
- `main.rs` - Entry point, root permission check
- `lib.rs` - Main application logic, operation dispatch

**Core Repository Management:**
- `repository.rs` - Repository representation struct with entries, keys, file paths
- `sourceslist.rs` (641 lines) - **MOST CRITICAL** - Reads/writes `/etc/apt/sources.list` and `.d/` files, handles backup logic
- `sources.rs` - Parses individual source entries (deb/deb-src lines)

**Repository Type Handlers:**
- `ppa.rs` (380 lines) - PPA parsing, Launchpad API integration, URL construction
- `cloudarchive.rs` - Ubuntu Cloud Archive support (bobcat, caracal releases)
- `deb822.rs` - Modern .sources file format support

**Supporting Systems:**
- `gpg.rs` - GPG key management, imports keys to `/etc/apt/trusted.gpg.d/`
- `auth.rs` - Private PPA authentication, credential management
- `global.rs` - Global operations (components, pockets, sources enablement)
- `validation.rs` - Input validation, repository template matching

**Utilities:**
- `cli.rs` - Command-line argument parsing via clap
- `config.rs` - Configuration constants (paths, URLs)
- `debug.rs` - Debug logging macros and utilities
- `utils.rs` - Shared utility functions
- `error.rs` - Error types (AppError) and Result wrapper

### Key Data Flow

1. **CLI Parsing** (`cli.rs`) → Parse arguments into `Cli` struct
2. **Root Check** (`main.rs`) → Verify permissions for write operations
3. **Operation Dispatch** (`lib.rs`) → Route to add/remove/list/global operations
4. **Repository Resolution**:
   - PPA: `ppa.rs` queries Launchpad API for signing key, constructs Ubuntu URLs
   - Cloud Archive: `cloudarchive.rs` maps release names to apt URLs
   - Custom URI: Direct construction from --uri/--dist/--component
5. **Repository Construction** (`repository.rs`) → Create `Repository` with:
   - Source entries (deb + optionally deb-src)
   - GPG key data/URL
   - Target file path
6. **File Operations**:
   - **Add**: `sourceslist.rs` writes to `/etc/apt/sources.list.d/`, `gpg.rs` imports keys
   - **Remove**: `sourceslist.rs` deletes or moves files to `.save`
   - **Global ops**: `global.rs` modifies main `sources.list` directly
7. **Validation** (`validation.rs`) → Check suite/component combinations, warn on issues

### File System Interactions

**Reads:**
- `/etc/apt/sources.list` - Main repository configuration
- `/etc/apt/sources.list.d/*.list` - Additional repositories
- `/etc/apt/sources.list.d/*.sources` - DEB822 format repositories
- `/etc/lsb-release` - Detect Ubuntu release codename

**Writes:**
- `/etc/apt/sources.list` - Modified by global operations (components, pockets)
- `/etc/apt/sources.list.d/<name>.list` - Created/removed for PPAs and repositories
- `/etc/apt/trusted.gpg.d/<name>.gpg` - GPG keys imported here

**External APIs:**
- `https://launchpad.net/api/1.0/~<user>/+archive/ubuntu/<ppa>` - PPA metadata and signing keys

## Key Conventions

### Python Compatibility Requirements

This is a **drop-in replacement** for the Python version. Behavioral compatibility is critical:

1. **Same command-line interface** - All flags must work identically to Python version
2. **Same file operations** - Must modify identical files in identical ways
3. **Same output format** - User-facing messages should match closely
4. **Same exit codes** - 0 (success), 1 (error), 2 (invalid input)
5. **Same global operations** - Component/pocket/source management behavior must match

**IMPORTANT:** When implementing features, **Python source code is the PRIMARY reference**, not man pages or documentation. The behavior must match the actual Python implementation, not what the docs claim it does.

### Repository File Naming

Files in `/etc/apt/sources.list.d/` follow specific patterns:

- **PPAs**: `{user}-ubuntu-{ppa}-{codename}.list`
- **Cloud Archive**: `cloudarchive-{release}.list`
- **Custom repos**: Generate from URI/description with `.list` extension

### DEB822 vs One-Line Format

- **One-line format** (default): Traditional `deb http://... dist component` style
- **DEB822 format** (optional): Modern structured format in `.sources` files
- Use `use_deb822` flag in `Repository` struct to control format

### Testing Patterns

**Unit tests** (in `#[cfg(test)] mod tests` within each module):
- Test individual functions and parsing logic
- Use sample data, no file I/O
- Fast, comprehensive coverage

**Integration tests** (`tests/integration_test.rs`):
- Test full CLI workflows with `--dry-run`
- Verify exit codes and output
- No root required, no system changes

**E2E tests** (manual, documented in TESTING.md):
- Test real system integration
- Require root, modify actual files
- Use safe test repositories only

### Error Handling

Use `Result<T>` with `AppError` throughout:

```rust
use crate::error::{AppError, Result};

pub fn some_operation() -> Result<()> {
    // Use ? for propagation
    let data = fetch_data()?;
    
    // Use custom errors
    if invalid {
        return Err(AppError::InvalidInput("reason".to_string()));
    }
    
    Ok(())
}
```

Error types in `error.rs`:
- `InvalidInput` - User provided bad input
- `IoError` - File system errors
- `NetworkError` - API/download failures
- `ValidationError` - Repository validation failures

### Debug Logging

Use `debug_log!()` macro for debug output (enabled with `--debug` flag):

```rust
use crate::debug_log;

debug_log!("Operation: {} on {}", op, target);
```

Debug output goes to stderr with `[DEBUG]` prefix.

### Dependency Management

**Production dependencies (4 total):**
- `clap` (4.4+) - CLI argument parsing with derive macros
- `libc` (0.2) - Low-level system calls (root permission checks)
- `serde` (1.0) - Serialization framework with derive support
- `serde_json` (1.0) - JSON parsing for Launchpad API

**Dev dependencies:**
- `tempfile` (3) - Temporary directories/files for unit tests

**Removed dependencies:**
- ~~`chrono`~~ - Removed in v0.2.0 (unused after backup system changed to .save suffix)
- ~~`anyhow`~~ - Removed in v0.2.0 (custom AppError provides better control)
- ~~`thiserror`~~ - Removed in v0.2.0 (manual error impl needed for exit codes)

**System library bindings** (linked at runtime, not in Cargo.toml):
- `libapt-pkg-dev` - APT library integration
- `libgpgme-dev` - GPG key operations
- `libssl-dev` - HTTPS connections

**Dependency philosophy:**
- Keep minimal (currently 4 production deps)
- Prioritize std library over external crates when practical
- Manual implementations acceptable when they provide better control
- Consider behavioral compatibility with Python version
- Avoid proc-macro heavy crates unless they provide significant value

### Commit Practices

This project follows **Trunk-Based Development** workflow:
- All changes merge to `trunk` branch via Pull Requests
- Feature branches are short-lived (hours to days, not weeks)
- Commits must include Co-authored-by trailer: `Co-authored-by: Copilot <223556219+Copilot@users.noreply.github.com>`
- Commit messages should be descriptive with conventional commits format (`feat:`, `fix:`, `chore:`, `docs:`)

**Pull Request requirements:**
- All 95 tests must pass (`cargo test`)
- Code must pass clippy lints (`cargo clippy`)
- Changes must maintain Python behavioral compatibility
- Documentation updates for user-facing changes

**Development documentation:**
- `docs/development/plan.md` - Complete implementation history and strategy
- `docs/development/source-analysis.md` - Python code analysis and behavioral notes
- `docs/development/PHASE-{0-12}-SUMMARY.md` - Detailed phase documentation
- `CONTRIBUTING.md` - Complete workflow and contribution guidelines
- `docs/development/PHASE-{0-12}-SUMMARY.md` - Detailed phase documentation

## Development Notes

### Critical Implementation Details

**Backup Strategy:**
The project uses selective backup with `.save` extension (matching Python version):
- Only files that are actually modified get backed up (tracked via `HashSet<PathBuf>`)
- Backup files use `.save` extension (e.g., `file.list.save`)
- APT properly ignores `.save` files during repository scanning
- The backup logic is in `sourceslist.rs` using dirty file tracking

**Incremental Development Approach:**
- Implement → Test → Commit → Review (not all at once)
- Behavioral compatibility is the goal, not line-by-line code translation
- Python source code behavior is authoritative, not documentation

### WSL Considerations

When developing on Windows Subsystem for Linux:
- Use Linux filesystem (`/home/user/`), NOT Windows mounts (`/mnt/c/`)
- Performance is 10-40x slower on Windows mounts
- See [WSL.md](../WSL.md) for full details

### Build Dependencies

Must be installed before building:
```bash
sudo apt install -y build-essential pkg-config \
  libapt-pkg-dev libgpgme-dev libgpg-error-dev libssl-dev
```

See [DEPENDENCIES.md](../DEPENDENCIES.md) for complete list.

### Co-installation Design

This package is designed to **coexist** with `software-properties-common`:
- Binary named `rust-add-apt-repository` (not `add-apt-repository`)
- No file conflicts with Python version
- Users can install both and choose which to use
- Both modify same system files, so behavior must match exactly

### Long-term Goal

Working toward becoming a "respected alternative" (like ripgrep) that coexists with the original. Official Ubuntu/Debian adoption is a multi-year goal requiring community building, production readiness proofs, and establishing credibility. Version 0.2.0 marks production readiness.

## Quick Reference

```bash
# Full development cycle
cargo fmt && cargo clippy && cargo test && cargo build --release

# Test single module
cargo test ppa

# Build and test binary
cargo build && ./target/debug/rust-add-apt-repository --help

# Package build
debuild -us -uc -b
sudo dpkg -i ../rust-add-apt-repository_*.deb
```
