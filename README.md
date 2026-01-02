# Prism 🔍

**Psychology Survey Data Pipeline**

_Transforming raw survey data into analysis-ready insights_

---

Prism automates the tedious parts of psychology research data processing: reverse-scoring, scale computation, quality control, and statistical reporting. What used to take 45+ minutes of manual Excel work now takes 30 seconds.

⏱️ **Time Saved:** 45-60 minutes of manual work → 30 seconds automated

---

## Features

### ✅ Core Processing

- **Automatic reverse scoring** - Specify items in config, handled automatically
- **Scale score calculation** - Total and mean scores for each scale
- **Configurable surveys** - Define any survey structure via TOML config
- **CSV input/output** - Works with standard survey export formats

### ✅ Quality Checks

- **Straightlining detection** - Flags participants answering all items identically
- **Missing data analysis** - Identifies scales with >threshold% missing responses
- **Out-of-range detection** - Catches data entry errors outside valid scale range
- **Quality flagging** - All issues tracked per participant

### ✅ Statistical Reporting

- **Aggregate statistics** - Mean, SD, min, max, N across all participants
- **Summary statistics file** - Formatted report with all scale statistics
- **Quality report** - Detailed breakdown of all data quality issues

### ✅ Dual Interface

- **Command-line tool** - For batch processing and automation
- **GUI application** - Simple desktop app for non-technical users

---

## Quick Start

### Installation

1. **Install Rust** (if not already installed):

   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Build the CLI**:

   ```bash
   cargo build --release
   ```

3. **Optional: Build the GUI**:
   ```bash
   cd src-tauri
   cargo tauri build
   ```

### Usage

#### Command-Line Interface

**Basic usage:**

```bash
prism --input data.csv --config survey.toml --output clean_data.csv
```

**With quality reports:**

```bash
prism \
  --input data.csv \
  --config survey.toml \
  --output clean_data.csv \
  --stats-output summary.txt \
  --quality-report quality.txt
```

**Arguments:**

- `-i, --input <FILE>` - Raw CSV data file
- `-c, --config <FILE>` - TOML configuration file
- `-o, --output <FILE>` - Output CSV path (default: `clean_data.csv`)
- `--stats-output <FILE>` - Generate summary statistics file (optional)
- `--quality-report <FILE>` - Generate quality report file (optional)

#### GUI Application

1. Launch the app
2. Click "Select CSV File"
3. Results are automatically saved to `clean_data.csv`

---

## Configuration

Create a `.toml` file defining your survey structure:

```toml
[survey]
name = "Burnout Study"
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
items = ["Q15", "Q16", "Q17", "Q18", "Q19", "Q20"]
reverse_scored = ["Q15", "Q16", "Q17", "Q18", "Q19", "Q20"]  # All reversed
```

### Config Fields

**`[survey]`**

- `name` - Study name (appears in reports)
- `min_score` - Minimum valid response value
- `max_score` - Maximum valid response value

**`[quality]` (optional)**

- `max_missing_percent` - Threshold for missing data flags (0.0-1.0)
- `flag_straightlining` - Enable straightlining detection (true/false)

**`[scales.<scale_name>]`**

- `items` - List of column names for this scale
- `reverse_scored` - List of items to reverse-score (optional)

---

## Examples

### Input CSV

```csv
id,Q1,Q2,Q3,Q4,Q5
P001,5,6,7,6,5
P002,3,3,3,3,3
P003,1,2,3,4,5
```

### Config

```toml
[survey]
name = "Test Study"
min_score = 1
max_score = 7

[quality]
max_missing_percent = 0.20
flag_straightlining = true

[scales.stress]
items = ["Q1", "Q2", "Q3"]
reverse_scored = ["Q2"]

[scales.coping]
items = ["Q4", "Q5"]
reverse_scored = []
```

### Output: `clean_data.csv`

```csv
id,Q1,Q2,Q3,Q4,Q5,stress_total,stress_mean,coping_total,coping_mean,quality_flag
P001,5,6,7,6,5,14.00,4.67,11.00,5.50,OK
P002,3,3,3,3,3,11.00,3.67,6.00,3.00,Straightlining: stress; Straightlining: coping
P003,1,2,3,4,5,10.00,3.33,9.00,4.50,OK
```

### Output: `summary_stats.txt`

```
TEST STUDY - Summary Statistics
Generated: 2026-01-02 10:31:30

Total Participants: 3
Complete Responses: 3 (100.0%)

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

SCALE: stress (3 items)
Items: Q1, Q2*, Q3  (* = reverse scored)

  Mean (M)              = 3.89
  Standard Deviation    = 0.69
  Range                 = [3.33, 4.67]
  N                     = 3

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

SCALE: coping (2 items)
Items: Q4, Q5

  Mean (M)              = 4.33
  Standard Deviation    = 1.26
  Range                 = [3.00, 5.50]
  N                     = 3

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

DATA QUALITY: 2 issues detected (see quality report for details)
```

### Output: `quality_report.txt`

```
DATA QUALITY REPORT
Generated: 2026-01-02 10:31:30

Total Participants: 3
Flagged Issues: 2

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Straightlining (2 occurrences):

  • Participant P002: Straightlining: stress
  • Participant P002: Straightlining: coping

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

RECOMMENDATIONS:
• Review flagged participants manually
• Consider excluding straightliners from analysis
• Check out-of-range values for data entry errors
• Assess whether missing data is random or systematic
```

---

## Architecture

Prism uses a **minimal wrapper design** to avoid code duplication:

```
GUI (Tauri Desktop App)
    ↓ calls via subprocess
CLI Binary (Rust)
    ├── Config parsing
    ├── CSV processing
    ├── Scale calculations
    ├── Quality checks
    └── Report generation
```

**Benefits:**

- ✅ Single source of truth - all logic in CLI
- ✅ Easy to maintain - changes affect both interfaces
- ✅ Testable - CLI is independently testable
- ✅ Scriptable - CLI can be used in automation

See [`ARCHITECTURE.md`](ARCHITECTURE.md) for details.

---

## Project Structure

```
prism/
├── src/              # CLI implementation
│   ├── main.rs
│   └── config.rs
├── src-tauri/        # GUI wrapper
│   └── src/lib.rs
├── ui/               # GUI frontend
│   └── index.html
├── examples/         # Sample data and configs
│   ├── sample_data.csv
│   └── study_config.toml
├── tests/            # Test suite
│   └── fixtures/
├── docs/             # Documentation
│   ├── HOW_TO_USE.md
│   ├── ARCHITECTURE.md
│   └── ...
└── data/             # Local testing (gitignored)
    ├── raw/
    └── processed/
```

---

## Quality Checks

### Straightlining

Detects participants who answer all items in a scale identically (e.g., all 5s, all 3s). Common indicator of careless responding.

**Example:** All responses = [3, 3, 3, 3, 3]

### Missing Data

Flags scales where a participant has more than the threshold percentage of items missing.

**Example:** Config sets `max_missing_percent = 0.10`, participant missing 2/5 items (40%) → Flagged

### Out-of-Range

Detects values outside the valid scale range, indicating data entry errors.

**Example:** Response = 9 on a 1-7 scale → Flagged

---

## Real-World Use Cases

### Burnout Study (Maslach Burnout Inventory)

```bash
prism \
  --input burnout_responses.csv \
  --config mbi_config.toml \
  --output clean_burnout.csv \
  --stats-output burnout_stats.txt \
  --quality-report burnout_quality.txt
```

### Longitudinal Data

Process multiple timepoints:

```bash
for timepoint in T1 T2 T3; do
  prism --input ${timepoint}_data.csv \
        --config survey_config.toml \
        --output clean_${timepoint}.csv
done
```

### Integration with R/SPSS

Output CSV can be directly imported into statistical software:

```r
# R example
data <- read.csv("clean_data.csv")
summary(data$emotional_exhaustion_mean)
```

---

## 📖 Documentation

**[📚 Visit the Complete Wiki →](docs/README.md)**

### Quick Links

**Getting Started:**

- [Installation Guide](docs/INSTALLATION.md) - Setup and requirements
- [Tutorial](docs/TUTORIAL.md) - Step-by-step walkthrough
- [How to Use](docs/HOW_TO_USE.md) - Complete usage guide
- [Quick Reference](docs/QUICK_REFERENCE.md) - Command cheat sheet

**User Guides:**

- [Configuration Guide](docs/CONFIGURATION_GUIDE.md) - Config file reference
- [Quality Checks](docs/QUALITY_CHECKS.md) - Understanding quality reports
- [Workflow Examples](docs/WORKFLOW_EXAMPLE.md) - Real-world scenarios
- [Best Practices](docs/BEST_PRACTICES.md) - Tips and recommendations

**Reference:**

- [FAQ](docs/FAQ.md) - Frequently asked questions
- [Troubleshooting](docs/TROUBLESHOOTING.md) - Common issues and solutions
- [Glossary](docs/GLOSSARY.md) - Terminology definitions

**For Developers:**

- [Architecture](docs/ARCHITECTURE.md) - System design
- [API Reference](docs/API_REFERENCE.md) - Technical documentation
- [Development Guide](docs/DEVELOPMENT.md) - Local setup
- [Contributing](docs/CONTRIBUTING.md) - How to contribute
- [Testing Guide](docs/TESTING.md) - Testing practices

---

## Development

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
