# release-test-project

A test project demonstrating automated releases for Rust multi-workspace projects.

> **✅ Working Solution**: This repo uses `release-please` for automated releases.  
> **❌ Failed Attempt**: `release-plz` requires packages to be published to crates.io.  
> **📖 Setup Guide**: See [RELEASE_PLEASE_SETUP.md](RELEASE_PLEASE_SETUP.md) for step-by-step instructions.

## Purpose

This repository serves as a working example of automated releases in a Rust workspace with multiple interdependent crates.

**🎉 SOLUTION FOUND: Use `release-please` instead of `release-plz`!**

See [RELEASE_PLEASE_SETUP.md](RELEASE_PLEASE_SETUP.md) for complete setup instructions that work with unpublished packages.

## Structure

```
├── crates/
│   ├── core/        # Core library with data models
│   ├── utils/       # Utilities (depends on core)
│   └── cli/         # CLI application (depends on core and utils)
├── release-plz.toml # Release automation config
└── cliff.toml       # Changelog generation config
```

## Key Features

- Multi-workspace Rust project setup
- Automated version bumping with conventional commits
- Changelog generation per crate
- GitHub Actions workflows for CI/CD
- Internal dependency version management

## Testing Release-plz

### Basic Commands
```bash
# Check what would be released
release-plz update

# Create a release PR (requires GitHub token)
release-plz release-pr --git-token $GITHUB_TOKEN

# Local testing with verbose output
release-plz update --verbose
```

### Debugging Commands
```bash
# See detailed logs of what release-plz is doing
release-plz update --verbose

# Check current package versions
grep "^version" crates/*/Cargo.toml

# List all tags
git tag -l

# Check if commits follow conventional format
git log --oneline --since="last tag"

# Manually create initial tags (for first release)
git tag release-test-core-v0.1.0
git tag release-test-utils-v0.1.0
git tag release-test-cli-v0.1.0
git push origin --tags
```

## Workflows

- **Push to main**: Triggers release-plz to create a PR with version bumps
- **Merge release PR**: Creates git tags and GitHub releases
- **Conventional commits**: Automatically determine version bumps (feat = minor, fix = patch, breaking = major)

## Issues Encountered & Solutions

### 1. ❌ "Package not found in registry" Error
**Issue**: When running `release-plz update` with unpublished packages:
```
ERROR: package `release-test-core` not found in the registry, but the git tag release-test-core-v0.1.0 exists
```
**Cause**: release-plz expects packages to be published to crates.io
**Solutions**:
- Use GitHub Actions which handle this better in CI
- For local testing, create initial tags manually
- Consider `cargo-release` for purely local projects

### 2. ❌ Invalid Config Fields
**Issue**: Config parsing errors with fields like `registry` or `changelog_include_dependencies`
```
ERROR: invalid config file release-plz.toml - unknown field
```
**Cause**: These fields don't exist or are at wrong level
**Solution**: Keep config minimal, check docs for valid fields

### 3. ⚠️ No Version Bumps Detected
**Issue**: Running `release-plz update` shows "repository is already up-to-date"
**Causes**:
- No conventional commits since last tag
- No initial tags to compare against
- Changes not significant enough for version bump
**Solution**: Ensure commits follow conventional format (feat:, fix:, etc.)

### 4. ⚠️ No Upstream Configured Warning
**Issue**: `WARN: no upstream configured for branch master`
**Cause**: Local branch not tracking remote
**Solution**: Use `git push -u origin main` when pushing

### 5. ❌ Git Token Required for Release
**Issue**: `ERROR: git release not configured. Did you specify git-token and forge?`
**Cause**: release-plz needs GitHub token for creating releases
**Solution**: 
- Set `GITHUB_TOKEN` in CI/CD
- For local testing, use `--git-token` flag
- Disable git releases with `git_release_enable = false`

### 6. ⚠️ Workspace Dependencies Not Updating
**Issue**: Internal crate versions not bumping together
**Cause**: Missing `dependencies_update = true` in config
**Solution**: Enable in `[workspace]` section of release-plz.toml

### 7. ❌ Breaking Changes Not Detected
**Issue**: Major version not bumping on breaking changes
**Cause**: Commit message format incorrect
**Solution**: Use `feat!:` or include `BREAKING CHANGE:` in commit body

### 8. ❌ GitHub Actions Billing/Limits
**Issue**: "The job was not started because recent account payments have failed or your spending limit needs to be increased"
**Cause**: GitHub Actions requires billing setup for private repos or when limits exceeded
**Solutions**:
- Enable GitHub Actions in repository settings
- Add payment method to GitHub account
- Use free tier (public repos)
- Run release-plz locally instead

### 9. ❌ Release-plz Requires Published Packages (CRITICAL)
**Issue**: Even in CI, release-plz fails with unpublished packages
```
ERROR: package `release-test-core` not found in the registry, but the git tag release-test-core-v0.1.0 exists
```
**Cause**: release-plz is designed for packages published to crates.io
**This is a fundamental limitation - release-plz cannot work with unpublished packages**
**Solutions**:
- Publish packages to crates.io (not always possible/desired)
- Use alternative tools like `cargo-release` for local packages
- Create custom release scripts (see MANUAL_RELEASE.md)
- Switch to release-please (works without publishing!)

### 10. ❌ GitHub Actions Cannot Create PRs (release-please)
**Issue**: "GitHub Actions is not permitted to create or approve pull requests"
**Cause**: Default GITHUB_TOKEN has limited permissions
**Solutions**:
1. Go to Settings → Actions → General
2. Under "Workflow permissions" select "Read and write permissions"
3. Check "Allow GitHub Actions to create and approve pull requests"
4. Or create a Personal Access Token (PAT) with PR permissions

## Working Configuration Summary

✅ **What Works**:
- Multi-workspace version management (with caveats)
- Internal dependency updates
- Changelog generation per crate
- GitHub Actions integration
- Conventional commit parsing

❌ **What Doesn't Work (Locally)**:
- Unpublished packages without workarounds
- Local releases without GitHub token
- Registry-less operation

## Resources

- [release-plz documentation](https://release-plz.ieni.dev/)
- [Conventional Commits](https://www.conventionalcommits.org/)