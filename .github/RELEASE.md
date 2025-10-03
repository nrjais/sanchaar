# Release Process

This document describes how to create a new release for Sanchaar.

## Prerequisites

1. Ensure all tests pass: `cargo test`
2. Ensure the code is properly formatted: `cargo fmt`
3. Ensure there are no clippy warnings: `cargo clippy`
4. Update the version in `Cargo.toml` (workspace.package.version)
5. Update the CHANGELOG.md with release notes
6. Commit all changes

## Creating a Release

### 1. Tag the Release

Create and push a git tag with the version number:

```bash
git tag -a 0.1.0 -m "Release v0.1.0"
git push origin 0.1.0
```

The tag should follow semantic versioning: `MAJOR.MINOR.PATCH`

For pre-releases, append a suffix:
- Alpha: `0.1.0-alpha.1`
- Beta: `0.1.0-beta.1`
- Release Candidate: `0.1.0-rc.1`
- Prerelease: `0.1.0-prerelease.1`

### 2. Automated Build Process

Once the tag is pushed, GitHub Actions will automatically:

1. **Build for all platforms:**
   - Linux (x86_64 and aarch64)
     - AppImage (x86_64 only)
     - .tar.gz
     - .deb package
   - macOS (Intel and Apple Silicon)
     - .dmg installer
     - .tar.gz with .app bundle
   - Windows (x86_64)
     - .zip archive
     - .msi installer

2. **Generate checksums (SHA256)** for all artifacts

3. **Create a GitHub Release** with:
   - All build artifacts
   - SHA256 checksums
   - Automatically generated release notes

### 3. Verify the Release

After the workflow completes:

1. Check that all artifacts are present in the GitHub release
2. Download and test installers on different platforms
3. Verify checksums match

## Platform-Specific Notes

### Linux

- **AppImage**: Universal format that runs on most Linux distributions
- **.deb**: For Debian/Ubuntu-based distributions
- **.tar.gz**: For manual installation on any distribution

To install the .deb package:
```bash
sudo dpkg -i Sanchaar-*.deb
sudo apt-get install -f  # Install dependencies if needed
```

### macOS

- **.dmg**: Drag-and-drop installer for macOS
- **.tar.gz**: Contains the .app bundle for manual installation

**Important Note about macOS Security:**

The builds are ad-hoc signed but not notarized by Apple. Users will need to remove the quarantine attribute:
```bash
xattr -cr /Applications/Sanchaar.app
```

### Windows

- **.msi**: Standard Windows installer with Start Menu integration and PATH option
- **.zip**: Portable version for manual extraction

## Rollback

If you need to delete a release:

1. Delete the GitHub release
2. Delete the git tag:
```bash
git tag -d 0.1.0
git push origin :refs/tags/0.1.0
```

## Troubleshooting

### Build Failures

- Check the GitHub Actions logs for specific errors
- Common issues:
  - Missing dependencies (check CI.yml for required system packages)
  - Compilation errors (test locally with `cargo build --release`)
  - Icon/asset issues (ensure all referenced files exist)

### Manual Builds

If automated builds fail, you can build manually:

```bash
# Linux
cargo build --release --target x86_64-unknown-linux-gnu

# macOS Intel
cargo bundle --release --target x86_64-apple-darwin

# macOS Apple Silicon
cargo bundle --release --target aarch64-apple-darwin

# Windows
cargo build --release --target x86_64-pc-windows-msvc
```

For package formats:

```bash
# Debian package
cargo deb --target x86_64-unknown-linux-gnu

# Windows MSI
cargo wix --target x86_64-pc-windows-msvc
```
