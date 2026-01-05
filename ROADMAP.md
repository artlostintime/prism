# 🗺️ Prism Development Roadmap

This document outlines planned features and enhancements for future releases.

## ✅ Version 0.2.0 (Previous Release)

- ✅ Core data processing pipeline
- ✅ Reverse scoring and scale computation
- ✅ Quality checks (straightlining, missing data, out-of-range)
- ✅ Statistical reporting (Cronbach's alpha, descriptives)
- ✅ Multi-format output (CSV, Excel, SPSS, R, JSON)
- ✅ CLI and GUI applications
- ✅ Self-installing CLI executable
- ✅ Comprehensive test suite (42 tests)
- ✅ Cross-platform support (Windows, Linux, macOS)

---

## ✅ Version 0.3.0 (Previous Release)

### 📚 Pre-built Scale Libraries ✅

**Status:** Completed | **Priority:** High | **Complexity:** Medium

Library of commonly used psychology scales with pre-configured settings.

**Features:**

- Template configs for popular scales:
  - PHQ-9 (Patient Health Questionnaire-9)
  - GAD-7 (Generalized Anxiety Disorder-7)
  - PSS (Perceived Stress Scale)
  - PANAS (Positive and Negative Affect Schedule)
  - BDI-II (Beck Depression Inventory)
  - BAI (Beck Anxiety Inventory)
- Citation information for each scale
- Scoring rules and reverse items pre-defined
- Optional normative data for comparison
- Quick command: `prism generate --scale PHQ-9`

**Technical Implementation:**

```
scales/
├── phq9.toml
├── gad7.toml
├── pss.toml
├── panas.toml
└── README.md (with citations)
```

**Benefits:**

- Researchers save time on config setup
- Reduces errors in scale scoring
- Standardized configurations across studies
- Easy citation tracking

---

### 📊 Longitudinal Data Support ✅

**Status:** Completed | **Priority:** High | **Complexity:** High

Support for multi-wave/repeated measures studies.

**Features:**

- ✅ Merge data from multiple time points (T1, T2, T3...)
- ✅ Automatic ID matching across waves
- ✅ Wide vs. long format conversion
- ✅ Reliable change index calculations
- ✅ Growth curve modeling preparation

**Technical Implementation:**

```bash
# Merge multiple waves
prism merge --wave1 t1.csv --wave2 t2.csv --wave3 t3.csv -o merged.csv

# Convert to long format
prism reshape --wide merged.csv -o long.csv

# Calculate reliable change
prism rci --baseline t1.csv --followup t2.csv --scale anxiety
```

**New Config Options:**

```toml
[longitudinal]
id_column = "ParticipantID"
time_column = "Wave"
waves = ["T1", "T2", "T3"]
format = "wide"  # or "long"
```

**Benefits:**

- Handle longitudinal studies seamlessly
- Prepare data for repeated measures ANOVA
- Calculate change scores automatically
- Detect impossible time patterns

---

## ✅ Version 0.4.0 (Previous Release)

### 📈 Power Analysis Helper ✅

**Status:** Completed | **Priority:** Medium | **Complexity:** Medium

Integrated power analysis tools for study planning and evaluation.

**Features:**

- ✅ A priori power calculation (sample size planning)
- ✅ Post-hoc power analysis (observed power)
- ✅ Effect size estimation and interpretation
- ✅ Multiple test types (t-test, correlation)
- ✅ Export power analysis reports

**Technical Implementation:**

```bash
# A priori: Calculate required sample size
prism power --test t-test --effect-size 0.5 --power 0.80 --alpha 0.05

# Post-hoc: Calculate observed power
prism power --data results.csv --test correlation --observed-r 0.35

# Effect size from data
prism power --estimate-effect --group1 control.csv --group2 treatment.csv
```

**New Config Options:**

```toml
[power_analysis]
test_type = "independent_t"
effect_size = 0.5
power = 0.80
alpha = 0.05
tails = 2
```

**Benefits:**

- Plan studies with adequate power
- Justify sample sizes in grant applications
- Report power in methods sections
- Avoid underpowered studies

---

## ✅ Version 0.5.0 (Previous Release)

### 📊 Advanced SPSS Integration ✅

**Status:** Completed | **Priority:** High | **Complexity:** Medium

Comprehensive SPSS syntax generation with full transformation documentation.

**Features:**

- ✅ Enhanced SPSS syntax generator (.sps files)
- ✅ Comprehensive VARIABLE LABELS with reverse-scoring indicators
- ✅ VALUE LABELS for Likert scales (5-point, 7-point, custom)
- ✅ RECODE commands for reverse scoring transformations
- ✅ COMPUTE commands for scale totals and means
- ✅ Missing value declarations ($SYSMIS)
- ✅ Quality flag labels and descriptions
- ✅ DESCRIPTIVES and RELIABILITY examples
- ✅ Production-ready SPSS syntax output

**Technical Implementation:**

```bash
# Generate enhanced SPSS syntax
prism process --input data.csv --config study.toml --output results.csv --format spss
# Creates results.sps with complete SPSS transformation syntax
```

**Generated Syntax Includes:**

- GET DATA with UTF-8 encoding and proper delimiters
- Variable labels for all items and computed scales
- Value labels matching survey scale (1-5, 1-7, 0-10)
- Reverse scoring with RECODE commands
- Scale computation with MEAN() for missing data handling
- Missing value handling for out-of-range responses
- Example DESCRIPTIVES and RELIABILITY commands

**Benefits:**

- Copy-paste ready SPSS syntax
- Full transparency of all transformations
- Reproducible analysis pipeline
- Proper handling of missing data
- Professional SPSS output for publication

---

## 🚀 Version 0.6.0 (Current)

### 📋 Reproducibility Features ✅

**Status:** Completed | **Priority:** High | **Complexity:** Medium

Comprehensive R and Python analysis scripts for reproducible research.

**Features:**

- ✅ Enhanced R script generation with full analysis pipeline
- ✅ Python script generation with pandas/pingouin/matplotlib
- ✅ Reliability analysis (Cronbach's alpha with psych/pingouin)
- ✅ Descriptive statistics and data summaries
- ✅ Data visualization (distributions, box plots, correlation matrices)
- ✅ Quality check filtering and comparisons
- ✅ Export-ready summary tables
- ✅ Professional plot generation

**Technical Implementation:**

```bash
# Generate enhanced R analysis script
prism process --input data.csv --config study.toml --output results.csv --format r

# Generate Python analysis script
prism process --input data.csv --config study.toml --output results.csv --format python
```

**Generated Scripts Include:**

- Complete data import and cleaning pipeline
- Reliability analysis with Cronbach's alpha (psych::alpha or pingouin)
- Comprehensive descriptive statistics for all scales
- Distribution histograms with mean lines
- Box plots comparing quality flags
- Correlation matrices and heatmaps
- Export summary statistics tables
- Publication-quality visualizations (300 DPI)

**Benefits:**

- Reproducible analysis workflows
- Ready-to-run code for R or Python users
- No manual coding required for standard analyses
- Professional visualizations included
- Full transparency of analytical steps

---

## ✅ Version 0.7.0 (Current Release)

### 📊 Data Visualization & HTML Reports ✅

**Status:** Completed | **Priority:** High | **Complexity:** Medium

Interactive HTML reports with distribution plots and quality dashboards.

**Features:**

- HTML report generation with `--format html-report`
- Interactive Chart.js visualizations
- Distribution histograms for all scales
- Overview dashboard (participants, clean/flagged counts)
- Scale statistics table (M, SD, Min, Max, N)
- Quality issues dashboard with issue type breakdown
- Responsive, professional styling
- Print-friendly design
- Zero external dependencies (standalone HTML files)

**Benefits:**

- Instant visual data exploration
- Share with non-technical collaborators
- No coding required to view results
- Quality assessment at a glance
- Publication-ready charts

**Technical Implementation:**

- New `src/visualization.rs` module (508 lines)
- Chart.js via CDN for interactive charts
- Modern CSS with CSS variables for theming
- Histogram generation with automatic binning
- Quality badge system (clean vs. flagged)
- 9 comprehensive integration tests

---

## � Version 0.8.0 (In Progress)

### 🔍 Advanced Quality Checks (Partial)

**Status:** In Progress | **Priority:** High | **Complexity:** Medium

Enhanced quality validation with sophisticated pattern detection and comprehensive quality metrics.

**Completed Features:**

- ✅ Response time analysis (fast/slow completion detection)
- ✅ Diagonal pattern detection (ascending/descending sequences like 1,2,3,4,5)
- ✅ Alternating pattern detection (e.g., 1,5,1,5,1,5)
- ✅ Block pattern detection (uniform response blocks)
- ✅ 14 comprehensive pattern detection tests

**In Development:**

- 🔄 Semantic inconsistency checks (contradictory responses)
- 🔄 CONSORT flowchart data generation
- 🔄 Data dictionary export (CSV/JSON)
- 🔄 Enhanced HTML reports with pattern visualization
- 🔄 IP duplicate detection
- 🔄 Bot detection patterns

**Technical Implementation:**

- New quality check functions in `src/quality.rs`
- Integrated into processing pipeline
- Pattern detection algorithms with configurable thresholds
- Test suite: 149 tests (14 new pattern detection tests)

---

## �🚀 Version 0.8.0+ (Future)

### Survey Platform Integration

- Direct import from Qualtrics API
- SurveyMonkey, LimeSurvey support
- Automatic column mapping
- Real-time processing during data collection

### Advanced Visualizations

- Box plots for outlier detection
- Missing data heatmaps
- Correlation matrices with p-values
- Time series plots for longitudinal data

### Advanced Quality Checks

- Response time analysis
- IP duplicate detection
- Bot detection patterns
- Semantic inconsistency checks
- CONSORT flowchart generator
- Data dictionary export

---

## 💡 Feature Requests

Have an idea for Prism? We'd love to hear it!

**How to Submit:**

1. Check existing issues on GitHub
2. Use our [Feature Request template](.github/ISSUE_TEMPLATE/feature_request.md)
3. Describe your use case and expected behavior
4. Vote on existing requests with 👍

**Priority Criteria:**

- 🔥 High impact for researchers
- 🎯 Aligns with Prism's mission (preprocessing for SPSS/stats)
- 🛠️ Technically feasible
- 📊 Community demand (upvotes)

---

## 🤝 Contributing

Interested in implementing a roadmap feature?

1. Check the [CONTRIBUTING.md](CONTRIBUTING.md) guide
2. Comment on the relevant GitHub issue
3. Submit a pull request with tests
4. Update documentation

---

## 📅 Release Schedule

- **Minor versions (0.x.0):** Every 2-3 months with new features
- **Patch versions (0.x.y):** As needed for bug fixes
- **Major version (1.0.0):** When API is stable and battle-tested

---

**Last Updated:** January 5, 2026  
**Current Version:** 0.6.0
