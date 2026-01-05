# Changelog

All notable changes to Prism will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.0] - 2026-01-05

### Added

- **🔬 Pre-built Scale Libraries** (Major Feature)

  - **8 Validated Psychology Scales** with ready-to-use configurations:

    - PHQ-9 (Patient Health Questionnaire-9) - Depression screening
    - GAD-7 (Generalized Anxiety Disorder-7) - Anxiety screening
    - PSS-10 (Perceived Stress Scale - 10 item)
    - PSS-14 (Perceived Stress Scale - 14 item original)
    - PANAS (Positive and Negative Affect Schedule)
    - BDI-II (Beck Depression Inventory-II)
    - BAI (Beck Anxiety Inventory)
    - SWLS (Satisfaction With Life Scale)

  - **Complete Scale Metadata**:

    - Full citation information for proper attribution
    - Scoring rules pre-configured (including reverse-scored items)
    - Normative data with means, SDs, and clinical cutoffs
    - Severity ranges for interpretation
    - Number of items and score ranges

  - **CLI Commands**:

    - `prism generate --list-scales` - View all available scales
    - `prism generate --scale PHQ-9` - Generate scale config instantly
    - `prism generate --scale-info GAD-7` - View detailed scale information
    - Case-insensitive scale names (PHQ-9, phq-9, PHQ9 all work)

  - **GUI Integration**:

    - New "Pre-built Psychology Scales" section in config templates
    - One-click loading of validated scale configurations
    - Scales dynamically loaded from Rust backend
    - Seamless integration with existing example templates

  - **Tauri Commands** (Backend):

    - `get_available_scales()` - Returns list of all scales
    - `get_scale_info(scale_id)` - Returns detailed metadata as JSON
    - `generate_scale_config(scale_id)` - Generates TOML config

  - **New Module**: `src/scales.rs` (850+ lines)
    - `ScaleMetadata` struct with comprehensive scale information
    - `NormativeData` struct for population statistics
    - Individual config generators for each scale
    - Metadata functions for each scale

### Changed

- Updated GUI modal title from "Configuration Examples" to "Configuration Templates"
- Enhanced example modal layout with separate sections for scales and templates
- Improved `init()` function to load scale library on startup

### Testing

- Added comprehensive test suite: `tests/scale_library_test.rs`
- 22 new tests covering:
  - Scale listing and generation
  - Case-insensitive scale names
  - Metadata retrieval
  - TOML validity for all configs
  - Required sections in all configs
  - Citation and interpretation presence
- **Total test count: 64 tests** (up from 42)

### Documentation

- Updated ROADMAP.md to mark pre-built scales as completed
- Updated version numbers to 0.3.0 across all manifests
- Enhanced scale configs with detailed comments and citations

### Dependencies

- Added `serde_json` to `src-tauri/Cargo.toml` for JSON serialization
- Added `prism` crate dependency to Tauri for scale library access

---

## [0.2.0] - 2026-01-03

### Added

- **Modern GUI with major UX improvements**

  - Animated progress bars with real-time status updates
  - CSV file info display (row/column count)
  - Config validation button with instant feedback
  - Four example configuration templates (Basic, Burnout, Alliance, Complex)
  - Open output folder button for direct file explorer access
  - Keyboard shortcuts (Ctrl+O, Ctrl+K, Ctrl+Enter)
  - Contextual tooltips on all sections
  - Help dialog with quick reference
  - Smooth animations and transitions throughout
  - Professional gradient design with improved color scheme

- **Backend enhancements**

  - `get_csv_info()` command for file statistics
  - `open_folder()` command for cross-platform file explorer integration
  - JSON serialization support with serde

- **Documentation**
  - UI_IMPROVEMENTS.md documenting all new features
  - Enhanced GUI_USAGE.md with screenshots and workflows
  - USER_ACCEPTANCE_TESTING.md with comprehensive test checklist

### Changed

- Upgraded UI from basic form to modern, polished interface
- Improved error messages with more context
- Better visual feedback for all user actions
- Enhanced config editor with validation and examples

### Fixed

- File picker opening in infinite loop when clicking "Choose Different" button
- CLI command invocation missing `process` subcommand
- Event propagation issues in file selection UI
- Tauri IPC serialization errors with custom structs

## [0.1.0] - 2025-12-15

### Added

- **Core Processing Pipeline**

  - Automated survey scoring with reverse-score support
  - Scale calculation (totals and means)
  - Multi-participant batch processing
  - CSV input/output with clean data export

- **Quality Control System**

  - Missing data detection with configurable thresholds
  - Straightlining detection (identical responses)
  - Low variance detection for response patterns
  - Out-of-range value validation
  - Comprehensive quality reporting

- **Statistical Analysis**

  - Descriptive statistics (mean, SD, min, max, N)
  - Cronbach's alpha reliability calculation
  - Per-scale statistical breakdowns
  - Summary statistics export

- **Multi-Format Export**

  - Clean CSV output with computed scales
  - Excel (.xlsx) with formatted output
  - JSON structured data export
  - SPSS syntax file generation
  - R script generation for import

- **CLI Interface**

  - `process` command for data processing
  - `validate` command for config checking
  - `generate` command for template creation
  - Progress bars and status indicators
  - Colored terminal output

- **Configuration System**

  - TOML-based configuration files
  - Survey metadata (name, score range)
  - Scale definitions with item lists
  - Reverse-scored item specification
  - Quality control thresholds
  - Output format preferences

- **Basic GUI**

  - File selection via drag-drop zones
  - Config editor with template support
  - Process button with status display
  - Output location preview
  - Error message display

- **Performance Optimizations**

  - Parallel processing with rayon
  - Pre-allocated data structures
  - Link-time optimization (LTO)
  - Optimized release builds
  - Minimal memory footprint

- **Testing**

  - Integration test suite
  - Property-based tests with proptest
  - Calculation accuracy tests
  - Config validation tests
  - Quality check tests

- **Documentation**
  - README.md with quick start guide
  - INSTALLATION.md for setup instructions
  - HOW_TO_USE.md with detailed examples
  - TUTORIAL.md for first-time users
  - API_REFERENCE.md for developers
  - ARCHITECTURE.md for system design
  - CONFIGURATION_GUIDE.md for setup
  - TROUBLESHOOTING.md for common issues
  - FAQ.md for quick answers

### Technical Details

- Built with Rust 2021 edition
- Tauri v2.9 for GUI framework
- Clap v4 for CLI parsing
- Serde for serialization
- CSV crate for data handling
- rust_xlsxwriter for Excel output

## Version History Summary

- **v0.2.0**: Major GUI overhaul with modern UX
- **v0.1.0**: Initial release with core functionality

---

## Categories

### Added

New features or functionality.

### Changed

Changes to existing functionality.

### Deprecated

Features that will be removed in future versions.

### Removed

Features that have been removed.

### Fixed

Bug fixes.

### Security

Security-related changes or fixes.

---

## Links

- [GitHub Repository](https://github.com/artlostintime/prism)
- [Documentation](./docs/)
- [Issue Tracker](https://github.com/artlostintime/prism/issues)
