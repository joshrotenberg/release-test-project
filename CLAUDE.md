# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Multi-workspace Rust project demonstrating automated releases using **release-please** (working solution). Previously attempted release-plz but it requires crates.io publication for unpublished packages.

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
cargo test test_name  # Run single test
```

### Code Quality
Before committing, always run:
```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
```

## Release Management with release-please

This project uses **release-please** for automated releases. The system automatically creates release PRs when conventional commits are pushed to main.

### Configuration Files

1. **release-please-config.json**: Multi-package configuration
2. **.release-please-manifest.json**: Version tracking (currently core: 0.2.0, utils: 0.4.1, cli: 0.3.1)
3. **.github/workflows/release-please.yml**: GitHub Action for automated releases
4. **cliff.toml**: Enhanced changelog formatting with emojis (optional)

### Release Workflow

1. Make changes in feature branch with conventional commits:
   - `feat:` → Minor version bump (0.1.0 → 0.2.0)
   - `fix:` → Patch version bump (0.1.0 → 0.1.1) 
   - `feat!:` or `BREAKING CHANGE:` → Major version bump (0.1.0 → 1.0.0)
2. Merge to main → release-please creates PR automatically
3. Merge release PR → Creates GitHub releases and tags

### Verifying Release Status
```bash
gh run list --workflow=release-please.yml  # Check workflow runs
gh pr list  # Check for release PRs
gh release list  # View created releases
```

## Code Architecture

Multi-workspace structure (Rust 2024 edition):
- **crates/core/** (v0.2.0): Core library with `DataModel` and business logic
- **crates/utils/** (v0.4.1): Utility functions (`format_data`, `serialize_data`, statistics functions) - depends on core
- **crates/cli/** (v0.3.1): CLI application with `process` and `format` commands - depends on both core and utils

### Workspace Dependencies
- Shared dependencies in root `Cargo.toml` under `[workspace.dependencies]`:
  - anyhow 1.0, thiserror 2.0, serde 1.0, clap 4.5
- Internal dependencies use both path and version: `{ path = "../core", version = "0.2.0" }`
- Version updates cascade through dependent packages automatically

## Important Notes

- GitHub Actions must have permission to create PRs (Settings → Actions → General → "Allow GitHub Actions to create and approve pull requests")
- Follow conventional commits for automatic version bumping
- Tests exist in core and utils modules (use `#[cfg(test)]` mod tests pattern)
- Binary name is `release-test` from the CLI crate