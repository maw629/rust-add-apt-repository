# Building and Running in WSL (Windows Subsystem for Linux)

This guide provides WSL-specific instructions and considerations for building and running `rust-add-apt-repository` on Windows Subsystem for Linux.

## Table of Contents

- [WSL Environment Overview](#wsl-environment-overview)
- [WSL Prerequisites](#wsl-prerequisites)
- [Key Differences from Native Ubuntu](#key-differences-from-native-ubuntu)
- [Installation Steps for WSL](#installation-steps-for-wsl)
- [Known Issues in WSL](#known-issues-in-wsl)
- [Testing Limitations in WSL](#testing-limitations-in-wsl)
- [WSL-Specific Workarounds](#wsl-specific-workarounds)
- [Best Practices](#best-practices)

## WSL Environment Overview

### What is WSL?

Windows Subsystem for Linux (WSL) allows you to run a Linux environment directly on Windows without the overhead of a traditional virtual machine or dual-boot setup.

### WSL Versions

- **WSL 1**: Translation layer that converts Linux system calls to Windows equivalents
  - Faster file access on Windows filesystem (`/mnt/c`)
  - No systemd support
  - Limited kernel functionality

- **WSL 2**: Full Linux kernel running in a lightweight VM
  - Full system call compatibility
  - Better I/O performance on Linux filesystem
  - systemd support (Windows 11 / recent Windows 10)
  - **Recommended for this project**

Check your WSL version:
```bash
wsl -l -v
```

### Supported Ubuntu Versions in WSL

This project works with:
- **Ubuntu 24.04 LTS** (recommended)
- **Ubuntu 22.04 LTS** (recommended)
- **Ubuntu 20.04 LTS**

Install from Microsoft Store or via PowerShell:
```powershell
# List available distributions
wsl --list --online

# Install Ubuntu 24.04
wsl --install -d Ubuntu-24.04
```

## WSL Prerequisites

### 1. Update WSL

From Windows PowerShell (as Administrator):

```powershell
# Update WSL to latest version
wsl --update

# Check WSL version
wsl --version
```

### 2. Check Ubuntu Version in WSL

Inside your WSL terminal:

```bash
# Check distribution and version
lsb_release -a

# Should show Ubuntu 20.04, 22.04, or 24.04
```

### 3. Install Windows Terminal (Recommended)

Windows Terminal provides a better experience than the default console:

- Download from Microsoft Store: "Windows Terminal"
- Or install via winget:
  ```powershell
  winget install Microsoft.WindowsTerminal
  ```

Benefits:
- Tabbed interface
- Better Unicode support
- Customizable appearance
- Split panes

### 4. Enable systemd (WSL 2 Only)

If using WSL 2, enable systemd for better compatibility:

```bash
# Create or edit /etc/wsl.conf
sudo nano /etc/wsl.conf
```

Add these lines:
```ini
[boot]
systemd=true
```

Then restart WSL from PowerShell:
```powershell
wsl --shutdown
```

Verify systemd is running:
```bash
ps -p 1 -o comm=
# Should output: systemd
```

## Key Differences from Native Ubuntu

### 1. systemd Availability

**Native Ubuntu**: Always has systemd
**WSL 1**: No systemd support
**WSL 2**: systemd available (must be enabled manually)

**Impact on this project**:
- Most functionality works without systemd
- Service management features may be limited
- Repository updates still work via `apt-get update`

**Check systemd status**:
```bash
ps -p 1 -o comm=

# systemd available: outputs "systemd"
# systemd not available: outputs "init" or "/init"
```

### 2. File Systems

**Critical**: Always work in the Linux filesystem, not the Windows filesystem.

```bash
# ✓ GOOD: Linux filesystem (fast, correct permissions)
cd ~
cd /home/username/projects

# ✗ BAD: Windows filesystem (slow, permission issues)
cd /mnt/c/Users/username/Documents
```

**Why this matters**:
- Windows filesystem (`/mnt/c`, `/mnt/d`) is much slower
- File permissions don't work correctly on Windows filesystem
- Git operations are slow on Windows filesystem
- Build tools expect Linux permissions

**Access your files from Windows**:
```bash
# From WSL, open current directory in Windows Explorer
explorer.exe .
```

Your Linux home directory is accessible from Windows at:
```
\\wsl$\Ubuntu-24.04\home\username
```

### 3. Network Configuration

**Native Ubuntu**: Direct network access
**WSL**: Shares Windows network stack

**Implications**:
- Uses Windows firewall rules
- DNS configuration from Windows
- Network operations usually "just work"
- May need Windows firewall exceptions for servers

**Check network connectivity**:
```bash
# Test internet connection
ping -c 3 archive.ubuntu.com

# Check DNS resolution
nslookup launchpad.net
```

### 4. GPG/Keyring Behavior

**Native Ubuntu**: Full desktop keyring (gnome-keyring, KDE wallet)
**WSL**: No desktop environment

**Implications**:
- Cannot use desktop keyring for credentials
- Must use file-based authentication
- Set `LP_CREDENTIALS_FILE` environment variable for Launchpad

**Solution for Launchpad authentication**:
```bash
# Store credentials in a file
mkdir -p ~/.launchpad
chmod 700 ~/.launchpad
nano ~/.launchpad/credentials

# Add to ~/.bashrc:
export LP_CREDENTIALS_FILE=~/.launchpad/credentials
```

### 5. apt-get Behavior

**Good news**: APT works normally in WSL!

```bash
# Update package lists - works perfectly
sudo apt update

# Install packages - works perfectly
sudo apt install build-essential

# Add repositories - works perfectly (with sudo)
sudo add-apt-repository ppa:example/ppa
```

### 6. Permissions and sudo

**Native Ubuntu**: Standard Linux permissions
**WSL**: Mostly the same, with some quirks

```bash
# sudo works normally
sudo apt update

# File ownership works normally in Linux filesystem
chmod 644 file.txt
chown user:user file.txt

# Windows files appear as owned by root
ls -l /mnt/c/  # All owned by root
```

## Installation Steps for WSL

Follow the same steps as [BUILDING.md](BUILDING.md), with these WSL-specific notes:

### 1. Ensure You're in Linux Filesystem

```bash
# Navigate to home directory
cd ~

# Create projects directory
mkdir -p ~/projects
cd ~/projects
```

### 2. Install System Dependencies

```bash
sudo apt update
sudo apt install -y build-essential pkg-config libapt-pkg-dev \
  libgpgme-dev libgpg-error-dev libssl-dev curl
```

**Note**: This works identically to native Ubuntu. No special steps needed.

### 3. Install Rust via rustup

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

**Verify**:
```bash
rustc --version
cargo --version
```

### 4. Clone and Build

```bash
# Clone repository
cd ~/projects
git clone https://github.com/maw629/rust-add-apt-repository.git
cd rust-add-apt-repository

# Build release version
cargo build --release

# Test it
./target/release/rust-add-apt-repository --help
```

### 5. Install Debian Package (Optional)

```bash
# Install packaging tools
sudo apt install -y debhelper devscripts dh-cargo

# Build package
debuild -us -uc -b

# Install package
sudo dpkg -i ../rust-add-apt-repository_*.deb
```

## Known Issues in WSL

### 1. systemd-Dependent Features

**Issue**: Some services require systemd

**Affected**:
- System service management
- Automatic service startup
- Some D-Bus functionality

**Workaround**: Enable systemd in WSL 2 (see [WSL Prerequisites](#wsl-prerequisites))

**Status**: Not critical for this project

### 2. Launchpad Authentication

**Issue**: Desktop keyring not available in WSL

**Affected**:
- Adding private PPAs that require Launchpad authentication
- Storing credentials in system keyring

**Workaround**: Use file-based credentials

```bash
# Create credentials file
mkdir -p ~/.launchpad
chmod 700 ~/.launchpad
nano ~/.launchpad/credentials

# Format (username:password)
# Example content:
launchpad-user:api-token-here

# Set environment variable
export LP_CREDENTIALS_FILE=~/.launchpad/credentials

# Add to ~/.bashrc to persist:
echo 'export LP_CREDENTIALS_FILE=~/.launchpad/credentials' >> ~/.bashrc
```

**Status**: Workaround available

### 3. D-Bus Limitations

**Issue**: D-Bus session bus may not be available

**Affected**:
- Desktop notifications
- Inter-process communication with desktop apps

**Workaround**: Not needed for command-line usage

**Status**: Does not affect core functionality

### 4. File Permission Edge Cases

**Issue**: Windows filesystem has unusual permission behavior

**Affected**:
- Files on `/mnt/c`, `/mnt/d`
- Everything appears executable
- Cannot set proper Unix permissions

**Workaround**: Always work in Linux filesystem (`~` or `/home`)

**Status**: Avoided by using Linux filesystem

### 5. Network Port Binding

**Issue**: Windows reserves some ports

**Affected**:
- Running test servers on certain ports
- Some low-numbered ports require Windows firewall rules

**Workaround**: 
- Use high-numbered ports (> 8000)
- Add Windows firewall exceptions if needed

**Status**: Minor, rarely affects this project

## Testing Limitations in WSL

### What Works Fully

✅ **Building the project**
```bash
cargo build --release
```

✅ **Running unit tests**
```bash
cargo test
```

✅ **Code quality checks**
```bash
cargo clippy
cargo fmt --check
```

✅ **Dry-run mode** (no system modifications)
```bash
./target/release/rust-add-apt-repository --dry-run ppa:example/ppa
```

✅ **Help and version info**
```bash
./target/release/rust-add-apt-repository --help
./target/release/rust-add-apt-repository --version
```

✅ **Debian package building**
```bash
debuild -us -uc -b
```

### What Has Limitations

⚠️ **Adding actual repositories** (requires sudo)
- Works, but modifies your WSL environment
- Test carefully to avoid breaking your WSL setup
- Use a test WSL distribution if possible

⚠️ **GPG key operations**
- Works, but keys stored in WSL's GPG keyring
- Desktop keyring not available

⚠️ **Launchpad authentication**
- Requires file-based credentials (see workaround above)

### Recommended Testing Approach

For comprehensive testing:

1. **WSL**: Build, unit tests, dry-run mode
2. **Docker in WSL**: Isolated environment testing
3. **Native Ubuntu VM**: Full integration testing
4. **GitHub Actions**: Automated CI/CD testing

## WSL-Specific Workarounds

### 1. Safe Testing Without Breaking WSL

Create a test snapshot of your sources:

```bash
# Backup sources.list files
sudo cp -r /etc/apt/sources.list.d /etc/apt/sources.list.d.backup
sudo cp /etc/apt/sources.list /etc/apt/sources.list.backup

# Test the command
sudo rust-add-apt-repository --dry-run ppa:example/ppa

# If something goes wrong, restore:
sudo rm -rf /etc/apt/sources.list.d
sudo mv /etc/apt/sources.list.d.backup /etc/apt/sources.list.d
sudo mv /etc/apt/sources.list.backup /etc/apt/sources.list
```

### 2. Using Docker Inside WSL for Isolated Testing

Install Docker in WSL 2:

```bash
# Install Docker
curl -fsSL https://get.docker.com -o get-docker.sh
sudo sh get-docker.sh

# Add your user to docker group
sudo usermod -aG docker $USER

# Log out and back in, or:
newgrp docker
```

Test in isolated Ubuntu container:

```bash
# Run Ubuntu container
docker run -it --rm -v $(pwd):/workspace ubuntu:24.04 bash

# Inside container:
cd /workspace
apt update
apt install -y build-essential pkg-config libapt-pkg-dev \
  libgpgme-dev libgpg-error-dev libssl-dev curl

# Install rust and build
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source $HOME/.cargo/env
cargo build --release

# Test
./target/release/rust-add-apt-repository --help
```

### 3. Using Separate WSL Distributions

Create a test WSL distribution:

```powershell
# From PowerShell (as Administrator)

# Export your current distribution
wsl --export Ubuntu-24.04 ubuntu-backup.tar

# Import as test distribution
wsl --import Ubuntu-Test C:\WSL\Ubuntu-Test ubuntu-backup.tar

# Use test distribution
wsl -d Ubuntu-Test

# When done, remove test distribution
wsl --unregister Ubuntu-Test
```

### 4. Quick WSL Reset

If you break your WSL setup:

```powershell
# From PowerShell

# Terminate WSL
wsl --shutdown

# Restart your distribution
wsl -d Ubuntu-24.04

# If really broken, reinstall:
wsl --unregister Ubuntu-24.04
wsl --install -d Ubuntu-24.04
```

## Best Practices

### Development Workflow in WSL

1. **Store code in Linux filesystem**
   ```bash
   ~/projects/rust-add-apt-repository
   ```

2. **Use Windows Terminal** for better experience

3. **Edit code with**:
   - VS Code with Remote-WSL extension (recommended)
   - Vim/Neovim in terminal
   - Any editor, accessing `\\wsl$\` path

4. **Version control**: Git works perfectly in WSL
   ```bash
   git clone <repo>
   git commit -m "message"
   git push
   ```

### VS Code Integration

Install VS Code Remote-WSL extension:

```bash
# From WSL terminal, open VS Code:
code .
```

This automatically:
- Connects VS Code to WSL
- Uses WSL's tools and environment
- Provides native Linux development experience
- Full IntelliSense and debugging support

### Performance Tips

1. **Always use Linux filesystem** - Cannot emphasize enough
2. **Enable systemd** for full compatibility
3. **Use WSL 2** for better performance
4. **Allocate enough memory** - Edit `.wslconfig`:

```ini
# In Windows: C:\Users\<username>\.wslconfig
[wsl2]
memory=4GB
processors=4
```

### Security Considerations

1. **Credentials**: Store in Linux filesystem only
   ```bash
   chmod 600 ~/.launchpad/credentials
   ```

2. **File permissions**: Work in Linux filesystem for proper permissions

3. **sudo access**: WSL has direct sudo access - be careful

4. **Network**: Uses Windows firewall, generally secure

## Troubleshooting WSL-Specific Issues

### Issue: "cannot execute binary file"

**Cause**: Trying to run Windows .exe from WSL or vice versa

**Solution**: Ensure you're running the Linux binary

### Issue: Very slow build times

**Cause**: Working in Windows filesystem (`/mnt/c`)

**Solution**: Move project to Linux filesystem (`~/projects`)

### Issue: Permission denied errors

**Cause**: Working in Windows filesystem

**Solution**: Move to Linux filesystem or check file permissions

### Issue: systemd not starting

**Cause**: Not enabled in `/etc/wsl.conf`

**Solution**: Follow [Enable systemd](#4-enable-systemd-wsl-2-only) steps

### Issue: Cannot connect to network

**Cause**: DNS or network configuration issue

**Solution**:
```bash
# Check DNS
cat /etc/resolv.conf

# Regenerate if needed
sudo rm /etc/resolv.conf
```

Then restart WSL from PowerShell:
```powershell
wsl --shutdown
```

## Additional Resources

- **WSL Documentation**: https://learn.microsoft.com/en-us/windows/wsl/
- **WSL GitHub Issues**: https://github.com/microsoft/WSL/issues
- **Ubuntu on WSL**: https://ubuntu.com/wsl
- **Rust on WSL**: https://rust-lang.github.io/rustup/installation/windows.html#wsl

## Getting Help

For WSL-specific issues:

1. Check WSL documentation
2. Review GitHub issues for WSL
3. Ask on WSL community forums
4. Report project-specific issues to our issue tracker

For general building issues, see [BUILDING.md](BUILDING.md).
