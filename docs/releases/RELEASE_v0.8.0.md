# Prism v0.8.0 Release Notes

**Release Date:** January 6, 2026

## 🎉 Major New Features: Advanced Quality Checks

Version 0.8.0 introduces sophisticated data quality validation to detect careless responding patterns and ensure research data integrity. This release adds 8 comprehensive quality check categories with publication-ready reporting capabilities.

---

## ✨ What's New

### 🔍 Pattern Detection Algorithms

Automatically detect careless response patterns that indicate low-quality data:

**Diagonal Patterns**

- Detects sequential responses: 1,2,3,4,5 or 5,4,3,2,1
- Catches participants mindlessly clicking down or up the scale
- Requires minimum 4 items for detection

**Alternating Patterns**

- Identifies extreme alternating: 1,5,1,5,1,5
- Flags suspicious back-and-forth responding
- Minimum 4 items required

**Block Patterns**

- Detects uniform response blocks: 1,1,1,5,5,5
- Different from straightlining (not all same value)
- Requires minimum 6 items with distinct halves

**Response Time Validation**

- Flags suspiciously fast completion (<30 seconds)
- Identifies unusually slow responses (>300 seconds)
- Helps detect inattentive or interrupted responses

### 🧠 Semantic Inconsistency Detection

**New in v0.8.0!** Detect contradictory responses on theoretically-related scales:

- Check for high stress + high wellbeing (negative correlation expected)
- Flag mismatched engagement/satisfaction patterns (positive correlation expected)
- Configurable thresholds and scale ranges
- Supports both positive and negative expected correlations

```rust
// Programmatic usage
check_semantic_inconsistency(
    "stress", 6.5,
    "wellbeing", 6.8,
    "P001",
    "negative",  // Expected correlation
    1.0, 7.0,    // Scale range
    0.7,         // Threshold
    &mut flags, &mut issues
);
```

### 📖 Data Dictionary Export

Generate comprehensive variable documentation for reproducibility:

**CSV Format**

```bash
prism dictionary --config study.toml --output dictionary.csv
```

**JSON Format**

```bash
prism dictionary --config study.toml --output dictionary.json --format json
```

**Includes:**

- Participant ID documentation
- All survey items with scale membership
- Value ranges and reverse-scoring status
- Computed variables (totals, means)
- Quality check descriptions

**Perfect for:**

- Open science / data sharing initiatives
- Journal supplementary materials
- Collaborator onboarding
- Grant compliance (data management plans)

### 📊 CONSORT Flowchart Generation

Publication-ready participant flow reports following CONSORT guidelines:

**Text Format Example:**

```text
CONSORT Participant Flow Report
================================

Participants Screened
  n = 100

  ↓

Excluded (Quality Issues)
  n = 15 (15.0%)

  Exclusion Breakdown:
    - Missing data: 5 issue(s)
    - Straightlining: 4 issue(s)
    - Diagonal pattern: 3 issue(s)

  ↓

Final Analysis Sample
  n = 85 (85.0%)
```

**Features:**

- Tracks exclusions by reason with detailed breakdown
- Calculates retention and exclusion rates
- No double-counting (one participant = one exclusion)
- Both text and JSON formats
- CONSORT-compliant for journal submissions

### 🎨 Enhanced HTML Reports

HTML reports now include advanced visualizations:

**Doughnut Chart**

- Visual breakdown of quality issue types
- Interactive Chart.js visualization
- Color-coded by issue severity

**Pattern Detection Alert Box**

- Highlighted yellow warning section
- Lists all detected careless patterns
- Includes pattern descriptions

**Issue Statistics**

- Count of each quality check violation
- Participant-level flagging
- Clean vs. flagged summaries

---

## 📋 Complete Quality Check Categories

Prism v0.8.0 now includes **8 comprehensive quality checks**:

1. **Missing Data Detection** - Identifies excessive missing responses
2. **Straightlining Detection** - Catches uniform responses (all same value)
3. **Low Variance Detection** - Flags minimal response variation
4. **Diagonal Pattern Detection** - Sequential patterns (1,2,3,4,5)
5. **Alternating Pattern Detection** - Extreme alternating (1,5,1,5)
6. **Block Pattern Detection** - Uniform response blocks
7. **Response Time Validation** - Too fast (<30s) or slow (>300s)
8. **Semantic Inconsistency Detection** - Contradictory scale combinations

---

## 🔧 Technical Improvements

### New Functions

**src/quality.rs**

- `check_diagonal_pattern()` - Detects ascending/descending sequences
- `check_alternating_pattern()` - Identifies alternating responses
- `check_block_pattern()` - Detects uniform response blocks
- `check_semantic_inconsistency()` - Flags contradictory responses

**src/output.rs**

- `generate_data_dictionary_csv()` - CSV documentation
- `generate_data_dictionary_json()` - JSON documentation
- `generate_consort_report()` - Publication-ready text
- `generate_consort_json()` - Structured JSON

### Test Coverage

- **171 total tests** (all passing)
- 14 pattern detection tests
- 13 semantic inconsistency tests
- 8 CONSORT flowchart tests
- 136 previous tests maintained

### Performance

- Pattern detection algorithms optimized for large datasets
- Minimal performance impact on processing pipeline
- Efficient participant grouping for CONSORT reports

---

## 📚 Documentation Updates

- **README.md** - Added data dictionary section
- **ROADMAP.md** - Marked v0.8.0 as completed
- **Examples** - Added CONSORT and dictionary examples
- **API Documentation** - Comprehensive docstrings for all new functions

---

## 🎯 Use Cases

### Research Data Quality

- Detect careless responders before analysis
- Document data cleaning procedures transparently
- Provide justification for participant exclusions

### Publication Requirements

- CONSORT-compliant flowcharts for journals
- Data dictionaries for supplementary materials
- Reproducible quality check procedures

### Collaboration

- Share data dictionaries with team members
- Standardize quality check thresholds
- Document variable transformations

### Grant Compliance

- Meet funder data management requirements
- Demonstrate rigorous quality control
- Support open science initiatives

---

## 🚀 Getting Started

### Install

```bash
cargo install prism
```

### Process Data with Quality Checks

```bash
prism process -i data.csv -c config.toml -o clean.csv --quality-report quality.txt
```

### Generate Data Dictionary

```bash
prism dictionary --config config.toml --output dictionary.csv
```

### Generate HTML Report with Visualizations

```bash
prism process -i data.csv -c config.toml -o results.csv --format html-report
```

---

## 📦 Files Added

- `tests/pattern_detection_test.rs` - 14 pattern detection tests
- `tests/semantic_inconsistency_test.rs` - 13 semantic tests
- `tests/consort_test.rs` - 8 CONSORT tests
- `examples/pattern_test.csv` - Example pattern data
- `examples/pattern_test_config.toml` - Example configuration
- `examples/data_dictionary.csv` - Example CSV dictionary
- `examples/data_dictionary.json` - Example JSON dictionary
- `examples/consort_example.txt` - Example CONSORT report
- `examples/consort_example.json` - Example CONSORT JSON

---

## 🔄 Breaking Changes

None. All existing functionality preserved and backward compatible.

---

## 🐛 Bug Fixes

- Improved handling of `Option<Vec<String>>` in reverse-scoring checks
- Fixed doctest compilation issues
- Corrected JSON macro imports

---

## 📊 Statistics

- **Lines of Code Added:** ~1,800
- **New Functions:** 8
- **New Tests:** 35
- **Test Coverage:** 171 tests (100% passing)
- **Documentation:** 4 new sections in README

---

## 🙏 Acknowledgments

Thank you to the research community for feedback on data quality needs and CONSORT reporting requirements.

---

## 🔮 What's Next (v0.9.0)

Future enhancements planned:

- IP duplicate detection
- Bot detection patterns
- Geographic location validation
- Advanced attention check analysis
- Survey platform API integrations

---

## 📖 Full Changelog

See [CHANGELOG.md](CHANGELOG.md) for complete version history.

## 🆘 Support

- **Documentation:** https://github.com/artlostintime/prism/tree/main/docs
- **Issues:** https://github.com/artlostintime/prism/issues
- **Discussions:** https://github.com/artlostintime/prism/discussions

---

**Happy data processing! 🎉**
