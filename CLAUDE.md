# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Multi-workspace Rust project demonstrating release-plz configuration for automated releases.

## Development Commands

### Build
```bash
cargo build
cargo build --release
```

### Run
```bash
cargo run --bin release-test
cargo run --release --bin release-test
```

### Test
```bash
cargo test --lib --all-features
cargo test --test '*' --all-features
cargo test  # Run all tests
```

### Code Quality
Before committing, always run:
```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
```

### Single Test Execution
```bash
cargo test test_name
cargo test --test integration_test_name
```

## Release Management with release-plz

### Local Testing Commands
```bash
# Check what would be released
release-plz update

# Update versions and changelogs locally
release-plz update --update-deps

# Generate changelog only
release-plz changelog
```

### Configuration Files

1. **release-plz.toml**: Main configuration for release-plz
2. **cliff.toml**: Git-cliff configuration for changelog generation
3. **.github/workflows/release-plz.yml**: Automated PR creation for releases
4. **.github/workflows/release.yml**: Automated binary releases on tags

## Code Architecture

Multi-workspace structure with three crates:
- `crates/core/`: Core library with data models and business logic
- `crates/utils/`: Utility functions that depend on core
- `crates/cli/`: CLI application that uses both core and utils

### Workspace Dependencies
- Shared dependencies defined in root `Cargo.toml` under `[workspace.dependencies]`
- Internal crate dependencies use path and version: `{ path = "../core", version = "0.1.0" }`

## Release-plz Setup for Multi-Workspace Projects

### Key Configuration Points

1. **Workspace-level settings** in release-plz.toml:
   - `changelog_update = true` for all packages
   - `dependencies_update = true` to update internal dependencies
   - `git_tag_name` format for consistent tagging

2. **Package-specific settings**:
   - Individual `changelog_path` for each crate
   - Optional `semver_check` for stricter versioning

3. **GitHub Actions Integration**:
   - Automatic PR creation on push to main
   - Release workflow triggered by version tags

### Common Issues and Solutions

1. **"Package not found in registry" error**: 
   - This happens with unpublished packages
   - Solution: Use `--allow-dirty` flag for local testing
   - For CI: Ensure CARGO_REGISTRY_TOKEN is set if publishing

2. **Version not bumping**:
   - Ensure commits follow conventional format (feat:, fix:, etc.)
   - Check that there are actual changes since last tag
   - Use `release-plz update --verbose` for debugging

3. **Multi-workspace dependencies**:
   - Internal dependencies must specify both path and version
   - Version updates cascade through dependent packages

## Important Notes

- Always run `cargo test` and `cargo clippy` after making changes
- Follow conventional commits for automatic version bumping
- Use Rust 2024 edition idioms
- Internal packages depend on each other; changes cascade