# Testing Guide for rust-add-apt-repository

This guide covers both automated and manual testing procedures for `rust-add-apt-repository`.

## Table of Contents

- [Automated Testing](#automated-testing)
- [Manual End-to-End Testing](#manual-end-to-end-testing)
- [Test Workflows](#test-workflows)
- [Safe Test Repositories](#safe-test-repositories)
- [Troubleshooting](#troubleshooting)

---

## Automated Testing

### Overview

The project includes 95 automated tests that run without root privileges:
- **74 unit tests** - Individual module and function testing
- **21 integration tests** - CLI interface testing with `--dry-run`
- **2 ignored tests** - Network-dependent (manual only)

### Running Automated Tests

```bash
# Run all tests
cargo test

# Run with detailed output
cargo test -- --nocapture

# Run specific test suites
cargo test --lib                    # Unit tests only
cargo test --test integration_test  # Integration tests only

# Run a specific test
cargo test test_ppa_parsing

# Run ignored tests (requires network)
cargo test -- --ignored
```

### Test Categories

#### Unit Tests (`cargo test --lib`)

Located in `src/*/tests` modules:
- Repository parsing and validation
- PPA URL construction
- Cloud Archive release mapping
- GPG key operations
- Source entry parsing
- Component and pocket handling
- Error handling

#### Integration Tests (`cargo test --test integration_test`)

Located in `tests/integration_test.rs`:
- CLI argument parsing
- Help and version output
- Repository format handling (PPA, Cloud Archive, URI)
- Global operations (components, pockets, sources)
- Validation and error messages
- Exit code verification

**Note**: Integration tests use `--dry-run` mode, so they don't require root or modify your system.

---

## Manual End-to-End Testing

### What is E2E Testing?

End-to-End (E2E) testing verifies the complete workflow with actual system changes:
- Real file modifications in `/etc/apt/sources.list.d/`
- Actual GPG key imports
- Real `apt update` operations
- Package installation verification

### Prerequisites

- Root/sudo access
- Ubuntu/Debian system
- Active internet connection
- Backup of `/etc/apt/sources.list.d/` (recommended)

### Safety First

```bash
# Backup your repository configuration
sudo cp -r /etc/apt/sources.list.d /etc/apt/sources.list.d.backup
sudo cp /etc/apt/sources.list /etc/apt/sources.list.backup

# If something goes wrong, restore:
sudo rm -rf /etc/apt/sources.list.d
sudo mv /etc/apt/sources.list.d.backup /etc/apt/sources.list.d
sudo cp /etc/apt/sources.list.backup /etc/apt/sources.list
```

---

## Test Workflows

### Workflow 1: PPA Add/Remove Cycle

This tests the most common use case: adding and removing a PPA.

```bash
# === SETUP: Verify initial state ===
echo "1. Checking initial state..."
ls -la /etc/apt/sources.list.d/
apt-cache policy | grep -i graphics

# === ACTION: Add PPA ===
echo "2. Adding PPA..."
sudo rust-add-apt-repository ppa:graphics-drivers/ppa

# === VERIFY: Check PPA was added ===
echo "3. Verifying PPA added..."
ls -la /etc/apt/sources.list.d/ | grep graphics
cat /etc/apt/sources.list.d/graphics-drivers-ubuntu-ppa-*.list

# === UPDATE: Refresh package lists ===
echo "4. Updating package lists..."
sudo apt update

# === VERIFY: Check packages available ===
echo "5. Verifying packages available..."
apt-cache policy | grep graphics-drivers
apt search nvidia-driver

# === CLEANUP: Remove PPA ===
echo "6. Removing PPA..."
sudo rust-add-apt-repository --remove ppa:graphics-drivers/ppa

# === VERIFY: Check PPA removed ===
echo "7. Verifying PPA removed..."
ls -la /etc/apt/sources.list.d/ | grep graphics || echo "PPA removed successfully"

# === FINAL UPDATE ===
echo "8. Final apt update..."
sudo apt update
```

**Expected Results**:
- Step 3: `.list` file created in `/etc/apt/sources.list.d/`
- Step 5: Packages from PPA appear in search results
- Step 7: `.list` file removed (or moved to `.save`)
- No errors throughout

### Workflow 2: Component Management

Test enabling/disabling repository components.

```bash
# === SETUP: Check current components ===
echo "1. Current repository configuration..."
grep -h "^deb " /etc/apt/sources.list /etc/apt/sources.list.d/*.list 2>/dev/null | head -5

# === ACTION: Enable universe component ===
echo "2. Enabling universe component..."
sudo rust-add-apt-repository --component universe

# === VERIFY: Check universe enabled ===
echo "3. Verifying universe enabled..."
grep -h "^deb.*universe" /etc/apt/sources.list

# === UPDATE ===
echo "4. Updating package lists..."
sudo apt update

# === VERIFY: Test universe package ===
echo "5. Testing universe package availability..."
apt search cowsay  # cowsay is typically in universe

# === CLEANUP: Disable universe (optional) ===
echo "6. To disable, edit /etc/apt/sources.list and remove 'universe' from components"
```

**Expected Results**:
- Step 3: Lines in `sources.list` should include `universe` component
- Step 5: Packages from universe should be searchable

### Workflow 3: Cloud Archive

Test Ubuntu Cloud Archive repository management.

```bash
# === SETUP: Check current state ===
echo "1. Checking for existing cloud archive..."
ls -la /etc/apt/sources.list.d/ | grep cloud || echo "No cloud archive found"

# === ACTION: Add cloud archive ===
echo "2. Adding Ubuntu Cloud Archive for OpenStack Bobcat..."
sudo rust-add-apt-repository cloud-archive:bobcat

# === VERIFY: Check cloud archive added ===
echo "3. Verifying cloud archive configuration..."
ls -la /etc/apt/sources.list.d/ | grep cloud
cat /etc/apt/sources.list.d/cloudarchive-*.list

# === UPDATE ===
echo "4. Updating package lists..."
sudo apt update

# === VERIFY: Check for OpenStack packages ===
echo "5. Searching for OpenStack packages..."
apt search nova-compute | head -10

# === CLEANUP: Remove cloud archive ===
echo "6. Removing cloud archive..."
sudo rust-add-apt-repository --remove cloud-archive:bobcat

# === VERIFY: Check removed ===
echo "7. Verifying removal..."
ls -la /etc/apt/sources.list.d/ | grep cloud || echo "Cloud archive removed"

# === FINAL UPDATE ===
sudo apt update
```

**Expected Results**:
- Step 3: Cloud archive `.list` file created
- Step 5: OpenStack packages available
- Step 7: Cloud archive removed

### Workflow 4: Custom Repository by URI

Test adding a repository by explicit URI.

```bash
# === ACTION: Add repository with explicit URI ===
echo "1. Adding repository by URI..."
sudo rust-add-apt-repository \
  --uri http://archive.ubuntu.com/ubuntu \
  --dist noble \
  --component universe

# === VERIFY: Check repository added ===
echo "2. Checking sources.list.d..."
ls -la /etc/apt/sources.list.d/
# Or check main sources.list if global operation
grep "universe" /etc/apt/sources.list

# === UPDATE ===
echo "3. Updating package lists..."
sudo apt update
```

**Expected Results**:
- Repository configuration updated
- `apt update` succeeds without errors

### Workflow 5: Source Repositories

Test enabling source code repositories.

```bash
# === SETUP: Check current sources ===
echo "1. Checking for existing deb-src lines..."
grep -c "^deb-src" /etc/apt/sources.list

# === ACTION: Enable sources ===
echo "2. Enabling source repositories..."
sudo rust-add-apt-repository -s

# === VERIFY: Check sources enabled ===
echo "3. Verifying deb-src lines added..."
grep "^deb-src" /etc/apt/sources.list | head -5

# === UPDATE ===
echo "4. Updating package lists..."
sudo apt update

# === VERIFY: Test source package ===
echo "5. Testing source package download..."
apt source hello  # Downloads hello source package
ls -la hello*/
rm -rf hello*  # Cleanup downloaded sources
```

**Expected Results**:
- Step 3: `deb-src` lines present in sources.list
- Step 5: Source package downloads successfully

---

## Safe Test Repositories

Use these repositories for testing without breaking your system:

### Recommended Test PPAs

```bash
# Small, stable PPAs for testing

# 1. Official Ubuntu Security PPA (always safe)
sudo rust-add-apt-repository ppa:ubuntu-security-proposed/ppa

# 2. Git PPA (stable, commonly used)
sudo rust-add-apt-repository ppa:git-core/ppa

# 3. Test with dry-run first
sudo rust-add-apt-repository --dry-run ppa:any-ppa/test
```

### Test with Popular, Stable PPAs

These are widely used and unlikely to cause issues:

```bash
# Graphics drivers (NVIDIA proprietary)
ppa:graphics-drivers/ppa

# Wine (Windows compatibility)
ppa:ubuntu-wine/ppa

# LibreOffice fresh
ppa:libreoffice/ppa
```

### Avoid Testing With

- ❌ PPAs with system-critical packages (libc, kernel)
- ❌ Experimental or development PPAs
- ❌ PPAs that conflict with base system packages
- ❌ Unknown or untrusted PPAs

---

## Complete Test Script

Save this as `test-e2e.sh` for comprehensive testing:

```bash
#!/bin/bash
set -e

echo "============================================"
echo "rust-add-apt-repository E2E Test Suite"
echo "============================================"
echo ""

# Check if running as root
if [ "$EUID" -ne 0 ]; then 
    echo "ERROR: This script must be run as root (use sudo)"
    exit 1
fi

# Backup
echo "Creating backup..."
cp -r /etc/apt/sources.list.d /etc/apt/sources.list.d.backup-$(date +%s)
echo "Backup created"
echo ""

# Test 1: PPA Add/Remove
echo "TEST 1: PPA Add/Remove"
echo "======================"
rust-add-apt-repository ppa:git-core/ppa
apt update -qq
rust-add-apt-repository --remove ppa:git-core/ppa
apt update -qq
echo "✅ Test 1 passed"
echo ""

# Test 2: Dry-run mode
echo "TEST 2: Dry-run Mode"
echo "===================="
rust-add-apt-repository --dry-run ppa:test/test
echo "✅ Test 2 passed"
echo ""

# Test 3: List repositories
echo "TEST 3: List Repositories"
echo "========================="
rust-add-apt-repository --list | head -10
echo "✅ Test 3 passed"
echo ""

# Test 4: Help output
echo "TEST 4: Help Output"
echo "==================="
rust-add-apt-repository --help > /dev/null
echo "✅ Test 4 passed"
echo ""

# Test 5: Version output
echo "TEST 5: Version Output"
echo "======================"
rust-add-apt-repository --version
echo "✅ Test 5 passed"
echo ""

echo "============================================"
echo "All tests passed! ✅"
echo "============================================"
```

Run with:
```bash
chmod +x test-e2e.sh
sudo ./test-e2e.sh
```

---

## Verification Checklist

Use this checklist to verify functionality:

### Basic Operations
- [ ] `--help` displays comprehensive help
- [ ] `--version` shows version number
- [ ] `--list` displays current repositories
- [ ] `--dry-run` previews changes without modifying system

### PPA Operations
- [ ] Add PPA: `sudo rust-add-apt-repository ppa:user/ppa`
- [ ] Remove PPA: `sudo rust-add-apt-repository --remove ppa:user/ppa`
- [ ] PPA file created in `/etc/apt/sources.list.d/`
- [ ] PPA file removed after `--remove`
- [ ] GPG key imported automatically

### Cloud Archive
- [ ] Add Cloud Archive: `sudo rust-add-apt-repository cloud-archive:bobcat`
- [ ] Repository configuration correct
- [ ] Packages available after `apt update`

### Global Operations
- [ ] Enable component: `sudo rust-add-apt-repository --component universe`
- [ ] Enable sources: `sudo rust-add-apt-repository -s`
- [ ] Add pocket: `sudo rust-add-apt-repository --pocket security`
- [ ] Changes reflected in `/etc/apt/sources.list`

### Custom Repositories
- [ ] Add by URI with all parameters
- [ ] Repository validates correctly
- [ ] `apt update` succeeds

### Error Handling
- [ ] Invalid PPA shows error message
- [ ] Missing parameters show helpful error
- [ ] Network errors handled gracefully
- [ ] Exit codes correct (0=success, 1=error)

---

## Troubleshooting

### Test Failures

**Issue**: `apt update` fails after adding repository

```bash
# Check the repository configuration
cat /etc/apt/sources.list.d/problematic-repo.list

# Try manual apt update with details
sudo apt update

# Remove problematic repository
sudo rust-add-apt-repository --remove <repository>
```

**Issue**: GPG key import fails

```bash
# Check GPG error details
sudo apt update  # Look for GPG errors

# Try importing key manually
wget -qO- <key-url> | sudo gpg --dearmor -o /etc/apt/trusted.gpg.d/<name>.gpg
```

**Issue**: Permission denied

```bash
# Ensure running with sudo
sudo rust-add-apt-repository <options>

# Check file permissions
ls -la /etc/apt/sources.list.d/
```

### Cleanup After Failed Tests

```bash
# Remove all test repositories
sudo rm /etc/apt/sources.list.d/*test*.list

# Restore from backup
sudo cp -r /etc/apt/sources.list.d.backup/* /etc/apt/sources.list.d/

# Clean apt cache
sudo apt clean
sudo apt update
```

### Reset to Clean State

```bash
# Remove all custom repositories (CAREFUL!)
sudo rm /etc/apt/sources.list.d/*.list

# Keep only official Ubuntu repos
# Edit /etc/apt/sources.list to ensure base repos exist

# Update
sudo apt update
```

---

## WSL-Specific Testing

If testing on Windows Subsystem for Linux:

### Important Considerations

1. **Filesystem Location**: Run tests from Linux filesystem (`/home/user/`), not Windows mounts (`/mnt/c/`)
2. **Performance**: Tests are 10-40x slower on Windows mounts
3. **Permissions**: Some permission operations may behave differently
4. **Systemd**: Limited in WSL 1, works better in WSL 2

### WSL Test Commands

```bash
# Verify you're on Linux filesystem
pwd | grep "^/mnt" && echo "WARNING: On Windows mount!" || echo "OK: On Linux filesystem"

# Run tests
cd ~/rust-add-apt-repository
cargo test

# E2E tests work the same
sudo ./test-e2e.sh
```

See [WSL.md](WSL.md) for more WSL-specific information.

---

## Performance Benchmarks

Expected test execution times on typical hardware:

| Test Suite | Time | Notes |
|------------|------|-------|
| Unit tests (74) | 0.3-0.5s | Fast, no I/O |
| Integration tests (21) | 0.5-1.0s | CLI execution |
| E2E single workflow | 5-15s | Includes `apt update` |
| Full E2E suite | 30-60s | Multiple repos |

**Note**: Times are significantly higher on:
- WSL Windows mounts (10-40x slower)
- Slow network connections (for key downloads)
- Systems with many existing repositories

---

## Continuous Integration

For automated CI/CD pipelines:

```bash
# Run all safe tests (no root required)
cargo test

# Run with coverage (optional)
cargo tarpaulin --out Html

# Run clippy for linting
cargo clippy -- -D warnings

# Check formatting
cargo fmt --check
```

**Do not run E2E tests in CI** unless you have:
- Isolated test environment
- Root access
- Proper cleanup after tests
- Snapshot/restore capability

---

## Additional Resources

- **Unit Test Source**: `src/*/tests` modules in each source file
- **Integration Tests**: `tests/integration_test.rs`
- **Usage Examples**: [EXAMPLES.md](EXAMPLES.md)
- **Build Guide**: [BUILDING.md](BUILDING.md)
- **Installation Guide**: [INSTALL.md](INSTALL.md)

---

## Contributing Tests

When adding new features:

1. **Write unit tests** for new functions
2. **Add integration tests** for new CLI functionality
3. **Document E2E procedures** in this file
4. **Ensure all tests pass** before committing

Example test structure:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_feature() {
        // Arrange
        let input = "test-data";
        
        // Act
        let result = new_feature(input);
        
        // Assert
        assert_eq!(result, expected);
    }
}
```

---

## Summary

This guide provides:
- ✅ Automated test execution (no root)
- ✅ Manual E2E test workflows (requires root)
- ✅ Safe test repositories
- ✅ Troubleshooting procedures
- ✅ Verification checklists

**Remember**: 
- Automated tests are safe and fast
- E2E tests verify real-world functionality
- Always backup before E2E testing
- Use safe, known repositories for testing

For questions or issues, see [TROUBLESHOOTING.md](TROUBLESHOOTING.md) or open an issue on the project repository.
