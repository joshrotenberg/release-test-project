# Prompt for Setting Up release-please in a Rust Multi-Workspace Project

Copy this entire prompt and paste it into Claude when you want to set up release-please for your Rust workspace project.

---

## The Prompt

I need help setting up release-please for automated releases in my Rust multi-workspace project. This replaces any existing release-plz setup which doesn't work with unpublished packages.

Please help me:
1. Analyze my current workspace structure
2. Create the necessary release-please configuration files
3. Set up the GitHub Actions workflow
4. Ensure proper dependency cascading between workspace members
5. Fix any common issues

Here's what I need you to do:

### Step 1: Analyze the Workspace

First, examine my workspace structure by checking:
- `Cargo.toml` in the root (to identify workspace members)
- Each crate's `Cargo.toml` to understand dependencies
- Current versions of each crate

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

Ensure internal dependencies in each crate's `Cargo.toml` use BOTH path and version:
```toml
# CORRECT - uses both path and version
my-core = { path = "../core", version = "0.1.0" }

# WRONG - only path
my-core = { path = "../core" }
```

### Step 6: Enable GitHub Permissions

**CRITICAL**: Tell me to go to my repository's Settings → Actions → General and:
1. Under "Workflow permissions" select "Read and write permissions"
2. ✅ Check "Allow GitHub Actions to create and approve pull requests"

### Step 7: Test the Setup

After setting up:
1. Create a test branch with a conventional commit (e.g., `fix: test release-please setup`)
2. Merge it to main
3. Wait for release-please to create a PR
4. Check the PR has correct version bumps and changelog entries

### Important Notes

1. **Conventional Commits**: From now on, ALL commits must use conventional format:
   - `fix:` for patch bumps (0.1.0 → 0.1.1)
   - `feat:` for minor bumps (0.1.0 → 0.2.0)
   - `feat!:` or with `BREAKING CHANGE:` for major bumps (0.1.0 → 1.0.0)

2. **Dependency Cascading**: When a lower-level crate changes, dependent crates will also bump versions automatically

3. **Multi-Package Releases**: Multiple packages can release together, each with its own tag

4. **No crates.io Required**: Unlike release-plz, this works perfectly with unpublished packages

### Troubleshooting

If release PR is not created:
- Check GitHub Actions permissions are enabled
- Verify conventional commit format
- Check workflow runs: `gh run list --workflow=release-please.yml`
- Check for errors: `gh run view [run-id] --log-failed`

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