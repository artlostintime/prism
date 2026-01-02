# Frequently Asked Questions

**[📚 Wiki Home](README.md)** | **[📖 How to Use](HOW_TO_USE.md)** | **[🔧 Configuration](CONFIGURATION_GUIDE.md)** | **[🐛 Troubleshooting](TROUBLESHOOTING.md)**

---

## Table of Contents

- [General Questions](#general-questions)
- [Installation & Setup](#installation--setup)
- [Configuration](#configuration)
- [Data Processing](#data-processing)
- [Quality Checks](#quality-checks)
- [Output Files](#output-files)
- [Statistical Questions](#statistical-questions)
- [Troubleshooting](#troubleshooting)

---

## General Questions

### What is Prism?

Prism is a command-line tool for automating psychology survey data processing. It handles reverse scoring, scale calculations, quality checks, and statistical reporting.

### Who is it for?

- Psychology researchers
- Research assistants
- Graduate students
- Anyone processing Likert-scale survey data

### What file formats does it support?

**Input:**

- CSV files (with headers)
- TOML configuration files

**Output:**

- CSV (clean data with scale scores)
- TXT (summary statistics and quality reports)

### Is it free?

Yes! Prism is open-source under the MIT license.

### Does it have a graphical interface?

Yes, Prism includes a minimal GUI wrapper, but the CLI is recommended for most workflows.

---

## Installation & Setup

### How do I install Prism?

See the [Installation Guide](INSTALLATION.md). Quick version:

```bash
# From source
git clone https://github.com/artlostintime/prism.git
cd prism
cargo build --release
```

### Do I need to know programming?

No! You just need to:

1. Create a config file (simple text format)
2. Run one command
3. Open the output in Excel/R/SPSS

### What are the system requirements?

- Any modern computer (Windows/Mac/Linux)
- 512 MB RAM minimum
- Rust (if building from source)

### Can I use it on multiple computers?

Yes! Just copy the binary or install on each machine.

---

## Configuration

### How do I create a config file?

See [Configuration Guide](CONFIGURATION_GUIDE.md) or use this template:

```toml
[survey]
name = "My Survey"

[quality]
missing_threshold = 10.0

[scales.my_scale]
items = ["q1", "q2", "q3"]
```

### What if my items need reverse scoring?

Add them to the config:

```toml
[scales.my_scale]
items = ["q1", "q2", "q3", "q4"]
reverse_scored = ["q2", "q4"]
```

### How does reverse scoring work?

Formula: `(max + min) - original_value`

Example (1-5 scale):

- Original: 5 → Reversed: (5+1)-5 = 1
- Original: 2 → Reversed: (5+1)-2 = 4

### Can I have multiple scales?

Yes! Add multiple scale sections:

```toml
[scales.scale_a]
items = ["a1", "a2"]

[scales.scale_b]
items = ["b1", "b2"]
```

### What if my scales use different response formats?

Specify min/max for each:

```toml
[scales.likert_5]
items = ["q1", "q2"]
min_value = 1.0
max_value = 5.0

[scales.likert_7]
items = ["q3", "q4"]
min_value = 1.0
max_value = 7.0
```

---

## Data Processing

### What format should my CSV be in?

**Requirements:**

- First row = column headers
- First column = participant ID
- Subsequent columns = item responses
- Numeric values only (empty for missing)

**Example:**

```csv
participant_id,q1,q2,q3,q4
001,5,4,3,5
002,3,2,,4
```

### How are missing values handled?

- Empty cells are treated as missing
- Don't use "NA", "N/A", or other text
- Missing data percentage is calculated per participant

### Can I process multiple files at once?

Yes, use a loop:

**Bash:**

```bash
for file in *.csv; do
  prism -i "$file" -c config.toml -o "clean_$file"
done
```

**PowerShell:**

```powershell
Get-ChildItem *.csv | ForEach-Object {
  prism -i $_.Name -c config.toml -o "clean_$($_.Name)"
}
```

### Does it modify my original data?

No! Original files are never modified. All output goes to new files.

### How fast is it?

Very fast! Typical processing times:

- 100 participants: < 1 second
- 1,000 participants: 1-2 seconds
- 10,000 participants: 5-10 seconds

---

## Quality Checks

### What quality checks are performed?

1. **Straightlining:** Same response to all items in a scale
2. **Missing data:** Percentage of skipped items
3. **Out-of-range:** Values outside valid scale range

See [Quality Checks Guide](QUALITY_CHECKS.md) for details.

### Should I always exclude flagged participants?

No. Review each flag in context:

- Straightlining may be valid (floor/ceiling effects)
- Some missing data is acceptable
- Out-of-range suggests data errors

### How do I adjust quality thresholds?

In your config:

```toml
[quality]
missing_threshold = 15.0  # More lenient
straightlining_enabled = false  # Disable if needed
```

### Can I add custom quality checks?

Not currently, but this is on the roadmap. You can export to R/Python for additional checks.

---

## Output Files

### What files does Prism create?

1. **Clean CSV:** Original data + scale scores
2. **Summary stats (optional):** Aggregate statistics
3. **Quality report (optional):** Flagged participants

### What are "total" vs "mean" scores?

- **Total:** Sum of all items (e.g., 3+4+5 = 12)
- **Mean:** Average of all items (e.g., 12/3 = 4.0)

Both are included in output.

### What statistics are calculated?

**Per participant:**

- Scale total
- Scale mean

**Aggregate (in summary stats):**

- Mean (M)
- Standard deviation (SD) - sample SD (n-1)
- Minimum
- Maximum
- Sample size (N)

### Can I import the output to SPSS/R/Python?

Yes! The clean CSV can be imported to any statistical software:

**SPSS:**

```
GET DATA /TYPE=TXT /FILE='clean_data.csv' /DELIMITERS=",".
```

**R:**

```r
data <- read.csv("clean_data.csv")
```

**Python:**

```python
import pandas as pd
df = pd.read_csv("clean_data.csv")
```

---

## Statistical Questions

### What standard deviation formula is used?

**Sample SD (n-1):** `sqrt(sum((x - mean)²) / (n - 1))`

This is the standard for sample data (vs. population).

### Can it calculate Cronbach's alpha?

Not yet, but planned for future releases. Currently export to R/Python:

```r
library(psych)
alpha(data[, c("q1", "q2", "q3")])
```

### Does it handle weighted scores?

No. All items are weighted equally (simple sum/mean).

### Can it compute composite scores?

Yes, by defining a scale with all items:

```toml
[scales.composite]
items = ["scale_a1", "scale_a2", "scale_b1", "scale_b2"]
```

### What about factor scores?

Not currently. Export to R/lavaan or Python/factor_analyzer for factor analysis.

---

## Troubleshooting

### "Column not found" error

**Problem:** Config references column that doesn't exist in CSV.

**Solution:**

- Check column names match exactly (case-sensitive)
- Ensure no spaces/typos
- Verify CSV has headers

### "Failed to parse config"

**Problem:** TOML syntax error.

**Solution:**

- Check brackets: `[scales.name]`
- Check quotes: `name = "value"`
- Check arrays: `items = ["a", "b"]`

### "No valid numeric data"

**Problem:** Non-numeric values in scale columns.

**Solution:**

- Ensure scale items contain only numbers
- Use empty cells for missing (not "NA")
- Check for text responses

### Output file is empty

**Problem:** Processing failed but no error shown.

**Solution:**

- Check for validation errors in console
- Verify config has at least one scale
- Ensure CSV has data rows (not just headers)

### "Permission denied" when saving

**Problem:** Output file is open in another program.

**Solution:**

- Close Excel/other programs using the file
- Use a different output filename
- Check folder write permissions

See [Troubleshooting Guide](TROUBLESHOOTING.md) for more solutions.

---

## Advanced Questions

### Can I use it in automated workflows?

Yes! Prism is designed for automation:

```bash
# Cron job example (Linux/Mac)
0 2 * * * cd /path/to/project && prism -i data.csv -c config.toml -o output.csv
```

### Can I integrate it with my survey platform?

Yes! Export CSV from your platform, then process with Prism:

```bash
# Example: Qualtrics → Prism → R
python download_qualtrics.py  # Download from Qualtrics
prism -i survey.csv -c config.toml -o clean.csv
Rscript analyze.R  # Run analysis
```

### Can I modify the source code?

Yes! Prism is open-source (MIT license). Fork and customize as needed.

### Where can I request features?

Open an issue on the GitHub repository or contribute via pull request!

---

## Still Have Questions?

- 📖 [Full documentation](README.md)
- 🐛 [Troubleshooting guide](TROUBLESHOOTING.md)
- 💬 Open an issue on GitHub
- 📧 Contact the maintainers

---

## Wiki Navigation

- [📚 Back to Wiki Home](README.md)
- [📖 How to Use](HOW_TO_USE.md)
- [🎓 Tutorial](TUTORIAL.md)
- [🔧 Configuration](CONFIGURATION_GUIDE.md)
- [🐛 Troubleshooting](TROUBLESHOOTING.md)

---

[⬆ Back to Top](#frequently-asked-questions) | [📚 Wiki Home](README.md)
