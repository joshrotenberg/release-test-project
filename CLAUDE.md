# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Simple Rust project using Rust 2024 edition.

## Development Commands

### Build
```bash
cargo build
cargo build --release
```

### Run
```bash
cargo run
cargo run --release
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

## Code Architecture

Currently a minimal Rust application with:
- `src/main.rs`: Entry point with basic "Hello, world!" implementation
- Uses Rust 2024 edition
- No external dependencies

## Important Notes

- Always run `cargo test` and `cargo clippy` after making changes
- Follow Rust 2024 edition idioms
- Use `anyhow` for application errors, `thiserror` for library errors when adding error handling