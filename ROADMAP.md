# 🗺️ Prism Development Roadmap

This document outlines planned features and enhancements for future releases.

## ✅ Version 0.2.0 (Current)

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

## 🎯 Version 0.3.0 (Planned)

### 📚 Pre-built Scale Libraries

**Priority:** High | **Complexity:** Medium

Add a library of commonly used psychology scales with pre-configured settings.

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

### 📊 Longitudinal Data Support

**Priority:** High | **Complexity:** High

Support for multi-wave/repeated measures studies.

**Features:**

- Merge data from multiple time points (T1, T2, T3...)
- Automatic ID matching across waves
- Wide vs. long format conversion
- Time-based quality checks (e.g., impossible response times)
- Reliable change index calculations
- Growth curve modeling preparation

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

### 📈 Power Analysis Helper

**Priority:** Medium | **Complexity:** Medium

Integrated power analysis tools for study planning and evaluation.

**Features:**

- A priori power calculation (sample size planning)
- Post-hoc power analysis (observed power)
- Effect size estimation from existing data
- Multiple test types (t-test, ANOVA, correlation, regression)
- Export power analysis reports

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

## 🚀 Version 0.4.0+ (Future)

### Advanced SPSS Integration

- Native `.sav` file export
- SPSS syntax generator for all transformations
- Automatic variable labels and value labels
- COMPUTE commands for derived variables

### Survey Platform Integration

- Direct import from Qualtrics API
- SurveyMonkey, LimeSurvey support
- Automatic column mapping
- Real-time processing during data collection

### Data Visualization

- Distribution histograms
- Box plots for outlier detection
- Missing data heatmaps
- Quality issue dashboards

### Advanced Quality Checks

- Response time analysis
- IP duplicate detection
- Bot detection patterns
- Semantic inconsistency checks

### Reproducibility Features

- Generate R/Python analysis scripts
- APA-formatted methods section generator
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

**Last Updated:** January 3, 2026  
**Current Version:** 0.2.1
