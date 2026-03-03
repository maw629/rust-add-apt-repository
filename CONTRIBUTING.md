# Contributing to rust-add-apt-repository

Thank you for your interest in contributing to rust-add-apt-repository! This document provides guidelines and instructions for contributing to the project.

## Development Workflow - Trunk-Based Development

This project uses **Trunk-Based Development (TBD)**, a modern development workflow where all work flows through the `trunk` branch. This keeps the workflow simple and enables continuous integration.

### Key Principles

- **`trunk` branch is always releasable** - All commits must pass tests and build successfully
- **Short-lived feature branches** - Keep branches small and merge within 1-2 days
- **Small, focused commits** - Easier to review and less likely to break things
- **Frequent integration** - Merge to trunk often to avoid conflicts

### Why Trunk-Based Development?

- ✅ Simpler workflow for contributors (no confusion about which branch)
- ✅ Faster feedback through continuous integration
- ✅ Easier to maintain - no long-lived branches to manage
- ✅ Industry standard used by Google, Kubernetes, Rust, and Deno projects
- ✅ Enables continuous delivery and faster releases
- ✅ Branch name `trunk` makes the workflow self-documenting

## How to Contribute

### 1. Fork and Clone

```bash
# Fork the repository on GitHub first, then:
git clone https://github.com/<your-username>/rust-add-apt-repository.git
cd rust-add-apt-repository

# Add upstream remote
git remote add upstream https://github.com/maw629/rust-add-apt-repository.git
```

### 2. Create a Feature Branch

Always create a branch from `trunk`:

```bash
# Update your trunk branch first
git checkout trunk
git pull upstream trunk

# Create a feature branch
git checkout -b fix-something
```

**Branch naming conventions:**
- `fix-*` - Bug fixes
- `feat-*` - New features
- `docs-*` - Documentation changes
- `refactor-*` - Code refactoring
- `test-*` - Test additions/fixes

### 3. Make Your Changes

Follow these guidelines:

#### Code Quality Requirements

```bash
# Before committing, always run:
cargo fmt              # Format code
cargo clippy           # Check for issues
cargo test             # Run all tests

# Or run all at once:
cargo fmt && cargo clippy -- -D warnings && cargo test
```

**All contributions must:**
- ✅ Pass `cargo test` (all 95 tests)
- ✅ Pass `cargo clippy -- -D warnings` (no warnings)
- ✅ Follow `cargo fmt` formatting
- ✅ Maintain behavioral compatibility with Python version
- ✅ Include tests for new functionality
- ✅ Update documentation if needed

#### Commit Guidelines

- Write clear, descriptive commit messages
- Keep commits focused on a single change
- Reference issues in commit messages: `fix: Resolve #123 - description`

**Good commit message format:**
```
<type>: <short summary>

<optional detailed description>

Fixes #issue-number
```

**Types:** `fix`, `feat`, `docs`, `refactor`, `test`, `chore`

### 4. Test Your Changes

#### Automated Tests (No Root Required)

```bash
# Run all automated tests
cargo test

# Run specific test suite
cargo test --lib                    # Unit tests only
cargo test --test integration_test  # Integration tests only

# Run with output
cargo test -- --nocapture

# Run a specific test
cargo test test_ppa_parsing
```

#### Manual E2E Testing (Requires Root)

For changes affecting repository operations, GPG keys, or file system:
- See [TESTING.md](TESTING.md) for comprehensive E2E test workflows
- Test with `--dry-run` first to verify logic
- Use safe test repositories (see TESTING.md)

### 5. Push and Create Pull Request

```bash
# Push your branch
git push origin fix-something

# Create PR on GitHub:
# - Base branch: trunk
# - Compare branch: <your-username>:fix-something
```

### 6. Pull Request Guidelines

Your PR should:

**Required:**
- [ ] Have a clear, descriptive title
- [ ] Reference related issues (Fixes #123)
- [ ] Pass all CI checks
- [ ] Include tests for new functionality
- [ ] Update documentation if needed
- [ ] Keep changes focused and minimal

**PR Title Format:**
```
fix: Brief description of the fix
feat: Brief description of the feature
docs: Brief description of doc changes
```

**PR Description Template:**

A template is provided automatically when you create a PR. Fill it out completely to help reviewers understand your changes.

### 7. Review Process

1. **Automated checks run** - CI builds, tests, and lints your code
2. **Maintainer review** - Code review and feedback
3. **Address feedback** - Make requested changes if needed
4. **Approval and merge** - Once approved, maintainer merges to trunk
5. **Branch cleanup** - Delete your branch after merge

## Development Environment Setup

### Prerequisites

**System packages required:**
```bash
sudo apt install -y build-essential pkg-config rustc cargo \
  libapt-pkg-dev libgpgme-dev libgpg-error-dev libssl-dev
```

**Rust toolchain (recommended):**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

See [BUILDING.md](BUILDING.md) for detailed build instructions.

### First Build

```bash
# Build debug version
cargo build

# Run the binary
./target/debug/rust-add-apt-repository --help

# Build release version
cargo build --release
```

## Project Architecture

Understanding the codebase helps you contribute effectively.

### Module Overview

- `src/main.rs` - Entry point, root permission check
- `src/lib.rs` - Main application logic, operation dispatch
- `src/repository.rs` - Repository representation and handling
- `src/sourceslist.rs` - **Most critical** - APT sources management (641 lines)
- `src/ppa.rs` - PPA handling and Launchpad API (380 lines)
- `src/cloudarchive.rs` - Ubuntu Cloud Archive support
- `src/gpg.rs` - GPG key management
- `src/validation.rs` - Input validation and checks

See [.github/copilot-instructions.md](.github/copilot-instructions.md) for detailed architecture.

### Key Conventions

1. **Python Compatibility is Critical**
   - This is a drop-in replacement for the Python version
   - Behavioral compatibility is more important than code structure
   - When in doubt, match Python behavior (not documentation)

2. **Error Handling**
   - Use `Result<T>` with `AppError` throughout
   - Provide helpful error messages with context

3. **Testing**
   - Unit tests in `#[cfg(test)] mod tests` within each module
   - Integration tests in `tests/integration_test.rs`
   - Manual E2E tests documented in `TESTING.md`

4. **Debug Logging**
   - Use `debug_log!()` macro for debug output
   - Enabled with `--debug` flag, goes to stderr

## Types of Contributions

### 🐛 Bug Fixes

Found a bug? Great!

1. **Check existing issues** - Someone might already be working on it
2. **Create an issue first** - Describe the bug, expected vs actual behavior
3. **Fork and fix** - Create PR with fix and tests
4. **Reference the issue** - Use "Fixes #123" in commit/PR

### ✨ New Features

Want to add a feature?

1. **Open an issue first** - Discuss the feature before implementing
2. **Get approval** - Ensure it aligns with project goals
3. **Implement incrementally** - Break large features into smaller PRs
4. **Add tests** - Comprehensive test coverage required
5. **Update docs** - Document new functionality

### 📚 Documentation

Documentation improvements are always welcome!

- Fix typos or unclear explanations
- Add examples or clarifications
- Update outdated information
- Improve code comments

Documentation changes don't need extensive testing, but should be reviewed for accuracy.

### 🧪 Tests

Adding tests improves code quality:

- Add test cases for untested scenarios
- Improve test coverage
- Add integration tests for CLI workflows
- Document E2E test procedures

## What NOT to Contribute

Please avoid:

- ❌ Reformatting code without functional changes (use `cargo fmt`)
- ❌ Adding dependencies without strong justification
- ❌ Breaking changes to CLI interface (breaks compatibility)
- ❌ Changes that break behavioral compatibility with Python version
- ❌ Large, unfocused PRs touching many unrelated areas

## Getting Help

### Resources

- **Documentation:**
  - [README.md](README.md) - Project overview and quick start
  - [BUILDING.md](BUILDING.md) - Build instructions
  - [TESTING.md](TESTING.md) - Testing guide
  - [EXAMPLES.md](EXAMPLES.md) - Usage examples
  - [docs/development/](docs/development/) - Development history and plans

- **Questions:**
  - Open a GitHub issue with the "question" label
  - Check existing issues for similar questions

### Understanding Python Behavior

The Python implementation is in the `software-properties-common` package:

```bash
# Install Python version to study behavior
sudo apt install software-properties-common

# View Python source (if available)
dpkg -L software-properties-common | grep -E '\.py$'
```

See [docs/development/source-analysis.md](docs/development/source-analysis.md) for notes on Python behavior.

## Code of Conduct

Be respectful and constructive:

- ✅ Be welcoming and inclusive
- ✅ Be respectful of differing viewpoints
- ✅ Accept constructive criticism gracefully
- ✅ Focus on what's best for the project
- ❌ No harassment, trolling, or derogatory comments

## License

By contributing, you agree that your contributions will be licensed under the GNU General Public License v2.0 or later (GPL-2.0-or-later), matching the license of the original Python implementation.

## Questions?

Don't hesitate to ask! Open an issue with the "question" label, and we'll be happy to help.

---

**Thank you for contributing to rust-add-apt-repository!** 🎉
