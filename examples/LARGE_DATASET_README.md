# Large Test Dataset Documentation

## Overview

This directory contains a comprehensive large-scale test dataset designed to validate all features and functionality of the Prism statistical analysis tool.

## Dataset Specifications

### Size

- **Participants:** 620
- **Survey Items:** 61 per participant
- **Demographics:** 3 fields (age, gender, education)
- **Total Data Points:** ~38,000

### Generation

The dataset is generated using `examples/generate_large_dataset.py`, which creates realistic psychology survey data with controlled distributions and intentional quality issues.

## Included Scales

### 1. PHQ-9 (Patient Health Questionnaire - Depression)

- **Items:** 9 (phq1-phq9)
- **Scale Range:** 1-4 (originally 0-3, adjusted for consistency)
- **Reverse Scored:** None
- **Clinical Cutoffs:**
  - 5-9: Mild depression
  - 10-14: Moderate depression
  - 15-19: Moderately severe depression
  - 20-27: Severe depression

### 2. GAD-7 (Generalized Anxiety Disorder)

- **Items:** 7 (gad1-gad7)
- **Scale Range:** 1-4 (originally 0-3)
- **Reverse Scored:** None
- **Clinical Cutoffs:**
  - 5-9: Mild anxiety
  - 10-14: Moderate anxiety
  - 15-21: Severe anxiety

### 3. PSS-10 (Perceived Stress Scale)

- **Items:** 10 (pss1-pss10)
- **Scale Range:** 1-5
- **Reverse Scored:** pss4, pss5, pss7, pss8
- **Interpretation:** Higher scores = higher perceived stress

### 4. PANAS (Positive and Negative Affect Schedule)

- **Total Items:** 20
- **Positive Affect:** 10 items (pa1-pa10)
- **Negative Affect:** 10 items (na1-na10)
- **Scale Range:** 1-5
- **Reverse Scored:** None

### 5. Custom Wellbeing Scale

- **Items:** 15 (wb1-wb15)
- **Scale Range:** 1-7
- **Reverse Scored:** wb3, wb7, wb11
- **Subscales:**
  - Emotional (wb1-wb5): reverse scored wb3
  - Social (wb6-wb10): reverse scored wb7
  - Psychological (wb11-wb15): reverse scored wb11

### 6. Mental Health Composite

- **Purpose:** Cross-scale analysis
- **Combines:** PHQ-9 + GAD-7 + PSS-10
- **Items:** 26 total
- **Reverse Scored:** PSS items (pss4, pss5, pss7, pss8)

## Participant Type Distribution

### Normal/Healthy Profiles (250 participants)

- Low depression (PHQ-9: M=1.5, SD=0.6)
- Low anxiety (GAD-7: M=1.6, SD=0.6)
- Moderate stress (PSS-10: M=2.5, SD=0.9)
- Balanced affect

### Clinical Profiles (230 participants)

- **Depressed (80):** High PHQ-9, moderate GAD-7, moderate-high stress
- **Anxious (70):** High GAD-7, moderate PHQ-9, moderate-high stress
- **Stressed (50):** High PSS-10, moderate depression/anxiety
- **Positive (30):** High positive affect, low negative affect

### Quality Issue Cases (120 participants)

- **Straightlining (20):** All items same value on PHQ-9
- **Diagonal Ascending (20):** Sequential increasing pattern on GAD-7
- **Diagonal Descending (20):** Sequential decreasing pattern on PSS-10
- **Alternating Pattern (20):** Oscillating values on PANAS
- **Block Pattern (20):** Two distinct value clusters on Wellbeing
- **Excessive Missing (20):** 80% missing data across all scales

### Edge Cases (20 participants)

- **All Minimum Values (10):** Testing lower bounds
- **All Maximum Values (10):** Testing upper bounds

## Quality Issues Injected

The dataset intentionally includes realistic data quality problems to test detection capabilities:

| Issue Type        | Count | Description                               |
| ----------------- | ----- | ----------------------------------------- |
| Straightlining    | 20    | All responses identical on a scale        |
| Diagonal Patterns | 40    | Sequential increasing/decreasing values   |
| Alternating       | 20    | Values alternate between two extremes     |
| Block Patterns    | 20    | First half one value, second half another |
| Excessive Missing | 20    | >80% missing data                         |
| Edge Cases        | 20    | All min or all max values                 |

## Missing Data Patterns

### Realistic Missing Data (5% average)

- PHQ-9: ~5% random missing
- GAD-7: ~5% random missing
- PSS-10: ~3% random missing
- PANAS: ~2% random missing
- Wellbeing: ~4% random missing

### Excessive Missing Cases

- 20 participants with 80% missing data
- Tests max_missing_percent threshold

## Testing Prism Features

### ✓ Core Functionality

- Scale computation with multiple scales (11 total)
- Reverse scoring (PSS-10, Wellbeing scales)
- Cross-scale analysis (Mental Health Composite)
- Subscale analysis (PANAS, Wellbeing)

### ✓ Quality Checks

- **Straightlining detection:** 160 occurrences expected
- **Pattern detection:**
  - Diagonal patterns (ascending/descending)
  - Alternating patterns
  - Block patterns
- **Missing data flagging:** 257 occurrences expected
- **Variance checks:** Low variance detection

### ✓ Statistical Analysis

- Cronbach's Alpha for all scales
- Descriptive statistics (M, SD, Range)
- Sample size per scale
- Reliability assessment

### ✓ Edge Cases

- All missing data handling
- All identical values
- Extreme min/max values
- Zero variance scenarios

### ✓ Performance Testing

- Large dataset processing (620 participants)
- Multiple scale computation
- Parallel processing validation
- Throughput measurement

## Usage

### Generate Fresh Dataset

```bash
python examples/generate_large_dataset.py
```

### Run Prism Analysis

```bash
cargo run --release -- process \
  -i data/test_dataset_large.csv \
  -c test_large_config.toml \
  -o data/processed/test_large_output.csv \
  --stats-output data/processed/test_large_stats.txt \
  --quality-report data/processed/test_large_quality.txt
```

### Expected Performance

- **Processing Time:** ~0.03-0.10 seconds
- **Throughput:** 15,000-20,000 records/second
- **Clean Records:** ~44% (275/620)
- **Flagged Records:** ~56% (345/620)
- **Total Quality Issues:** ~950

## Output Files

### 1. test_large_output.csv

- Processed data with computed scale scores
- Quality flags per participant
- Total and mean scores for each scale

### 2. test_large_stats.txt

- Comprehensive descriptive statistics
- Cronbach's Alpha for each scale
- Sample sizes and distributions

### 3. test_large_quality.txt

- Detailed quality issue report
- Categorized by issue type
- Participant-level flags

## Validation Checklist

Use this dataset to verify:

- [ ] All 11 scales compute correctly
- [ ] Reverse scoring works (PSS-10, Wellbeing)
- [ ] Straightlining detection (20 cases)
- [ ] Diagonal pattern detection (40 cases)
- [ ] Alternating pattern detection (20 cases)
- [ ] Block pattern detection (20 cases)
- [ ] Missing data flagging (20 excessive cases)
- [ ] Cronbach's Alpha calculation
- [ ] Edge case handling (all min/max)
- [ ] Performance benchmarks met
- [ ] No crashes or panics
- [ ] Correct statistical output

## Demographics

The dataset includes realistic demographic data:

- **Age:** 18-75 years (random distribution)
- **Gender:** M, F, NB, Other
- **Education:** HS, Bachelor, Master, PhD

These can be used for future subgroup analysis features.

## Data Quality Metrics

### Expected Results

- **Total Participants:** 620
- **Clean Records:** ~275 (44.4%)
- **Flagged Records:** ~345 (55.6%)
- **Total Issues Detected:** ~950+

### Issue Breakdown

- Straightlining: ~160
- Missing Data: ~257
- Block Pattern: ~9
- Other patterns: Varies

### Cronbach's Alpha

Note: The generated data may show low alpha values (0.00) because:

1. Data is randomly generated (not real responses)
2. Quality issues are intentionally injected
3. This is expected behavior for synthetic data

For real-world applications, alpha values typically range 0.70-0.95.

## Regenerating Data

To create a new dataset with different random patterns:

```python
# Edit the seed in generate_large_dataset.py
random.seed(42)  # Change to any number

# Or remove the seed line for truly random data
```

## Notes

- **Seed:** Dataset uses seed=42 for reproducibility
- **Realistic Distributions:** Normal distributions for each participant type
- **Clinical Validity:** Cutoff scores match published literature
- **Extensibility:** Easy to add new scales or adjust parameters

## Future Enhancements

Potential additions:

- Longitudinal data (multiple timepoints)
- Subscale validation
- Response time data
- Item-level statistics
- Demographic subgroup analyses

---

**Generated:** January 6, 2026  
**Tool:** Prism v0.8.7  
**Purpose:** Comprehensive feature validation and performance testing
