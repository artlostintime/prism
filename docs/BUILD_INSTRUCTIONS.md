# Building Prism for Distribution

## Quick Start

### Build CLI Only

```bash
cargo build --release
```

Binary location: `target/release/prism.exe`

### Build GUI Application

```bash
cd src-tauri
cargo tauri build
```

Installers will be in: `src-tauri/target/release/bundle/`

---

## Platform-Specific Build Instructions

### Windows

#### Prerequisites

```powershell
# Install Rust
winget install Rustlang.Rustup

# Install Node.js (for Tauri)
winget install OpenJS.NodeJS

# Install Visual Studio Build Tools
winget install Microsoft.VisualStudio.2022.BuildTools
```

#### Build Process

```powershell
# Clone repository
git clone https://github.com/artlostintime/prism.git
cd prism

# Build CLI
cargo build --release

# Test CLI
.\target\release\prism.exe --version

# Build GUI
cd src-tauri
cargo tauri build

# Installers created:
# - src-tauri/target/release/bundle/msi/Prism_0.2.0_x64_en-US.msi
# - src-tauri/target/release/bundle/nsis/Prism_0.2.0_x64-setup.exe
```

#### Testing Installer

```powershell
# Test MSI installer
msiexec /i "src-tauri\target\release\bundle\msi\Prism_0.2.0_x64_en-US.msi"

# Verify installation
cd "C:\Program Files\Prism"
.\Prism.exe
```

---

### macOS

#### Prerequisites

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Xcode Command Line Tools
xcode-select --install
```

#### Build Process

```bash
# Clone repository
git clone https://github.com/artlostintime/prism.git
cd prism

# Build CLI
cargo build --release

# Test CLI
./target/release/prism --version

# Build GUI
cd src-tauri
cargo tauri build

# Application bundle created:
# - src-tauri/target/release/bundle/macos/Prism.app
# - src-tauri/target/release/bundle/dmg/Prism_0.2.0_x64.dmg
```

#### Testing Application

```bash
# Open app bundle
open src-tauri/target/release/bundle/macos/Prism.app

# Or mount DMG
open src-tauri/target/release/bundle/dmg/Prism_0.2.0_x64.dmg
```

---

### Linux

#### Prerequisites (Ubuntu/Debian)

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install dependencies
sudo apt update
sudo apt install -y \
    libwebkit2gtk-4.0-dev \
    build-essential \
    curl \
    wget \
    file \
    libssl-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev
```

#### Build Process

```bash
# Clone repository
git clone https://github.com/artlostintime/prism.git
cd prism

# Build CLI
cargo build --release

# Test CLI
./target/release/prism --version

# Build GUI
cd src-tauri
cargo tauri build

# Packages created:
# - src-tauri/target/release/bundle/deb/prism_0.2.0_amd64.deb
# - src-tauri/target/release/bundle/appimage/prism_0.2.0_amd64.AppImage
```

#### Testing Package

```bash
# Install DEB package
sudo dpkg -i src-tauri/target/release/bundle/deb/prism_0.2.0_amd64.deb

# Or run AppImage
chmod +x src-tauri/target/release/bundle/appimage/prism_0.2.0_amd64.AppImage
./src-tauri/target/release/bundle/appimage/prism_0.2.0_amd64.AppImage
```

---

## Build Configuration

### Optimized Release Build (Already Configured)

The `Cargo.toml` includes aggressive optimizations:

```toml
[profile.release]
opt-level = 3           # Maximum optimization
lto = "fat"             # Link-time optimization
codegen-units = 1       # Better optimization
strip = true            # Remove debug symbols
panic = 'abort'         # Smaller binary
```

**Expected binary sizes:**

- CLI: ~8-12 MB (stripped)
- GUI: ~15-25 MB (includes webview)

### Custom Build Options

#### Smaller Binary (Trade-off: Slower)

```bash
cargo build --release --config profile.release.opt-level='"z"'
```

#### Faster Compilation (Development)

```bash
cargo build
```

#### With Debug Info (For Profiling)

```bash
cargo build --release --config profile.release.strip=false
```

---

## Tauri Bundle Configuration

### Customize Installer (tauri.conf.json)

```json
{
  "bundle": {
    "active": true,
    "targets": ["msi", "nsis"], // Windows
    // "targets": ["dmg", "app"],   // macOS
    // "targets": ["deb", "appimage"],  // Linux
    "icon": [
      "icons/32x32.png",
      "icons/128x128.png",
      "icons/icon.icns",
      "icons/icon.ico"
    ],
    "windows": {
      "certificateThumbprint": null,
      "digestAlgorithm": "sha256",
      "timestampUrl": ""
    }
  }
}
```

---

## Automated Build Script

### build_all.ps1 (Windows PowerShell)

```powershell
#!/usr/bin/env pwsh

Write-Host "Building Prism v0.2.0..." -ForegroundColor Cyan

# Build CLI
Write-Host "`n[1/3] Building CLI..." -ForegroundColor Yellow
cargo build --release
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

# Test CLI
Write-Host "`n[2/3] Testing CLI..." -ForegroundColor Yellow
.\target\release\prism.exe --version

# Build GUI
Write-Host "`n[3/3] Building GUI..." -ForegroundColor Yellow
cd src-tauri
cargo tauri build
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
cd ..

Write-Host "`n✅ Build complete!" -ForegroundColor Green
Write-Host "Installers location: src-tauri\target\release\bundle\" -ForegroundColor Cyan
```

### build_all.sh (macOS/Linux)

```bash
#!/bin/bash
set -e

echo "Building Prism v0.2.0..."

# Build CLI
echo -e "\n[1/3] Building CLI..."
cargo build --release

# Test CLI
echo -e "\n[2/3] Testing CLI..."
./target/release/prism --version

# Build GUI
echo -e "\n[3/3] Building GUI..."
cd src-tauri
cargo tauri build
cd ..

echo -e "\n✅ Build complete!"
echo "Installers location: src-tauri/target/release/bundle/"
```

---

## Pre-Build Checklist

- [ ] Update version in `Cargo.toml`
- [ ] Update version in `src-tauri/Cargo.toml`
- [ ] Update version in `src-tauri/tauri.conf.json`
- [ ] Update `CHANGELOG.md`
- [ ] Run tests: `cargo test --release`
- [ ] Run benchmarks: `cargo bench`
- [ ] Update documentation
- [ ] Commit all changes
- [ ] Create git tag: `git tag v0.2.0`

---

## Post-Build Checklist

- [ ] Test CLI on target platform
- [ ] Test GUI installer
- [ ] Verify all output formats work
- [ ] Check file sizes are reasonable
- [ ] Test on clean system (no Rust installed)
- [ ] Verify uninstaller works
- [ ] Document known issues

---

## Code Signing (Optional)

### Windows

```powershell
# Sign the executable
signtool sign /f certificate.pfx /p password /t http://timestamp.digicert.com target\release\prism.exe
```

### macOS

```bash
# Sign and notarize
codesign --deep --force --verify --verbose --sign "Developer ID" target/release/bundle/macos/Prism.app
xcrun notarytool submit target/release/bundle/dmg/Prism_0.2.0_x64.dmg --wait
```

---

## Distribution

### GitHub Releases

1. Create new release: `https://github.com/artlostintime/prism/releases/new`
2. Tag: `v0.2.0`
3. Upload installers:
   - `Prism_0.2.0_x64_en-US.msi`
   - `Prism_0.2.0_x64-setup.exe`
   - `Prism_0.2.0_x64.dmg`
   - `prism_0.2.0_amd64.deb`
   - `prism_0.2.0_amd64.AppImage`
4. Include CLI binaries (optional)
5. Write release notes from CHANGELOG

### Crates.io (CLI Only)

```bash
# Login
cargo login

# Publish
cargo publish
```

---

## Troubleshooting Build Issues

### Error: Missing Dependencies (Linux)

```bash
sudo apt install libwebkit2gtk-4.0-dev build-essential curl wget file libssl-dev
```

### Error: Linker Not Found (Windows)

Install Visual Studio Build Tools with C++ support.

### Error: Cannot Find Tauri CLI

```bash
cargo install tauri-cli --version "^2.0"
```

### Error: Bundle Failed

Check `src-tauri/tauri.conf.json` for invalid configuration.

### Large Binary Size

- Ensure `strip = true` in `Cargo.toml`
- Check for unnecessary dependencies
- Use `cargo bloat --release` to analyze

---

## Performance Verification

After building, run benchmarks:

```bash
# Quick smoke test
./target/release/prism process -i examples/sample_data.csv -c examples/study_config.toml

# Full benchmark suite
cargo run --release --bin performance_benchmark

# Expected results:
# - Small (100 rows): < 1 second
# - Medium (1k rows): < 3 seconds
# - Large (10k rows): < 10 seconds
# - Very Large (50k rows): < 60 seconds
```

---

## CI/CD Integration (Future)

### GitHub Actions Example

```yaml
name: Build Release

on:
  push:
    tags:
      - "v*"

jobs:
  build:
    strategy:
      matrix:
        os: [windows-latest, macos-latest, ubuntu-latest]
    runs-on: ${{ matrix.os }}

    steps:
      - uses: actions/checkout@v3
      - uses: dtolnay/rust-toolchain@stable

      - name: Build CLI
        run: cargo build --release

      - name: Build GUI
        run: |
          cd src-tauri
          cargo tauri build

      - name: Upload Artifacts
        uses: actions/upload-artifact@v3
        with:
          name: installers-${{ matrix.os }}
          path: src-tauri/target/release/bundle/
```

---

## Support

For build issues:

1. Check troubleshooting section above
2. Review Tauri docs: https://tauri.app/
3. Open GitHub issue: https://github.com/artlostintime/prism/issues
