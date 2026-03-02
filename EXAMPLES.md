# rust-add-apt-repository Examples

This document provides comprehensive examples of using `rust-add-apt-repository` for various repository management tasks.

## Table of Contents
- [Basic Usage](#basic-usage)
- [PPA Management](#ppa-management)
- [Cloud Archive](#cloud-archive)
- [URI Repositories](#uri-repositories)
- [Global Operations](#global-operations)
- [Advanced Usage](#advanced-usage)
- [Troubleshooting](#troubleshooting)

## Basic Usage

### Show Help
```bash
rust-add-apt-repository --help
```

### Show Version
```bash
rust-add-apt-repository --version
```

### List All Repositories
```bash
rust-add-apt-repository --list
```
Lists all configured repositories from `/etc/apt/sources.list` and `/etc/apt/sources.list.d/`.

## PPA Management

### Add a Public PPA
```bash
sudo rust-add-apt-repository ppa:graphics-drivers/ppa
```
Adds the NVIDIA graphics drivers PPA to your system.

### Add a PPA with Source Code Support
```bash
sudo rust-add-apt-repository -s ppa:mozillateam/ppa
```
Adds both binary (deb) and source (deb-src) repositories.

### Add a PPA for Specific Release
```bash
sudo rust-add-apt-repository ppa:user/ppa-name/jammy
```
Adds a PPA for Ubuntu 22.04 (Jammy) even if you're on a different release.

### Remove a PPA
```bash
sudo rust-add-apt-repository --remove ppa:graphics-drivers/ppa
```
Removes the PPA and optionally cleans up the GPG key.

### Add Private PPA (Requires Authentication)
```bash
sudo rust-add-apt-repository --login ppa:username/private-ppa
```
Prompts for Launchpad credentials to access private PPAs.

### Dry-Run Before Adding PPA
```bash
sudo rust-add-apt-repository --dry-run ppa:test/ppa
```
Shows what would be added without making changes.

## Cloud Archive

### Add Cloud Archive Release
```bash
sudo rust-add-apt-repository cloud-archive:bobcat
```
Adds the OpenStack Bobcat release from Ubuntu Cloud Archive.

### Alternative Cloud Archive Syntax
```bash
sudo rust-add-apt-repository uca:caracal
```
Short form: `uca:` is equivalent to `cloud-archive:`.

### Add Cloud Archive with Proposed Pocket
```bash
sudo rust-add-apt-repository cloud-archive:bobcat-proposed
```
Adds the proposed pocket for testing upcoming Cloud Archive updates.

### Remove Cloud Archive
```bash
sudo rust-add-apt-repository --remove cloud-archive:bobcat
```

## URI Repositories

### Add Repository by URI
```bash
sudo rust-add-apt-repository \
  --uri http://apt.postgresql.org/pub/repos/apt \
  --dist noble-pgdg \
  --component main
```
Adds the PostgreSQL repository for Ubuntu 24.04.

### Add Multi-Component Repository
```bash
sudo rust-add-apt-repository \
  --uri http://archive.ubuntu.com/ubuntu \
  --dist noble \
  --component main \
  --component universe \
  --component multiverse
```
Multiple components can be specified with repeated `--component` flags.

### Add Repository with Source Code
```bash
sudo rust-add-apt-repository -s \
  --uri http://example.com/repo \
  --dist stable \
  --component main
```
Adds both deb and deb-src entries.

### Add Repository Without apt-get update
```bash
sudo rust-add-apt-repository --no-update \
  --uri http://example.com/repo \
  --dist stable \
  --component main
```
Skips running `apt-get update` after adding the repository.

## Sources.list Line Format

### Add Using Full sources.list Line
```bash
sudo rust-add-apt-repository --sourceslist \
  "deb http://archive.ubuntu.com/ubuntu noble multiverse"
```

### Alternative: Positional Argument (Deprecated but Supported)
```bash
sudo rust-add-apt-repository \
  "deb http://archive.ubuntu.com/ubuntu noble multiverse"
```

### Add Source Repository Line
```bash
sudo rust-add-apt-repository \
  "deb-src http://archive.ubuntu.com/ubuntu noble main"
```

### Add Signed Repository
```bash
sudo rust-add-apt-repository --sourceslist \
  "deb [signed-by=/usr/share/keyrings/example.gpg] http://example.com/repo stable main"
```

## Global Operations

Global operations modify all existing repositories when no specific repository is provided.

### Enable Component Globally

Add `universe` component to all Ubuntu repositories:
```bash
sudo rust-add-apt-repository --component universe
```

Add `multiverse` component:
```bash
sudo rust-add-apt-repository --component multiverse
```

### Remove Component Globally
```bash
sudo rust-add-apt-repository --remove --component multiverse
```
Removes `multiverse` from all repositories in `/etc/apt/sources.list`.

### Enable Pocket Globally

Add updates pocket:
```bash
sudo rust-add-apt-repository --pocket updates
```

Add security pocket:
```bash
sudo rust-add-apt-repository --pocket security
```

Add backports pocket:
```bash
sudo rust-add-apt-repository --pocket backports
```

### Remove Pocket Globally
```bash
sudo rust-add-apt-repository --remove --pocket backports
```

### Enable Source Code Repositories Globally

Enable source code for existing repositories:
```bash
sudo rust-add-apt-repository -s
```
Uncomments existing `# deb-src` lines.

Add all missing deb-src lines:
```bash
sudo rust-add-apt-repository -ss
```
Creates new `deb-src` entries for all `deb` repositories that don't have them.

### Disable Source Code Repositories Globally
```bash
sudo rust-add-apt-repository --remove -s
```
Comments out or removes all `deb-src` lines.

## Advanced Usage

### Non-Interactive Mode
```bash
sudo rust-add-apt-repository --yes ppa:test/ppa
```
Assumes "yes" to all prompts. Useful for automation and scripts.

### Debug Mode
```bash
sudo rust-add-apt-repository --debug ppa:test/ppa
```
Prints detailed debug information to stderr for troubleshooting.

### Combine Dry-Run with Debug
```bash
sudo rust-add-apt-repository --debug --dry-run ppa:test/ppa
```
Shows exactly what would happen without making changes.

### Add Multiple Repositories in Sequence
```bash
sudo rust-add-apt-repository ppa:graphics-drivers/ppa
sudo rust-add-apt-repository ppa:mozillateam/ppa
sudo rust-add-apt-repository cloud-archive:bobcat
```

### Add Repository and Skip Update (Batch Operations)
```bash
sudo rust-add-apt-repository --no-update ppa:repo1/ppa
sudo rust-add-apt-repository --no-update ppa:repo2/ppa
sudo rust-add-apt-repository --no-update ppa:repo3/ppa
sudo apt-get update
```
Saves time by running `apt-get update` only once after adding multiple repositories.

### Specify Different Distribution
```bash
sudo rust-add-apt-repository \
  --uri http://example.com/repo \
  --dist jammy \
  --component main
```
Useful when you want packages for a different Ubuntu release.

## Troubleshooting

### Check What Would Be Added
```bash
sudo rust-add-apt-repository --dry-run ppa:test/ppa
```
Always safe to run - shows the operation without making changes.

### Verify Repository Addition
```bash
# Add repository
sudo rust-add-apt-repository ppa:test/ppa

# List to verify
rust-add-apt-repository --list | grep test
```

### Debug Network Issues
```bash
sudo rust-add-apt-repository --debug ppa:test/ppa 2>&1 | tee debug.log
```
Saves debug output to a file for analysis.

### Check Component Validation
```bash
sudo rust-add-apt-repository --dry-run \
  --uri http://example.com/repo \
  --dist noble \
  --component custom-component
```
Shows warnings for unknown components.

### Test Permission Issues
```bash
# This will fail - not running as root
rust-add-apt-repository ppa:test/ppa

# This works
sudo rust-add-apt-repository ppa:test/ppa
```

### Recover from Failed Addition
```bash
# If repository was partially added, remove it
sudo rust-add-apt-repository --remove ppa:test/ppa

# Then try again
sudo rust-add-apt-repository ppa:test/ppa
```

## Common Workflows

### Setting Up Development Environment

```bash
# Enable universe and multiverse
sudo rust-add-apt-repository --component universe
sudo rust-add-apt-repository --component multiverse

# Add required PPAs
sudo rust-add-apt-repository ppa:deadsnakes/ppa  # Python versions
sudo rust-add-apt-repository ppa:git-core/ppa    # Latest Git

# Add PostgreSQL
sudo rust-add-apt-repository \
  --uri http://apt.postgresql.org/pub/repos/apt \
  --dist noble-pgdg \
  --component main

# Update package cache
sudo apt-get update
```

### Enabling Source Code for Package Development

```bash
# Enable all source repositories
sudo rust-add-apt-repository -ss

# Update
sudo apt-get update

# Now you can get source code
apt-get source package-name
```

### Cloud Server Setup

```bash
# Add Cloud Archive for OpenStack
sudo rust-add-apt-repository --yes cloud-archive:bobcat

# Enable updates and security
sudo rust-add-apt-repository --yes --pocket updates
sudo rust-add-apt-repository --yes --pocket security

# Update and upgrade
sudo apt-get update && sudo apt-get upgrade -y
```

### Minimal System (Remove Extra Components)

```bash
# Remove multiverse and restricted
sudo rust-add-apt-repository --remove --component multiverse
sudo rust-add-apt-repository --remove --component restricted

# Disable source repositories
sudo rust-add-apt-repository --remove -s

# Update
sudo apt-get update
```

## Exit Codes

The command uses standard exit codes:

- **0**: Success
- **1**: General error (permissions, I/O, network)
- **2**: Invalid input (malformed repository specification)

Example checking exit code:
```bash
if sudo rust-add-apt-repository --dry-run ppa:test/ppa; then
    echo "Valid repository"
else
    echo "Invalid repository (exit code: $?)"
fi
```

## Notes

- Most operations require root privileges (`sudo`)
- `--list` and `--help` work without root
- `--dry-run` is always safe and shows what would happen
- `--debug` helps troubleshoot issues
- Component validation warns about unknown components but doesn't fail
- The tool maintains compatibility with the Python version

## See Also

- Man page: `man rust-add-apt-repository`
- APT sources: `man sources.list`
- APT configuration: `man apt.conf`
