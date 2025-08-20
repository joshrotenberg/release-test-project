# Complete release-please Setup Guide for Rust Multi-Workspace Projects

This comprehensive guide covers everything you need to set up release-please for your Rust multi-workspace project, including specific fixes for the redl project.

---

## Part 1: Fixing the redl Project Specifically

### Current Issues Found in redl

1. **Missing version in internal dependency** (CRITICAL)
   - Location: `crates/cli/Cargo.toml` line 40
   - Current: `redis-commands = { path = "../redis-commands" }`
   - Should be: `redis-commands = { path = "../redis-commands", version = "0.5.0" }`

2. **Component naming inconsistency** (Minor)
   - The CLI package is named "redl" but component is "cli"
   - Consider changing component to "redl" for clearer tags

3. **Workspace version field** (Minor)
   - Root Cargo.toml has `version = "0.1.0"` but crates are at 0.5.0

### Immediate Fix for redl

```bash
# 1. Create a branch for the fixes
git checkout -b fix/release-please-internal-deps

# 2. Edit crates/cli/Cargo.toml line 40
# Change from: redis-commands = { path = "../redis-commands" }
# To: redis-commands = { path = "../redis-commands", version = "0.5.0" }

# 3. Test that everything still builds
cargo build && cargo test

# 4. Commit with conventional message
git add -A
git commit -m "fix: add version to internal redis-commands dependency

The redis-commands dependency was missing a version field, which can cause
release-please to incorrectly handle version bumps and dependency cascading."

# 5. Push and create PR
git push -u origin fix/release-please-internal-deps
gh pr create
```

**Good news**: Your redl project already has release-please working! This fix just ensures dependency cascading works correctly.

---

## Part 2: Setting Up release-please From Scratch

Use this section if you're starting fresh or need to completely reset.

### Prerequisites

- Rust workspace project with `crates/` directory structure
- GitHub repository
- Willingness to use conventional commits

### Step 1: Prepare Your Workspace Structure

Expected structure:
```
.
├── Cargo.toml              # Workspace root
├── crates/
│   ├── core/              # Core functionality
│   │   ├── Cargo.toml
│   │   └── src/
│   ├── utils/             # Utilities
│   │   ├── Cargo.toml  
│   │   └── src/
│   └── cli/               # CLI application
│       ├── Cargo.toml
│       └── src/
```

Root `Cargo.toml`:
```toml
[workspace]
resolver = "2"
members = [
    "crates/core",
    "crates/utils",
    "crates/cli",
]

[workspace.package]
edition = "2024"
authors = ["Your Name <you@example.com>"]
license = "MIT OR Apache-2.0"
repository = "https://github.com/yourusername/yourrepo"

[workspace.dependencies]
anyhow = "1.0"
thiserror = "2.0"
serde = { version = "1.0", features = ["derive"] }
```

### Step 2: Fix Internal Dependencies (CRITICAL!)

Each crate's `Cargo.toml` MUST specify both path AND version for internal deps:

```toml
[package]
name = "your-package-name"
version = "0.1.0"
edition.workspace = true
authors.workspace = true
license.workspace = true

[dependencies]
# CORRECT - has both path and version
your-core = { path = "../core", version = "0.1.0" }
your-utils = { path = "../utils", version = "0.1.0" }

# WRONG - missing version
# your-core = { path = "../core" }
```

### Step 3: Create release-please Configuration

Create `release-please-config.json`:
```json
{
  "packages": {
    "crates/core": {
      "release-type": "rust",
      "package-name": "your-core",
      "component": "your-core"
    },
    "crates/utils": {
      "release-type": "rust",
      "package-name": "your-utils",
      "component": "your-utils"
    },
    "crates/cli": {
      "release-type": "rust",
      "package-name": "your-cli",
      "component": "your-cli"
    }
  },
  "release-type": "rust",
  "bump-minor-pre-major": true,
  "bump-patch-for-minor-pre-major": false,
  "draft": false,
  "prerelease": false,
  "include-component-in-tag": true,
  "include-v-in-tag": true,
  "changelog-sections": [
    {"type": "feat", "section": "🚀 Features", "hidden": false},
    {"type": "fix", "section": "🐛 Bug Fixes", "hidden": false},
    {"type": "perf", "section": "⚡ Performance", "hidden": false},
    {"type": "docs", "section": "📚 Documentation", "hidden": false},
    {"type": "deps", "section": "📦 Dependencies", "hidden": false},
    {"type": "refactor", "section": "♻️ Code Refactoring", "hidden": false},
    {"type": "test", "section": "✅ Tests", "hidden": false},
    {"type": "build", "section": "🏗️ Build System", "hidden": false},
    {"type": "ci", "section": "👷 CI/CD", "hidden": false},
    {"type": "chore", "section": "🔧 Miscellaneous", "hidden": false},
    {"type": "revert", "section": "⏪ Reverts", "hidden": false},
    {"type": "style", "section": "💄 Styling", "hidden": false}
  ]
}
```

Create `.release-please-manifest.json`:
```json
{
  "crates/core": "0.1.0",
  "crates/utils": "0.1.0",
  "crates/cli": "0.1.0"
}
```

### Step 4: Create GitHub Actions Workflow

Create `.github/workflows/release-please.yml`:
```yaml
name: release-please

on:
  push:
    branches:
      - main

permissions:
  contents: write
  pull-requests: write
  issues: write

jobs:
  release-please:
    runs-on: ubuntu-latest
    steps:
      - uses: googleapis/release-please-action@v4
        id: release
        with:
          config-file: release-please-config.json
          manifest-file: .release-please-manifest.json
```

### Step 5: Add Binary Building (Optional)

Create `.github/workflows/build-release-binaries.yml`:
```yaml
name: Build Release Binaries

on:
  release:
    types: [created]

env:
  CARGO_TERM_COLOR: always

jobs:
  build-binaries:
    name: Build ${{ matrix.target }}
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        include:
          - target: x86_64-unknown-linux-gnu
            os: ubuntu-latest
            binary: your-binary-name
          - target: x86_64-apple-darwin
            os: macos-latest
            binary: your-binary-name
          - target: aarch64-apple-darwin
            os: macos-latest
            binary: your-binary-name
          - target: x86_64-pc-windows-msvc
            os: windows-latest
            binary: your-binary-name.exe

    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          targets: ${{ matrix.target }}
      
      - name: Build binary
        run: cargo build --release --target ${{ matrix.target }} --bin your-binary-name

      - name: Package and upload
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
        run: |
          # Package and upload logic here
          gh release upload ${{ github.event.release.tag_name }} ...
```

### Step 6: Enable GitHub Permissions (CRITICAL!)

Go to your repository Settings → Actions → General:
1. Under "Workflow permissions" select "Read and write permissions"
2. ✅ Check "Allow GitHub Actions to create and approve pull requests"

Without this, you'll get: `GitHub Actions is not permitted to create or approve pull requests`

### Step 7: Initialize Git With Proper Commits

If starting fresh (nuclear option):

```bash
# Remove old git history
rm -rf .git

# Initialize new repository
git init
git branch -M main

# Add release-please files first
git add release-please-config.json .release-please-manifest.json
git add .github/workflows/
git commit -m "build: add release-please configuration for automated releases"

# Add your code
git add .
git commit -m "feat: initial implementation of multi-workspace project"

# Create GitHub repo and push
gh repo create your-repo-name --private --source=. --remote=origin --push
```

---

## Part 3: Conventional Commits Reference

### MUST Use These Formats

**Version-bumping commits:**
```bash
# Patch version (0.1.0 → 0.1.1)
git commit -m "fix: resolve memory leak in data processor"
git commit -m "fix(cli): correct argument parsing"

# Minor version (0.1.0 → 0.2.0)
git commit -m "feat: add authentication system"
git commit -m "feat(auth): implement JWT validation"

# Major version (0.1.0 → 1.0.0)
git commit -m "feat!: redesign API to use async/await"
git commit -m "fix!: change config format from TOML to JSON"
```

**Non-releasing but included in changelog:**
```bash
git commit -m "docs: update README"
git commit -m "test: add integration tests"
git commit -m "refactor: simplify error handling"
git commit -m "style: apply rustfmt"
git commit -m "chore: update dependencies"
git commit -m "ci: add Windows to test matrix"
git commit -m "build: optimize binary size"
```

### BAD Commits (Won't Work!)
```bash
"Updated README"          # No type prefix
"Fix bug"                 # No colon after type
"FEAT: add feature"       # Uppercase type
"feat add feature"        # Missing colon
"wip: add feature"        # Invalid type
```

---

## Part 4: How Releases Work

### Dependency Cascading

When you have this structure:
```
CLI → depends on → Utils → depends on → Core
```

- Change Core only → All three packages bump
- Change Utils only → Utils and CLI bump
- Change CLI only → Only CLI bumps

### Version Progression Example

Starting point (all at 0.1.0):
```
After various commits over time:
core:  0.2.3  (2 features, 3 fixes)
utils: 0.5.1  (5 features, 1 fix)
cli:   0.3.7  (3 features, 7 fixes)
```

This divergence is normal and expected!

---

## Part 5: Troubleshooting

### Release PR Not Created?

1. Check ALL commits follow conventional format: `git log --oneline`
2. Verify GitHub Actions permissions are enabled
3. Check workflow: `gh run list --workflow=release-please.yml`
4. View errors: `gh run view [run-id] --log-failed`

### Wrong Version Bump?

- `fix:` should bump patch
- `feat:` should bump minor
- `feat!:` or `BREAKING CHANGE:` should bump major

### Multi-Package Release Issues?

For artifact upload with multiple packages, use package-specific outputs:
```yaml
if: ${{ steps.release.outputs['your-cli--release_created'] }}
# Use: ${{ steps.release.outputs['your-cli--tag_name'] }}
```

### Dependencies Not Cascading?

Check internal dependencies have BOTH path AND version:
```toml
# MUST have both!
my-lib = { path = "../lib", version = "0.1.0" }
```

---

## Part 6: Common Issues and Solutions

### Issue: Non-Conventional Commits in History
**Solution**: Start fresh with new git history (nuclear option shown above)

### Issue: GitHub Actions Can't Create PRs
**Solution**: Enable permissions in Settings → Actions → General

### Issue: release-plz vs release-please
**Solution**: Use release-please - it works with unpublished packages!

### Issue: Versions Not Updating in Cargo.lock
**Solution**: This is normal - Cargo.lock updates when you build after merge

---

## Working Example

This guide is based on the working example at:
https://github.com/joshrotenberg/release-test-project

Clone it to see:
- Proper configuration files
- Working GitHub Actions
- Multiple successful releases
- Dependency cascading in action

---

## Quick Checklist

- [ ] Workspace structure with `crates/` directory
- [ ] Internal deps have `path` AND `version`
- [ ] Created `release-please-config.json`
- [ ] Created `.release-please-manifest.json`
- [ ] Created `.github/workflows/release-please.yml`
- [ ] Enabled GitHub Actions permissions
- [ ] Using conventional commits
- [ ] Tested with a `fix:` or `feat:` commit

If all checked, release-please should work perfectly!