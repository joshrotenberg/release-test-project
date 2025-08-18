# release-test-project

A test project to demonstrate and debug `release-plz` configuration for Rust multi-workspace projects.

## Purpose

This repository serves as a working example of how to properly configure `release-plz` for automated releases in a Rust workspace with multiple interdependent crates.

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

```bash
# Check what would be released
release-plz update

# Create a release PR (requires GitHub token)
release-plz release-pr --git-token $GITHUB_TOKEN

# Local testing with verbose output
release-plz update --verbose
```

## Workflows

- **Push to main**: Triggers release-plz to create a PR with version bumps
- **Merge release PR**: Creates git tags and GitHub releases
- **Conventional commits**: Automatically determine version bumps (feat = minor, fix = patch, breaking = major)

## Common Issues Solved

1. ✅ Multi-workspace version management
2. ✅ Internal dependency updates
3. ✅ Unpublished crate handling
4. ✅ Automated changelog generation
5. ✅ GitHub Actions integration

## Resources

- [release-plz documentation](https://release-plz.ieni.dev/)
- [Conventional Commits](https://www.conventionalcommits.org/)