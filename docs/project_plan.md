Survey Data Pipeline Tool - Project Guide

## What This Is

A command-line tool that automatically processes psychology survey data. Takes raw CSV responses and outputs clean, analysis-ready data with scale scores and quality checks.

- [ ] **Time saved:** 45-60 minutes of manual work → 30 seconds automated

---

## The Problem It Solves

When you collect survey data, you have to:

1. **Reverse-score items** manually in Excel/SPSS
2. **Calculate scale totals** by hand
3. **Compute descriptive statistics** (M, SD, range)
4. **Check data quality** (missing values, outliers, patterns)
5. **Create summary tables** for your report

This tool does ALL of that automatically.

---

## How It Works

### Step 1: User creates a config file

```toml
# burnout_survey.toml

[survey]
name = "Burnout Study"
scale_range = [1, 7]

[scales.emotional_exhaustion]
items = [1, 2, 3, 4, 5, 6, 7, 8, 9]
reverse_scored = [3, 5, 7]

[scales.depersonalization]
items = [10, 11, 12, 13, 14]
reverse_scored = [12]
```

### Step 2: Run the command

```bash
psych-pipeline process --input responses.csv --config burnout_survey.toml
```

### Step 3: Get output files

- `clean_data.csv` - Processed data with scale scores
- `summary_stats.txt` - Descriptive statistics for each scale
- `quality_report.txt` - Flagged responses (optional)

---

## Core Features

### Phase 1: MVP (Week 1)

- [x] Read CSV files
- [x] Parse config file (TOML)
- [x] Reverse-score specified items
- [x] Calculate scale totals and means
- [x] Generate descriptive statistics (M, SD, min, max, N)
- [x] Export clean CSV

### Phase 2: Quality Checks (Week 2)

- [ ] Detect missing data (>10% missing = flag)
- [ ] Detect out-of-range values (e.g., "8" on 1-7 scale)
- [ ] Detect straightlining (all same answer)
- [ ] Detect pattern responding (1-2-3-4-5 repeated)
- [ ] Flag suspiciously fast responses (if timing data available)

### Phase 3: Advanced Features (Optional)

- [ ] Cronbach's alpha (internal consistency)
- [ ] Correlation matrix between scales
- [ ] Generate SPSS syntax file (.sps)
- [ ] Handle different CSV formats (comma, semicolon, tab)
- [ ] Support for different missing value codes (NA, -99, blank)
- [ ] Generate APA-formatted tables

---

## Technical Details

### Tech Stack

- **Language:** Rust
- **Key Libraries:**
  - `csv` - Read/write CSV files
  - `serde` + `toml` - Parse config files
  - `clap` - Command-line interface
  - `statrs` - Statistical functions (optional)

### Project Structure

```
psych-pipeline/
├── Cargo.toml
├── README.md
├── src/
│   ├── main.rs          # CLI entry point
│   ├── config.rs        # Config file parsing
│   ├── reader.rs        # CSV reading
│   ├── processor.rs     # Main processing logic
│   ├── reverse.rs       # Reverse scoring
│   ├── scales.rs        # Scale score calculation
│   ├── stats.rs         # Descriptive statistics
│   ├── quality.rs       # Quality checks
│   └── writer.rs        # Output generation
├── tests/
└── examples/
    ├── sample_config.toml
    └── sample_data.csv
```

---

## Key Algorithms

### Reverse Scoring

```rust
fn reverse_score(value: i32, min: i32, max: i32) -> i32 {
    (max + min) - value
}

// Example: 1-7 scale, answer = 2
// reverse_score(2, 1, 7) = (7 + 1) - 2 = 6
```

### Scale Total Calculation

```rust
fn calculate_scale(response: &Response, scale: &Scale) -> ScoreResult {
    let mut total = 0;

    for &item_num in &scale.items {
        let value = if scale.reverse_scored.contains(&item_num) {
            reverse_score(response.get(item_num), scale.min, scale.max)
        } else {
            response.get(item_num)
        };
        total += value;
    }

    let mean = total as f64 / scale.items.len() as f64;
    ScoreResult { total, mean }
}
```

### Descriptive Statistics

```rust
fn calculate_stats(values: &[f64]) -> Stats {
    let n = values.len();
    let mean = values.iter().sum::<f64>() / n as f64;

    let variance = values.iter()
        .map(|v| (v - mean).powi(2))
        .sum::<f64>() / n as f64;

    let sd = variance.sqrt();
    let min = values.iter().fold(f64::INFINITY, |a, &b| a.min(b));
    let max = values.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));

    Stats { mean, sd, min, max, n }
}
```

### Straightlining Detection

```rust
fn detect_straightlining(responses: &[i32]) -> bool {
    if responses.len() < 3 {
        return false;
    }

    // Check if all values are the same
    let first = responses[0];
    responses.iter().all(|&x| x == first)
}
```

---

## Example Input/Output

### Input: `responses.csv`

```csv
participant_id,Q1,Q2,Q3,Q4,Q5,Q6,Q7,Q8,Q9
P001,5,6,6,7,7,5,6,6,5
P002,3,4,3,5,2,4,3,4,3
P003,2,2,1,3,1,2,2,3,2
```

### Config: `survey.toml`

```toml
[survey]
name = "Stress Coping Study"
scale_range = [1, 7]

[scales.problem_focused]
items = [1, 2, 3, 4, 5]
reverse_scored = [3, 5]

[scales.emotion_focused]
items = [6, 7, 8, 9]
reverse_scored = []
```

### Output: `clean_data.csv`

```csv
participant_id,Q1,Q2,Q3,Q3_rev,Q4,Q5,Q5_rev,Q6,Q7,Q8,Q9,PF_total,PF_mean,EF_total,EF_mean
P001,5,6,6,2,7,7,1,5,6,6,5,21,4.2,22,5.5
P002,3,4,3,5,5,2,6,4,3,4,3,23,4.6,14,3.5
P003,2,2,1,7,3,1,7,2,2,3,2,21,4.2,9,2.25
```

### Output: `summary_stats.txt`

```
STRESS COPING STUDY - Summary Statistics
Generated: 2024-12-19 15:30:22

Total Participants: 3
Complete Responses: 3 (100%)

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

SCALE: Problem-Focused Coping (5 items)
Items: 1, 2, 3*, 4, 5*  (* = reverse scored)

  Mean (M)              = 4.33
  Standard Deviation    = 0.23
  Range                 = [4.2, 4.6]
  N                     = 3

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

SCALE: Emotion-Focused Coping (4 items)
Items: 6, 7, 8, 9

  Mean (M)              = 3.75
  Standard Deviation    = 1.67
  Range                 = [2.25, 5.5]
  N                     = 3

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

DATA QUALITY: No issues detected
```

---

## Weekend Implementation Plan

### Saturday (2-3 hours): Setup & Basic Reading

1. **Create project**

   ```bash
   cargo new psych-pipeline
   cd psych-pipeline
   ```

2. **Add dependencies to `Cargo.toml`**

   ```toml
   [dependencies]
   csv = "1.3"
   serde = { version = "1.0", features = ["derive"] }
   toml = "0.8"
   clap = { version = "4.5", features = ["derive"] }
   anyhow = "1.0"
   ```

3. **Create sample data**

   - Make `examples/sample_data.csv` with 3-5 fake responses
   - Make `examples/sample_config.toml` describing the survey

4. **Write code to read CSV and print it**

   ```rust
   // src/main.rs
   use csv::Reader;

   fn main() -> Result<(), Box<dyn std::error::Error>> {
       let mut reader = Reader::from_path("examples/sample_data.csv")?;

       for result in reader.records() {
           let record = result?;
           println!("{:?}", record);
       }

       Ok(())
   }
   ```

5. **Test it:** `cargo run`

### Sunday (3-4 hours): Core Processing

1. **Parse config file**

   - Define structs for Config, Survey, Scale
   - Use serde to deserialize TOML

2. **Implement reverse scoring**

   - Write `reverse_score()` function
   - Test with sample values

3. **Calculate one scale**

   - For first participant, calculate one scale total
   - Print result to verify correctness

4. **Write output CSV**
   - Add reversed columns
   - Add scale total/mean columns
   - Save to `output/clean_data.csv`

**Success Criteria:** Process 3 participants with 1 reverse-scored scale correctly

---

## Command-Line Interface

### Basic Usage

```bash
# Process survey data
psych-pipeline process --input data.csv --config survey.toml

# Specify output location
psych-pipeline process -i data.csv -c survey.toml -o results/clean.csv

# Include quality checks
psych-pipeline process -i data.csv -c survey.toml --check-quality

# Generate SPSS syntax
psych-pipeline process -i data.csv -c survey.toml --spss-output syntax.sps

# Verbose output
psych-pipeline process -i data.csv -c survey.toml -v
```

### Planned Commands

```bash
# Validate config file
psych-pipeline validate survey.toml

# Generate template config
psych-pipeline init --questions 20 --scales 3

# Quick stats only (no cleaning)
psych-pipeline stats data.csv

# Check data quality without processing
psych-pipeline check data.csv
```

---

## Config File Format

### Full Example

```toml
[survey]
name = "My Psychology Survey"
description = "Measuring stress and coping in students"
scale_range = [1, 7]  # Min and max of Likert scale
missing_values = ["NA", "-99", ""]

[scales.emotional_exhaustion]
name = "Emotional Exhaustion"
description = "MBI-HSS Emotional Exhaustion subscale"
items = [1, 2, 3, 6, 8, 13, 14, 16, 20]
reverse_scored = [3, 8, 13]
citation = "Maslach et al. (1996)"

[scales.depersonalization]
name = "Depersonalization"
items = [5, 10, 11, 15, 22]
reverse_scored = [10]

[scales.personal_accomplishment]
name = "Personal Accomplishment"
items = [4, 7, 9, 12, 17, 18, 19, 21]
reverse_scored = [4, 7, 9, 12, 17, 18, 19, 21]  # All reversed!

[quality_checks]
flag_missing_threshold = 0.10  # Flag if >10% missing
detect_straightlining = true
detect_speeding = true
min_completion_time_minutes = 5
```

---

## Error Handling

### Common Errors

1. **Missing items in data**

   - Error: "Question Q5 specified in config but not found in CSV"
   - Solution: Check column names match

2. **Out-of-range values**

   - Warning: "Participant P023 has Q7=9 (scale is 1-7)"
   - Action: Flag for review

3. **Config parsing errors**

   - Error: "Invalid TOML: expected table at line 12"
   - Solution: Fix config file syntax

4. **Empty CSV**
   - Error: "No data rows found in input file"
   - Solution: Check CSV has data beyond headers

---

## Testing Strategy

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_score_7_point_scale() {
        assert_eq!(reverse_score(1, 1, 7), 7);
        assert_eq!(reverse_score(7, 1, 7), 1);
        assert_eq!(reverse_score(4, 1, 7), 4);
    }

    #[test]
    fn test_scale_calculation() {
        let response = Response::new(vec![5, 6, 2, 7, 3]);
        let scale = Scale {
            items: vec![1, 2, 3, 4, 5],
            reverse_scored: vec![3, 5],
        };

        let result = calculate_scale(&response, &scale);
        assert_eq!(result.total, 29);  // 5+6+(8-2)+7+(8-3)
    }
}
```

### Integration Tests

- Test with real survey data (anonymized)
- Test edge cases (all missing, all same value)
- Test different CSV formats

---

## Future Enhancements

### Phase 4: Advanced Statistics

- [ ] Cronbach's alpha for reliability
- [ ] Item-total correlations
- [ ] Factor analysis suggestions
- [ ] Normality tests (Shapiro-Wilk)

### Phase 5: Output Formats

- [ ] Generate APA-formatted tables (Markdown, LaTeX)
- [ ] Export to R data frame format
- [ ] Export to SPSS .sav format (if possible)
- [ ] JSON output for web applications

### Phase 6: User Experience

- [ ] Interactive mode (prompts for config)
- [ ] Web interface (upload CSV, download results)
- [ ] Progress bar for large datasets
- [ ] Colored terminal output

### Phase 7: Advanced Features

- [ ] Handle longitudinal data (T1, T2, T3)
- [ ] Merge datasets from multiple timepoints
- [ ] Calculate change scores
- [ ] Missing data imputation (mean, regression)

---

## Real-World Usage

### Your Burnout Study

```bash
# Process burnout survey data
psych-pipeline process \
  --input burnout_responses.csv \
  --config mbi_config.toml \
  --output clean_burnout_data.csv \
  --stats-output burnout_summary.txt \
  --check-quality
```

### Your Stress/Coping Study (Already Done)

```bash
# Reprocess with automated tool
psych-pipeline process \
  --input stress_coping_2023.csv \
  --config stress_config.toml \
  --output reprocessed_data.csv
```

---

## How This Looks on Your CV

```
Survey Data Processing Pipeline | Rust                    2024-2025
github.com/artlostintime/psych-pipeline

• Developed command-line tool automating psychology survey data
  processing workflows including reverse-scoring, scale computation,
  and statistical summary generation
• Reduced typical data preparation time from 45-60 minutes to under
  30 seconds for datasets with 50+ participants
• Implemented quality control algorithms detecting straightlining,
  pattern responding, and missing data patterns
• Created flexible configuration system supporting diverse survey
  structures, scale ranges, and scoring methods
• Actively used by 3 research projects at Ambedkar University Delhi,
  processing 200+ survey responses
```

---

## Resources & Documentation

### Learning Resources

- Rust CSV crate: https://docs.rs/csv
- TOML format: https://toml.io/en/
- Clap CLI tutorial: https://docs.rs/clap

### Statistical References

- Reverse scoring: Standard psychometric practice
- Cronbach's alpha: α > 0.70 is acceptable, α > 0.80 is good
- Missing data: >10% missing is concerning

### Similar Tools (for inspiration)

- SPSS syntax (what you're automating)
- R packages: `psych`, `psychometric`
- Python: `pandas` for data manipulation

---

## Getting Help

### If you get stuck:

1. **Rust compilation errors:** Search error message on docs.rs or stackoverflow
2. **Statistical questions:** Refer to your research methods textbook
3. **CSV parsing issues:** Check the `csv` crate documentation
4. **Logic bugs:** Add `println!()` statements to debug

### When you're ready to share:

1. Push to GitHub
2. Add README with usage examples
3. Create example data (make it public, no real participant data!)
4. Write tests
5. Share with lab mates

---

## Success Metrics

**You'll know this project is successful when:**

- ✅ You use it for your burnout study
- ✅ A lab mate asks to use it
- ✅ It saves you 30+ minutes on your next data analysis
- ✅ You can explain it confidently in an interview
- ✅ Someone stars it on GitHub

---

## Quick Start Checklist

- [ ] Create Rust project: `cargo new psych-pipeline`
- [ ] Add dependencies to Cargo.toml
- [ ] Create `examples/` folder with sample data
- [ ] Write code to read CSV
- [ ] Implement reverse_score() function
- [ ] Test reverse scoring with sample data
- [ ] Calculate scale total for one participant
- [ ] Write output to CSV
- [ ] Add command-line arguments with clap
- [ ] Write README with usage instructions
- [ ] Push to GitHub
- [ ] Add to your CV

---

## Contact

arthur - shuvi
