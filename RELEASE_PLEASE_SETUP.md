# Complete Guide: Setting Up release-please for Rust Multi-Workspace Projects

This guide documents the **working** setup for automated releases in Rust multi-workspace projects using release-please. Unlike release-plz, this solution works perfectly with unpublished packages.

## Prerequisites

1. GitHub repository
2. Rust workspace with multiple crates
3. Conventional commits convention

## Step-by-Step Setup

### 1. Create release-please Configuration

Create `release-please-config.json` in your repository root:

```json
{
  "packages": {
    "crates/core": {
      "release-type": "rust",
      "package-name": "your-package-core",
      "component": "your-package-core"
    },
    "crates/utils": {
      "release-type": "rust", 
      "package-name": "your-package-utils",
      "component": "your-package-utils"
    },
    "crates/cli": {
      "release-type": "rust",
      "package-name": "your-package-cli",
      "component": "your-package-cli"
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
    {"type": "feat", "section": "Features", "hidden": false},
    {"type": "fix", "section": "Bug Fixes", "hidden": false},
    {"type": "perf", "section": "Performance Improvements", "hidden": false},
    {"type": "docs", "section": "Documentation", "hidden": false},
    {"type": "chore", "section": "Miscellaneous", "hidden": false}
  ]
}
```

### 2. Create Version Manifest

Create `.release-please-manifest.json` to track current versions:

```json
{
  "crates/core": "0.1.0",
  "crates/utils": "0.1.0", 
  "crates/cli": "0.1.0"
}
```

### 3. Create GitHub Actions Workflow

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
          
      # The following steps only run if a release was created
      - uses: actions/checkout@v4
        if: ${{ steps.release.outputs.releases_created }}
        
      - name: Setup Rust
        if: ${{ steps.release.outputs.releases_created }}
        uses: dtolnay/rust-toolchain@stable
        
      - name: Build release binaries
        if: ${{ steps.release.outputs.releases_created }}
        run: cargo build --release
        
      - name: Upload Release Artifacts
        if: ${{ steps.release.outputs.releases_created }}
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
        run: |
          # Upload binary to the release (if you have a CLI)
          gh release upload ${{ steps.release.outputs.tag_name }} \
            target/release/your-binary \
            --clobber || true
```

### 4. Enable GitHub Actions Permissions

**CRITICAL**: Go to your repository Settings → Actions → General:
1. Under "Workflow permissions" select "Read and write permissions"
2. ✅ Check "Allow GitHub Actions to create and approve pull requests"

Without this, you'll get: `GitHub Actions is not permitted to create or approve pull requests`

### 5. Set Up Workspace Dependencies

In your workspace `Cargo.toml`:

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
# Shared dependencies go here
```

For internal dependencies, use both path and version:

```toml
# In crates/utils/Cargo.toml
[dependencies]
your-package-core = { path = "../core", version = "0.1.0" }
```

## How It Works

1. **Push to main** with conventional commits (feat:, fix:, etc.)
2. **release-please creates a PR** with:
   - Version bumps based on commit types
   - Updated CHANGELOGs for each crate
   - Updated manifest file
3. **Merge the PR** to trigger:
   - Git tags creation
   - GitHub releases
   - Optional artifact uploads

## Version Bumping Rules

- `feat:` → Minor version bump (0.1.0 → 0.2.0)
- `fix:` → Patch version bump (0.1.0 → 0.1.1)
- `feat!:` or `BREAKING CHANGE:` → Major version bump (0.1.0 → 1.0.0)
- Dependencies cascade: if core bumps, packages depending on it also bump

## Common Issues & Solutions

### Issue 1: GitHub Actions Can't Create PRs
**Solution**: Enable "Allow GitHub Actions to create and approve pull requests" in Settings

### Issue 2: No Release PR Created
**Solution**: Ensure commits follow conventional format and are pushed to main branch

### Issue 3: Version Not Bumping
**Solution**: Check that commits have proper prefixes (feat:, fix:, etc.)

## Advantages Over release-plz

✅ **Works with unpublished packages** - No crates.io requirement
✅ **Handles monorepos well** - Designed for multi-package repos
✅ **Language agnostic** - Can mix Rust with other languages
✅ **Battle-tested** - Used by Google and many large projects
✅ **Automatic dependency updates** - Cascades version bumps

## Testing Your Setup

1. Create a feature branch
2. Make changes with conventional commits
3. Merge to main
4. Wait ~30 seconds for release PR
5. Merge release PR
6. Check releases and tags were created

## Example Workflow

```bash
# 1. Create feature branch
git checkout -b feat/new-feature

# 2. Make changes and commit
git add .
git commit -m "feat(utils): add new utility function"

# 3. Push and create PR
git push origin feat/new-feature
gh pr create --title "feat(utils): add new utility" --body "..."

# 4. Merge PR
gh pr merge --merge

# 5. Release PR appears automatically
# 6. Merge release PR to create releases
```

## Files Created by This Setup

```
.
├── release-please-config.json    # Configuration
├── .release-please-manifest.json # Version tracking
├── .github/
│   └── workflows/
│       └── release-please.yml    # GitHub Action
└── crates/
    ├── core/
    │   └── CHANGELOG.md          # Auto-generated
    ├── utils/
    │   └── CHANGELOG.md          # Auto-generated
    └── cli/
        └── CHANGELOG.md          # Auto-generated
```

## Troubleshooting

Run `gh run list --workflow=release-please.yml` to check workflow status
Run `gh pr list` to see if release PR was created
Check `gh release list` for created releases

## Migration from release-plz

1. Remove/disable release-plz workflow
2. Remove release-plz.toml and cliff.toml
3. Add release-please configuration as shown above
4. Push to main and watch the magic happen!

---

**Note**: This setup has been tested and confirmed working with Rust 2024 edition multi-workspace projects that are NOT published to crates.io.