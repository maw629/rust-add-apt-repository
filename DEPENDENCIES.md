# Dependencies Quick Reference

## System Packages Required

### For Building on Ubuntu/Debian

```bash
# Update package lists
sudo apt update

# Install Rust development tools (if not using rustup)
sudo apt install rustc cargo

# Install build essentials
sudo apt install build-essential pkg-config

# Install APT development libraries
sudo apt install libapt-pkg-dev

# Install GPG/Crypto libraries
sudo apt install libgpgme-dev libgpg-error-dev

# Install SSL/TLS libraries
sudo apt install libssl-dev

# Install Debian packaging tools (for building .deb packages)
sudo apt install debhelper devscripts dh-cargo

# One-liner for all build dependencies:
sudo apt install -y build-essential pkg-config rustc cargo \
  libapt-pkg-dev libgpgme-dev libgpg-error-dev libssl-dev \
  debhelper devscripts dh-cargo
```

### For Runtime (Installed via .deb package dependencies)

```bash
# These are automatically installed by the .deb package
libapt-pkg6.0 (or libapt-pkg5.0 on older Ubuntu)
libgpgme11
libgpg-error0
libssl3 (or libssl1.1 on older Ubuntu)
gnupg
apt
```

## Rust Installation (Recommended Method)

```bash
# Install rustup (Rust version manager)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Follow prompts, then:
source $HOME/.cargo/env

# Verify installation
rustc --version
cargo --version
```

## Rust Crates (Cargo Dependencies)

These are specified in `Cargo.toml` and installed automatically by cargo:

```toml
[dependencies]
clap = { version = "4.4", features = ["derive"] }
libc = "0.2"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

[dev-dependencies]
tempfile = "3"
```

**Dependency notes:**
- `clap`: CLI argument parsing with derive macros (essential for --flags)
- `libc`: System calls for root permission checks (`geteuid()`)
- `serde` + `serde_json`: JSON deserialization for Launchpad PPA API
- `tempfile`: Testing only - creates temporary directories for unit tests

**Previously used (removed in v0.2.0+):**
- ~~`chrono`~~: Was for timestamp-based backups, but `.save` suffix used instead
- ~~`anyhow`~~: Removed - custom `AppError` provides better control for exit codes
- ~~`thiserror`~~: Removed - manual error impl needed for Python compatibility

## WSL-Specific Considerations

### Additional Windows-side Requirements
- WSL 2 (recommended): `wsl --update`
- Windows Terminal (optional but recommended)

### Same apt packages as native Ubuntu
No additional Ubuntu packages needed in WSL.

### Differences to Note
1. **Systemd**: May not be available in WSL 1
   - Check: `ps -p 1 -o comm=`
   - Enable in WSL 2: Edit `/etc/wsl.conf`
   
2. **Desktop Keyring**: Not available
   - Use file-based auth only
   - Set `LP_CREDENTIALS_FILE` environment variable

3. **File System**: Use Linux filesystem (/home), not Windows (/mnt/c)
   - Better performance
   - Correct file permissions

4. **Network**: Works through Windows network stack
   - May need Windows firewall rules for some operations

## Version Requirements

- **Rust**: 1.70.0 or newer (for MSRV)
- **Ubuntu/Debian**: 
  - Ubuntu 20.04 LTS (Focal) or newer
  - Debian 11 (Bullseye) or newer
- **WSL**: WSL 2 recommended (WSL 1 may work with limitations)

## Library Version Notes

### APT Library Versions by Ubuntu Release
- Ubuntu 24.04 (Noble): libapt-pkg6.0
- Ubuntu 22.04 (Jammy): libapt-pkg6.0
- Ubuntu 20.04 (Focal): libapt-pkg6.0
- Ubuntu 18.04 (Bionic): libapt-pkg5.0 (unsupported, may work)

### Ensure Compatibility
The Rust implementation should dynamically link to whatever libapt-pkg version is installed on the system.

## Testing Your Environment

```bash
# Check if all build tools are available
which rustc cargo gcc pkg-config

# Check library installations
pkg-config --exists apt-pkg && echo "apt-pkg found" || echo "apt-pkg NOT found"
pkg-config --exists gpgme && echo "gpgme found" || echo "gpgme NOT found"

# Check debhelper version
dpkg -l | grep debhelper

# Try a test build (once project is set up)
cargo build --release

# Try building debian package (once debian/ is set up)
debuild -us -uc -b
```

## Common Issues

### Issue: "apt-pkg.h: No such file or directory"
**Solution**: Install `libapt-pkg-dev`

### Issue: "gpgme.h: No such file or directory"
**Solution**: Install `libgpgme-dev`

### Issue: "cannot find -lssl"
**Solution**: Install `libssl-dev`

### Issue: cargo not found
**Solution**: Install rust via rustup or apt package `cargo`

### Issue: debuild command not found
**Solution**: Install `devscripts`

### WSL Issue: systemd not available
**Solution**: Enable systemd in WSL 2 via `/etc/wsl.conf`:
```ini
[boot]
systemd=true
```
Then restart WSL: `wsl --shutdown` (from Windows)

## References

- Rust Installation: https://rustup.rs/
- Cargo Documentation: https://doc.rust-lang.org/cargo/
- Debian Packaging: https://www.debian.org/doc/manuals/maint-guide/
- WSL Documentation: https://learn.microsoft.com/en-us/windows/wsl/
