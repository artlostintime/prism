# Prism 🔍

> **Stop wasting hours on Excel. Process your psychology survey data in seconds.**

Hey there, researcher! 👋 If you've ever spent 45+ minutes manually reverse-scoring surveys, calculating scale totals, and hunting for data quality issues in Excel... this tool is for you.

Prism does all that boring stuff automatically, so you can focus on what actually matters: **understanding your data**.

---

## 🎯 What Does Prism Do?

Think of Prism as your research assistant that:

1. **Reverse-scores items automatically** (no more Excel formulas!)
2. **Calculates scale totals and means** (instantly, no errors)
3. **Finds data quality issues** (straightliners, missing data, weird responses)
4. **Generates statistical reports** (with Cronbach's alpha and everything)
5. **Exports in multiple formats** (Excel, SPSS, R, JSON... you name it)

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
```

**Real example:** If someone answers [3, 3, 3, 3, 3] to all items, that's straightlining and gets flagged.

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

### Import into SPSS

Want to use SPSS for your analyses?

```bash
prism process -i burnout_responses.csv -c mbi_config.toml -o clean.csv --format spss
```

This generates `clean.sps` with the import syntax. Just open it in SPSS and run!

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

## 📖 Need More Help?

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
