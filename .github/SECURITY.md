# Security Policy

## Supported Versions

We provide security updates for the following versions:

| Version | Supported          |
| ------- | ------------------ |
| 0.2.x   | :white_check_mark: |
| < 0.2.0 | :x:                |

## Reporting a Vulnerability

If you discover a security vulnerability in rust-add-apt-repository, please report it responsibly.

### How to Report

**Do NOT open a public GitHub issue for security vulnerabilities.**

Instead, please report security issues via email:

- **Email:** maw.signup@gmail.com
- **Subject:** [SECURITY] rust-add-apt-repository - Brief description

### What to Include

Please provide the following information in your report:

1. **Description:** Clear description of the vulnerability
2. **Impact:** What can an attacker do with this vulnerability?
3. **Reproduction Steps:** Detailed steps to reproduce the issue
4. **Affected Versions:** Which versions are affected?
5. **Suggested Fix:** If you have one (optional but appreciated)
6. **Your Contact:** How we can reach you for follow-up

### What to Expect

- **Acknowledgment:** We will acknowledge your report within **48 hours**
- **Initial Assessment:** We will provide an initial assessment within **5 business days**
- **Fix Timeline:** We aim to release a fix within **7 days for critical issues**, longer for less severe issues
- **Credit:** We will credit you in the security advisory (unless you prefer to remain anonymous)
- **Disclosure:** We follow responsible disclosure practices

### Security Update Process

1. We assess the severity and impact of the vulnerability
2. We develop and test a fix
3. We release a security patch version
4. We publish a security advisory on GitHub
5. We notify users via release notes and GitHub Security Advisories

## Security Practices

This project maintains security through:

- ✅ **Automated Dependency Scanning:** Dependabot monitors dependencies for known vulnerabilities
- ✅ **Code Review:** All changes require pull request review before merging to trunk
- ✅ **Behavioral Compatibility Testing:** Changes must maintain compatibility with Python version
- ✅ **Signed Releases:** All releases are tagged and include checksums (SHA256)
- ✅ **Minimal Dependencies:** We keep dependencies to a minimum to reduce attack surface
- ✅ **Root Permission Checks:** Operations requiring elevated privileges are explicitly checked

## Known Security Considerations

### Root Privileges Required

This tool requires root/sudo privileges for most operations because it:
- Modifies system files in `/etc/apt/`
- Imports GPG keys to system keyring
- Updates APT package sources

**Best Practices:**
- Review the code before running with sudo
- Use `--dry-run` flag to preview changes
- Only install from trusted sources

### APT Sources Configuration

This tool modifies critical system configuration:
- `/etc/apt/sources.list`
- `/etc/apt/sources.list.d/*.list`
- `/etc/apt/trusted.gpg.d/*.gpg`

**Best Practices:**
- Backup your APT configuration before use: `sudo cp -r /etc/apt /etc/apt.backup`
- Only add repositories from trusted sources (official PPAs, verified publishers)
- Review repository URLs before adding them
- Use official package from releases, not random builds

### PPA Security

When adding PPAs, this tool:
- Downloads signing keys from Launchpad API over HTTPS
- Verifies key authenticity through Launchpad
- Imports keys to system keyring

**Best Practices:**
- Only add PPAs from trusted maintainers
- Verify PPA ownership on Launchpad.net before adding
- Be cautious with PPAs that provide system-critical packages

## Security Scope

### In Scope

Security vulnerabilities in:
- Repository parsing and validation
- GPG key handling and verification
- File system operations and permissions
- Command injection or code execution
- Authentication credential handling
- Input validation and sanitization

### Out of Scope

Issues that are not security vulnerabilities:
- Bugs that don't have security implications
- Feature requests
- Performance issues
- Compatibility issues (report as regular bugs)
- Issues in dependencies (report to upstream projects)
- Issues in the Python version (report to Canonical/Ubuntu)

## Security Contacts

- **Primary:** Hardy Nguyen - maw.signup@gmail.com
- **GitHub Security Advisories:** https://github.com/maw629/rust-add-apt-repository/security/advisories

## Additional Resources

- [Debian Security](https://www.debian.org/security/)
- [Ubuntu Security](https://ubuntu.com/security)
- [APT Security](https://wiki.debian.org/SecureApt)
- [Launchpad PPA Security](https://help.launchpad.net/Packaging/PPA)

## Past Security Advisories

No security advisories have been published yet.

Future advisories will be listed here and published at:
https://github.com/maw629/rust-add-apt-repository/security/advisories

---

**Thank you for helping keep rust-add-apt-repository and its users safe!**
