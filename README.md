# 🌈 Prism: Your Data's New Best Friend

<div align="center">

**Stop manually scoring surveys. Start actually doing research.**

[![Version](https://img.shields.io/badge/version-0.8.7-blue.svg)](https://github.com/artlostintime/prism/releases)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)
[![Tests](https://img.shields.io/badge/tests-229%20passing-brightgreen.svg)](#)
[![Rust](https://img.shields.io/badge/rust-1.92%2B-orange.svg)](https://www.rust-lang.org)

[Quick Start](#-quick-start-in-60-seconds) • [Features](#-what-makes-prism-special) • [Documentation](#-documentation) • [Examples](#-real-world-examples)</div>

---

## 🎭 The Story

Picture this: It's 11 PM. You're in Excel. Again. Manually reverse-scoring item 47 of 200. Again. Wondering if you accidentally flipped items 23-31. Again. Your coffee's cold. Your deadline's tomorrow. And somewhere, deep in your soul, you know there's a better way.

**Enter Prism.**

Think of it as your research assistant who never sleeps, never makes mistakes, and can process 1,000 participants' worth of survey data in the time it takes you to say "wait, which items are reverse-scored again?"

---

## ✨ What Makes Prism Special

### For the "I Just Need It to Work" Crowd

- 🎯 **Pre-built Scale Library** - PHQ-9, GAD-7, PSS, PANAS, BDI-II, BAI, SWLS ready to go (with proper citations!)
- 🔄 **Automatic Reverse Scoring** - Never manually calculate `(max+min)-value` again
- ⚡ **Lightning Fast** - Process 1,000 participants in under a second
- 🛡️ **Quality Control Ninja** - Catches straightliners, speeders, and careless responders
- 📊 **Multiple Export Formats** - CSV, Excel, SPSS, R, Python, JSON

### For the "Show Me the Math" Crowd

- ✅ **Verified Formulas** - All 229 tests passing, math verified against academic literature
- 📈 **Cronbach's Alpha** - Calculated correctly (with Bessel's correction, naturally)
- 🔬 **RCI Calculations** - Reliable Change Index for clinical significance
- 💪 **Power Analysis** - Built-in sample size planning (Cohen would be proud)
- 🧪 **Reproducible** - Every calculation documented and verifiable

### For the "I Have Messy Data" Crowd

- 🔍 **Pattern Detection** - Finds diagonal patterns (1,2,3,4,5), alternating (1,5,1,5), and block responses
- 🚨 **Quality Flags** - Automatically labels suspicious participants
- 📋 **CONSORT Flow** - Generate participant exclusion flowcharts
- 🎨 **Interactive Reports** - Beautiful HTML dashboards with charts
- 🧹 **Column Mapping** - Handle those awful Qualtrics export names

---

## 🚀 Quick Start (in 60 Seconds)

### Step 1: Get Rust

**Mac/Linux:**

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

**Windows:** Download from [rustup.rs](https://rustup.rs) (it's a 2-click install)

### Step 2: Build Prism

```bash
cargo build --release
```

_Go grab that coffee now. This takes ~2 minutes on first build._

### Step 3: Run Your First Analysis

```bash
# Generate a config for PHQ-9 (depression scale)
./target/release/prism generate --scale PHQ-9 > my_study.toml

# Process your data
./target/release/prism process \
  -i your_data.csv \
  -c my_study.toml \
  -o results.csv \
  --all-outputs
```

**BAM.** You just:

- ✅ Reverse-scored all items correctly
- ✅ Calculated scale totals and means
- ✅ Ran quality checks
- ✅ Computed Cronbach's alpha
- ✅ Generated a statistical summary

What used to take 45 minutes in Excel just happened in 0.3 seconds. Welcome to the future. 🎉

---

## 🎯 Real-World Examples

### Example 1: "I Have a Pre-Built Scale"

```bash
# See what's available
prism generate --list-scales
# Output: PHQ-9, GAD-7, PSS-10, PSS-14, PANAS, BDI-II, BAI, SWLS

# Generate config for anxiety screening
prism generate --scale GAD-7 > anxiety_study.toml

# Process data
prism process -i responses.csv -c anxiety_study.toml -o clean.csv --all-outputs
```

**You get:**

- `clean.csv` - Data with computed scores
- `summary_stats.txt` - Means, SDs, Cronbach's α
- `quality_report.txt` - Red flags and suspicious patterns

### Example 2: "I Built My Own Survey"

```bash
# Generate template
prism generate --template > burnout.toml
```

Edit the config:

```toml
[survey]
name = "Teacher Burnout Study"
min_score = 1  # "Never"
max_score = 7  # "Every day"

[scales.emotional_exhaustion]
items = ["Q1", "Q2", "Q3", "Q6", "Q8", "Q13"]
reverse_scored = []  # None

[scales.cynicism]
items = ["Q4", "Q5", "Q7", "Q9"]
reverse_scored = ["Q7"]  # Just Q7 flips

[quality]
max_missing_percent = 0.10  # Flag if >10% missing
flag_straightlining = true  # Catch "3,3,3,3" responders
```

```bash
prism process -i burnout_data.csv -c burnout.toml -o results.csv --all-outputs
```

### Example 3: "I Need SPSS Syntax"

```bash
prism process -i data.csv -c config.toml -o results.csv --format spss
```

Gets you production-ready SPSS syntax with:

- Variable labels
- Value labels (1="Strongly Disagree", etc.)
- Reverse-scoring transformations (documented!)
- COMPUTE statements for scales
- Example RELIABILITY and DESCRIPTIVES commands

**Just open in SPSS and run.** No copy-paste errors, no missing steps.

### Example 4: "I Have Longitudinal Data"

```bash
# Merge three waves
prism merge \
  --waves Pre:baseline.csv Post:treatment.csv FU:followup.csv \
  --id ParticipantID \
  --join outer \
  -o merged.csv

# Calculate clinical significance
prism rci \
  -i merged.csv \
  --baseline depression_Pre \
  --followup depression_Post \
  --reliability 0.89 \
  -o rci_results.csv

# Convert to long format for multilevel modeling
prism reshape \
  -i merged.csv \
  --format wide-to-long \
  --waves Pre Post FU \
  --id ParticipantID \
  -o long_format.csv
```

---

## 🎨 Features That'll Make You Smile

### 🔍 **Detective Mode: Quality Control**

Prism doesn't just process data—it investigates it.

**Catches:**

- **Straightliners**: Someone answered "3" to _every_ question
- **Diagonal Doomscrollers**: 1,2,3,4,5,6,7 in perfect sequence (really?)
- **Alternators**: 1,5,1,5,1,5 pattern (not paying attention)
- **Block Responders**: First half all 1s, second half all 5s (suspicious!)
- **Speeders**: Completed 50 items in 30 seconds (did they even read?)

**Example Report:**

```
Quality Report for Survey "Teacher Burnout"
==========================================
Total participants: 342
Flagged for quality issues: 18 (5.3%)

Issues detected:
- Straightlining: 8 participants (all responses identical)
- Diagonal patterns: 3 participants
- Excessive missing data: 7 participants (>10% missing)
```

### 📊 **The Math Checks Out**

Every formula is verified against published literature:

- **Cronbach's Alpha**: Implemented per Tavakol & Dennick (2011)
- **Variance**: Uses Bessel's correction (n-1 denominator)
- **Reverse Scoring**: (max + min) - value
- **RCI**: Jacobson & Truax (1991) criteria
- **Power Analysis**: Cohen (1988) effect sizes

With **229 tests passing** at 100% success rate. Because nobody has time for incorrect statistics.

### 🎨 **Beautiful Reports**

Generate interactive HTML dashboards:

```bash
prism process -i data.csv -c config.toml -o results.csv --html report.html
```

You get:

- 📈 Distribution plots with normality indicators
- 🎯 Scale reliability tables (with color-coded thresholds)
- 🚨 Quality issue visualizations
- 📉 Score distributions across scales
- 💾 Downloadable data tables

Perfect for sharing with collaborators who don't speak "CSV."

---

## ⚙️ Configuration: Make It Yours

### The Anatomy of a Config File

```toml
[survey]
name = "My Awesome Study"
min_score = 1     # Lowest response option
max_score = 5     # Highest response option

# Define your scales
[scales.depression]
items = ["PHQ1", "PHQ2", "PHQ3", "PHQ4", "PHQ5",
         "PHQ6", "PHQ7", "PHQ8", "PHQ9"]
reverse_scored = []  # None for PHQ-9

[scales.anxiety]
items = ["GAD1", "GAD2", "GAD3", "GAD4", "GAD5", "GAD6", "GAD7"]
reverse_scored = []  # None for GAD-7

# Quality control settings
[quality]
max_missing_percent = 0.10      # Flag if >10% missing
flag_straightlining = true       # Detect identical responses
flag_diagonal = true             # Detect sequential patterns (1,2,3,4...)
flag_alternating = true          # Detect oscillating (1,5,1,5...)
consecutive_threshold = 8        # How many identical in a row = suspicious

# Output customization
[output]
include_summary_stats = true
include_cronbach_alpha = true
include_quality_flags = true
```

### Pro Tips 💡

**1. Column Mapping** - Handle messy Qualtrics exports:

```toml
[column_mappings]
"Q1_1" = "PHQ1"
"Q1_2" = "PHQ2"
"QID47" = "GAD1"
```

**2. Multiple Participant IDs** - Support different naming:

```toml
[survey]
id_columns = ["ParticipantID", "SubjectID", "ResponseID"]
```

**3. Custom Missing Values** - Define what counts as missing:

```toml
[survey]
missing_values = ["NA", "-999", ""]
```

---

## 🧪 The Nitty-Gritty: Understanding Your Output

### 1. **Processed Data CSV**

Your original data + computed scales:

```csv
ParticipantID,PHQ1,PHQ2,...,depression_total,depression_mean,anxiety_total,...
001,2,1,3,...,11,1.22,8,...
002,0,0,1,...,4,0.44,12,...
```

### 2. **Summary Statistics**

```
Scale: depression (PHQ-9)
==========================================
N: 342
Mean: 7.82 (SD = 5.41)
Range: 0-27
Cronbach's α: 0.89 (Excellent internal consistency!)

Scale: anxiety (GAD-7)
==========================================
N: 342
Mean: 6.15 (SD = 4.92)
Range: 0-21
Cronbach's α: 0.91 (Excellent!)
```

**Interpreting Cronbach's α:**

- α ≥ 0.90: Excellent
- 0.80 ≤ α < 0.90: Good
- 0.70 ≤ α < 0.80: Acceptable
- α < 0.70: Questionable (⚠️ check your scale!)

### 3. **Quality Report**

Lists every participant flagged and why:

```
Participant ID: 078
Issues: Straightlining (all responses = 3), Excessive missing (15% missing)
Action: Consider excluding from analysis

Participant ID: 142
Issues: Diagonal pattern detected (items 1-10)
Action: Review manually
```

### 4. **SPSS-Ready Syntax**

Get production-quality SPSS code:

```spss
* Generated by Prism v0.8.7
* Survey: Teacher Burnout Study
* Generated: 2024-01-15

* Reverse score items
COMPUTE PHQ2_R = (4 + 0) - PHQ2.
EXECUTE.

* Calculate scale scores
COMPUTE depression_total = SUM(PHQ1, PHQ2_R, PHQ3, PHQ4, ...).
COMPUTE depression_mean = MEAN(PHQ1, PHQ2_R, PHQ3, PHQ4, ...).
EXECUTE.

* Reliability analysis
RELIABILITY
  /VARIABLES=PHQ1 PHQ2_R PHQ3 PHQ4 PHQ5 PHQ6 PHQ7 PHQ8 PHQ9
  /SCALE('Depression') ALL
  /MODEL=ALPHA
  /STATISTICS=DESCRIPTIVE SCALE
  /SUMMARY=TOTAL.
```

Just copy-paste into SPSS. No typos, no missing steps, no tears.

---

## 🔧 Troubleshooting (When Things Go Sideways)

### "My column names don't match!"

Use column mappings in your config:

```toml
[column_mappings]
"Q1_Text" = "PHQ1"
"Q2_Text" = "PHQ2"
```

### "Some responses are -999, not blank"

Define custom missing values:

```toml
[survey]
missing_values = ["-999", "NA", ""]
```

### "I have both 'ID' and 'ParticipantID' columns"

Tell Prism to check both:

```toml
[survey]
id_columns = ["ID", "ParticipantID", "SubjectID"]
```

### "Cronbach's α is negative?!"

This usually means:

1. Forgot to reverse-score items ← **Most common!**
2. Items are measuring different constructs
3. Data entry errors

**Quick fix:** Double-check your `reverse_scored` list!

### "The build failed!"

Make sure you have Rust 1.92+ installed:

```bash
rustc --version  # Should show 1.92 or higher
rustup update    # Updates Rust to latest
```

---

## 🌊 Longitudinal Data: When Time Matters

Track change across measurement occasions with ease.

### Merge Multiple Waves

```bash
prism merge \
  --waves Baseline:wave1.csv Month3:wave2.csv Month6:wave3.csv \
  --id ParticipantID \
  --join outer \  # Keep all participants
  -o merged.csv
```

**Result:** One file with columns like `depression_Baseline`, `depression_Month3`, `depression_Month6`

### Calculate Reliable Change

Is the change clinically significant or just noise?

```bash
prism rci \
  -i merged.csv \
  --baseline depression_Baseline \
  --followup depression_Month6 \
  --reliability 0.89 \  # Cronbach's α from your data
  -o rci_results.csv
```

**Output includes:**

- `change_score`: Raw difference
- `rci_value`: Reliable Change Index
- `clinically_significant`: TRUE/FALSE
- `improvement_category`: "Recovered", "Improved", "No change", "Deteriorated"

**Interpretation Guide:**

```
|RCI| < 1.96: Change could be measurement error
|RCI| ≥ 1.96: Statistically reliable change (p < .05)

Clinical significance: Did they cross a clinical threshold?
- Recovered: Reliable improvement + moved into normal range
- Improved: Reliable improvement but still clinical
- No change: RCI not significant
- Deteriorated: Reliable worsening
```

### Reshape for Analysis

**Wide to Long** (for multilevel modeling):

```bash
prism reshape \
  -i merged.csv \
  --format wide-to-long \
  --waves Baseline Month3 Month6 \
  --id ParticipantID \
  -o long_format.csv
```

Before (wide):

```csv
ID,depression_Baseline,depression_Month3,depression_Month6
001,15,10,8
```

After (long):

```csv
ID,Time,depression
001,Baseline,15
001,Month3,10
001,Month6,8
```

**Long to Wide** (for repeated measures ANOVA):

```bash
prism reshape \
  -i long_format.csv \
  --format long-to-wide \
  --value-column depression \
  --time-column Time \
  --id ParticipantID \
  -o wide_format.csv
```

---

## 💪 Power Analysis: Plan Smarter Studies

Stop collecting "as many as we can get" and start with an actual plan.

### A Priori Power (Sample Size Planning)

**Scenario:** You're planning an RCT to reduce anxiety.

```bash
prism power \
  --effect-size 0.5 \      # Expecting medium effect (Cohen's d)
  --alpha 0.05 \            # Significance level
  --power 0.80 \            # Desired power (80%)
  --test independent-t \    # Two independent groups
  --output plan.txt
```

**Result:**

```
Recommended sample size: N = 128 (64 per group)

Effect size interpretation:
- Small effect (d = 0.2): ~394 participants needed
- Medium effect (d = 0.5): ~128 participants needed
- Large effect (d = 0.8): ~52 participants needed

At N=128 with α=0.05:
- Power to detect d=0.5: 80%
- Power to detect d=0.3: 45% (underpowered!)
- Power to detect d=0.8: 97% (comfortable margin)
```

### Post-Hoc Power (What did we actually have?)

**Scenario:** You collected N=75 but found d=0.35.

```bash
prism power \
  --effect-size 0.35 \
  --sample-size 75 \
  --alpha 0.05 \
  --test independent-t \
  --output posthoc.txt
```

**Result:**

```
Observed effect size: d = 0.35 (small-to-medium)
Sample size: N = 75
Statistical power: 48%

Interpretation: You had less than 50% chance of detecting
this effect even if it was real. Consider this finding
preliminary and plan replication with N ≥ 200.
```

### Real-World Example

```bash
# Planning a paired-samples pre-post design
prism power \
  --effect-size 0.6 \       # Expecting medium-large effect
  --alpha 0.05 \
  --power 0.90 \            # Want 90% power (high confidence)
  --test paired-t \
  --output prepost_plan.txt
```

**Decision tree:**

- d = 0.8 (large): Need ~19 participants
- d = 0.5 (medium): Need ~44 participants
- d = 0.3 (small): Need ~118 participants

**Pro tip:** Always plan for 20% attrition. If you need 44, recruit 53.

---

## 📊 Data Dictionary: Document Everything

Good documentation = future you will thank present you.

### Generate CSV Format

```bash
prism dict \
  -i processed_data.csv \
  --format csv \
  -o data_dictionary.csv
```

**Output:**

```csv
Variable,Type,Description,Valid_Range,Missing,Example
ParticipantID,String,Unique identifier,,0,P001
PHQ1,Integer,Little interest or pleasure,0-3,2,1
PHQ2,Integer,Feeling down or hopeless,0-3,1,2
depression_total,Integer,PHQ-9 total score,0-27,0,11
depression_mean,Float,PHQ-9 mean score,0.0-3.0,0,1.22
quality_flag,Boolean,Data quality issue detected,TRUE/FALSE,0,FALSE
```

### Generate JSON Format (for codebooks)

```bash
prism dict \
  -i processed_data.csv \
  --format json \
  --include-stats \
  -o codebook.json
```

**Output:**

```json
{
  "variables": [
    {
      "name": "depression_total",
      "type": "integer",
      "description": "PHQ-9 total score",
      "valid_range": [0, 27],
      "statistics": {
        "n": 342,
        "mean": 7.82,
        "sd": 5.41,
        "min": 0,
        "max": 27,
        "missing": 0
      },
      "interpretation": {
        "0-4": "Minimal depression",
        "5-9": "Mild depression",
        "10-14": "Moderate depression",
        "15-19": "Moderately severe",
        "20-27": "Severe depression"
      }
    }
  ]
}
```

Perfect for uploading to data repositories (OSF, Dataverse) or including in supplementary materials.

---

## 🔬 Reproducible Analysis Scripts

Generate ready-to-run R or Python code.

### R Script

```bash
prism process \
  -i data.csv \
  -c config.toml \
  -o results.csv \
  --format r
```

**Gets you:**

```r
# Generated by Prism v0.8.7
# Date: 2024-01-15

library(tidyverse)
library(psych)

# Load data
data <- read.csv("results.csv")

# Descriptive statistics
describe(data[, c("depression_total", "anxiety_total")])

# Reliability analysis
alpha(data[, c("PHQ1", "PHQ2", "PHQ3", ...)])

# Visualization
ggplot(data, aes(x = depression_total)) +
  geom_histogram(binwidth = 2, fill = "skyblue", color = "black") +
  labs(title = "PHQ-9 Distribution",
       x = "Depression Score",
       y = "Frequency") +
  theme_minimal()
```

### Python Script

```bash
prism process \
  -i data.csv \
  -c config.toml \
  -o results.csv \
  --format python
```

**Gets you:**

```python
# Generated by Prism v0.8.7
import pandas as pd
import matplotlib.pyplot as plt
from scipy.stats import describe
import pingouin as pg

# Load data
df = pd.read_csv("results.csv")

# Descriptive statistics
print(df[["depression_total", "anxiety_total"]].describe())

# Cronbach's alpha
items = ["PHQ1", "PHQ2", "PHQ3", "PHQ4", "PHQ5",
         "PHQ6", "PHQ7", "PHQ8", "PHQ9"]
alpha = pg.cronbach_alpha(df[items])
print(f"Cronbach's α: {alpha[0]:.3f}")

# Distribution plot
plt.hist(df["depression_total"], bins=20, edgecolor="black")
plt.xlabel("Depression Score")
plt.ylabel("Frequency")
plt.title("PHQ-9 Distribution")
plt.show()
```

Perfect for "Methods" sections: _"Data were processed using Prism v0.8.7. Analysis code is available at [link]."_

---

## 📚 Documentation & Resources

### Learn More

- **[Full Tutorial](docs/TUTORIAL.md)** - Step-by-step walkthrough
- **[API Reference](docs/API_REFERENCE.md)** - Complete command list
- **[Configuration Guide](docs/CONFIGURATION_GUIDE.md)** - All the options
- **[Best Practices](docs/BEST_PRACTICES.md)** - Tips from real studies
- **[Architecture](docs/ARCHITECTURE.md)** - How it works under the hood

### Pre-Built Scales Documentation

Each scale comes with:

- ✅ Original citation
- ✅ Scoring algorithm
- ✅ Interpretation guidelines
- ✅ Normative data (where available)

See [Scale Library Documentation](docs/SCALE_LIBRARY.md) for details on PHQ-9, GAD-7, PSS, PANAS, BDI-II, BAI, and SWLS.

### Research Using Prism

If you use Prism in your research, we'd love to hear about it! Drop us a line or open an issue to share your publication.

**Suggested citation:**

```
Data processing performed using Prism v0.8.7
(https://github.com/artlostintime/prism), an open-source
tool for survey data analysis.
```

---

## 🧪 Testing (Yes, We Test Our Tests)

**229 tests. 100% passing. Always.**

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test suite
cargo test mathematical_validation
cargo test quality_checks
cargo test longitudinal
```

### What We Test

- ✅ **Mathematical correctness** - Formulas match published literature
- ✅ **Edge cases** - Empty data, single participant, all missing
- ✅ **Reverse scoring** - Every possible configuration
- ✅ **Quality detection** - Pattern recognition accuracy
- ✅ **Longitudinal merges** - Different join types
- ✅ **Power calculations** - Against G\*Power reference values
- ✅ **File I/O** - CSV, Excel, JSON, TOML parsing
- ✅ **Stress tests** - 10,000 participants, 100 items

See [TESTING.md](docs/TESTING.md) for comprehensive test coverage report.

---

## 🤝 Contributing

Found a bug? Have an idea? Want to add a scale?

### Quick Contributions

- 🐛 **Bug reports:** [Open an issue](https://github.com/artlostintime/prism/issues)
- 💡 **Feature requests:** [Start a discussion](https://github.com/artlostintime/prism/discussions)
- 📚 **Documentation:** Found a typo? PRs welcome!
- 🎓 **Add a scale:** See [Contributing Guide](docs/CONTRIBUTING.md)

### Development Setup

```bash
# Clone the repo
git clone https://github.com/artlostintime/prism.git
cd prism

# Run tests
cargo test

# Check code quality
cargo clippy -- -D warnings
cargo fmt --check

# Build docs
cargo doc --no-deps --open
```

### Wanted: These Features Next

- [ ] GUI application (in progress! 🎨)
- [ ] Factor analysis / PCA
- [ ] Missing data imputation
- [ ] Multilevel model prep
- [ ] Mediation/moderation setup
- [ ] More pre-built scales (send suggestions!)

See [ROADMAP.md](ROADMAP.md) for the full plan.

---

## 🛠️ Tech Stack (For the Curious)

Built with Rust for speed and reliability:

- **CLI:** [clap](https://crates.io/crates/clap) - Elegant command-line parsing
- **Data:** [csv](https://crates.io/crates/csv), [serde](https://crates.io/crates/serde) - Fast CSV processing
- **Excel:** [rust_xlsxwriter](https://crates.io/crates/rust_xlsxwriter) - Native Excel export
- **Stats:** Custom implementation (all formulas verified!)
- **Parallel:** [rayon](https://crates.io/crates/rayon) - Multi-core processing
- **Testing:** [proptest](https://crates.io/crates/proptest) - Property-based tests

**Why Rust?**

- 🚀 **Fast:** Process 1,000 participants in <1 second
- 🛡️ **Safe:** Compile-time guarantees prevent data corruption
- 📦 **Single binary:** No Python environment hell
- 🔧 **Reliable:** If it compiles, it works

---

## 🚀 Future Enhancements

### Coming Soon™

- 🎨 **GUI Version** - Click-based interface (no terminal required)
- 📊 **Factor Analysis** - Exploratory and confirmatory
- 🧬 **Missing Data** - Multiple imputation
- 🌐 **Web Version** - Run in browser
- 🔗 **REDCap Integration** - Direct API connection
- 📱 **Mobile Data Collection** - Built-in survey app

### Dream Features (Help Welcome!)

- Real-time data monitoring dashboards
- Automated outlier detection with ML
- Natural language config ("Calculate PHQ-9 from columns A-I")
- Integration with OSF, Dataverse, and other repositories

---

## 📝 License

MIT License - Use it however you want! See [LICENSE](LICENSE) for details.

**Translation:** Free for academic, commercial, personal use. No attribution required (but appreciated!).

---

## 💌 Contact & Support

- **Issues:** [GitHub Issues](https://github.com/artlostintime/prism/issues)
- **Discussions:** [GitHub Discussions](https://github.com/artlostintime/prism/discussions)
- **Email:** [your email or organization]
- **Twitter/X:** [@yourusername]

---

<div align="center">

**Made with ❤️ by researchers, for researchers**

_Because life's too short for Excel formulas_

[⬆ Back to Top](#-prism-your-datas-new-best-friend)

</div>
[survey]
name = "My Burnout Study"
min_score = 1    # Your scale's minimum (e.g., 1 = "Strongly Disagree")
max_score = 7    # Your scale's maximum (e.g., 7 = "Strongly Agree")

[quality]
max_missing_percent = 0.10 # Flag if someone skips >10% of items
flag_straightlining = true # Catch people who answer "3" to everything

[scales.emotional_exhaustion]
items = ["Q1", "Q2", "Q3", "Q4", "Q5", "Q6", "Q7", "Q8", "Q9"]
reverse_scored = [] # None are reversed

[scales.depersonalization]
items = ["Q10", "Q11", "Q12", "Q13", "Q14"]
reverse_scored = ["Q12"] # Q12 is reverse-scored

[scales.personal_accomplishment]
items = ["Q15", "Q16", "Q17", "Q18", "Q19", "Q20"]
reverse_scored = ["Q15", "Q16", "Q17", "Q18", "Q19", "Q20"] # All reversed!

````

### 2️⃣ Process Your Data

```bash
prism process -i my_data.csv -c my_survey.toml -o clean_data.csv --all-outputs
````

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

## � Data Dictionary Export (NEW in v0.8.0!)

Generate comprehensive documentation of your dataset variables to support reproducibility and data sharing!

### Export Data Dictionary

```bash
# CSV format (default)
prism dictionary --config study_config.toml --output data_dictionary.csv

# JSON format for programmatic use
prism dictionary --config study_config.toml --output data_dictionary.json --format json
```

### What's Included

The data dictionary documents **all variables** in your output dataset:

- ✅ **Participant ID** - Type, description, notes
- ✅ **Individual Items** - Scale membership, value range, reverse-scoring status
- ✅ **Scale Totals** - Item counts, reverse-scoring annotations
- ✅ **Scale Means** - Value ranges, computation formulas
- ✅ **Quality Flags** - Description of automated quality checks

### CSV Format Example

```csv
Variable,Description,Type,Scale_Membership,Value_Range,Reverse_Scored,Notes
ID,Participant identifier,ID,,,No,Unique identifier for each participant
WAI_1,Survey item,Item,alliance_total,1-7,No,Raw item response
WAI_4,Survey item,Item,alliance_total,1-7,Yes,Raw item response (will be reverse-scored)
alliance_total_total,Scale total score,Computed,alliance_total,Continuous,No,Sum of 12 items (after reverse-scoring 2 items)
alliance_total_mean,Scale mean score,Computed,alliance_total,1-7,No,Mean of 12 items (total / 12)
quality_flag,Quality control flags,Flag,Quality,Varies,No,Automated quality checks (OK if no issues)
```

### JSON Format Example

```json
{
  "survey": {
    "name": "Clinical Interactions & Trainee Well-Being",
    "min_score": 1,
    "max_score": 7
  },
  "variables": [
    {
      "variable": "WAI_4",
      "description": "Survey item",
      "type": "Item",
      "scale_membership": "alliance_total",
      "value_range": "1-7",
      "reverse_scored": true,
      "notes": "Raw item response (will be reverse-scored)"
    }
  ],
  "scales": [
    {
      "name": "alliance_total",
      "items": ["WAI_1", "WAI_2", ..., "WAI_12"],
      "reverse_scored": ["WAI_4", "WAI_10"],
      "item_count": 12
    }
  ],
  "quality_checks": [
    "Missing data detection",
    "Straightlining detection",
    ...
  ]
}
```

### Why Use Data Dictionaries?

- **Reproducibility** - Document every variable for future reference
- **Data Sharing** - Help collaborators understand your dataset
- **Publications** - Include as supplementary material
- **Compliance** - Meet funder/journal data documentation requirements
- **Onboarding** - Help new team members understand data structure

**Perfect for:**

- Open science / data sharing initiatives
- Pre-registration and registered reports
- Grant proposals (data management plans)
- Thesis/dissertation appendices
- Collaborations with non-experts

---

## �🔬 Reproducible Analysis Scripts (v0.6.0)

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

---

## 🧪 Testing & Development

### Comprehensive Test Dataset

Prism includes a large realistic test dataset (620 participants) with:

- **11 psychology scales** (PHQ-9, GAD-7, PSS-10, PANAS, Wellbeing)
- **Quality issues** (~900 intentionally injected for testing detection)
- **Edge cases** (missing data, min/max values, patterns)

**Quick test:**

```bash
python examples/generate_large_dataset.py
.\test_large_dataset.ps1
```

**Or run manually:**

```bash
cargo run --release -- process \
  -i data/test_dataset_large.csv \
  -c test_large_config.toml \
  -o data/processed/test_large_output.csv \
  --stats-output data/processed/test_large_stats.txt \
  --quality-report data/processed/test_large_quality.txt
```

📖 **See:** [examples/LARGE_DATASET_README.md](examples/LARGE_DATASET_README.md) for details

### Run All Tests

```bash
cargo test --release
```

**Test coverage:** 227 tests covering:

- Scale computation & reverse scoring
- Quality detection (straightlining, patterns, missing data)
- Statistical calculations (Cronbach's Alpha, M, SD)
- Edge cases & error handling
- Performance benchmarks

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

## 📚 Documentation

- **[Getting Started](docs/INSTALLATION.md)** - Installation and setup
- **[User Guide](docs/HOW_TO_USE.md)** - Complete usage guide
- **[Configuration](docs/CONFIGURATION_GUIDE.md)** - Config file reference
- **[API Reference](docs/API_REFERENCE.md)** - Developer documentation
- **[Release Notes](docs/releases/)** - Version history and changelogs
- **[Development History](docs/archive/)** - Technical reports and refactoring docs

---

## Citation

If you use Prism in your research, please cite:

```
Shuvi. (2026). Prism: Psychology Research Data Processing Pipeline (v0.8.5).
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
