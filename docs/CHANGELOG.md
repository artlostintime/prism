# Changelog

**[📚 Wiki Home](README.md)** | **[✅ Implementation Status](IMPLEMENTATION_STATUS.md)**

---

## [Unreleased]

### Planned Features

- Cronbach's alpha calculation
- Pattern responding detection (1-2-3-4 sequences)
- Speeding detection (requires timing data)
- Correlation matrices
- SPSS syntax generation
- Batch processing GUI

---

## [0.1.0] - 2026-01-02

### Added - Initial Release

**Core Features:**

- ✅ CSV input/output processing
- ✅ TOML configuration file support
- ✅ Automatic reverse scoring
- ✅ Scale total and mean calculation
- ✅ Aggregate statistics (M, SD, min, max, N)
- ✅ Quality checks (straightlining, missing data, out-of-range)
- ✅ Summary statistics report generation
- ✅ Quality report generation
- ✅ Command-line interface (CLI)
- ✅ Graphical user interface (GUI) via Tauri

**CLI Arguments:**

- `-i, --input` - Input CSV file
- `-c, --config` - Configuration TOML file
- `-o, --output` - Output CSV file
- `--stats-output` - Summary statistics file
- `--quality-report` - Quality report file

**Quality Checks:**

- Straightlining detection (all identical responses)
- Missing data percentage calculation
- Out-of-range value detection
- Configurable thresholds

**Configuration Options:**

- Survey metadata (name, description)
- Scale definitions (items, reverse scoring)
- Quality settings (missing threshold)
- Response range validation (min/max values)

**Documentation:**

- Complete wiki with 15+ guides
- Installation guide
- Configuration guide
- Tutorial
- API reference
- Best practices
- Troubleshooting guide
- FAQ
- Quality checks guide
- Glossary
- Examples and workflow guides

**Project Structure:**

- Organized docs/, examples/, tests/ folders
- Comprehensive .gitignore
- MIT License
- README files in all major directories

### Technical Details

**Architecture:**

- Minimal wrapper GUI pattern (GUI calls CLI binary)
- Rust CLI with helper functions for code organization
- Sample standard deviation (n-1) for statistics

**Code Quality:**

- Main function reduced from 180 → 70 lines
- Extracted 6 helper functions
- Validation before processing
- Comprehensive error handling

**Statistics:**

- Mean: Arithmetic average
- SD: Sample standard deviation (n-1 denominator)
- Min/Max: Range of values
- N: Sample size

**Formula Reference:**

- Reverse scoring: `(max + min) - original_value`
- Sample SD: `sqrt(sum((x - mean)²) / (n - 1))`

### Performance

**Processing Speed:**

- 100 participants: < 1 second
- 1,000 participants: 1-2 seconds
- 10,000 participants: 5-10 seconds

**Binary Size:**

- CLI: ~2-5 MB (release build)
- GUI: ~10-15 MB (release build)

### Dependencies

```toml
[dependencies]
csv = "1.4.0"
serde = { version = "1.0.228", features = ["derive"] }
toml = "0.9.10"
clap = { version = "4.5.53", features = ["derive"] }
anyhow = "1.0.100"
chrono = "0.4"
```

### Known Limitations

- No Cronbach's alpha calculation (planned)
- No pattern responding detection (planned)
- No speeding detection (requires timing data)
- No correlation matrices (planned)
- No SPSS syntax generation (planned)
- GUI is minimal wrapper (full GUI features planned)

---

## Version History

### Pre-Release Development

**Phase 1: MVP (Initial Development)**

- Basic CSV processing
- Config parsing
- Reverse scoring
- Scale calculations

**Phase 2: Quality Checks**

- Straightlining detection
- Missing data analysis
- Out-of-range validation

**Phase 3: Reporting**

- Summary statistics generation
- Quality report generation
- Aggregate statistics

**Phase 4: GUI**

- Tauri integration
- Minimal wrapper implementation
- File picker dialogs

**Phase 5: Code Quality**

- Helper function extraction
- Validation improvements
- Error handling enhancements
- Code duplication elimination

**Phase 6: Documentation**

- Complete wiki creation
- User guides and tutorials
- API reference
- Best practices guide

**Phase 7: Project Organization**

- Folder restructuring
- .gitignore improvements
- License addition
- README updates

---

## Upgrade Guide

### From Development to 0.1.0

No migration needed for first release.

### Future Upgrades

Breaking changes will be documented here with migration instructions.

---

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for how to contribute to Prism.

---

## Links

- **Repository:** [GitHub](https://github.com/artlostintime/prism)
- **Issues:** [GitHub Issues](https://github.com/artlostintime/prism/issues)
- **Releases:** [GitHub Releases](https://github.com/artlostintime/prism/releases)

---

## Versioning

Prism follows [Semantic Versioning](https://semver.org/):

- **Major (X.0.0):** Breaking changes
- **Minor (0.X.0):** New features (backward compatible)
- **Patch (0.0.X):** Bug fixes

---

## Wiki Navigation

- [📚 Back to Wiki Home](README.md)
- [✅ Implementation Status](IMPLEMENTATION_STATUS.md)
- [🔄 Refactoring Notes](REFACTORING_NOTES.md)
- [📖 How to Use](HOW_TO_USE.md)

---

[⬆ Back to Top](#changelog) | [📚 Wiki Home](README.md)
