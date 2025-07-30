# Release and Publication Guide

This document describes the process for releasing new versions of the Nessus library to crates.io.

## Overview

Nessus uses a workspace-based project structure with two crates:
- `nessus-derive`: Procedural macros for deriving traits
- `nessus`: Main library that depends on `nessus-derive`

The publication process is automated through GitHub Actions but requires careful version management.

## Prerequisites

### 1. Repository Permissions

- Push access to the main repository
- Ability to create and push tags

### 2. Crates.io Setup

- A crates.io account with publishing permissions for both crates
- Generate an API token from [crates.io/me](https://crates.io/me)
- Add the token as a GitHub secret named `CRATES_IO_TOKEN`

### 3. Local Development Setup

```bash
# Ensure you have the latest stable Rust
rustup update stable

# Install helpful tools
cargo install cargo-edit   # For version bumping
cargo install cargo-audit  # For security audits
```

## Release Process

You can use either the **automated release script** (recommended) or follow the **manual process** below.

### Option A: Automated Release (Recommended)

Use the provided release script for a streamlined, safe release process:

```bash
./scripts/release.sh
```

**What the script does:**
1. ✅ Validates prerequisites (clean git status, correct branch)
2. ✅ Runs comprehensive tests (`cargo test`, `clippy`, `fmt`)
3. ✅ Prompts for version type (patch/minor/major) or custom version
4. ✅ Updates both `Cargo.toml` files automatically
5. ✅ Creates and pushes git tag
6. ✅ Triggers automated GitHub Actions release

**Interactive Example:**
```bash
🚀 Nessus Release Script
=======================

[INFO] Current version: 0.1.0

Version types:
  patch: 0.1.0 → 0.1.1
  minor: 0.1.0 → 0.2.0
  major: 0.1.0 → 1.0.0

Enter new version (or 'patch'/'minor'/'major'): minor
[INFO] New version will be: 0.2.0
Continue? (y/N): y

[SUCCESS] Release 0.2.0 has been initiated!
```

### Option B: Manual Release Process

If you prefer manual control, follow these steps:

### Step 1: Prepare the Release

1. **Ensure all changes are merged** to the main branch
2. **Run comprehensive tests** locally:
   ```bash
   cargo test --all-features --workspace
   cargo clippy --all-targets --all-features -- -D warnings
   cargo fmt --all -- --check
   ```

3. **Update documentation** if needed:
   ```bash
   cargo doc --no-deps --all-features
   ```

### Step 2: Version Management

Both crates must have synchronized versions. Update versions in:

- `Cargo.toml` (main crate)
- `nessus-derive/Cargo.toml` (derive crate)

#### For a patch release (0.1.0 → 0.1.1):

```bash
# Using cargo-edit
cargo set-version --workspace 0.1.1

# Or manually edit both Cargo.toml files
```

#### For a minor release (0.1.0 → 0.2.0):

```bash
cargo set-version --workspace 0.2.0
```

#### For a major release (0.1.0 → 1.0.0):

```bash
cargo set-version --workspace 1.0.0
```

### Step 3: Update Dependencies

Ensure `nessus-derive` dependency version is updated in the main `Cargo.toml`:

```toml
[dependencies]
nessus-derive = { version = "0.2.0", path = "./nessus-derive" }  # Update version here
```

### Step 4: Update Changelog

Update `CHANGELOG.md` (if it exists) or create release notes with:

- New features
- Bug fixes  
- Breaking changes (if any)
- Migration guide (for breaking changes)

### Step 5: Commit and Tag

1. **Commit version changes**:

```bash
git add .
git commit -m "chore: bump version to v0.2.0"
```

2. **Create and push the tag**:

```bash
git tag v0.2.0
git push origin main
git push origin v0.2.0
```

   **Important**: The tag format must be `v{MAJOR}.{MINOR}.{PATCH}` and must exactly match the versions in `Cargo.toml` files.

### Step 6: Monitor the Release

1. **Watch GitHub Actions**: Go to the Actions tab and monitor the release workflow
2. **Check crates.io**: Verify both crates appear at [crates.io](https://crates.io)
3. **Test installation**: Try installing the new version:

```bash
cargo install nessus --version 0.2.0
```

## Automated Release Workflow

The GitHub Actions workflow performs these steps:

1. **Version Validation**: Ensures the Git tag matches versions in both `Cargo.toml` files
2. **Testing**: Runs the full test suite before publishing
3. **Dry Run**: Performs a dry run to catch issues before actual publishing
4. **Sequential Publishing**: 
   - Publishes `nessus-derive` first
   - Waits for it to become available on crates.io
   - Then publishes `nessus`
5. **GitHub Release**: Creates a GitHub release with the tag

## Troubleshooting

### Release Script Issues

#### "Working directory is not clean" Error
```bash
git status  # See what's uncommitted
git add .   # Stage changes
git commit -m "your message"  # Or stash: git stash
```

#### "You are not on the main/master branch" Warning
The script warns if you're not on the main branch. You can:
- Switch to main: `git checkout main`  
- Continue anyway when prompted (not recommended)

#### Version Format Validation Error
Ensure version follows SemVer format:
- ✅ Valid: `1.0.0`, `0.2.1`, `1.0.0-beta.1`
- ❌ Invalid: `v1.0.0`, `1.0`, `latest`

### CI/CD Issues

#### Version Mismatch Error

```
Error: Tag version (0.2.0) does not match main crate version (0.1.0)
```

**Solution**: Ensure all `Cargo.toml` files have the same version as your Git tag.

#### Dependency Not Found

```
error: failed to select a version for the requirement `nessus-derive = "^0.2.0"`
```

**Solution**: The derive crate hasn't propagated to crates.io yet. The workflow waits 60 seconds and retries.

#### Publishing Permission Denied

```
error: failed to publish to registry at https://crates.io/
```

**Solutions**: 
- Verify your `CRATES_IO_TOKEN` is correct
- Ensure you're listed as an owner of both crates
- Check if the version already exists

### Manual Recovery

If the automated process fails, you can publish manually:

```bash
# Login with your token
cargo login YOUR_CRATES_IO_TOKEN

# Publish derive crate first
cd nessus-derive
cargo publish

# Wait a few minutes, then publish main crate
cd ..
cargo publish
```

## Security Considerations

- **API Token Security**: Never commit your crates.io token to the repository
- **Dependency Auditing**: Run `cargo audit` before releases
- **Code Review**: Ensure all changes are reviewed before release

## Version Numbering Guidelines

Follow [Semantic Versioning (SemVer)](https://semver.org/):

- **MAJOR**: Breaking changes that require user code modifications
- **MINOR**: New features that are backward compatible
- **PATCH**: Bug fixes that are backward compatible

### Breaking Changes

For breaking changes, provide:

1. Clear migration guide in release notes
2. Deprecation warnings in the previous version (when possible)
3. Updated examples and documentation

## Post-Release Tasks

1. **Update documentation**: Ensure docs.rs builds correctly
2. **Update examples**: Verify all examples work with the new version
3. **Social media**: Announce significant releases
4. **Issue triage**: Close issues that were fixed in the release

## Emergency Procedures

### Yanking a Release

If a critical issue is discovered:

```bash
cargo yank --version 0.2.0 nessus
cargo yank --version 0.2.0 nessus-derive
```

Then immediately prepare a patch release with the fix.

### Security Releases

For security issues:

1. Fix the issue privately
2. Coordinate with security team if needed
3. Release patch versions for all affected major versions
4. Publish security advisory after release

## Quick Reference

### Automated Release
```bash
# Recommended: Use the release script
./scripts/release.sh

# Follow prompts for version selection
# Script handles everything automatically
```

### Manual Release
```bash
# 1. Update versions
cargo set-version --workspace X.Y.Z

# 2. Update dependency version in main Cargo.toml
# Edit: nessus-derive = { version = "X.Y.Z", path = "./nessus-derive" }

# 3. Commit and tag
git add .
git commit -m "chore: bump version to vX.Y.Z"
git tag vX.Y.Z
git push origin main
git push origin vX.Y.Z
```

### Emergency Procedures
```bash
# Yank a problematic release
cargo yank --version X.Y.Z nessus
cargo yank --version X.Y.Z nessus-derive

# Then release a fix immediately
```
