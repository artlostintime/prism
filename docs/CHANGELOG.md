# Changelog

**[📚 Wiki Home](README.md)** | **[✅ Implementation Status](IMPLEMENTATION_STATUS.md)**

---

## [Unreleased]

### Planned Features

- Pattern responding detection (1-2-3-4 sequences)
- Speeding detection (requires timing data)
- Correlation matrices
- Advanced SPSS integration features
- Batch processing GUI enhancements
- Missing data imputation algorithms

---

## [0.8.8] - 2026-01-07

### Fixed

- **GUI Critical Fix - Tauri API Initialization**
  - Fixed Calculate Power and Merge Longitudinal buttons not responding
  - Added proper Tauri API loading with polling mechanism
  - Improved error handling for async Tauri operations
  - All Analysis Tools tab features now work reliably

---

## [0.3.0+] - 2026-01-05

### Added - Longitudinal Data Support 🔄

**New Module: `longitudinal`**

- ✅ Multi-wave data merging with automatic ID matching
- ✅ Wide-to-long format conversion for growth curve modeling
- ✅ Long-to-wide format conversion for repeated measures ANOVA
- ✅ Reliable Change Index (RCI) calculation with clinical interpretation
- ✅ Support for inner and outer join types
- ✅ Comprehensive error handling for longitudinal operations

**New CLI Commands:**

- `merge` - Merge multiple wave files by participant ID

  - Supports inner/outer join types
  - Handles missing participants gracefully
  - Validates wave specifications

- `reshape` - Convert between wide and long formats

  - Wide-to-long: `var_T1, var_T2, var_T3` → multiple rows with `Time` column
  - Long-to-wide: Reverse transformation with automatic variable detection
  - Preserves all non-time-varying variables

- `rci` - Calculate Reliable Change Index
  - Formula: RCI = (X2 - X1) / SE_diff
  - SE*diff = SD * sqrt(2 \_ (1 - reliability))
  - Automatic clinical interpretation (Improved/Deteriorated/No reliable change)
  - Supports custom baseline SD for normative comparisons
  - Critical value: ±1.96 (p < .05)

**Configuration:**

- Added optional `[longitudinal]` section to TOML config
- Supports wave specifications: `wave_name:file_path`
- Configurable reliability coefficients for RCI calculations

**Documentation:**

- Updated [README.md](../README.md) with longitudinal examples
- Extended [API_REFERENCE.md](API_REFERENCE.md) with new module documentation
- Added longitudinal workflow to [HOW_TO_USE.md](HOW_TO_USE.md)
- Updated [examples/README.md](../examples/README.md) with longitudinal examples

**Testing:**

- 11 new integration tests for longitudinal features
- Test coverage for merge, reshape, and RCI operations
- Error handling tests for invalid inputs
- Windows path compatibility testing

**Performance:**

- Efficient CSV parsing with csv crate
- Memory-efficient processing for large longitudinal datasets
- Streaming I/O for minimal memory footprint

---

## [0.3.0] - 2026-01-03

### Added - Pre-built Scale Libraries 📚

**Scale Library System:**

- ✅ Comprehensive pre-built scales with citations and normative data
- ✅ PHQ-9 (Patient Health Questionnaire - Depression)
- ✅ GAD-7 (Generalized Anxiety Disorder Scale)
- ✅ PSS-10 and PSS-14 (Perceived Stress Scale)
- ✅ PANAS (Positive and Negative Affect Schedule)
- ✅ BDI-II (Beck Depression Inventory - II)
- ✅ BAI (Beck Anxiety Inventory)
- ✅ SWLS (Satisfaction With Life Scale)

**New CLI Commands:**

- `generate --list-scales` - List all available pre-built scales
- `generate --scale <NAME>` - Generate config for specific scale
- `generate --scale-info <NAME>` - Show detailed scale information with citations
- `generate --template` - Generate blank configuration template

**Scale Information:**

- Full citations with authors and publication years
- Scoring guidelines and interpretation ranges
- Test-retest reliability coefficients
- Internal consistency (Cronbach's alpha) values
- Normative data where available

**Documentation:**

- Added scale library documentation to README
- Examples for using pre-built scales
- Citation information for academic use

---

## [0.1.0] - 2026-01-02

### Added - Initial Release

**Core Features:**

- ✅ CSV input/output processing
- ✅ TOML configuration file support
- ✅ Automatic reverse scoring
- ✅ Scale total and mean calculation
- ✅ Aggregate statistics (M, SD, min, max, N)
- ✅ Cronbach's alpha calculation
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
