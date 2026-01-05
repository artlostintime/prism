# Release Notes

Official release notes for all Prism versions.

## Latest Release

**[v0.8.5](RELEASE_v0.8.5.md)** - Critical Statistical Correctness Fix (January 6, 2026)

- 🔴 CRITICAL: Fixed variance calculation bug in RCI analysis
- Complete statistical validation with mathematical proofs
- Production-ready with 173 tests passing

---

## Release History

### [v0.8.5](RELEASE_v0.8.5.md) - January 6, 2026

**Critical Bug Fix + Statistical Validation**

- Fixed population variance → sample variance bug in longitudinal RCI
- Comprehensive mathematical validation (2500+ lines)
- All formulas verified against peer-reviewed literature
- 18/18 library tests passing

### [v0.8.0](RELEASE_v0.8.0.md) - January 5, 2026

**Major Feature Release**

- Pre-built psychology scale libraries (PHQ-9, GAD-7, PSS, etc.)
- GUI integration for scale selection
- CLI commands for scale generation
- Complete scale metadata with citations

### [v0.3.0](RELEASE_v0.3.0.md) - January 5, 2026

**Scale Library Initial Release**

- 8 validated psychology scales
- Scale metadata and normative data
- Tauri commands for GUI integration

### [v0.2.0](RELEASE_v0.2.0.md)

**Early Release**

- Initial feature set
- Basic CLI functionality

---

## Quick Links

- **Current Version**: v0.8.5
- **Repository**: https://github.com/artlostintime/prism
- **Documentation**: [/docs](/docs)
- **Changelog**: [/CHANGELOG.md](/CHANGELOG.md)

---

## Installation

```bash
git clone https://github.com/artlostintime/prism.git
cd prism
git checkout v0.8.5
cargo build --release
```

See individual release notes for version-specific installation instructions and breaking changes.
