# GitHub Labels Guide

This document describes the GitHub labels used in this repository.

## Current Labels (Automatically Created)

These labels were automatically created when we added issue templates:

### Priority & Difficulty
- **`good first issue`** (purple #7057ff) - Great for newcomers to the project
- **`help wanted`** (green #008672) - Extra attention needed from community

### Type
- **`bug`** (red #d73a4a) - Something isn't working
- **`enhancement`** (cyan #a2eeef) - New feature or request
- **`documentation`** (blue #0075ca) - Improvements or additions to documentation
- **`question`** (pink #d876e3) - Further information is requested

### Status
- **`duplicate`** (gray #cfd3d7) - This issue or pull request already exists
- **`invalid`** (yellow #e4e669) - This doesn't seem right
- **`wontfix`** (white #ffffff) - This will not be worked on

## Recommended Additional Labels

These labels can be added manually via GitHub UI for better issue management:

### Project Specific
```bash
# Behavioral compatibility with Python version
breaking-change      #d93f0b    Changes that break backward compatibility
python-compatibility #c5def5    Related to Python add-apt-repository compatibility

# Technical areas
dependencies         #0366d6    Dependency updates or issues
security             #ee0701    Security-related issues or improvements
performance          #0e8a16    Performance improvements or issues
testing              #1d76db    Related to tests or test infrastructure
ci-cd                #bfd4f2    CI/CD pipeline changes or fixes

# Workflow
needs-review         #fbca04    Awaiting maintainer review
blocked              #b60205    Blocked by other issues or external factors
```

## How to Add Labels Manually

1. Go to: https://github.com/maw629/rust-add-apt-repository/labels
2. Click "New label"
3. Enter:
   - Name (e.g., `breaking-change`)
   - Description (e.g., "Changes that break backward compatibility")
   - Color (e.g., `#d93f0b` or pick from palette)
4. Click "Create label"

## How to Apply Labels

### To Issues
- Open the issue
- Click "Labels" in the right sidebar
- Select appropriate labels

### To Pull Requests
- Open the PR
- Click "Labels" in the right sidebar
- Select appropriate labels

### Via Issue Templates
Our issue templates automatically apply:
- `bug` label for bug reports
- `enhancement` label for feature requests

## Label Usage Guidelines

### `good first issue`
Use for issues that:
- Are well-defined and scoped
- Don't require deep knowledge of the codebase
- Have clear acceptance criteria
- Are suitable for first-time contributors

Examples:
- Documentation typo fixes
- Adding examples to EXAMPLES.md
- Simple test cases
- Minor UI/output improvements

### `help wanted`
Use for issues that:
- Need community input or contributions
- You don't have time to work on immediately
- Require specific expertise (e.g., Debian packaging)

### `bug` vs `enhancement`
- **bug**: Existing functionality is broken or incorrect
- **enhancement**: Request for new functionality or improvements

### `documentation`
Use for:
- README/doc updates
- Code comment improvements
- Man page updates
- Example additions

## Label Colors Reference

GitHub's default color palette:
- Red (#d73a4a): Critical issues, bugs, security
- Orange (#d93f0b): Warnings, breaking changes
- Yellow (#fbca04): Needs attention, review
- Green (#008672): Good first issue, help wanted
- Blue (#0075ca, #0366d6): Documentation, dependencies
- Purple (#7057ff): Good first issue
- Pink (#d876e3): Questions
- Gray (#cfd3d7): Duplicates, won't fix

## Automation

Labels are automatically applied by:
- ✅ Issue templates (`bug`, `enhancement`)
- ✅ Dependabot PRs (`dependencies` - if added)
- 🔄 Future: GitHub Actions could auto-label based on file changes

## References

- [GitHub Labels Documentation](https://docs.github.com/en/issues/using-labels-and-milestones-to-track-work/managing-labels)
- [Sane GitHub Labels](https://medium.com/@dave_lunny/sane-github-labels-c5d2e6004b63)
- [Label Colors](https://github.com/github/feedback/discussions/8016)
