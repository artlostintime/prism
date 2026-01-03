# 🚀 Release Checklist - Prism v0.2.0

## Pre-Release Tasks

### Code & Version

- [x] Update version to `0.2.0` in all files
  - [x] `Cargo.toml`
  - [x] `src-tauri/Cargo.toml`
  - [x] `src-tauri/tauri.conf.json`
- [x] Update `CHANGELOG.md` with all changes
- [x] Review all documentation for accuracy
- [ ] Run full test suite: `cargo test --release`
- [ ] Fix any failing tests

### Quality Assurance

- [ ] Test CLI on sample data
- [ ] Test GUI with various workflows
- [ ] Verify all keyboard shortcuts work
- [ ] Check error handling
- [ ] Test file picker functionality
- [ ] Verify output folder button works

### Documentation

- [x] `CHANGELOG.md` is complete
- [x] `BUILD_INSTRUCTIONS.md` created
- [x] `USER_ACCEPTANCE_TESTING.md` created
- [x] Performance benchmark suite created
- [ ] README.md updated with v0.2.0 features
- [ ] Screenshots updated (if applicable)

---

## Build Process

### Step 1: Build CLI

```bash
cd D:\Prism\prism
cargo build --release
```

**Verify:**

```bash
.\target\release\prism.exe --version
# Should output: prism 0.2.0

.\target\release\prism.exe --help
# Should show all commands
```

### Step 2: Test CLI

```bash
.\target\release\prism.exe process -i examples\sample_data.csv -c examples\study_config.toml -o test_output.csv
```

Expected: Clean output with no errors

### Step 3: Build GUI Installer

```bash
cd src-tauri
cargo tauri build
```

**This creates:**

- `src-tauri\target\release\bundle\msi\Prism_0.2.0_x64_en-US.msi`
- `src-tauri\target\release\bundle\nsis\Prism_0.2.0_x64-setup.exe`

### Step 4: Test Installer

1. Copy MSI to clean test environment
2. Run installer
3. Launch Prism from Start Menu
4. Test basic workflow:
   - Select CSV
   - Select/Edit config
   - Process data
   - Open output folder
5. Uninstall and verify clean removal

---

## Performance Benchmarks

### Run Benchmark Suite

```bash
cargo run --release --bin performance_benchmark
```

**Expected Results:**

- Small (100×50): < 1s
- Medium (1k×100): < 3s
- Large (10k×100): < 10s
- Very Large (50k×100): < 60s
- Wide (1k×500): < 5s

**If slower:** Investigate and optimize before release.

---

## Git & GitHub

### Commit & Tag

```bash
git add .
git commit -m "Release v0.2.0 - Modern GUI and UX improvements"
git tag v0.2.0
git push origin main
git push origin v0.2.0
```

### Create GitHub Release

1. Go to: `https://github.com/artlostintime/prism/releases/new`
2. Select tag: `v0.2.0`
3. Title: `v0.2.0 - Modern GUI & Performance`
4. Description: Copy from CHANGELOG.md
5. Upload files:
   - `Prism_0.2.0_x64_en-US.msi`
   - `Prism_0.2.0_x64-setup.exe`
   - `prism-v0.2.0-cli-windows-x64.zip` (optional)
6. Check "This is a pre-release" if applicable
7. Click "Publish release"

---

## Crates.io Publication (CLI Only)

### Prerequisites

```bash
# One-time setup
cargo login
# Paste API token from https://crates.io/me
```

### Pre-Publish Checks

```bash
# Verify package contents
cargo package --list

# Do a dry run
cargo publish --dry-run
```

### Publish

```bash
cargo publish
```

**Note:** This makes the CLI available via `cargo install prism`

---

## Distribution Channels

### Primary

- [x] GitHub Releases (Windows installers)
- [ ] Crates.io (CLI via `cargo install`)

### Future

- [ ] Winget (Windows Package Manager)
- [ ] Chocolatey
- [ ] Homebrew (macOS)
- [ ] Snap Store (Linux)
- [ ] Flathub (Linux)

---

## Post-Release Tasks

### Immediate

- [ ] Test download links work
- [ ] Install from GitHub release on clean system
- [ ] Announce release (if applicable)
- [ ] Update project website/docs

### Communication

- [ ] Update README badges (if any)
- [ ] Tweet/post about release
- [ ] Notify beta testers
- [ ] Update lab/research group

### Monitoring

- [ ] Watch for bug reports
- [ ] Monitor download counts
- [ ] Gather user feedback
- [ ] Track performance in real-world use

---

## Rollback Plan

If critical bugs found after release:

1. **Immediate:**

   - Add warning to release page
   - Document workaround if available

2. **Fix:**

   - Create hotfix branch
   - Fix issue
   - Test thoroughly
   - Release v0.2.1

3. **Communication:**
   - Update release notes
   - Notify users
   - Provide upgrade path

---

## Success Metrics

**v0.2.0 is successful if:**

- [ ] No critical bugs in first week
- [ ] GUI works on Windows 10/11
- [ ] Installation success rate > 95%
- [ ] User feedback is positive
- [ ] Performance meets benchmarks
- [ ] Documentation is clear

---

## Version 0.3.0 Planning

**Next features to consider:**

- Drag-and-drop file support
- Recent files menu
- Batch processing UI
- Dark mode theme
- Additional export formats (Stata, MATLAB)
- Cloud storage integration
- Multi-language support

---

## Quick Commands Reference

```bash
# Build everything
cargo build --release
cd src-tauri && cargo tauri build

# Test everything
cargo test --release
cargo run --release --bin performance_benchmark

# Clean build
cargo clean
cd src-tauri && cargo clean

# Check binary size
dir target\release\prism.exe

# Package for crates.io
cargo package

# Publish to crates.io
cargo publish
```

---

## Contact for Release Issues

- GitHub Issues: https://github.com/artlostintime/prism/issues
- Email: [your-email]
- Discord: [if applicable]

---

## Sign-Off

**Release Manager:** ******\_******  
**Date:** ******\_******  
**Version:** v0.2.0  
**Status:** ⬜ Ready ⬜ Not Ready

**Blockers:** **********************\_**********************

---

**Notes:**

---

---

---
