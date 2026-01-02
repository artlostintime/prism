# Quality Checks Guide

**[📚 Wiki Home](README.md)** | **[📖 How to Use](HOW_TO_USE.md)** | **[🔧 Configuration](CONFIGURATION_GUIDE.md)** | **[🎓 Tutorial](TUTORIAL.md)**

---

## Overview

Prism automatically checks your data quality during processing. This guide explains each quality check, how to interpret findings, and what actions to take.

---

## Table of Contents

1. [Straightlining Detection](#straightlining-detection)
2. [Missing Data Analysis](#missing-data-analysis)
3. [Out-of-Range Detection](#out-of-range-detection)
4. [Understanding Reports](#understanding-reports)
5. [Best Practices](#best-practices)
6. [Research Recommendations](#research-recommendations)

---

## Straightlining Detection

### What It Is

**Straightlining** occurs when a participant gives the **same response to every item** in a scale, suggesting inattentive responding.

### How It's Detected

Prism checks if all items in each scale have identical values:

```
Scale: burnout (5 items)
Participant 023: burn1=3, burn2=3, burn3=3, burn4=3, burn5=3
→ FLAGGED: All values identical
```

### When It's Flagged

```
=== STRAIGHTLINING DETECTED ===
Participant 023: All values identical in scale 'burnout' (all items = 3.0)
```

### Important Considerations

**Not always invalid:**

- **Floor effects:** Genuinely no symptoms (all 1s)
- **Ceiling effects:** Maximum symptoms (all 7s)
- **True consistency:** Legitimately uniform feelings

**Likely invalid when:**

- Combined with other quality flags
- Across multiple different scales
- Middle-point straightlining (all 4s on 1-7 scale)
- Pattern continues across entire survey

### What To Do

1. **Review context:**

   - Check other scales for same participant
   - Look at demographics/open-ended responses
   - Consider scale characteristics (floor/ceiling)

2. **Decision criteria:**

   - ✅ Keep: Straightlining on 1 scale only + plausible pattern
   - ⚠️ Review: Straightlining on 2+ scales
   - ❌ Exclude: Straightlining on majority of scales + other flags

3. **Document:**
   - Report exclusion criteria in paper
   - Include straightlining rate in results
   - Consider sensitivity analysis

### Configuration

```toml
[quality]
straightlining_enabled = true  # Default
# Or disable if not needed:
# straightlining_enabled = false
```

---

## Missing Data Analysis

### What It Is

**Missing data** occurs when participants skip items. Excessive missing data can bias results and reduce statistical power.

### How It's Detected

Prism calculates the percentage of missing responses per participant:

```
Participant has 50 total items
Missing values: 8 items
Missing percentage: 8/50 = 16%
```

### When It's Flagged

Default threshold: **10%**

```
=== MISSING DATA ===
Participant 045: 16.0% missing data (threshold: 10.0%)
```

### Interpreting Missing Data

**Low missing (< 5%):**

- Generally acceptable
- Random item skips
- Minimal impact on analyses

**Moderate missing (5-15%):**

- Review patterns (random vs. systematic)
- Consider imputation
- May exclude from specific analyses

**High missing (> 15%):**

- Strong candidate for exclusion
- Likely disengaged participant
- Imputation may be inappropriate

### What To Do

1. **Analyze patterns:**

   ```r
   # Check if missing is random
   library(mice)
   md.pattern(data)
   ```

2. **Options:**

   - **Keep:** If missing < threshold and appears random
   - **Impute:** Use mean, regression, or multiple imputation
   - **Exclude:** If > 20% missing or systematic pattern

3. **Report:**
   - Percentage of participants with >10% missing
   - How missing data was handled
   - Sensitivity analyses

### Configuration

**Adjust threshold:**

```toml
[quality]
# Conservative (clinical research)
missing_threshold = 5.0

# Standard (most research)
missing_threshold = 10.0

# Lenient (exploratory studies)
missing_threshold = 20.0
```

---

## Out-of-Range Detection

### What It Is

**Out-of-range values** are responses outside the valid response scale, indicating data entry errors or system issues.

### How It's Detected

When you specify `min_value` and `max_value` in your config:

```toml
[scales.anxiety]
items = ["anx1", "anx2", "anx3"]
min_value = 1.0
max_value = 5.0  # Valid range: 1-5
```

Prism flags values outside this range:

```
Participant 032: anx2 = 8
→ FLAGGED: 8.0 outside valid range (1.0-5.0)
```

### When It's Flagged

```
=== OUT-OF-RANGE VALUES ===
Participant 032: Out-of-range value 8.0 in 'anx2' (valid: 1.0-5.0)
```

### Common Causes

1. **Data entry errors:**

   - Typo: Entered 55 instead of 5
   - Wrong scale: Used 1-10 instead of 1-5

2. **System issues:**

   - Coding errors in online survey
   - Export formatting problems
   - Database corruption

3. **Multiple scales:**
   - Mixed 1-5 and 1-7 items in same file
   - Forgot to specify different ranges

### What To Do

1. **Verify in original data:**

   - Check raw survey output
   - Cross-reference with survey platform

2. **Fix if possible:**

   - Correct obvious typos
   - Verify correct scale used
   - Rerun Prism after correction

3. **If unfixable:**
   - Recode to missing
   - Document in paper
   - Exclude participant if multiple errors

### Configuration

**Specify ranges per scale:**

```toml
[scales.likert_5]
items = ["q1", "q2", "q3"]
min_value = 1.0
max_value = 5.0

[scales.likert_7]
items = ["q4", "q5", "q6"]
min_value = 1.0
max_value = 7.0

[scales.vas]
items = ["vas1", "vas2"]
min_value = 0.0
max_value = 100.0
```

**Disable if not needed:**

```toml
[quality]
out_of_range_enabled = false  # Skip range checks
```

---

## Understanding Reports

### Report Structure

**Quality report sections:**

1. **Straightlining Detected**

   - Participants with identical responses
   - Scale name and value

2. **Missing Data**

   - Participants exceeding threshold
   - Percentage missing

3. **Out-of-Range Values**
   - Invalid responses
   - Item name and actual value

### Example Report

```
Quality Report
Generated: 2026-01-02 14:30:00

Survey: Clinical Outcomes Study
Total Participants: 150

=== STRAIGHTLINING DETECTED ===
Participant 003: All values identical in scale 'depersonalization' (all items = 1.0)
Participant 027: All values identical in scale 'burnout' (all items = 4.0)
Participant 089: All values identical in scale 'alliance' (all items = 5.0)

=== MISSING DATA ===
Participant 015: 12.5% missing data (threshold: 10.0%)
Participant 041: 18.0% missing data (threshold: 10.0%)
Participant 076: 25.0% missing data (threshold: 10.0%)

=== OUT-OF-RANGE VALUES ===
Participant 023: Out-of-range value 8.0 in 'burn3' (valid: 1.0-7.0)
Participant 098: Out-of-range value 0.0 in 'anx5' (valid: 1.0-5.0)
```

### Reading the Report

**Assess severity:**

- Count participants with multiple flags
- Calculate percentage flagged overall
- Identify systematic patterns

**Example assessment:**

```
Total N = 150
Straightlining: 3 (2%)
Missing data: 3 (2%)
Out-of-range: 2 (1.3%)
Multiple flags: 1 (0.7%)
→ Good quality overall
```

---

## Best Practices

### Before Data Collection

1. **Configure your survey:**

   - Force response on key items
   - Add attention checks
   - Limit survey length (< 20 min)

2. **Set quality thresholds:**
   - Review literature standards
   - Consider your research context
   - Balance retention vs. quality

### During Data Collection

1. **Monitor quality:**

   - Run Prism on early data
   - Check quality report regularly
   - Adjust survey if needed

2. **Pilot test:**
   - Test full survey flow
   - Verify scale ranges
   - Check for confusing items

### After Data Collection

1. **Review all flags:**

   - Examine each flagged participant
   - Look for patterns
   - Make exclusion decisions

2. **Document decisions:**

   - Create exclusion flowchart
   - Report all criteria
   - Archive quality reports

3. **Conduct sensitivity analyses:**
   - Compare results with/without exclusions
   - Report if results differ
   - Discuss implications

---

## Research Recommendations

### Reporting Standards

**Include in your paper:**

1. **Data quality section:**

   - Number/percentage flagged per check
   - Exclusion criteria and process
   - Final sample size

2. **Example text:**
   ```
   "Data quality was assessed using automated checks.
   Participants were excluded if they exhibited: (a)
   straightlining (identical responses) on >2 scales,
   (b) >15% missing data, or (c) out-of-range responses.
   This resulted in exclusion of 12 participants (7.5%),
   leaving a final sample of N = 148."
   ```

### Decision Guidelines

| Flag Type           | Action      | Rationale         |
| ------------------- | ----------- | ----------------- |
| Straightlining only | Review      | May be valid      |
| Missing < 15%       | Keep        | Acceptable        |
| Missing 15-25%      | Review      | Case-by-case      |
| Missing > 25%       | Exclude     | Insufficient data |
| Out-of-range        | Fix/exclude | Data error        |
| Multiple flags      | Exclude     | Likely invalid    |

### Pre-Registration

**Specify before data collection:**

```
Quality Exclusion Criteria:
1. Straightlining on ≥3 scales → Exclude
2. Missing data >20% → Exclude
3. Out-of-range values → Review and fix or exclude
4. Multiple quality flags (≥2) → Exclude
```

---

## Advanced Topics

### Calculating Quality Metrics

**From quality report:**

```r
# Example in R
n_total <- 150
n_straightline <- 3
n_missing <- 3
n_range <- 2

# Percentages
pct_straightline <- (n_straightline / n_total) * 100  # 2%
pct_missing <- (n_missing / n_total) * 100           # 2%
pct_range <- (n_range / n_total) * 100               # 1.3%

# Overall quality
n_any_flag <- 7  # Count unique IDs
pct_quality <- ((n_total - n_any_flag) / n_total) * 100  # 95.3%
```

### Multiple Imputation

**After identifying missing data:**

```r
library(mice)

# Run multiple imputation
imputed <- mice(data, m = 5, method = "pmm")

# Analyze
fit <- with(imputed, lm(outcome ~ predictor))
pooled <- pool(fit)
summary(pooled)
```

### Attention Checks

**Add to your survey:**

```toml
# Example: Bogus item
[scales.attention]
items = ["attention_check"]  # "Please select '3' for this item"
# Flag if not 3
```

---

## FAQ

**Q: Should I always exclude straightlining?**  
A: No. Consider context (floor/ceiling effects), scale type, and whether it occurs across multiple scales.

**Q: What missing data threshold should I use?**  
A: 10% is standard. Use 5% for clinical research, 15-20% for exploratory studies.

**Q: Can I fix out-of-range values?**  
A: Only if you can verify the correct value (e.g., obvious typo). Otherwise, recode to missing.

**Q: How many exclusions is too many?**  
A: >10% excluded raises concerns. Report thoroughly and discuss limitations.

**Q: Should I impute missing data?**  
A: Depends on amount and pattern. < 5% missing: Use available-case analysis. 5-15%: Consider imputation. > 15%: Exclude or use advanced methods.

---

## Wiki Navigation

- [📚 Back to Wiki Home](README.md)
- [📖 How to Use](HOW_TO_USE.md)
- [🔧 Configuration Guide](CONFIGURATION_GUIDE.md)
- [📊 Workflow Examples](WORKFLOW_EXAMPLE.md)
- [✅ Best Practices](BEST_PRACTICES.md)

---

**Related:**

- [Configuration Guide →](CONFIGURATION_GUIDE.md)
- [Best Practices →](BEST_PRACTICES.md)
- [FAQ →](FAQ.md)

---

[⬆ Back to Top](#quality-checks-guide) | [📚 Wiki Home](README.md)
