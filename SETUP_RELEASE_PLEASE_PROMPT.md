# Prompt for Setting Up release-please in a Rust Multi-Workspace Project

Copy this entire prompt and paste it into Claude when you want to set up release-please for your Rust workspace project.

---

## The Prompt

I need help setting up release-please for automated releases in my Rust multi-workspace project. I'm starting with a fresh git history (removing .git and recreating the GitHub repo) to ensure all commits follow conventional commit format from the beginning.

**Important Context:**
- I'm starting fresh - will delete `.git` directory and recreate the GitHub repository
- All commits from the very first one MUST follow conventional commit format
- This replaces any existing release-plz setup which doesn't work with unpublished packages

Please help me:
1. Analyze my current workspace structure
2. Create the necessary release-please configuration files
3. Set up the GitHub Actions workflow
4. Guide me through the initial git setup with proper commit messages
5. Ensure proper dependency cascading between workspace members

Here's what I need you to do:

### Step 1: Analyze the Workspace

First, examine my workspace structure by checking:
- `Cargo.toml` in the root (to identify workspace members)
- Workspace structure (typically `crates/` directory with subdirectories for each crate)
- Each crate's `Cargo.toml` to understand dependencies
- Current versions of each crate

Expected structure:
```
.
├── Cargo.toml              # Workspace root
├── crates/
│   ├── core/              # Example: core functionality
│   │   ├── Cargo.toml
│   │   └── src/
│   ├── utils/             # Example: utility functions
│   │   ├── Cargo.toml
│   │   └── src/
│   └── cli/               # Example: CLI application
│       ├── Cargo.toml
│       └── src/
```

Root `Cargo.toml` should have:
```toml
[workspace]
resolver = "2"
members = [
    "crates/core",
    "crates/utils",
    "crates/cli",
]

[workspace.package]
edition = "2024"  # or your edition
authors = ["Your Name <you@example.com>"]
license = "MIT OR Apache-2.0"
repository = "https://github.com/yourusername/yourrepo"

[workspace.dependencies]
# Shared dependencies across workspace
anyhow = "1.0"
thiserror = "2.0"
# ... other shared deps
```

### Step 2: Create release-please Configuration

Create a `release-please-config.json` file with:
```json
{
  "packages": {
    // For each crate in my workspace, add an entry like:
    "crates/[crate-name]": {
      "release-type": "rust",
      "package-name": "[actual-package-name-from-Cargo.toml]",
      "component": "[actual-package-name-from-Cargo.toml]"
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

### Step 3: Create Version Manifest

Create `.release-please-manifest.json` with current versions:
```json
{
  // For each crate, add:
  "crates/[crate-name]": "[current-version-from-Cargo.toml]"
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
          
      # Optional: Add steps to build and upload artifacts
      # The following only runs if a release was created
      - uses: actions/checkout@v4
        if: ${{ steps.release.outputs.releases_created }}
        
      - name: Setup Rust
        if: ${{ steps.release.outputs.releases_created }}
        uses: dtolnay/rust-toolchain@stable
        
      - name: Build release binaries
        if: ${{ steps.release.outputs.releases_created }}
        run: cargo build --release
        
      # If you have a CLI binary to upload, adjust this section:
      # For multi-package releases, use package-specific outputs
      # - name: Upload Release Artifacts
      #   if: ${{ steps.release.outputs['your-cli-package--release_created'] }}
      #   env:
      #     GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
      #   run: |
      #     gh release upload ${{ steps.release.outputs['your-cli-package--tag_name'] }} \
      #       target/release/your-binary-name \
      #       --clobber
```

### Step 5: Check Workspace Dependencies

Each crate's `Cargo.toml` needs proper configuration:

#### Individual Crate Cargo.toml Structure:
```toml
[package]
name = "your-package-name"  # Must match what's in release-please-config.json
version = "0.1.0"           # Must match .release-please-manifest.json
edition.workspace = true    # Inherit from workspace
authors.workspace = true
license.workspace = true
repository.workspace = true
description = "Description of this crate"

# For CLI crates with binaries:
[[bin]]
name = "your-binary-name"
path = "src/main.rs"

[dependencies]
# External deps can use workspace
anyhow.workspace = true

# CRITICAL: Internal deps MUST use both path AND version
your-core = { path = "../core", version = "0.1.0" }
your-utils = { path = "../utils", version = "0.1.0" }
```

**WRONG Internal Dependencies (will break releases):**
```toml
# Missing version - DON'T DO THIS
my-core = { path = "../core" }

# Missing path - DON'T DO THIS  
my-core = { version = "0.1.0" }

# Using * version - DON'T DO THIS
my-core = { path = "../core", version = "*" }
```

**CORRECT Internal Dependencies:**
```toml
# Always include both path and explicit version
my-core = { path = "../core", version = "0.1.0" }
```

The version should match the current version in that crate's Cargo.toml. release-please will automatically update these versions when creating releases.

### Step 6: Enable GitHub Permissions

**CRITICAL**: Tell me to go to my repository's Settings → Actions → General and:
1. Under "Workflow permissions" select "Read and write permissions"
2. ✅ Check "Allow GitHub Actions to create and approve pull requests"

### Step 7: Initialize Git with Proper Commits

Since we're starting fresh, let's set up git correctly from the beginning:

```bash
# Remove old git history (if exists)
rm -rf .git

# Initialize new repository
git init
git branch -M main

# Add release-please files first
git add release-please-config.json .release-please-manifest.json
git add .github/workflows/release-please.yml
git commit -m "build: add release-please configuration for automated releases"

# Add your existing code
git add .
git commit -m "feat: initial implementation of multi-workspace project"

# Create new GitHub repository (using gh CLI)
gh repo create [your-repo-name] --private --source=. --remote=origin --push
```

**CRITICAL**: Every commit from now on MUST use conventional format:
- Start with a type: `feat:`, `fix:`, `docs:`, `style:`, `refactor:`, `test:`, `chore:`, `build:`, `ci:`
- After the colon, add a space and then your description
- Use present tense ("add feature" not "added feature")

### Step 8: Test the Setup

After pushing to GitHub:
1. Create a test branch with a conventional commit (e.g., `fix: test release-please setup`)
2. Create and merge a PR to main
3. Wait ~30 seconds for release-please to create its PR
4. Check the PR has correct version bumps and changelog entries

### Important Notes

1. **Why Fresh Start Matters**: Non-conventional commits in history can cause release-please to:
   - Skip creating release PRs
   - Incorrectly calculate version bumps
   - Miss changes in the changelog
   - Get confused about what's been released
   Starting fresh ensures every commit is parseable by release-please

2. **Conventional Commits**: From now on, ALL commits must use conventional format:
   - `fix:` for patch bumps (0.1.0 → 0.1.1)
   - `feat:` for minor bumps (0.1.0 → 0.2.0)
   - `feat!:` or with `BREAKING CHANGE:` for major bumps (0.1.0 → 1.0.0)
   - Other types (`docs:`, `style:`, `refactor:`, `test:`, `chore:`) don't trigger releases but are included in changelogs

3. **Dependency Cascading**: When a lower-level crate changes, dependent crates will also bump versions automatically

4. **Multi-Package Releases**: Multiple packages can release together, each with its own tag

5. **No crates.io Required**: Unlike release-plz, this works perfectly with unpublished packages

### Common Commit Message Examples

Good commit messages that will work with release-please:
```bash
# Features (minor version bump)
git commit -m "feat: add new authentication system"
git commit -m "feat(auth): implement JWT token validation"

# Bug fixes (patch version bump)
git commit -m "fix: resolve memory leak in data processor"
git commit -m "fix(cli): correct argument parsing for --verbose flag"

# Breaking changes (major version bump)
git commit -m "feat!: redesign API to use async/await"
git commit -m "fix!: change config file format from TOML to JSON"

# Non-releasing but included in changelog
git commit -m "docs: update README with usage examples"
git commit -m "test: add integration tests for auth module"
git commit -m "refactor: simplify error handling logic"
git commit -m "style: apply rustfmt to all modules"
git commit -m "chore: update dependencies"
git commit -m "ci: add Windows to test matrix"
git commit -m "build: optimize release binary size"
```

BAD commit messages that will cause problems:
```bash
# These won't be recognized by release-please:
git commit -m "Updated README"  # No type prefix
git commit -m "Fix bug"         # No colon after type
git commit -m "FEAT: add feature"  # Uppercase type (should be lowercase)
git commit -m "feat add feature"   # Missing colon
git commit -m "wip: add feature"   # Invalid type
```

### Troubleshooting

If release PR is not created:
- Check ALL commits since last release follow conventional format
- Check GitHub Actions permissions are enabled
- Verify conventional commit format with: `git log --oneline`
- Check workflow runs: `gh run list --workflow=release-please.yml`
- Check for errors: `gh run view [run-id] --log-failed`
- If you have non-conventional commits, you may need to start fresh

### Example Working Repository

Reference implementation: https://github.com/joshrotenberg/release-test-project

This repository demonstrates:
- Multi-workspace Rust project with 3 crates
- Proper release-please configuration
- Working GitHub Actions workflow
- Various types of releases (patch, minor, major)
- Dependency cascading

---

## Additional Context for Claude

When implementing this:
1. Read all the files mentioned to understand the current structure
2. Create all configuration files in one go
3. Ensure package names match exactly what's in Cargo.toml
4. Double-check internal dependency versions
5. Remind me about GitHub permissions if I haven't mentioned setting them
6. Suggest a test commit to verify the setup works

The goal is to have automated releases that:
- Create PRs with version bumps and changelogs
- Handle multi-package workspaces correctly
- Work without publishing to crates.io
- Properly cascade dependency updates