# Complete Workflow Example

**[📚 Wiki Home](README.md)** | **[📖 How to Use](HOW_TO_USE.md)** | **[🔧 Configuration](CONFIGURATION_GUIDE.md)** | **[⚡ Quick Ref](QUICK_REFERENCE.md)**

---

This document demonstrates a complete end-to-end workflow using Prism.

---

## Scenario: Burnout Study

**Goal:** Process burnout survey data with 3 scales:

- Emotional Exhaustion (9 items)
- Depersonalization (5 items)
- Personal Accomplishment (8 items, all reverse-scored)

---

## Step 1: Prepare Your Data

### Raw CSV (`burnout_data.csv`)

```csv
id,Q1,Q2,Q3,Q4,Q5,Q6,Q7,Q8,Q9,Q10,Q11,Q12,Q13,Q14,Q15,Q16,Q17,Q18,Q19,Q20,Q21,Q22
P001,6,5,6,7,6,5,6,7,6,2,1,2,1,2,4,5,4,5,4,5,4,5
P002,3,3,3,3,3,3,3,3,3,7,7,7,7,7,2,2,2,2,2,2,2,2
P003,1,2,3,4,5,6,7,6,5,4,3,2,1,2,3,4,5,6,7,6,5,4
```

---

## Step 2: Create Configuration

### Config File (`burnout_config.toml`)

```toml
[survey]
name = "Maslach Burnout Inventory Study"
min_score = 1
max_score = 7

[quality]
max_missing_percent = 0.10  # Flag if >10% missing
flag_straightlining = true

[scales.emotional_exhaustion]
items = ["Q1", "Q2", "Q3", "Q4", "Q5", "Q6", "Q7", "Q8", "Q9"]
reverse_scored = []

[scales.depersonalization]
items = ["Q10", "Q11", "Q12", "Q13", "Q14"]
reverse_scored = []

[scales.personal_accomplishment]
items = ["Q15", "Q16", "Q17", "Q18", "Q19", "Q20", "Q21", "Q22"]
reverse_scored = ["Q15", "Q16", "Q17", "Q18", "Q19", "Q20", "Q21", "Q22"]
```

---

## Step 3: Run Prism

### Command

```bash
prism \
  --input burnout_data.csv \
  --config burnout_config.toml \
  --output clean_burnout.csv \
  --stats-output burnout_summary.txt \
  --quality-report burnout_quality.txt
```

### Output

```
Processing Survey: Maslach Burnout Inventory Study
Successfully processed 3 participants.
Output saved to: clean_burnout.csv
Summary statistics saved to: burnout_summary.txt
Quality report saved to: burnout_quality.txt
```

---

## Step 4: Review Outputs

### Output 1: `clean_burnout.csv`

```csv
id,Q1,Q2,Q3,Q4,Q5,Q6,Q7,Q8,Q9,Q10,Q11,Q12,Q13,Q14,Q15,Q16,Q17,Q18,Q19,Q20,Q21,Q22,emotional_exhaustion_total,emotional_exhaustion_mean,depersonalization_total,depersonalization_mean,personal_accomplishment_total,personal_accomplishment_mean,quality_flag
P001,6,5,6,7,6,5,6,7,6,2,1,2,1,2,4,5,4,5,4,5,4,5,54.00,6.00,8.00,1.60,36.00,4.50,OK
P002,3,3,3,3,3,3,3,3,3,7,7,7,7,7,2,2,2,2,2,2,2,2,27.00,3.00,35.00,7.00,48.00,6.00,Straightlining: emotional_exhaustion; Straightlining: depersonalization; Straightlining: personal_accomplishment
P003,1,2,3,4,5,6,7,6,5,4,3,2,1,2,3,4,5,6,7,6,5,4,39.00,4.33,12.00,2.40,36.00,4.50,OK
```

**Key Changes:**

- Added `emotional_exhaustion_total` and `_mean`
- Added `depersonalization_total` and `_mean`
- Added `personal_accomplishment_total` and `_mean` (reverse-scored)
- Added `quality_flag` column

**Reverse Scoring Example (Q15-Q22):**

- P001 Q15 = 4 → Reversed: (7 + 1) - 4 = 4
- P002 Q15 = 2 → Reversed: (7 + 1) - 2 = 6
- P003 Q15 = 3 → Reversed: (7 + 1) - 3 = 5

---

### Output 2: `burnout_summary.txt`

```
MASLACH BURNOUT INVENTORY STUDY - Summary Statistics
Generated: 2026-01-02 10:31:30

Total Participants: 3
Complete Responses: 3 (100.0%)

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

SCALE: emotional_exhaustion (9 items)
Items: Q1, Q2, Q3, Q4, Q5, Q6, Q7, Q8, Q9

  Mean (M)              = 4.44
  Standard Deviation    = 1.53
  Range                 = [3.00, 6.00]
  N                     = 3

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

SCALE: depersonalization (5 items)
Items: Q10, Q11, Q12, Q13, Q14

  Mean (M)              = 3.67
  Standard Deviation    = 2.83
  Range                 = [1.60, 7.00]
  N                     = 3

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

SCALE: personal_accomplishment (8 items)
Items: Q15*, Q16*, Q17*, Q18*, Q19*, Q20*, Q21*, Q22*  (* = reverse scored)

  Mean (M)              = 5.00
  Standard Deviation    = 0.87
  Range                 = [4.50, 6.00]
  N                     = 3

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

DATA QUALITY: 3 issues detected (see quality report for details)
```

**Interpretation:**

- Emotional Exhaustion: Moderate (M = 4.44 on 1-7 scale)
- Depersonalization: Low-Moderate (M = 3.67)
- Personal Accomplishment: Good (M = 5.00, higher is better)
- Low variability (SD) suggests homogeneous sample

---

### Output 3: `burnout_quality.txt`

```
DATA QUALITY REPORT
Generated: 2026-01-02 10:31:30

Total Participants: 3
Flagged Issues: 3

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Straightlining (3 occurrences):

  • Participant P002: Straightlining: emotional_exhaustion
  • Participant P002: Straightlining: depersonalization
  • Participant P002: Straightlining: personal_accomplishment

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

RECOMMENDATIONS:
• Review flagged participants manually
• Consider excluding straightliners from analysis
• Check out-of-range values for data entry errors
• Assess whether missing data is random or systematic
```

**Action Items:**

- P002 shows straightlining on all 3 scales
- Likely careless responding or satisficing
- Recommend: Exclude P002 from analysis or contact for re-test

---

## Step 5: Import to Statistical Software

### R Example

```r
# Load clean data
data <- read.csv("clean_burnout.csv")

# Filter out flagged participants
clean <- data[data$quality_flag == "OK", ]

# Basic statistics
summary(clean$emotional_exhaustion_mean)
summary(clean$depersonalization_mean)
summary(clean$personal_accomplishment_mean)

# Correlations
cor.test(clean$emotional_exhaustion_mean, clean$depersonalization_mean)

# T-test (example: comparing high vs low burnout)
t.test(emotional_exhaustion_mean ~ burnout_group, data = clean)
```

### SPSS Example

```spss
* Import clean data
GET DATA
  /TYPE=TXT
  /FILE="clean_burnout.csv"
  /DELIMITERS=","
  /FIRSTCASE=2
  /VARIABLES=
    id A10
    emotional_exhaustion_mean F8.2
    depersonalization_mean F8.2
    personal_accomplishment_mean F8.2
    quality_flag A50.

* Descriptive statistics
DESCRIPTIVES VARIABLES=emotional_exhaustion_mean depersonalization_mean personal_accomplishment_mean
  /STATISTICS=MEAN STDDEV MIN MAX.

* Reliability analysis (if you have item-level data)
RELIABILITY
  /VARIABLES=Q1 Q2 Q3 Q4 Q5 Q6 Q7 Q8 Q9
  /SCALE('Emotional Exhaustion') ALL
  /MODEL=ALPHA.
```

### Python Example

```python
import pandas as pd
import numpy as np
from scipy import stats

# Load data
df = pd.read_csv("clean_burnout.csv")

# Filter clean responses
clean_df = df[df['quality_flag'] == 'OK']

# Descriptive statistics
print(clean_df[['emotional_exhaustion_mean',
                'depersonalization_mean',
                'personal_accomplishment_mean']].describe())

# Correlation matrix
print(clean_df[['emotional_exhaustion_mean',
                'depersonalization_mean',
                'personal_accomplishment_mean']].corr())

# T-test example
high_burnout = clean_df[clean_df['emotional_exhaustion_mean'] > 5]
low_burnout = clean_df[clean_df['emotional_exhaustion_mean'] <= 3]
stats.ttest_ind(high_burnout['depersonalization_mean'],
                low_burnout['depersonalization_mean'])
```

---

## Step 6: Report Results

### Methods Section (Example)

```
Data Processing

Raw survey data were processed using Prism v0.1.0, an automated
psychology research data pipeline. The tool computed scale scores
for emotional exhaustion (9 items), depersonalization (5 items),
and personal accomplishment (8 items, reverse-scored).

Quality checks included straightlining detection and missing data
analysis. One participant (P002) was excluded due to straightlining
across all scales, indicating careless responding. Final sample:
N = 2 valid responses.
```

### Results Table (APA Format)

```
Table 1
Descriptive Statistics for Burnout Subscales

Scale                         M      SD    Range      N
───────────────────────────────────────────────────────
Emotional Exhaustion        5.17   1.30  4.33-6.00   2
Depersonalization          2.00   0.28  1.60-2.40   2
Personal Accomplishment    4.50   0.00  4.50-4.50   2
───────────────────────────────────────────────────────

Note. Scale range: 1-7. Higher scores indicate greater
emotional exhaustion and depersonalization. For personal
accomplishment, higher scores indicate greater sense of
accomplishment (reverse-scored).
```

---

## Advanced Use Cases

### Batch Processing Multiple Files

```bash
#!/bin/bash
# Process all CSV files in data/ directory

for file in data/*.csv; do
    basename=$(basename "$file" .csv)
    prism \
        --input "$file" \
        --config study_config.toml \
        --output "output/${basename}_clean.csv" \
        --stats-output "output/${basename}_stats.txt" \
        --quality-report "output/${basename}_quality.txt"
done
```

### Integration with Survey Platform

```bash
# Download from Qualtrics, process, upload to analysis server
qualtrics-export --survey-id SV_12345 --output raw_data.csv
prism --input raw_data.csv --config survey_config.toml --output clean_data.csv
rsync clean_data.csv analysis-server:/data/
```

### Automated Weekly Processing

```bash
# Cron job: Every Monday at 9 AM
0 9 * * 1 cd /research/burnout_study && prism --input latest_data.csv --config config.toml --output clean.csv --stats-output stats.txt
```

---

## Tips & Best Practices

### 1. Always Review Quality Reports

Don't blindly trust the data. Check:

- How many participants flagged?
- What types of issues?
- Are patterns systematic or random?

### 2. Keep Original Data Untouched

```bash
# Good: Separate raw and processed
data/
  raw/
    survey_responses.csv  # Never modify!
  processed/
    clean_data.csv        # Generated by Prism
```

### 3. Version Control Your Config

```bash
git add study_config.toml
git commit -m "Add reverse scoring for Q15-Q22"
```

### 4. Document Your Decisions

```toml
# In your config file, add comments:

[scales.personal_accomplishment]
items = ["Q15", "Q16", "Q17", "Q18", "Q19", "Q20", "Q21", "Q22"]
# All items reverse-scored per Maslach et al. (1996)
reverse_scored = ["Q15", "Q16", "Q17", "Q18", "Q19", "Q20", "Q21", "Q22"]
```

### 5. Sanity Check Results

After processing:

1. Spot-check a few participants manually
2. Verify scale means are within expected range
3. Compare aggregate stats to published norms
4. Check that reverse scoring worked (review sample rows)

---

## Troubleshooting

### Issue: "Item Q5 not found in CSV"

**Cause:** Config references column that doesn't exist
**Solution:** Check CSV headers match config item names exactly

### Issue: All scales showing "NA"

**Cause:** Column names mismatch or parsing errors
**Solution:**

```bash
# Check CSV headers
head -1 burnout_data.csv

# Verify config item names match
cat burnout_config.toml
```

### Issue: Unexpected reverse scoring results

**Cause:** Wrong items marked as reverse-scored
**Solution:** Review scale documentation, update config

### Issue: Too many straightlining flags

**Cause:** Legitimate scale usage (e.g., strongly agree to everything)
**Solution:** Review context. Not all straightlining is invalid.

---

## Summary

**Time Breakdown:**

- Manual processing (Excel): 45-60 minutes
- Prism automated processing: 30 seconds
- **Time saved: 99%**

**Quality Benefits:**

- Consistent calculations
- No copy-paste errors
- Automatic quality checks
- Reproducible workflow
- Audit trail via config

**Research Benefits:**

- Faster turnaround
- More reliable data
- Professional reporting
- Easy collaboration
- Scalable to large datasets
