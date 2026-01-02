# Prism Tutorial

**[📚 Wiki Home](README.md)** | **[📖 How to Use](HOW_TO_USE.md)** | **[🔧 Configuration](CONFIGURATION_GUIDE.md)** | **[⚡ Quick Ref](QUICK_REFERENCE.md)**

---

## Welcome! 👋

This step-by-step tutorial will guide you through using Prism for the first time. By the end, you'll be able to process your own survey data.

**Time:** 15-20 minutes

---

## What You'll Learn

1. ✅ Setting up your first survey config
2. ✅ Preparing your CSV data
3. ✅ Running Prism
4. ✅ Understanding the output
5. ✅ Interpreting quality reports

---

## Prerequisites

- Prism installed ([Installation Guide](INSTALLATION.md))
- A text editor (VS Code, Notepad++, etc.)
- Basic familiarity with CSV files

---

## Step 1: Understanding Your Data

Let's say you have survey data measuring **burnout** using 9 items:

- **Emotional Exhaustion:** Items 1-5 (1-7 scale)
- **Depersonalization:** Items 6-9 (1-7 scale)

**Your CSV might look like:**

```csv
participant_id,ee1,ee2,ee3,ee4,ee5,dp1,dp2,dp3,dp4
001,5,6,4,5,6,2,3,2,1
002,3,4,3,2,3,4,3,5,4
003,7,7,6,7,7,1,1,1,1
```

---

## Step 2: Create Your Config File

**Create `burnout_config.toml`:**

```toml
# Survey Information
[survey]
name = "Burnout Study"
description = "Measuring emotional exhaustion and depersonalization"

# Quality Checks
[quality]
missing_threshold = 10.0  # Flag if >10% missing

# Emotional Exhaustion Scale
[scales.emotional_exhaustion]
items = ["ee1", "ee2", "ee3", "ee4", "ee5"]
min_value = 1.0
max_value = 7.0

# Depersonalization Scale
[scales.depersonalization]
items = ["dp1", "dp2", "dp3", "dp4"]
min_value = 1.0
max_value = 7.0
```

**💡 Key Points:**

- Scale names can be descriptive
- `items` list column names from your CSV
- `min_value` and `max_value` help detect errors

---

## Step 3: Run Prism

**Basic command:**

```bash
prism -i survey_data.csv -c burnout_config.toml -o clean_data.csv
```

**With all reports:**

```bash
prism -i survey_data.csv -c burnout_config.toml -o clean_data.csv \
  --stats-output stats.txt --quality-report quality.txt
```

**Expected Output:**

```
Processing Survey: Burnout Study
Successfully processed 3 participants.
Output saved to: clean_data.csv
Summary statistics saved to: stats.txt
Quality report saved to: quality.txt
```

---

## Step 4: Examine the Output

### A. Clean Data CSV

**`clean_data.csv` contains:**

```csv
participant_id,ee1,ee2,ee3,ee4,ee5,dp1,dp2,dp3,dp4,emotional_exhaustion_total,emotional_exhaustion_mean,depersonalization_total,depersonalization_mean
001,5,6,4,5,6,2,3,2,1,26,5.2,8,2.0
002,3,4,3,2,3,4,3,5,4,15,3.0,16,4.0
003,7,7,6,7,7,1,1,1,1,34,6.8,4,1.0
```

**New columns added:**

- `emotional_exhaustion_total` - Sum of ee1-ee5
- `emotional_exhaustion_mean` - Average of ee1-ee5
- `depersonalization_total` - Sum of dp1-dp4
- `depersonalization_mean` - Average of dp1-dp4

### B. Summary Statistics

**`stats.txt` shows aggregate statistics:**

```
Summary Statistics Report
Generated: 2026-01-02

Survey: Burnout Study

emotional_exhaustion_total: M = 25.00, SD = 9.54, min = 15.00, max = 34.00, N = 3
emotional_exhaustion_mean: M = 5.00, SD = 1.91, min = 3.00, max = 6.80, N = 3
depersonalization_total: M = 9.33, SD = 6.11, min = 4.00, max = 16.00, N = 3
depersonalization_mean: M = 2.33, SD = 1.53, min = 1.00, max = 4.00, N = 3
```

### C. Quality Report

**`quality.txt` flags issues:**

```
Quality Report
Generated: 2026-01-02

Survey: Burnout Study

=== STRAIGHTLINING DETECTED ===
Participant 003: All values identical in scale 'depersonalization' (all items = 1.0)
```

---

## Step 5: Interpret Quality Issues

### Straightlining

**What it means:** Participant gave same response to all items in a scale.

**Example from our data:**

```
Participant 003: dp1=1, dp2=1, dp3=1, dp4=1
```

**What to do:**

1. Review this participant's data
2. Consider if response is valid (e.g., floor effect) or careless
3. May need to exclude if combined with other issues

### Missing Data

**What it means:** Participant skipped >10% of items.

**Example:**

```
Participant 005: 15.0% missing data (flagged because threshold = 10.0%)
```

**What to do:**

1. Check if missing systematically
2. Consider imputation or exclusion
3. Report missing data in your paper

### Out of Range

**What it means:** Response outside valid range.

**Example:**

```
Participant 007: Out-of-range value 8.0 in 'ee3' (valid: 1.0-7.0)
```

**What to do:**

1. Check if data entry error
2. Correct in original data and rerun
3. Or recode to missing

---

## Step 6: Add Reverse Scoring (Advanced)

Let's say `ee3` is negatively worded and needs reversing.

**Update your config:**

```toml
[scales.emotional_exhaustion]
items = ["ee1", "ee2", "ee3", "ee4", "ee5"]
reverse_scored = ["ee3"]  # ← Add this line
min_value = 1.0
max_value = 7.0
```

**What happens:**

- Original `ee3` = 4
- Reversed: (7 + 1) - 4 = 4
- For `ee3` = 2: (7 + 1) - 2 = 6

**Rerun Prism:**

```bash
prism -i survey_data.csv -c burnout_config.toml -o clean_data.csv
```

---

## Step 7: Import to Your Analysis Software

### R

```r
# Read clean data
data <- read.csv("clean_data.csv")

# Use scale scores
summary(data$emotional_exhaustion_mean)
cor(data$emotional_exhaustion_mean, data$depersonalization_mean)
```

### Python/Pandas

```python
import pandas as pd

# Read clean data
df = pd.read_csv("clean_data.csv")

# Use scale scores
df['emotional_exhaustion_mean'].describe()
df[['emotional_exhaustion_mean', 'depersonalization_mean']].corr()
```

### SPSS

```
GET DATA
  /TYPE=TXT
  /FILE='clean_data.csv'
  /DELIMITERS=","
  /FIRSTCASE=2
  /VARIABLES=
    participant_id F10.0
    emotional_exhaustion_mean F8.2
    depersonalization_mean F8.2.

DESCRIPTIVES emotional_exhaustion_mean depersonalization_mean
  /STATISTICS=MEAN STDDEV MIN MAX.
```

---

## Common Scenarios

### Scenario 1: "Some Items Need Reversing"

**Problem:** You have a mix of positive and negative items.

**Solution:**

```toml
[scales.satisfaction]
items = ["sat1", "sat2", "sat3", "sat4"]
reverse_scored = ["sat2", "sat4"]  # Negative items
```

### Scenario 2: "I Have Multiple Subscales"

**Problem:** Your measure has several dimensions.

**Solution:** Create a scale for each subscale:

```toml
[scales.subscale_a]
items = ["a1", "a2", "a3"]

[scales.subscale_b]
items = ["b1", "b2", "b3"]

[scales.total_score]
items = ["a1", "a2", "a3", "b1", "b2", "b3"]
```

### Scenario 3: "Quality Checks Too Strict"

**Problem:** Getting too many false positives.

**Solution:** Adjust thresholds:

```toml
[quality]
missing_threshold = 20.0  # More lenient (was 10.0)
```

### Scenario 4: "Need to Process Multiple Files"

**Problem:** You have 10 survey waves.

**Solution:** Use a loop:

```bash
# Bash
for file in wave_*.csv; do
  prism -i "$file" -c config.toml -o "clean_$file"
done

# PowerShell
Get-ChildItem wave_*.csv | ForEach-Object {
  prism -i $_.Name -c config.toml -o "clean_$($_.Name)"
}
```

---

## Troubleshooting

### "Column 'ee6' not found"

**Problem:** Config references non-existent column.

**Fix:** Check your CSV headers match config exactly (case-sensitive).

### "Failed to parse config file"

**Problem:** TOML syntax error.

**Fix:**

- Check brackets: `[scales.name]` not `scales.name`
- Check quotes: `name = "value"` not `name = value`
- Check arrays: `items = ["a", "b"]` not `items = [a, b]`

### "No valid numeric data"

**Problem:** Non-numeric values in scale columns.

**Fix:**

- Ensure all scale items contain numbers only
- Missing values should be empty, not "NA" or "N/A"
- Check for text responses in numeric columns

---

## Next Steps

**You've completed the tutorial!** 🎉

**What's next:**

1. **Process Your Own Data**

   - Create config for your survey
   - Run Prism on your data
   - Review quality reports

2. **Learn More**

   - [Configuration Guide](CONFIGURATION_GUIDE.md) - Deep dive into options
   - [Workflow Examples](WORKFLOW_EXAMPLE.md) - Real-world scenarios
   - [Best Practices](BEST_PRACTICES.md) - Tips and tricks

3. **Get Help**
   - [FAQ](FAQ.md) - Common questions
   - [Troubleshooting](TROUBLESHOOTING.md) - Solve problems

---

## Summary Checklist

✅ Created survey config file  
✅ Ran Prism with your data  
✅ Examined clean CSV output  
✅ Reviewed summary statistics  
✅ Understood quality report  
✅ Imported to analysis software

---

## Wiki Navigation

- [📚 Back to Wiki Home](README.md)
- [📖 Full User Guide](HOW_TO_USE.md)
- [🔧 Configuration Guide](CONFIGURATION_GUIDE.md)
- [📊 Workflow Examples](WORKFLOW_EXAMPLE.md)
- [❓ FAQ](FAQ.md)

---

**Need Help?** Check the [FAQ](FAQ.md) or [Troubleshooting](TROUBLESHOOTING.md) guide.

---

[⬆ Back to Top](#prism-tutorial) | [📚 Wiki Home](README.md)
