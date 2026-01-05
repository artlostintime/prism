# API Reference

**[📚 Wiki Home](README.md)** | **[🏗️ Architecture](ARCHITECTURE.md)** | **[💻 Development](DEVELOPMENT.md)** | **[🤝 Contributing](CONTRIBUTING.md)**

---

## Overview

This document provides technical reference for Prism's internal API, modules, and functions. For user-facing documentation, see [How to Use](HOW_TO_USE.md).

---

## Table of Contents

1. [CLI Interface](#cli-interface)
2. [Core Modules](#core-modules)
3. [Data Structures](#data-structures)
4. [Functions](#functions)
5. [Error Handling](#error-handling)

---

## CLI Interface

Prism provides multiple subcommands for different data processing tasks.

### Main Command: `process`

```bash
prism process [OPTIONS]
```

**Required Arguments:**

| Flag           | Description           | Type   |
| -------------- | --------------------- | ------ |
| `-i, --input`  | Input CSV file path   | String |
| `-c, --config` | Config TOML file path | String |
| `-o, --output` | Output CSV file path  | String |

**Optional Arguments:**

| Flag               | Description                           | Type   | Default |
| ------------------ | ------------------------------------- | ------ | ------- |
| `--stats-output`   | Summary statistics output path        | String | None    |
| `--quality-report` | Quality report output path            | String | None    |
| `--all-outputs`    | Generate all outputs                  | Flag   | false   |
| `--export-all`     | Export all formats                    | Flag   | false   |
| `--format`         | Output format (csv/excel/spss/r/json) | String | csv     |
| `--dry-run`        | Preview without writing               | Flag   | false   |
| `-h, --help`       | Print help information                | -      | -       |

**Example:**

```bash
prism process -i data.csv -c config.toml -o output.csv \
  --stats-output stats.txt --quality-report quality.txt
```

### Longitudinal Commands (v0.3.0+)

#### `merge` - Merge Multiple Waves

Combine data from multiple time points by participant ID.

```bash
prism merge --waves <WAVE:FILE>... --id <ID_COLUMN> --join <TYPE> -o <OUTPUT>
```

**Required Arguments:**

| Flag      | Description                     | Type                |
| --------- | ------------------------------- | ------------------- |
| `--waves` | Wave specifications (name:path) | String (repeatable) |
| `--id`    | Participant ID column name      | String              |
| `--join`  | Join type (inner/outer)         | String              |
| `-o`      | Output CSV file path            | String              |

**Example:**

```bash
prism merge \
  --waves T1:baseline.csv T2:followup.csv T3:final.csv \
  --id ParticipantID \
  --join outer \
  -o merged_data.csv
```

#### `reshape` - Convert Between Wide and Long Formats

Transform data structure for different statistical analyses.

```bash
prism reshape -i <INPUT> --format <FORMAT> --waves <WAVES>... --id <ID> -o <OUTPUT>
```

**Required Arguments:**

| Flag       | Description                                | Type                |
| ---------- | ------------------------------------------ | ------------------- |
| `-i`       | Input CSV file path                        | String              |
| `--format` | Reshape format (wide-to-long/long-to-wide) | String              |
| `--waves`  | Wave names (T1, T2, T3, etc.)              | String (repeatable) |
| `--id`     | Participant ID column name                 | String              |
| `-o`       | Output CSV file path                       | String              |

**Optional Arguments:**

| Flag         | Description                         | Type   | Default |
| ------------ | ----------------------------------- | ------ | ------- |
| `--time-col` | Time column name (for long-to-wide) | String | "Time"  |

**Examples:**

```bash
# Wide to Long (for growth curve modeling)
prism reshape \
  -i wide_data.csv \
  --format wide-to-long \
  --waves T1 T2 T3 \
  --id ParticipantID \
  -o long_data.csv

# Long to Wide (for repeated measures ANOVA)
prism reshape \
  -i long_data.csv \
  --format long-to-wide \
  --waves T1 T2 T3 \
  --id ParticipantID \
  --time-col Time \
  -o wide_data.csv
```

#### `rci` - Calculate Reliable Change Index

Determine clinically significant change between time points.

```bash
prism rci -i <INPUT> --baseline <COL> --followup <COL> --reliability <VALUE> --id <ID> -o <OUTPUT>
```

**Required Arguments:**

| Flag            | Description                         | Type   |
| --------------- | ----------------------------------- | ------ |
| `-i`            | Input CSV file path                 | String |
| `--baseline`    | Baseline score column name          | String |
| `--followup`    | Follow-up score column name         | String |
| `--reliability` | Test-retest reliability coefficient | Float  |
| `--id`          | Participant ID column name          | String |
| `-o`            | Output CSV file path                | String |

**Optional Arguments:**

| Flag            | Description                 | Type  | Default              |
| --------------- | --------------------------- | ----- | -------------------- |
| `--baseline-sd` | Baseline standard deviation | Float | Calculated from data |

**Example:**

```bash
# Calculate RCI with reliability from scale manual
prism rci \
  -i merged_data.csv \
  --baseline PHQ9_T1 \
  --followup PHQ9_T2 \
  --reliability 0.89 \
  --id ParticipantID \
  -o rci_results.csv

# Calculate RCI with custom baseline SD
prism rci \
  -i merged_data.csv \
  --baseline depression_T1 \
  --followup depression_T2 \
  --reliability 0.85 \
  --baseline-sd 8.5 \
  --id ParticipantID \
  -o rci_results.csv
```

### Other Commands

#### `generate` - Generate Configuration Templates

```bash
prism generate [OPTIONS]
```

**Options:**

| Flag            | Description                        | Type   |
| --------------- | ---------------------------------- | ------ |
| `--template`    | Generate blank TOML template       | Flag   |
| `--list-scales` | List all pre-built scales          | Flag   |
| `--scale`       | Generate config for specific scale | String |
| `--scale-info`  | Show detailed scale information    | String |

**Example:**

```bash
prism generate --list-scales
prism generate --scale PHQ-9 > phq9_config.toml
prism generate --scale-info GAD-7
```

#### `validate` - Validate Configuration

```bash
prism validate -c <CONFIG> -i <INPUT>
```

**Example:**

```bash
prism validate -c config.toml -i data.csv
```

---

## Core Modules

### `config` Module

**Location:** `src/config.rs`

**Purpose:** Configuration file parsing and data structures.

**Public Types:**

- `SurveyConfig` - Top-level config structure
- `SurveySettings` - Survey metadata
- `QualitySettings` - Quality check thresholds
- `ScaleDefinition` - Individual scale definition

**Example:**

```rust
use crate::config::SurveyConfig;

let config = SurveyConfig::from_file("config.toml")?;
```

### `longitudinal` Module (v0.3.0+)

**Location:** `src/longitudinal.rs`

**Purpose:** Longitudinal data analysis and repeated measures processing.

**Public Types:**

- `LongitudinalConfig` - Configuration for longitudinal analyses
- `MergeParams` - Parameters for merging multiple waves
- `ReshapeParams` - Parameters for wide/long conversion
- `RCIParams` - Parameters for reliable change index
- `RCIResult` - Result structure for RCI calculations
- `JoinType` - Enum for merge join types (Inner, Outer)
- `ReshapeFormat` - Enum for reshape formats (WideToLong, LongToWide)

**Public Functions:**

```rust
pub fn merge_waves(params: &MergeParams) -> Result<(), PrismError>
// Merge multiple wave files by participant ID

pub fn reshape_data(params: &ReshapeParams) -> Result<(), PrismError>
// Convert between wide and long formats

pub fn calculate_rci(params: &RCIParams) -> Result<(), PrismError>
// Calculate reliable change index

fn calculate_sd(values: &[f64]) -> f64
// Helper function for standard deviation calculation
```

**Example Usage:**

```rust
use prism::longitudinal::{MergeParams, JoinType};

let params = MergeParams {
    wave_files: vec![
        ("T1".to_string(), "baseline.csv".to_string()),
        ("T2".to_string(), "followup.csv".to_string()),
    ],
    id_column: "ParticipantID".to_string(),
    join_type: JoinType::Outer,
    output_path: "merged.csv".to_string(),
};

merge_waves(&params)?;
```

### `main` Module

**Location:** `src/main.rs`

**Purpose:** Core processing logic and CLI entry point.

**Public Functions:**

- `main()` - CLI entry point
- `validate_config()` - Pre-processing validation
- `process_scale()` - Scale computation
- `check_missing_data()` - Missing data analysis
- `check_straightlining()` - Straightlining detection
- `generate_summary_stats()` - Statistics generation
- `generate_quality_report()` - Quality report generation

---

## Data Structures

### `SurveyConfig`

**Definition:**

```rust
pub struct SurveyConfig {
    pub survey: SurveySettings,
    pub quality: QualitySettings,
    pub scales: HashMap<String, ScaleDefinition>,
}
```

**Fields:**

- `survey` - Survey metadata
- `quality` - Quality check settings
- `scales` - Map of scale name to definition

**Methods:**

```rust
impl SurveyConfig {
    pub fn from_file(path: &str) -> Result<Self>
    // Load config from TOML file
}
```

---

### `SurveySettings`

**Definition:**

```rust
pub struct SurveySettings {
    pub name: String,
}
```

**Fields:**

- `name` - Survey title

---

### `QualitySettings`

**Definition:**

```rust
pub struct QualitySettings {
    pub missing_threshold: f64,
}
```

**Fields:**

- `missing_threshold` - Maximum percentage of missing data allowed (0-100)

**Example:**

```toml
[quality]
missing_threshold = 10.0  # 10%
```

---

### `ScaleDefinition`

**Definition:**

```rust
pub struct ScaleDefinition {
    pub items: Vec<String>,
    pub reverse_scored: Option<Vec<String>>,
    pub min_value: Option<f64>,
    pub max_value: Option<f64>,
}
```

**Fields:**

- `items` - List of column names in the scale
- `reverse_scored` - Optional list of items to reverse
- `min_value` - Optional minimum valid value
- `max_value` - Optional maximum valid value

---

### `Stats`

**Definition:**

```rust
struct Stats {
    mean: f64,
    sd: f64,
    min: f64,
    max: f64,
    n: usize,
}
```

**Fields:**

- `mean` - Arithmetic mean
- `sd` - Sample standard deviation (n-1)
- `min` - Minimum value
- `max` - Maximum value
- `n` - Sample size

**Methods:**

```rust
impl Stats {
    fn calculate(values: &[f64]) -> Self
    // Compute statistics from values
}
```

**Algorithm:**

- Mean: `sum(x) / n`
- SD: `sqrt(sum((x - mean)²) / (n - 1))`
- Min: `min(x)`
- Max: `max(x)`

---

### `ScaleResult`

**Definition:**

```rust
struct ScaleResult {
    total: f64,
    mean: f64,
}
```

**Fields:**

- `total` - Sum of all items
- `mean` - Average of all items

---

### `QualityIssue`

**Definition:**

```rust
struct QualityIssue {
    participant_id: String,
    issue_type: String,
    details: String,
}
```

**Fields:**

- `participant_id` - Participant identifier
- `issue_type` - Type of issue (e.g., "Straightlining")
- `details` - Human-readable description

**Methods:**

```rust
impl QualityIssue {
    fn new(participant_id: String, issue_type: String, details: String) -> Self
    // Create new quality issue
}
```

---

### Longitudinal Data Structures (v0.3.0+)

#### `LongitudinalConfig`

**Definition:**

```rust
pub struct LongitudinalConfig {
    pub waves: Vec<String>,
    pub id_column: String,
}
```

**Fields:**

- `waves` - List of wave names (e.g., ["T1", "T2", "T3"])
- `id_column` - Column containing participant IDs

**Usage:** Optional field in `SurveyConfig` for longitudinal studies.

---

#### `MergeParams`

**Definition:**

```rust
pub struct MergeParams {
    pub wave_files: Vec<(String, String)>,
    pub id_column: String,
    pub join_type: JoinType,
    pub output_path: String,
}
```

**Fields:**

- `wave_files` - Vector of (wave_name, file_path) tuples
- `id_column` - Column name for participant IDs
- `join_type` - JoinType enum (Inner or Outer)
- `output_path` - Path for merged output CSV

---

#### `ReshapeParams`

**Definition:**

```rust
pub struct ReshapeParams {
    pub input_path: String,
    pub output_path: String,
    pub format: ReshapeFormat,
    pub waves: Vec<String>,
    pub id_column: String,
    pub time_column: String,
}
```

**Fields:**

- `input_path` - Input CSV file path
- `output_path` - Output CSV file path
- `format` - ReshapeFormat enum (WideToLong or LongToWide)
- `waves` - List of wave names
- `id_column` - Column name for participant IDs
- `time_column` - Column name for time variable (default: "Time")

---

#### `RCIParams`

**Definition:**

```rust
pub struct RCIParams {
    pub input_path: String,
    pub output_path: String,
    pub baseline_column: String,
    pub followup_column: String,
    pub id_column: String,
    pub reliability: f64,
    pub baseline_sd: Option<f64>,
}
```

**Fields:**

- `input_path` - Input CSV file path
- `output_path` - Output CSV file path
- `baseline_column` - Column name for baseline scores
- `followup_column` - Column name for follow-up scores
- `id_column` - Column name for participant IDs
- `reliability` - Test-retest reliability coefficient (0.0 to 1.0)
- `baseline_sd` - Optional custom baseline standard deviation

---

#### `RCIResult`

**Definition:**

```rust
pub struct RCIResult {
    pub participant_id: String,
    pub baseline_score: f64,
    pub followup_score: f64,
    pub change: f64,
    pub percent_change: f64,
    pub rci_score: f64,
    pub se_diff: f64,
    pub interpretation: String,
}
```

**Fields:**

- `participant_id` - Participant identifier
- `baseline_score` - Score at baseline (T1)
- `followup_score` - Score at follow-up (T2)
- `change` - Raw change score (T2 - T1)
- `percent_change` - Percentage change from baseline
- `rci_score` - Reliable change index value
- `se_diff` - Standard error of difference
- `interpretation` - Clinical interpretation ("Improved", "Deteriorated", or "No reliable change")

**Interpretation Criteria:**

- **RCI < -1.96**: "Improved" (clinically significant improvement)
- **RCI > 1.96**: "Deteriorated" (clinically significant worsening)
- **-1.96 ≤ RCI ≤ 1.96**: "No reliable change" (within measurement error)

---

#### `JoinType` (Enum)

**Definition:**

```rust
pub enum JoinType {
    Inner,
    Outer,
}
```

**Variants:**

- `Inner` - Keep only participants present in all waves
- `Outer` - Keep all participants from any wave

---

#### `ReshapeFormat` (Enum)

**Definition:**

```rust
pub enum ReshapeFormat {
    WideToLong,
    LongToWide,
}
```

**Variants:**

- `WideToLong` - Convert wide format (var_T1, var_T2) to long format (Time, var)
- `LongToWide` - Convert long format back to wide format

---

## Functions

### `validate_config()`

**Signature:**

```rust
fn validate_config(
    config: &SurveyConfig,
    headers: &[String]
) -> Result<()>
```

**Purpose:** Validate configuration before processing.

**Checks:**

- All scale items exist in CSV headers
- Reverse-scored items are in items list
- Min < max (if specified)

**Returns:** `Ok(())` if valid, `Err` with details if invalid

**Example:**

```rust
validate_config(&config, &headers)?;
```

---

### `process_scale()`

**Signature:**

```rust
fn process_scale(
    record: &csv::StringRecord,
    scale_def: &ScaleDefinition,
    headers: &[String]
) -> Option<ScaleResult>
```

**Purpose:** Compute scale total and mean for one participant.

**Steps:**

1. Extract item values from CSV record
2. Apply reverse scoring if specified
3. Calculate sum (total)
4. Calculate mean

**Returns:** `Some(ScaleResult)` if successful, `None` if missing data

**Reverse Scoring Formula:**

```
reversed_value = (max + min) - original_value
```

---

### `check_missing_data()`

**Signature:**

```rust
fn check_missing_data(
    record: &csv::StringRecord,
    scale_def: &ScaleDefinition,
    headers: &[String],
    participant_id: &str,
    threshold: f64
) -> Option<QualityIssue>
```

**Purpose:** Check if participant has excessive missing data.

**Algorithm:**

```
missing_count = 0
for each item in scale:
    if value is empty:
        missing_count += 1

missing_pct = (missing_count / total_items) * 100

if missing_pct > threshold:
    return QualityIssue
```

**Returns:** `Some(QualityIssue)` if exceeds threshold, `None` otherwise

---

### `check_straightlining()`

**Signature:**

```rust
fn check_straightlining(
    record: &csv::StringRecord,
    scale_def: &ScaleDefinition,
    headers: &[String],
    participant_id: &str,
    scale_name: &str
) -> Option<QualityIssue>
```

**Purpose:** Detect identical responses across all items.

**Algorithm:**

```
values = extract all item values
if all values are identical:
    return QualityIssue
```

**Returns:** `Some(QualityIssue)` if straightlining detected, `None` otherwise

---

### `generate_summary_stats()`

**Signature:**

```rust
fn generate_summary_stats(
    output_path: &str,
    scale_data: &HashMap<String, Vec<f64>>,
    survey_name: &str
) -> Result<()>
```

**Purpose:** Generate aggregate statistics report.

**Output Format:**

```
Summary Statistics Report
Generated: [timestamp]

Survey: [name]

[scale_name]_total: M = [mean], SD = [sd], min = [min], max = [max], N = [n]
[scale_name]_mean: M = [mean], SD = [sd], min = [min], max = [max], N = [n]
...
```

---

### `generate_quality_report()`

**Signature:**

```rust
fn generate_quality_report(
    output_path: &str,
    quality_issues: &[QualityIssue],
    survey_name: &str
) -> Result<()>
```

**Purpose:** Generate quality check report.

**Output Format:**

```
Quality Report
Generated: [timestamp]

Survey: [name]

=== [ISSUE_TYPE] ===
Participant [id]: [details]
...
```

**Issue Types:**

- STRAIGHTLINING DETECTED
- MISSING DATA
- OUT-OF-RANGE VALUES

---

## Error Handling

### Error Types

Prism uses `anyhow::Result` for error handling.

**Common Errors:**

| Error                    | Cause                                 | Solution                    |
| ------------------------ | ------------------------------------- | --------------------------- |
| `Failed to read CSV`     | File not found or locked              | Check path and permissions  |
| `Failed to parse config` | Invalid TOML syntax                   | Validate TOML syntax        |
| `Column not found`       | Config references non-existent column | Match config to CSV headers |
| `Invalid range`          | min >= max                            | Ensure min < max            |

### Error Propagation

```rust
fn example() -> Result<()> {
    let config = SurveyConfig::from_file("config.toml")?;
    validate_config(&config, &headers)?;
    Ok(())
}
```

---

## Constants

### Floating-Point Comparison

```rust
const FLOAT_EPSILON: f64 = 1e-6;
```

Used for comparing float values (straightlining detection).

### Quality Report Markers

```rust
const QUALITY_FLAG_OK: &str = "OK";
const QUALITY_FLAG_SEPARATOR: &str = "===";
```

Used in quality report formatting.

---

## Type Aliases

```rust
type Result<T> = anyhow::Result<T>;
```

Simplifies Result types throughout codebase.

---

## Dependencies

### External Crates

| Crate    | Version | Purpose                       |
| -------- | ------- | ----------------------------- |
| `csv`    | 1.4.0   | CSV parsing and writing       |
| `serde`  | 1.0.228 | Serialization/deserialization |
| `toml`   | 0.9.10  | TOML config parsing           |
| `clap`   | 4.5.53  | CLI argument parsing          |
| `anyhow` | 1.0.100 | Error handling                |
| `chrono` | 0.4     | Timestamp generation          |

---

## Examples

### Processing a Single Participant

```rust
// Load config
let config = SurveyConfig::from_file("config.toml")?;

// Get scale definition
let scale = config.scales.get("anxiety").unwrap();

// Process participant
let result = process_scale(&record, scale, &headers);

if let Some(scale_result) = result {
    println!("Total: {}, Mean: {}", scale_result.total, scale_result.mean);
}
```

### Computing Statistics

```rust
let values = vec![3.5, 4.0, 4.2, 3.8, 4.1];
let stats = Stats::calculate(&values);

println!("M = {:.2}, SD = {:.2}", stats.mean, stats.sd);
// Output: M = 3.92, SD = 0.28
```

---

## GUI API (Tauri Commands)

### `pick_file()`

**Command:** `pick_file`

**Purpose:** Open file picker dialog.

**Returns:** `Result<String>` - Selected file path

**Example:**

```javascript
const filePath = await invoke("pick_file");
```

### `run_analysis()`

**Command:** `run_analysis`

**Purpose:** Execute Prism CLI with parameters.

**Parameters:**

- `input` - Input CSV path
- `config` - Config TOML path
- `output` - Output CSV path

**Returns:** `Result<String>` - Processing result message

**Example:**

```javascript
const result = await invoke("run_analysis", {
  input: inputPath,
  config: configPath,
  output: outputPath,
});
```

---

## Wiki Navigation

- [📚 Back to Wiki Home](README.md)
- [🏗️ Architecture](ARCHITECTURE.md)
- [💻 Development Guide](DEVELOPMENT.md)
- [🤝 Contributing](CONTRIBUTING.md)
- [🧪 Testing](TESTING.md)

---

[⬆ Back to Top](#api-reference) | [📚 Wiki Home](README.md)
