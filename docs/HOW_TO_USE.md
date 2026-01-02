# How to Use Prism

**[📚 Wiki Home](README.md)** | **[⚡ Quick Reference](QUICK_REFERENCE.md)** | **[🔧 Configuration](CONFIGURATION_GUIDE.md)** | **[📊 Examples](WORKFLOW_EXAMPLE.md)**

---

## Quick Start (3 Steps)

### Step 1: Prepare Your Data

Export your survey data to a CSV file with these requirements:

- **First row = headers** (column names like Q1, Q2, Q3, etc.)
- **First column = participant IDs**
- **Remaining columns = survey responses**

Example:

```csv
id,Q1,Q2,Q3,Q4,Q5
P001,5,6,4,7,5
P002,3,3,3,3,3
P003,1,2,3,4,5
```

### Step 2: Create Configuration File

Create a file named `survey_config.toml` in your project folder:

```toml
[survey]
name = "My Study Name"
min_score = 1          # Lowest valid response
max_score = 7          # Highest valid response

[quality]
max_missing_percent = 0.10    # Flag if >10% missing
flag_straightlining = true    # Detect all-same responses

[scales.scale1]
items = ["Q1", "Q2", "Q3"]           # Items in this scale
reverse_scored = ["Q2"]               # Items to reverse (optional)

[scales.scale2]
items = ["Q4", "Q5"]
reverse_scored = []                   # No reverse scoring
```

**Important:**

- Item names must match your CSV column headers exactly
- Use the actual column names from your CSV (like "Q1", "WAI_4", "MSPSS_1", etc.)

### Step 3: Run Prism

Open a terminal/command prompt in your project folder:

```bash
# Windows
.\target\release\prism.exe -i data.csv -c survey_config.toml -o clean_data.csv

# Mac/Linux
./target/release/prism -i data.csv -c survey_config.toml -o clean_data.csv
```

**With full reports:**

```bash
prism -i data.csv -c survey_config.toml -o clean_data.csv --stats-output summary.txt --quality-report quality.txt
```

---

## Usage Options

### Using the GUI (Easiest)

1. Build the GUI: `cd src-tauri && cargo tauri build`
2. Run the app
3. Click "Select CSV File"
4. Results automatically saved in same folder as input

Output files:

- `clean_data.csv` - Processed data
- `summary_stats.txt` - Statistics report
- `quality_report.txt` - Quality issues

### Using the CLI (Most Flexible)

**Basic command:**

```bash
prism -i <input.csv> -c <config.toml> -o <output.csv>
```

**All options:**

```bash
prism [OPTIONS] --input <FILE> --config <FILE>

Options:
  -i, --input <FILE>              Raw CSV data file
  -c, --config <FILE>             Configuration file (TOML)
  -o, --output <FILE>             Output CSV [default: clean_data.csv]
      --stats-output <FILE>       Generate statistics report (optional)
      --quality-report <FILE>     Generate quality report (optional)
  -h, --help                      Print help
  -V, --version                   Print version
```

---

## Real Example: Burnout Study

### Your Data (`burnout_data.csv`):

```csv
id,Q1,Q2,Q3,Q4,Q5,Q6,Q7,Q8,Q9
P001,6,5,6,7,6,5,6,7,6
P002,3,3,3,3,3,3,3,3,3
P003,1,2,3,4,5,6,7,6,5
```

### Your Config (`burnout_config.toml`):

```toml
[survey]
name = "Burnout Study"
min_score = 1
max_score = 7

[quality]
max_missing_percent = 0.10
flag_straightlining = true

[scales.emotional_exhaustion]
items = ["Q1", "Q2", "Q3", "Q4", "Q5", "Q6", "Q7", "Q8", "Q9"]
reverse_scored = []
```

### Run:

```bash
prism -i burnout_data.csv -c burnout_config.toml -o clean_burnout.csv --stats-output burnout_stats.txt
```

### You Get:

**1. `clean_burnout.csv`** (original + computed scores):

```csv
id,Q1,Q2,Q3,Q4,Q5,Q6,Q7,Q8,Q9,emotional_exhaustion_total,emotional_exhaustion_mean,quality_flag
P001,6,5,6,7,6,5,6,7,6,54.00,6.00,OK
P002,3,3,3,3,3,3,3,3,3,27.00,3.00,Straightlining: emotional_exhaustion
P003,1,2,3,4,5,6,7,6,5,39.00,4.33,OK
```

**2. `burnout_stats.txt`** (aggregate statistics):

```
BURNOUT STUDY - Summary Statistics
Generated: 2026-01-02 11:00:00

Total Participants: 3
Complete Responses: 3 (100.0%)

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

SCALE: emotional_exhaustion (9 items)
Items: Q1, Q2, Q3, Q4, Q5, Q6, Q7, Q8, Q9

  Mean (M)              = 4.44
  Standard Deviation    = 1.53
  Range                 = [3.00, 6.00]
  N                     = 3
```

---

## Common Workflows

### Batch Processing Multiple Files

```bash
# Process all CSV files in a folder
for file in data/*.csv; do
    basename=$(basename "$file" .csv)
    prism -i "$file" -c config.toml -o "output/${basename}_clean.csv"
done
```

### Check Before Processing

```bash
# Validate config first (it auto-validates, but check output)
prism -i data.csv -c config.toml -o test.csv
```

### Import to R/Python

```r
# R
data <- read.csv("clean_data.csv")
summary(data$emotional_exhaustion_mean)

# Python
import pandas as pd
df = pd.read_csv("clean_data.csv")
df.describe()
```

---

## Troubleshooting

### Error: "Item 'Q5' not found in CSV"

**Problem:** Config references column that doesn't exist  
**Solution:** Check your CSV headers match config item names exactly

```bash
# Check your CSV headers
head -1 data.csv

# Make sure config uses same names
```

### Error: "Config validation failed"

**Problem:** Invalid configuration  
**Solution:** Check that:

- All scale items exist in CSV
- Reverse-scored items are listed in items
- min_score < max_score

### All results show "NA"

**Problem:** Column name mismatch  
**Solution:** CSV columns must match config item names exactly (case-sensitive)

### Too many straightlining flags

**Problem:** Legitimate uniform responses flagged  
**Solution:** Review context - not all straightlining is invalid

---

## Tips

### ✅ Best Practices

1. **Keep raw data separate**

   ```
   data/
     raw/            # Never modify
       survey.csv
     processed/      # Generated files
       clean.csv
   ```

2. **Version control your config**

   ```bash
   git add survey_config.toml
   git commit -m "Add burnout scale configuration"
   ```

3. **Review quality reports**

   - Always check quality_report.txt
   - Investigate flagged participants
   - Don't blindly trust all data

4. **Spot-check results**

   - Manually verify a few rows
   - Check reverse scoring worked
   - Compare totals to expectations

5. **Document decisions**
   ```toml
   # In your config:
   [scales.personal_accomplishment]
   # All items reverse-scored per Maslach et al. (1996)
   reverse_scored = ["Q15", "Q16", "Q17", "Q18"]
   ```

### ⚠️ Common Mistakes

1. ❌ Column names don't match (Q1 vs q1)
2. ❌ Forgot to list reverse items in both `items` and `reverse_scored`
3. ❌ Wrong scale range (1-5 when data is 1-7)
4. ❌ Config in wrong folder
5. ❌ CSV has no header row

---

## Next Steps

1. **Test with sample data** - Use 2-3 rows first
2. **Verify reverse scoring** - Check manually that it worked
3. **Review quality report** - Understand what's flagged
4. **Process full dataset** - Once confident
5. **Import to stats software** - R, SPSS, Python, etc.

---

## Getting Help

- 📖 Full documentation: `README.md`
- 🏗️ Architecture details: `ARCHITECTURE.md`
- 📊 Implementation status: `IMPLEMENTATION_STATUS.md`
- 📝 Complete example: `WORKFLOW_EXAMPLE.md`
- ⚡ Quick reference: `QUICK_REFERENCE.md`

---

## Example: Your Current Study

Based on your `study_config.toml`, here's how to use it:

```bash
# Your data structure:
# - Emotional Exhaustion: Q1-Q9
# - Depersonalization: Q10-Q14
# - Supervision: SWAI_1-SWAI_12
# - Peer Support: MSPSS_1-MSPSS_4
# - Alliance: WAI_1-WAI_12 (WAI_4, WAI_10 reversed)

# Process your data:
prism \
  -i data/raw/test_data.csv \
  -c study_config.toml \
  -o data/processed/clean.csv \
  --stats-output data/processed/stats.txt \
  --quality-report data/processed/quality.txt

# Import to R:
data <- read.csv("data/processed/clean.csv")
summary(data$emotional_exhaustion_mean)
summary(data$alliance_total_mean)

# Check quality:
cat data/processed/quality.txt
```

**That's it! You're ready to process survey data 🚀**

---

## Wiki Navigation

- [📚 Back to Wiki Home](README.md)
- [⚡ Quick Reference](QUICK_REFERENCE.md)
- [🔧 Configuration Guide](CONFIGURATION_GUIDE.md)
- [❓ FAQ](FAQ.md)
- [🐛 Troubleshooting](TROUBLESHOOTING.md)

---

**Next Steps:**

- [Configuration Guide →](CONFIGURATION_GUIDE.md)
- [Quality Checks →](QUALITY_CHECKS.md)
- [Best Practices →](BEST_PRACTICES.md)
