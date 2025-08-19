# Manual Release Process for Unpublished Crates

Since release-plz requires packages to be published to crates.io, here's a manual process for releases:

## Step 1: Update Versions Manually

```bash
# Check current versions
grep "^version" crates/*/Cargo.toml

# Update versions based on conventional commits
# - feat: minor bump (0.1.0 -> 0.2.0)
# - fix: patch bump (0.1.0 -> 0.1.1)
# - feat!: major bump (0.1.0 -> 1.0.0)
```

## Step 2: Update Changelogs

Create/update CHANGELOG.md in each crate:

```markdown
# Changelog

## [0.2.0] - 2025-08-19

### Features
- Add standard deviation calculation

### Bug Fixes
- ...
```

## Step 3: Update Dependencies

When bumping a dependency, update all crates that depend on it:

```toml
# If release-test-core goes from 0.1.0 to 0.2.0
# Update in crates/utils/Cargo.toml:
release-test-core = { path = "../core", version = "0.2.0" }
```

## Step 4: Commit Version Bumps

```bash
git add -A
git commit -m "chore: release version 0.2.0"
```

## Step 5: Create Tags

```bash
git tag release-test-core-v0.2.0
git tag release-test-utils-v0.2.0
git tag release-test-cli-v0.2.0
```

## Step 6: Push Everything

```bash
git push origin main
git push origin --tags
```

## Alternative: Use cargo-release

For local projects, `cargo-release` might be a better option:

```bash
cargo install cargo-release

# Dry run to see what would happen
cargo release patch --dry-run

# Actual release
cargo release patch --execute
```

## Script for Semi-Automation

```bash
#!/bin/bash
# release.sh

VERSION=$1
if [ -z "$VERSION" ]; then
    echo "Usage: ./release.sh <version>"
    exit 1
fi

# Update version in all Cargo.toml files
sed -i '' "s/^version = .*/version = \"$VERSION\"/" crates/*/Cargo.toml

# Update internal dependencies
sed -i '' "s/version = \"0.1.0\"/version = \"$VERSION\"/" crates/*/Cargo.toml

# Commit
git add -A
git commit -m "chore: release v$VERSION"

# Tag
git tag "release-test-core-v$VERSION"
git tag "release-test-utils-v$VERSION"
git tag "release-test-cli-v$VERSION"

# Push
git push origin main --tags

echo "Released version $VERSION"
```