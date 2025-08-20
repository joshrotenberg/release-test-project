# release-test-project

A complete working example of automated releases for Rust multi-workspace projects using release-please.

## Installation

### Quick Install (Unix/macOS/WSL)

```bash
curl -fsSL https://raw.githubusercontent.com/joshrotenberg/release-test-project/main/install.sh | bash
```

Or with wget:
```bash
wget -qO- https://raw.githubusercontent.com/joshrotenberg/release-test-project/main/install.sh | bash
```

### Download Binaries

Pre-built binaries are available for:
- Linux (x86_64, aarch64)
- macOS (x86_64, aarch64, universal)
- Windows (x86_64)

Download from the [releases page](https://github.com/joshrotenberg/release-test-project/releases).

> **✅ Working Solution**: `release-please` works perfectly with unpublished packages  
> **❌ Failed Attempt**: `release-plz` requires crates.io publication  
> **📖 Complete Guide**: [RELEASE_PLEASE_SETUP.md](RELEASE_PLEASE_SETUP.md) has step-by-step instructions

## Quick Start for Claude

To replicate this setup in another Rust workspace:

1. Copy these files to your project:
   - `release-please-config.json` - Multi-package configuration
   - `.release-please-manifest.json` - Version tracking
   - `.github/workflows/release-please.yml` - GitHub Action
   - `cliff.toml` - Enhanced changelog formatting (optional)

2. Update package paths in `release-please-config.json`

3. Enable GitHub Actions permissions:
   - Settings → Actions → General
   - ✅ "Allow GitHub Actions to create and approve pull requests"

4. Start using conventional commits (`feat:`, `fix:`, etc.)

## Current Setup

### Structure
```
├── crates/
│   ├── core/        # Core library (v0.2.0)
│   ├── utils/       # Utilities (v0.4.1) - depends on core
│   └── cli/         # CLI app (v0.3.1) - depends on both
├── .github/
│   └── workflows/
│       └── release-please.yml    # Automation workflow
├── release-please-config.json    # Release configuration
├── .release-please-manifest.json # Version tracking
└── cliff.toml                     # Changelog formatting
```

### Key Files

#### `release-please-config.json`
Configures how releases work - package paths, changelog sections, versioning strategy.

#### `.release-please-manifest.json`
Tracks current version of each package.

#### `.github/workflows/release-please.yml`
GitHub Action that runs on push to main, creates release PRs automatically.

#### `cliff.toml` (Optional)
Enhanced changelog formatting with emojis and better organization.

## How It Works

1. **Make changes** in a feature branch
2. **Use conventional commits**:
   - `feat:` → Minor version bump (0.1.0 → 0.2.0)
   - `fix:` → Patch version bump (0.1.0 → 0.1.1)
   - `feat!:` or `BREAKING CHANGE:` → Major version bump (0.1.0 → 1.0.0)
3. **Merge to main** → release-please creates a PR
4. **Merge release PR** → Creates GitHub releases and tags

## Version Bumping Examples

| Commit | Effect on Version | Example |
|--------|------------------|---------|
| `fix: handle NaN values` | Patch: 0.4.0 → 0.4.1 | Bug fixes |
| `feat: add new function` | Minor: 0.4.0 → 0.5.0 | New features |
| `feat!: change API` | Major: 0.4.0 → 1.0.0 | Breaking changes |

## Recent Releases

- **v0.4.1** - Bug fix: Handle NaN values in statistics
- **v0.4.0** - Feature: Add min/max functions
- **v0.3.0** - Feature: Add variance calculation
- **v0.2.0** - Feature: Add standard deviation

## Troubleshooting

### Release PR not created?
- Check GitHub Actions permissions are enabled
- Ensure commits follow conventional format
- Verify workflow is running: `gh run list --workflow=release-please.yml`

### Wrong version bump?
- Check commit message format
- Breaking changes need `!` or `BREAKING CHANGE:` in message

### Workflow failing?
- Ensure repository Settings allow Actions to create PRs
- Check `gh run view <run-id> --log-failed` for errors

## Documentation

- **[RELEASE_PLEASE_SETUP.md](RELEASE_PLEASE_SETUP.md)** - Complete setup guide
- **[MANUAL_RELEASE.md](MANUAL_RELEASE.md)** - Manual release process (fallback)

## Key Insights

1. **release-please works without publishing** - Unlike release-plz
2. **Conventional commits are essential** - They drive version bumping
3. **Dependencies cascade** - Updating core bumps dependent packages
4. **GitHub Actions need permissions** - Must allow PR creation

## Issues Documented

See the [Issues section](#issues-encountered--solutions) for problems we encountered and solved, including:
- release-plz failures with unpublished packages
- GitHub Actions permission issues
- Version bumping configuration

This is a fully working example - clone it, study the configuration, and adapt for your project!