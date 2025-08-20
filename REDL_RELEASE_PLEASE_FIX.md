# Fixing release-please Setup for redl

This document provides specific fixes for the redl repository's release-please configuration.

## Current Issues Found

1. **Missing version in internal dependency** (CRITICAL)
   - Location: `crates/cli/Cargo.toml` line 40
   - Current: `redis-commands = { path = "../redis-commands" }`
   - Should be: `redis-commands = { path = "../redis-commands", version = "0.5.0" }`

2. **Component naming inconsistency**
   - The CLI package is named "redl" but component is "cli"
   - This works but could be clearer

3. **Workspace version field**
   - Root Cargo.toml has `version = "0.1.0"` in workspace.package
   - This is outdated compared to actual crate versions (0.5.0)

## Immediate Fixes Needed

### Fix 1: Update Internal Dependency (MOST IMPORTANT)

Edit `crates/cli/Cargo.toml`:
```toml
# Line 40 - CHANGE FROM:
redis-commands = { path = "../redis-commands" }

# TO:
redis-commands = { path = "../redis-commands", version = "0.5.0" }
```

### Fix 2: Update Root Workspace Version

Edit root `Cargo.toml`:
```toml
[workspace.package]
version = "0.5.0"  # Update from 0.1.0 to match current versions
```

### Fix 3: Update release-please Configuration (Optional but Recommended)

Edit `release-please-config.json` to be consistent:
```json
{
  "packages": {
    "crates/cli": {
      "release-type": "rust",
      "package-name": "redl",
      "component": "redl"  // Changed from "cli" to match package name
    },
    "crates/redis-commands": {
      "release-type": "rust",
      "package-name": "redis-commands",
      "component": "redis-commands"
    }
  },
  // ... rest stays the same
}
```

## Commands to Apply Fixes

```bash
# 1. Create a branch for the fixes
git checkout -b fix/release-please-internal-deps

# 2. Apply the fixes to Cargo.toml files
# (Make the edits described above)

# 3. Test that everything still builds
cargo build
cargo test

# 4. Commit with conventional message
git add -A
git commit -m "fix: add version to internal redis-commands dependency for release-please

The redis-commands dependency in cli/Cargo.toml was missing a version field,
which can cause release-please to incorrectly handle version bumps and
dependency cascading.

Also updated workspace version to match current crate versions."

# 5. Push and create PR
git push -u origin fix/release-please-internal-deps
gh pr create --title "fix: add version to internal dependency for release-please" \
  --body "Fixes internal dependency declaration to include version for proper release-please operation"
```

## Why These Fixes Matter

1. **Missing version in path dependencies**: release-please needs both `path` and `version` to:
   - Properly update versions during releases
   - Handle dependency cascading (when redis-commands bumps, cli should too)
   - Generate correct Cargo.lock entries

2. **Component naming**: While not critical, having component match package name makes the tags clearer:
   - Current: `cli-v0.5.0`
   - Better: `redl-v0.5.0`

## Verification After Fixes

After merging the fix:
1. Make a small change to redis-commands
2. Commit with `fix:` or `feat:` prefix
3. Push to main
4. Verify release-please creates a PR that bumps BOTH crates (cascading)

## Good News

Your recent commits show release-please IS working! The main risk is that dependency cascading might not work correctly due to the missing version in the internal dependency.

## If You Want to Start Fresh (Nuclear Option)

Only if the above fixes don't work, here's how to completely reset:

```bash
# 1. Save your code
cp -r . ../redl-backup

# 2. Remove git history
rm -rf .git

# 3. Initialize fresh
git init
git branch -M main

# 4. First commit - infrastructure
git add release-please-config.json .release-please-manifest.json
git add .github/workflows/
git commit -m "build: add release-please configuration for automated releases"

# 5. Second commit - the code
git add .
git commit -m "feat: initial implementation of Redis REPL with command registry"

# 6. Force push to GitHub (THIS WILL DESTROY HISTORY)
git remote add origin https://github.com/redis-field-engineering/redl.git
git push -u origin main --force

# 7. Update GitHub settings
# Go to Settings → Actions → General
# ✅ "Allow GitHub Actions to create and approve pull requests"
```

## Summary

The redl project is VERY close to having perfect release-please setup. The main issue is the missing version in the internal dependency. Fix that first and test. The other issues are minor improvements.

Your commit history already shows good conventional commit usage, so you probably don't need to start fresh unless the dependency fix doesn't resolve your issues.