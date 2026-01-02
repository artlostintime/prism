# Best Practices Guide

**[📚 Wiki Home](README.md)** | **[📖 How to Use](HOW_TO_USE.md)** | **[🔧 Configuration](CONFIGURATION_GUIDE.md)** | **[✅ Quality](QUALITY_CHECKS.md)**

---

## Overview

This guide provides recommendations for using Prism effectively in research workflows, from study design through publication.

---

## Table of Contents

1. [Study Design](#study-design)
2. [Data Collection](#data-collection)
3. [Config File Management](#config-file-management)
4. [Quality Control](#quality-control)
5. [Data Processing](#data-processing)
6. [File Organization](#file-organization)
7. [Reproducibility](#reproducibility)
8. [Reporting](#reporting)

---

## Study Design

### Plan Your Scales Early

✅ **DO:**

- Design scales with reverse-coded items
- Document intended scale structure
- Create config file during study design
- Test config with pilot data

❌ **DON'T:**

- Wait until data collection to plan scales
- Use all positively-worded items
- Create config after data cleaning

### Choose Appropriate Thresholds

**Missing data thresholds:**

| Study Type        | Recommended Threshold |
| ----------------- | --------------------- |
| Clinical trials   | 5%                    |
| Standard research | 10%                   |
| Exploratory       | 15-20%                |

**Example:**

```toml
[quality]
# Clinical study
missing_threshold = 5.0

# Standard research
# missing_threshold = 10.0

# Exploratory/pilot
# missing_threshold = 20.0
```

### Pre-Register Quality Criteria

**Include in pre-registration:**

```
Data Quality Exclusions:
1. Straightlining on ≥3 scales
2. >15% missing data
3. Failed attention checks
4. Out-of-range responses (data errors)
```

---

## Data Collection

### Survey Design

✅ **DO:**

- Limit survey to <20 minutes
- Include attention checks
- Force response on key items
- Randomize item order (when possible)
- Use progress bars

❌ **DON'T:**

- Create surveys >30 minutes
- Skip attention checks
- Allow complete skipping of scales
- Use monotonous item order

### Attention Checks

**Add to config:**

```toml
[scales.attention]
items = ["attention_1", "attention_2"]
# Flag if not expected values
```

**Example items:**

- "Please select 'Strongly Agree' for this item"
- "To show you are reading, select '4'"

### Export Settings

**Ensure your survey platform exports:**

- ✅ Numeric values (not text labels)
- ✅ Empty cells for missing (not "NA")
- ✅ Consistent column names
- ✅ Participant IDs in first column

---

## Config File Management

### Naming Convention

```
study_[study-name]_v[version].toml

Examples:
study_burnout_v1.toml
study_burnout_v2.toml
study_therapy-alliance_v1.toml
```

### Version Control

✅ **DO:**

- Keep configs in Git
- Document changes in comments
- Never modify configs after data collection starts
- Archive old versions

**Example versioning:**

```toml
# study_config_v2.toml
# Changes from v1:
# - Added attention check scale
# - Changed missing threshold from 10% to 15%
# - Fixed reverse scoring on WAI_10
# Last modified: 2026-01-15

[survey]
name = "My Study"
version = "2.0"
```

### Documentation

**Add comments liberally:**

```toml
[scales.burnout]
# Maslach Burnout Inventory - Emotional Exhaustion subscale
# Items: 9-item subset from MBI-HSS
# Scale: 1 (Never) to 7 (Every day)
items = ["ee1", "ee2", "ee3", "ee4", "ee5"]

# No items are reverse-scored in this scale
# (all measure presence of exhaustion)
```

### Templates

**Create templates for common designs:**

```
templates/
├── likert_5_point.toml
├── likert_7_point.toml
├── burnout_study.toml
└── therapy_alliance.toml
```

---

## Quality Control

### Progressive Quality Checks

**Stage 1: During Data Collection**

```bash
# Check first 20 responses
prism -i pilot_data.csv -c config.toml -o pilot_clean.csv \
  --quality-report pilot_quality.txt

# Review quality report
# Adjust survey if needed
```

**Stage 2: Mid-Collection**

```bash
# Check first 50% of data
# Look for systematic patterns
# Address any issues
```

**Stage 3: Final**

```bash
# Full sample quality check
# Document all exclusions
# Run sensitivity analyses
```

### Quality Review Workflow

**1. Generate reports:**

```bash
prism -i full_data.csv -c config.toml -o clean.csv \
  --stats-output stats.txt \
  --quality-report quality.txt
```

**2. Review quality.txt:**

- Count flagged participants
- Identify patterns
- Make exclusion decisions

**3. Document decisions:**

- Create exclusion log
- Save quality reports
- Note any judgment calls

**4. Reprocess if needed:**

```bash
# After removing problematic cases
prism -i data_filtered.csv -c config.toml -o final_clean.csv
```

---

## Data Processing

### Standard Workflow

**1. Organize files:**

```
project/
├── data/
│   ├── raw/
│   │   └── survey_export.csv
│   └── processed/
│       ├── clean_data.csv
│       ├── stats.txt
│       └── quality.txt
├── config/
│   └── study_config.toml
└── docs/
    └── processing_log.md
```

**2. Process:**

```bash
prism -i data/raw/survey_export.csv \
  -c config/study_config.toml \
  -o data/processed/clean_data.csv \
  --stats-output data/processed/stats.txt \
  --quality-report data/processed/quality.txt
```

**3. Verify:**

```bash
# Check output
head data/processed/clean_data.csv
cat data/processed/stats.txt
cat data/processed/quality.txt
```

### Batch Processing

**Multiple time points:**

```bash
# Bash
for wave in wave1 wave2 wave3; do
  prism -i data/raw/${wave}.csv \
    -c config/study_config.toml \
    -o data/processed/${wave}_clean.csv \
    --stats-output data/processed/${wave}_stats.txt \
    --quality-report data/processed/${wave}_quality.txt
done
```

**PowerShell:**

```powershell
$waves = @("wave1", "wave2", "wave3")
foreach ($wave in $waves) {
    prism -i "data\raw\$wave.csv" `
        -c "config\study_config.toml" `
        -o "data\processed\${wave}_clean.csv" `
        --stats-output "data\processed\${wave}_stats.txt" `
        --quality-report "data\processed\${wave}_quality.txt"
}
```

---

## File Organization

### Project Structure

**Recommended:**

```
project_name/
├── README.md
├── data/
│   ├── raw/              # Original exports (never modify!)
│   │   ├── wave1.csv
│   │   └── wave2.csv
│   ├── processed/        # Prism outputs
│   │   ├── wave1_clean.csv
│   │   ├── wave1_stats.txt
│   │   └── wave1_quality.txt
│   └── analysis/         # Final analysis datasets
├── config/
│   ├── study_config_v1.toml
│   └── templates/
├── scripts/
│   ├── 01_process_data.sh
│   ├── 02_quality_check.R
│   └── 03_analysis.R
├── docs/
│   ├── protocol.md
│   ├── processing_log.md
│   └── exclusions.md
└── outputs/
    ├── tables/
    └── figures/
```

### Naming Conventions

**Files:**

```
# Raw data
survey_[study]_[wave]_[date].csv
survey_burnout_t1_2026-01-15.csv

# Processed data
[study]_[wave]_clean.csv
burnout_t1_clean.csv

# Reports
[study]_[wave]_[type].txt
burnout_t1_stats.txt
burnout_t1_quality.txt
```

---

## Reproducibility

### Documentation

**Create processing_log.md:**

````markdown
# Data Processing Log

## Wave 1 Processing

**Date:** 2026-01-15
**Data file:** survey_export_2026-01-15.csv
**Config version:** study_config_v1.toml
**Prism version:** 0.1.0

**Command:**

```bash
prism -i data/raw/wave1.csv -c config/study_config_v1.toml \
  -o data/processed/wave1_clean.csv \
  --stats-output data/processed/wave1_stats.txt \
  --quality-report data/processed/wave1_quality.txt
```
````

**Results:**

- Total participants: 150
- Excluded: 8 (5.3%)
  - Straightlining: 3
  - Missing data: 4
  - Out-of-range: 1
- Final N: 142

**Notes:**

- Participant 023 excluded for straightlining on 4/5 scales
- Consider adding attention check for Wave 2

````

### Automated Pipeline

**Create script (process_data.sh):**
```bash
#!/bin/bash
# Data processing pipeline for [Study Name]

# Configuration
INPUT="data/raw/survey_export.csv"
CONFIG="config/study_config_v1.toml"
OUTPUT_DIR="data/processed"

# Process
echo "Processing data..."
prism -i "$INPUT" \
  -c "$CONFIG" \
  -o "$OUTPUT_DIR/clean_data.csv" \
  --stats-output "$OUTPUT_DIR/stats.txt" \
  --quality-report "$OUTPUT_DIR/quality.txt"

# Verify
echo "Verification:"
echo "Input rows: $(wc -l < $INPUT)"
echo "Output rows: $(wc -l < $OUTPUT_DIR/clean_data.csv)"

# Archive
cp "$OUTPUT_DIR/quality.txt" "docs/quality_report_$(date +%Y%m%d).txt"

echo "Processing complete!"
````

**Make executable:**

```bash
chmod +x scripts/process_data.sh
./scripts/process_data.sh
```

### Version Tracking

**Track versions:**

```bash
# In README.md
Software Versions:
- Prism: 0.1.0
- R: 4.3.0
- Python: 3.11.0

Data Processing:
- Config: study_config_v1.toml
- Processed: 2026-01-15
- Command: [see scripts/process_data.sh]
```

---

## Reporting

### Methods Section

**Example text:**

```
Data Processing and Quality Control

Survey data were processed using Prism (version 0.1.0), an
automated survey processing tool. Scale scores were computed
as the mean of constituent items, with reverse scoring applied
as specified in the Maslach Burnout Inventory manual.

Quality checks identified participants with: (a) straightlining
(identical responses to all items within ≥3 scales), (b) >15%
missing data, or (c) out-of-range responses indicating data
errors. Of 150 initial participants, 8 (5.3%) were excluded
based on these criteria, resulting in a final sample of N = 142.

Detailed scale definitions and processing code are available
at [repository URL].
```

### Results Section

**Descriptive statistics:**

```
Scale descriptive statistics and internal consistency are
presented in Table 1. All scales showed acceptable reliability
(Cronbach's α > .70). [Data from stats.txt]
```

**Quality summary:**

```
Data quality was generally high, with 94.7% of participants
meeting all inclusion criteria. Straightlining was detected in
2% of participants (n = 3), excessive missing data in 2.7%
(n = 4), and data errors in 0.7% (n = 1).
```

### CONSORT-Style Flowchart

```
Initial participants (N = 150)
    ↓
Excluded (n = 8, 5.3%):
- Straightlining (n = 3)
- Missing data (n = 4)
- Data errors (n = 1)
    ↓
Final sample (N = 142)
```

### Sharing Data

**Include in repository:**

1. ✅ Config files (with documentation)
2. ✅ Processing scripts
3. ✅ Example data (synthetic or sanitized)
4. ❌ Raw participant data (unless approved)

**Example README for data repository:**

```markdown
# [Study Name] - Data Processing

## Files

- `config/study_config.toml` - Scale definitions
- `scripts/process_data.sh` - Processing pipeline
- `data/example_data.csv` - Synthetic example data

## Reproducing Analyses

1. Install Prism (see INSTALLATION.md)
2. Run: `./scripts/process_data.sh`
3. Import `data/processed/clean_data.csv` to R/SPSS

## Contact

[Your email]
```

---

## Checklist

**Before data collection:**

- ☐ Config file created and tested
- ☐ Pilot data processed successfully
- ☐ Quality thresholds specified
- ☐ Exclusion criteria pre-registered

**During data collection:**

- ☐ Progressive quality checks
- ☐ Survey adjusted if needed
- ☐ Processing pipeline tested

**After data collection:**

- ☐ All data processed
- ☐ Quality report reviewed
- ☐ Exclusions documented
- ☐ Final dataset verified

**Before publication:**

- ☐ Methods section written
- ☐ Quality metrics reported
- ☐ Config file archived
- ☐ Processing code shared

---

## Wiki Navigation

- [📚 Back to Wiki Home](README.md)
- [📖 How to Use](HOW_TO_USE.md)
- [🔧 Configuration Guide](CONFIGURATION_GUIDE.md)
- [✅ Quality Checks](QUALITY_CHECKS.md)
- [📊 Workflow Examples](WORKFLOW_EXAMPLE.md)

---

[⬆ Back to Top](#best-practices-guide) | [📚 Wiki Home](README.md)
