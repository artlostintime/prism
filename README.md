# Prism 🔍

> **Stop wasting hours on Excel. Process your psychology survey data in seconds.**

Hey there, researcher! 👋 If you've ever spent 45+ minutes manually reverse-scoring surveys, calculating scale totals, and hunting for data quality issues in Excel... this tool is for you.

Prism does all that boring stuff automatically, so you can focus on what actually matters: **understanding your data**.

---

## 🎯 What Does Prism Do?

Think of Prism as your research assistant that:

1. **Pre-built scale library** 🆕 (PHQ-9, GAD-7, PSS, PANAS, BDI-II, BAI, SWLS with citations!)
2. **Reverse-scores items automatically** (no more Excel formulas!)
3. **Calculates scale totals and means** (instantly, no errors)
4. **Finds data quality issues** (straightliners, missing data, weird responses)
5. **Generates statistical reports** (with Cronbach's alpha and everything)
6. **Exports in multiple formats** (Excel, **production-ready SPSS syntax** 🆕, R, JSON)

**Real talk:** What used to take me 45-60 minutes in Excel now takes 30 seconds. ⚡

---

## 🚀 Getting Started (Super Easy!)

### Step 1: Install Rust

Don't worry, it's just one command. Copy and paste this:

**On Mac/Linux:**

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

**On Windows:**
Download from [rustup.rs](https://rustup.rs/) and run the installer.

### Step 2: Build Prism

```bash
cargo build --release
```

Go grab a coffee ☕ — this takes about 2-3 minutes the first time.

### Step 3: You're Done! 🎉

The program is now ready in `target/release/prism` (or `prism.exe` on Windows).

---

## 💡 Your First Run (In 3 Minutes)

Let's say you have a burnout survey with questions Q1-Q20, and you need to:

- Reverse-score some items
- Calculate three scale scores
- Check data quality

### 1️⃣ Create Your Config File

**Option A: Use a Pre-built Scale** (NEW in v0.3.0! 🎉)

```bash
# See all available scales
prism generate --list-scales

# Generate config for PHQ-9 (depression screening)
prism generate --scale PHQ-9 > my_survey.toml

# Get detailed info about a scale
prism generate --scale-info GAD-7
```

**Available Pre-built Scales:**

- `PHQ-9` - Patient Health Questionnaire (Depression)
- `GAD-7` - Generalized Anxiety Disorder
- `PSS-10` / `PSS-14` - Perceived Stress Scale
- `PANAS` - Positive and Negative Affect
- `BDI-II` - Beck Depression Inventory
- `BAI` - Beck Anxiety Inventory
- `SWLS` - Satisfaction With Life Scale

Each comes with proper citations, scoring rules, and normative data!

**Option B: Create Your Own Template**

Run this to get a template:

```bash
prism generate --template > my_survey.toml
```

Open `my_survey.toml` and edit it to match your survey:

```toml
[survey]
name = "My Burnout Study"
min_score = 1    # Your scale's minimum (e.g., 1 = "Strongly Disagree")
max_score = 7    # Your scale's maximum (e.g., 7 = "Strongly Agree")

[quality]
max_missing_percent = 0.10      # Flag if someone skips >10% of items
flag_straightlining = true      # Catch people who answer "3" to everything

[scales.emotional_exhaustion]
items = ["Q1", "Q2", "Q3", "Q4", "Q5", "Q6", "Q7", "Q8", "Q9"]
reverse_scored = []  # None are reversed

[scales.depersonalization]
items = ["Q10", "Q11", "Q12", "Q13", "Q14"]
reverse_scored = ["Q12"]  # Q12 is reverse-scored

[scales.personal_accomplishment]
items = ["Q15", "Q16", "Q17", "Q18", "Q19", "Q20"]
reverse_scored = ["Q15", "Q16", "Q17", "Q18", "Q19", "Q20"]  # All reversed!
```

### 2️⃣ Process Your Data

```bash
prism process -i my_data.csv -c my_survey.toml -o clean_data.csv --all-outputs
```

That's it! You'll get:

- ✅ `clean_data.csv` - Your data with computed scale scores
- ✅ `summary_stats.txt` - Means, SDs, Cronbach's alpha for each scale
- ✅ `quality_report.txt` - List of any data quality issues

---

## 📚 Common Tasks (Copy & Paste Ready)

### Just Process Data (Basics)

```bash
prism process -i data.csv -c config.toml -o clean.csv
```

### Get Statistics + Quality Report

```bash
prism process -i data.csv -c config.toml -o clean.csv --all-outputs
```

### Export to All Formats at Once 🚀

```bash
prism process -i data.csv -c config.toml -o clean.csv --export-all
```

This creates:

- `clean.csv` - CSV format
- `clean.xlsx` - Excel format
- `clean.json` - JSON format
- `clean.sps` - SPSS syntax file
- `clean.R` - R script for loading data

Combine with `--all-outputs` for stats and quality reports too:

```bash
prism process -i data.csv -c config.toml -o clean.csv --export-all --all-outputs
```

### Export to Excel Instead of CSV

```bash
prism process -i data.csv -c config.toml -o clean.csv --format excel
```

### Check If Your Config is Correct (Before Running)

```bash
prism validate -c config.toml -i data.csv
```

### Preview What Would Happen (Dry Run)

```bash
prism process -i data.csv -c config.toml --dry-run
```

### Process Multiple Studies at Once

Create a file called `batch.txt`:

```
study1_data.csv
study2_data.csv
study3_data.csv
```

Then run:

```bash
prism process --batch batch.txt -c config.toml -o output.csv
```

---

---

## 🔧 Understanding the Config File

The config file tells Prism about your survey. Here's what each part does:

### The Survey Section (Required)

```toml
[survey]
name = "My Study"           # What you want to call your study
min_score = 1               # Lowest possible response (e.g., 1 = "Never")
max_score = 7               # Highest possible response (e.g., 7 = "Always")
```

**💡 Tip:** If someone enters an "8" on a 1-7 scale, Prism catches it automatically!

### The Quality Section (Optional but Recommended)

```toml
[quality]
max_missing_percent = 0.10    # Flag if >10% of items are missing
flag_straightlining = true    # Catch people answering the same to everything
min_response_variance = 0.5   # Flag low variation in responses (optional)
min_response_time = 30        # Flag suspiciously fast completions (< 30s)
max_response_time = 300       # Flag suspiciously slow completions (> 5 min)
```

**Real example:** If someone answers [3, 3, 3, 3, 3] to all items, that's straightlining and gets flagged.

**NEW in v0.8.0!** 🎉 **Advanced Pattern Detection:**

- **Diagonal Patterns**: Catches 1,2,3,4,5 or 5,4,3,2,1 sequences (careless responding)
- **Alternating Patterns**: Detects 1,5,1,5,1,5 patterns (not paying attention)
- **Block Patterns**: Identifies respondents who answer all 1s, then all 5s (suspicious behavior)
- **Response Time Analysis**: Flags too-fast or too-slow survey completions

These checks run automatically and help you maintain high data quality!

### The Scales Section (This is where the magic happens!)

```toml
[scales.depression]
items = ["Q1", "Q2", "Q3", "Q4", "Q5"]
reverse_scored = ["Q2", "Q4"]  # These get flipped automatically
```

**What "reverse-scored" means:**

- Normal item: Response of 7 = score of 7
- Reverse item: Response of 7 = score of 1 (it gets flipped!)
- Formula: `reversed_score = (max + min) - original_score`

### Handling Messy Column Names (Super Useful!)

Sometimes Qualtrics exports messy names like `Q1_Depression_VeryLongName`. Map them:

```toml
[column_mappings]
"Q1_Depression_VeryLongName" = "Q1"
"Q2_Depression_AnotherLongName" = "Q2"
```

Now you can just use "Q1", "Q2" in your scale definitions. Much cleaner!

---

## 📊 Understanding the Output

### What You Get

After running Prism, you'll have:

**1. `clean_data.csv`** - Your original data + new columns:

```
original_columns..., depression_total, depression_mean, anxiety_total, anxiety_mean, quality_flag
```

**2. `summary_stats.txt`** - The good stuff:

```
SCALE: Depression (5 items)
  Mean (M)              = 3.45
  Standard Deviation    = 1.23
  Range                 = [1.20, 6.80]
  N                     = 150
  Cronbach's Alpha (α)  = 0.87  (Good)
```

**💡 Cronbach's Alpha Cheat Sheet:**

- **> 0.90** = Excellent (you can trust this scale!)
- **0.80-0.89** = Good
- **0.70-0.79** = Acceptable
- **< 0.70** = Questionable (might want to check your items)

**3. `quality_report.txt`** - Red flags:

```
Straightlining (5 occurrences):
  • Participant P042: Straightlining in Depression scale
  • Participant P103: Straightlining in Anxiety scale
```

### The Quality Flag Column

Every participant gets a quality flag:

- **"OK"** = Clean data, no issues ✅
- **"Straightlining: depression"** = Answered same to all items in that scale ⚠️
- **"High missing data: anxiety (40% missing)"** = Skipped too many questions ⚠️
- **"Missing: stress"** = Skipped the entire scale ❌

You can use this in your analysis to decide whether to exclude participants.

---

## 🎓 Real-World Example (Burnout Study)

Let's walk through a complete example using the Maslach Burnout Inventory.

### Your Survey Data (`burnout_responses.csv`)

```csv
ParticipantID,Q1,Q2,Q3,Q4,Q5,Q6,Q7,Q8,Q9,Q10,Q11,Q12,Q13,Q14,Q15,Q16,Q17,Q18,Q19,Q20,Q21,Q22
P001,6,6,7,6,5,6,7,6,6,2,1,2,1,3,5,6,5,6,6,5,6,5
P002,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3
P003,1,2,3,4,5,4,3,2,1,1,2,1,2,1,6,5,6,5,6,5,6,5
...
```

### Your Config (`mbi_config.toml`)

```toml
[survey]
name = "Teacher Burnout Study 2024"
min_score = 1
max_score = 7

[quality]
max_missing_percent = 0.10
flag_straightlining = true
min_response_variance = 0.5

# Emotional Exhaustion subscale (9 items)
[scales.emotional_exhaustion]
items = ["Q1", "Q2", "Q3", "Q6", "Q8", "Q13", "Q14", "Q16", "Q20"]
reverse_scored = []

# Depersonalization subscale (5 items)
[scales.depersonalization]
items = ["Q5", "Q10", "Q11", "Q15", "Q22"]
reverse_scored = []

# Personal Accomplishment subscale (8 items - all reversed!)
[scales.personal_accomplishment]
items = ["Q4", "Q7", "Q9", "Q12", "Q17", "Q18", "Q19", "Q21"]
reverse_scored = ["Q4", "Q7", "Q9", "Q12", "Q17", "Q18", "Q19", "Q21"]
```

### Run It

```bash
prism process \
  -i burnout_responses.csv \
  -c mbi_config.toml \
  -o clean_burnout.csv \
  --all-outputs
```

### What You Get

**`clean_burnout.csv`:**

```csv
ParticipantID,...,emotional_exhaustion_total,emotional_exhaustion_mean,depersonalization_total,depersonalization_mean,personal_accomplishment_total,personal_accomplishment_mean,quality_flag
P001,...,53.00,5.89,9.00,1.80,45.00,5.63,OK
P002,...,27.00,3.00,15.00,3.00,24.00,3.00,Straightlining: emotional_exhaustion; Straightlining: depersonalization; Straightlining: personal_accomplishment
P003,...,20.00,2.22,7.00,1.40,44.00,5.50,OK
```

**Key insight:** P002 is straightlining (answering "3" to everything) and gets flagged automatically!

### Import into SPSS (NEW in v0.5.0! 🎉)

Prism generates **production-ready SPSS syntax** with full transformation documentation:

```bash
prism process -i burnout_responses.csv -c mbi_config.toml -o clean.csv --format spss
```

This creates `clean.sps` with:

- **GET DATA** - CSV import with UTF-8 encoding
- **VARIABLE LABELS** - Descriptive labels for all items and scales (with reverse-scoring indicators!)
- **VALUE LABELS** - Automatic Likert scale labels (1-5, 1-7, custom ranges)
- **RECODE** - Reverse scoring transformations documented
- **COMPUTE** - Scale total and mean calculations
- **Missing value handling** - Out-of-range values set to $SYSMIS
- **Example commands** - DESCRIPTIVES and RELIABILITY analysis ready to use

Just open `clean.sps` in SPSS and run — all your transformations are documented and reproducible!

**Example Output Preview:**

```spss
* ======================================================================
* SPSS Syntax for: Maslach Burnout Inventory
* Generated by Prism v0.5.0 on 2026-01-05
* ======================================================================

VARIABLE LABELS
  EE1 'emotional_exhaustion - Item 1'
  EE2 'emotional_exhaustion - Item 2'
  EE3 'emotional_exhaustion - Item 3 (reverse scored)'
  emotional_exhaustion_mean 'emotional_exhaustion Mean Score (1-7)'
  .

VALUE LABELS
  EE1 EE2 EE3
  1 'Strongly Disagree'
  4 'Neutral'
  7 'Strongly Agree'
  .

RECODE EE3 (1 = 7) (7 = 1).
  (2 = 6) (3 = 5) (4 = 4) (5 = 3) (6 = 2)

COMPUTE emotional_exhaustion_mean = MEAN(EE1, EE2, EE3).
EXECUTE.
```

---

## 🚨 Troubleshooting (When Things Go Wrong)

### "Item 'Q5' not found in CSV headers"

**Problem:** Your config mentions "Q5" but your CSV doesn't have that column.

**Solution:** Check your column names. Maybe it's "q5" (lowercase) or "Question5"?

**Pro tip:** Use column mappings:

```toml
[column_mappings]
"Question5" = "Q5"
```

### "Could not parse TOML config"

**Problem:** There's a syntax error in your config file.

**Common mistakes:**

- ❌ Forgot quotes around strings: `name = My Study`
- ✅ Fixed: `name = "My Study"`

- ❌ Typo in section name: `[scales.depresion]`
- ✅ Fixed: `[scales.depression]`

**Pro tip:** Run `prism validate -c config.toml -i data.csv` first!

### Numbers Look Wrong

**Problem:** Scale means are way off.

**Checklist:**

1. Did you specify reverse-scored items correctly?
2. Is your `min_score` and `max_score` correct?
3. Are column names mapped correctly if using messy exports?

**Debug trick:** Use `--dry-run` to preview:

```bash
prism process -i data.csv -c config.toml --dry-run
```

### "Help! I Have Messy Qualtrics Export Names!"

Your CSV has columns like:

```
Duration..in.seconds., Q1_1, Q2_1, Q3_1_TEXT
```

**Solution 1:** Rename them in Excel first (easiest)

**Solution 2:** Use column mappings:

```toml
[column_mappings]
"Q1_1" = "Q1"
"Q2_1" = "Q2"
"Q3_1_TEXT" = "Q3"
```

---

## 💪 Advanced Features (For Power Users)

### Export to Multiple Formats at Once

```bash
# Get CSV + Excel + SPSS syntax + R script
prism process -i data.csv -c config.toml -o output.csv --format excel
prism process -i data.csv -c config.toml -o output.csv --format spss
prism process -i data.csv -c config.toml -o output.csv --format r
```

### Fine-Tune Your Output

```toml
[output]
decimal_places = 3        # Want more precision? (default: 2)
include_item_scores = true  # Include individual item scores in output
```

### Detect More Quality Issues

```toml
[quality]
max_missing_percent = 0.10
flag_straightlining = true
min_response_variance = 0.5        # Flag low variance (optional)
max_response_time = 300            # Flag if >5 minutes (optional)
min_response_time = 30             # Flag if <30 seconds (optional)
```

### Process Data from Multiple Time Points

```bash
# Longitudinal study with 3 waves
for wave in Wave1 Wave2 Wave3; do
  prism process \
    -i ${wave}_data.csv \
    -c survey_config.toml \
    -o clean_${wave}.csv \
    --all-outputs
done
```

---

## � Longitudinal Data Analysis (NEW in v0.3.0+!)

Prism now includes powerful features for analyzing repeated measures and longitudinal data!

### Merge Multiple Waves

Combine data from multiple time points automatically:

```bash
# Merge three waves by participant ID
prism merge \
  --waves T1:wave1_data.csv T2:wave2_data.csv T3:wave3_data.csv \
  --id ParticipantID \
  --join outer \
  -o merged_data.csv
```

**Join Types:**

- `inner` - Keep only participants present in ALL waves (strictest)
- `outer` - Keep all participants, even if they missed some waves (most inclusive)

### Convert Between Wide and Long Formats

Transform your data for different analyses:

**Wide to Long** (for growth curve modeling, multilevel analysis):

```bash
# Convert: depression_T1, depression_T2 → multiple rows with Time column
prism reshape \
  -i merged_data.csv \
  --format wide-to-long \
  --waves T1 T2 T3 \
  --id ParticipantID \
  -o long_format.csv
```

**Before (Wide):**

```csv
ParticipantID,depression_T1,depression_T2,depression_T3
P001,12.5,10.2,8.1
P002,15.0,14.8,14.2
```

**After (Long):**

```csv
ParticipantID,Time,depression
P001,T1,12.5
P001,T2,10.2
P001,T3,8.1
P002,T1,15.0
P002,T2,14.8
P002,T3,14.2
```

**Long to Wide** (for repeated measures ANOVA):

```bash
# Convert back: Time column → depression_T1, depression_T2, depression_T3
prism reshape \
  -i long_format.csv \
  --format long-to-wide \
  --waves T1 T2 T3 \
  --id ParticipantID \
  --time-col Time \
  -o wide_format.csv
```

### Calculate Reliable Change Index (RCI)

Determine if changes are **clinically significant** (not just statistically significant):

```bash
# Calculate RCI between baseline (T1) and follow-up (T2)
prism rci \
  -i merged_data.csv \
  --baseline depression_T1 \
  --followup depression_T2 \
  --reliability 0.85 \
  --id ParticipantID \
  -o rci_results.csv
```

**Understanding RCI Output:**

The results include:

- `rci_score` - The reliable change index value
- `se_diff` - Standard error of difference
- `change` - Raw change score (T2 - T1)
- `percent_change` - Percentage change from baseline
- `interpretation` - Clinical significance:
  - **"Improved"** - RCI < -1.96 (clinically significant improvement)
  - **"Deteriorated"** - RCI > 1.96 (clinically significant worsening)
  - **"No reliable change"** - -1.96 ≤ RCI ≤ 1.96 (change within measurement error)

**Formula:** `RCI = (X2 - X1) / SE_diff` where `SE_diff = SD * sqrt(2 * (1 - reliability))`

**Custom Standard Deviation:**

If you have normative data, use a specific baseline SD:

```bash
prism rci \
  -i merged_data.csv \
  --baseline depression_T1 \
  --followup depression_T2 \
  --reliability 0.85 \
  --baseline-sd 8.5 \
  --id ParticipantID \
  -o rci_results.csv
```

### Real-World Example: Therapy Outcome Study

Let's analyze a depression treatment study with pre/post/follow-up assessments:

```bash
# Step 1: Merge three assessment time points
prism merge \
  --waves Pre:baseline.csv Post:post_treatment.csv FU:followup_6mo.csv \
  --id ParticipantID \
  --join inner \
  -o merged_therapy.csv

# Step 2: Calculate RCI for treatment effect (Pre → Post)
prism rci \
  -i merged_therapy.csv \
  --baseline PHQ9_Pre \
  --followup PHQ9_Post \
  --reliability 0.89 \
  --id ParticipantID \
  -o treatment_rci.csv

# Step 3: Calculate RCI for maintenance (Post → FU)
prism rci \
  -i merged_therapy.csv \
  --baseline PHQ9_Post \
  --followup PHQ9_FU \
  --reliability 0.89 \
  --id ParticipantID \
  -o maintenance_rci.csv

# Step 4: Convert to long format for growth curve analysis
prism reshape \
  -i merged_therapy.csv \
  --format wide-to-long \
  --waves Pre Post FU \
  --id ParticipantID \
  -o long_therapy.csv
```

### Tips for Longitudinal Analysis

**Best Practices:**

1. **Always use `--id`** to specify your participant ID column
2. **Use consistent wave names** (T1/T2/T3 or Pre/Post/FU)
3. **Document reliability coefficients** - Use test-retest reliability from the scale manual
4. **Check merge results** - Use `outer` join first to see who's missing data
5. **Calculate RCI carefully** - Requires good reliability estimate (α ≥ 0.70 recommended)

**Common Reliability Values:**

- PHQ-9: 0.89 (Kroenke et al., 2001)
- GAD-7: 0.83 (Spitzer et al., 2006)
- PSS-10: 0.78 (Cohen & Williamson, 1988)
- BDI-II: 0.93 (Beck et al., 1996)

---

## 📊 Power Analysis for Study Planning (NEW in v0.4.0!)

Never run an underpowered study again! Prism includes statistical power analysis to help you plan studies and report results.

### Calculate Required Sample Size (A Priori)

**Before collecting data**, determine how many participants you need:

```bash
# How many participants for a medium effect with 80% power?
prism power \
  --test independent-t \
  --effect-size 0.5 \
  --power 0.80 \
  --alpha 0.05
```

**Output:**

```
Sample Size:     63 per group
Power:           0.800 (80.0%)
Effect Size:     0.500 (Medium)
```

### Calculate Observed Power (Post-Hoc)

**After data collection**, check if your study had adequate power:

```bash
# What power did I achieve with my sample?
prism power \
  --test correlation \
  --effect-size 0.3 \
  --sample-size 100 \
  --alpha 0.05
```

**Output:**

```
Power:           0.862 (86.2%)
Sample Size:     100
Effect Size:     0.300 (Medium)
Adequate power (≥ 0.80): 86.18%
```

### Supported Test Types

- `independent-t` - Independent samples t-test
- `paired-t` - Paired samples t-test
- `one-sample-t` - One-sample t-test
- `correlation` - Pearson correlation

### Effect Size Guidelines (Cohen, 1988)

**For t-tests (Cohen's d):**

- Small: d = 0.2
- Medium: d = 0.5
- Large: d = 0.8

**For correlations (Pearson's r):**

- Small: r = 0.1
- Medium: r = 0.3
- Large: r = 0.5

### Save Results to File

```bash
prism power \
  --test paired-t \
  --effect-size 0.5 \
  --power 0.80 \
  --output power_analysis.txt
```

### Real-World Examples

**Example 1: Grant Proposal Planning**

You're planning a therapy outcome study. Previous literature shows a medium effect (d = 0.5):

```bash
prism power --test paired-t --effect-size 0.5 --power 0.80
# Result: Need 34 participants
```

Include in your proposal: _"Based on a power analysis (α = .05, power = .80, d = 0.5), we will recruit 34 participants."_

**Example 2: Published Study Power Check**

A published study reports r = 0.25 with n = 50. Did they have adequate power?

```bash
prism power --test correlation --effect-size 0.25 --sample-size 50
# Result: Power = 0.48 (48%)  → Underpowered!
```

**Example 3: One-Tailed Test**

```bash
prism power --test independent-t --effect-size 0.5 --power 0.80 --tails 1
# Result: n = 50 per group (vs. 63 for two-tailed)
```

**Example 4: Stricter Alpha Level**

```bash
prism power --test independent-t --effect-size 0.5 --power 0.80 --alpha 0.01
# Result: n = 90 per group (more conservative)
```

---

## � Data Visualization & HTML Reports (NEW in v0.7.0!)

Get instant visual insights with interactive HTML reports featuring distribution plots, quality dashboards, and professional styling!

### Generate HTML Report

```bash
prism process -i data.csv -c config.toml -o results.csv --format html-report
```

Creates `results_report.html` with:

- ✅ **Overview Dashboard** - Total participants, clean/flagged counts, scale summaries
- ✅ **Interactive Charts** - Distribution histograms for all scales (powered by Chart.js)
- ✅ **Scale Statistics Table** - Mean, SD, Min, Max, N for each scale
- ✅ **Quality Issues Dashboard** - Issue type summaries and detailed participant flags
- ✅ **Professional Styling** - Responsive design, print-friendly, publication-ready

**Perfect for:**

- Quick data exploration before analysis
- Quality assessment dashboards
- Sharing results with collaborators
- Presentations and reports

**Example Output:**

The HTML report includes:

- 📈 Histogram for each scale showing score distributions
- 📊 Color-coded quality badges (clean vs. flagged)
- 📋 Sortable statistics table
- ⚠️ Quality issue breakdown by type
- 🎨 Modern, professional design with Chart.js visualizations

**Why HTML Reports?**

- **No code required** - Just open in any web browser
- **Shareable** - Send to collaborators who don't use R/Python
- **Interactive** - Hover over charts for details
- **Fast** - Get immediate visual feedback on your data quality

---

## 🔬 Reproducible Analysis Scripts (v0.6.0)

Generate complete, ready-to-run analysis scripts in R or Python with visualizations, reliability analysis, and publication-quality outputs!

### R Analysis Script

```bash
prism process -i data.csv -c config.toml -o results.csv --format r
```

Creates `results.R` with:

- ✅ Data import & cleaning with tidyverse
- ✅ Reliability analysis (Cronbach's α with psych)
- ✅ Descriptive statistics
- ✅ Professional ggplot2 visualizations (300 DPI)
- ✅ Correlation matrices
- ✅ Export-ready tables

### Python Analysis Script

```bash
prism process -i data.csv -c config.toml -o results.csv --format python
```

Creates `results.py` with:

- ✅ pandas/numpy data manipulation
- ✅ pingouin for reliability (α with CIs)
- ✅ matplotlib/seaborn visualizations
- ✅ scipy statistical functions
- ✅ Publication-quality plots

### Benefits

🎯 **No Manual Coding** - Complete analysis scripts generated automatically  
📊 **Publication-Quality** - Professional plots at 300 DPI  
🔄 **Fully Reproducible** - Share scripts with your data for transparency  
📝 **Methods Documentation** - Scripts show exactly what was done  
🚀 **Time Savings** - Skip hours of coding for standard analyses

---

## �📖 Need More Help?

### Quick Links

- **[Complete Tutorial](docs/TUTORIAL.md)** - Step-by-step walkthrough with screenshots
- **[FAQ](docs/FAQ.md)** - Common questions answered
- **[Troubleshooting Guide](docs/TROUBLESHOOTING.md)** - Detailed error solutions
- **[Best Practices](docs/BEST_PRACTICES.md)** - Tips from experienced users

### Video Tutorials (Coming Soon!)

We're working on video guides for:

- Setting up your first survey
- Handling Qualtrics exports
- Interpreting quality reports
- Integrating with R/SPSS

---

## 🤝 Contributing

Found a bug? Have a feature idea? Want to improve the docs?

**We'd love your help!** Check out [CONTRIBUTING.md](docs/CONTRIBUTING.md) to get started.

Even fixing a typo in the docs helps. Seriously. 💙

---

## 📝 License

MIT License - Use it however you want! See [LICENSE](LICENSE) for details.

---

## 🙏 Acknowledgments

Built with frustration from spending too many hours in Excel. 😅

Special thanks to every researcher who's ever thought: _"There has to be a better way to do this."_

---

## Quick Start (Again, Because It's That Simple)

1. **Install:** `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
2. **Build:** `cargo build --release`
3. **Generate config:** `prism generate --template > my_config.toml`
4. **Edit config** to match your survey
5. **Run:** `prism process -i data.csv -c my_config.toml -o clean.csv --all-outputs`
6. **Done!** Check your `clean.csv`, `summary_stats.txt`, and `quality_report.txt`

**Questions?** Open an issue on GitHub or check the [FAQ](docs/FAQ.md).

Now go process some data! 🚀

### Running Tests

```bash
cargo test
```

### Building for Production

```bash
# CLI
cargo build --release

# GUI
cd src-tauri
cargo tauri build
```

### Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Submit a pull request

---

## Tech Stack

- **Rust** - Core processing engine
- **csv** - CSV parsing and writing
- **serde + toml** - Configuration parsing
- **clap** - Command-line interface
- **chrono** - Timestamp generation
- **Tauri v2** - Desktop GUI framework (optional)

---

## Future Enhancements

### Planned Features

- [ ] Cronbach's alpha calculation
- [ ] Correlation matrices between scales
- [ ] Pattern responding detection (1-2-3-4-5 sequences)
- [ ] SPSS syntax file generation
- [ ] Support for different CSV delimiters
- [ ] APA-formatted table output

### Advanced Features

- [ ] Longitudinal data analysis
- [ ] Missing data imputation
- [ ] Factor analysis integration
- [ ] Web-based interface

---

## Troubleshooting

### CSV parsing errors

**Issue:** "Could not parse CSV"
**Solution:** Ensure CSV has proper headers and matches item names in config

### Missing items

**Issue:** "Item Q5 not found in CSV"
**Solution:** Check that all items in config exist as columns in CSV

### Out-of-memory errors

**Issue:** Large datasets crash
**Solution:** Process in batches or increase system memory

---

## License

MIT License - see LICENSE file for details

---

## Citation

If you use Prism in your research, please cite:

```
Shuvi. (2026). Prism: Psychology Research Data Processing Pipeline.
GitHub: https://github.com/artlostintime/prism
```

---

## Contact

- **Issues:** Open a GitHub issue
- **GitHub:** [@artlostintime](https://github.com/artlostintime)
- **Twitter:** [@artlostintime](https://twitter.com/artlostintime)

---

## Acknowledgments

Built for psychology researchers who value reproducible, automated data processing workflows.

Special thanks to the Rust community and contributors to the csv, clap, and Tauri projects.
