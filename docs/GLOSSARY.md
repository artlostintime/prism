# Glossary

**[📚 Wiki Home](README.md)** | **[📖 How to Use](HOW_TO_USE.md)** | **[❓ FAQ](FAQ.md)**

---

## Overview

Definitions of key terms used in Prism documentation and psychology research contexts.

---

## General Terms

### Prism

The automated survey data processing tool described in this wiki.

### Survey

A research instrument consisting of multiple items designed to measure psychological constructs.

### Participant

An individual who completes a survey; synonymous with respondent or subject.

### Item

A single question or statement in a survey (e.g., "I feel emotionally drained").

---

## Data Terms

### CSV (Comma-Separated Values)

A text file format where data is organized in rows and columns, with commas separating values. Used for both input and output in Prism.

### Configuration File (Config)

A TOML file that defines survey structure, scales, and processing settings.

### TOML (Tom's Obvious, Minimal Language)

A human-readable configuration file format used by Prism.

### Column Header

The first row of a CSV file containing variable names (e.g., "participant_id", "q1", "q2").

### Missing Data

Responses that are absent/skipped by participants. Represented as empty cells in CSV.

---

## Scale Terms

### Scale

A set of related items that measure a common construct (e.g., burnout, anxiety). Also called a subscale, measure, or instrument.

### Subscale

A subset of items within a larger measure that assesses a specific dimension (e.g., emotional exhaustion is a subscale of burnout).

### Item

Individual question or statement within a scale.

### Scale Score

A summary value computed from multiple items, typically the sum or mean.

### Total Score

The sum of all item responses in a scale.

**Example:** Items [3, 4, 5] → Total = 12

### Mean Score

The average of all item responses in a scale.

**Example:** Items [3, 4, 5] → Mean = 12/3 = 4.0

---

## Scoring Terms

### Reverse Scoring (Reverse Coding)

Flipping the value of negatively-worded items so all items are scored in the same direction.

**Formula:** `(max + min) - original_value`

**Example (1-5 scale):**

- Original: 5 → Reversed: (5+1)-5 = 1
- Original: 1 → Reversed: (5+1)-1 = 5

**When Used:** For items like "I am NOT satisfied" in a satisfaction scale.

### Negatively-Worded Item

An item phrased in the opposite direction of the construct (e.g., "I lack confidence" in a confidence scale). Requires reverse scoring.

### Positively-Worded Item

An item phrased in the same direction as the construct (e.g., "I feel confident" in a confidence scale). Does not require reverse scoring.

---

## Response Terms

### Likert Scale

A psychometric scale commonly used in surveys where respondents specify their level of agreement/frequency/intensity.

**Common Formats:**

- 1-5: Strongly Disagree to Strongly Agree
- 1-7: Never to Always
- 0-10: Not at all to Extremely

### Response Format

The type and range of responses available for items (e.g., 1-5 Likert scale, 0-100 VAS).

### Out-of-Range Value

A response that falls outside the valid scale range (e.g., 8 on a 1-5 scale). Usually indicates data entry errors.

### Visual Analog Scale (VAS)

A measurement instrument using a continuous scale, typically 0-100, where participants mark a point on a line.

---

## Quality Check Terms

### Straightlining

When a participant gives the same response to all items in a scale, suggesting inattentive responding or lack of effort.

**Example:** All items answered as "4" on a 1-7 scale.

**Note:** May be legitimate (floor/ceiling effects) or indicate poor data quality.

### Missing Data Threshold

The maximum percentage of missing responses allowed before flagging a participant for potential exclusion.

**Default:** 10%

### Floor Effect

When many participants score at the minimum of a scale, creating a skewed distribution. May cause legitimate straightlining.

**Example:** Pain scale where healthy participants all report "1" (no pain).

### Ceiling Effect

When many participants score at the maximum of a scale, creating a skewed distribution. May cause legitimate straightlining.

**Example:** Satisfaction scale where all participants report "7" (very satisfied).

### Attention Check

An item designed to detect inattentive responding (e.g., "Please select 'Agree' for this item").

### Careless Responding

When participants do not read items carefully or respond randomly. Detected via straightlining, long-string analysis, or failed attention checks.

---

## Statistical Terms

### Mean (M)

The arithmetic average of values.

**Formula:** `sum(x) / n`

**Example:** [3, 4, 5] → M = 12/3 = 4.0

### Standard Deviation (SD)

A measure of variability or spread in data.

**Formula (sample):** `sqrt(sum((x - mean)²) / (n - 1))`

**Note:** Prism uses sample SD (n-1 denominator).

### Minimum (min)

The smallest value in a dataset.

### Maximum (max)

The largest value in a dataset.

### Sample Size (N/n)

The number of observations or participants.

### Aggregate Statistics

Statistics computed across multiple participants (vs. individual-level statistics).

**Example:** Mean burnout score across all participants.

### Descriptive Statistics

Summary statistics that describe basic features of data (mean, SD, min, max).

### Cronbach's Alpha (α)

A measure of internal consistency/reliability for a scale. Not currently computed by Prism.

**Interpretation:**

- α > .90: Excellent
- α > .80: Good
- α > .70: Acceptable
- α < .70: Poor

---

## Processing Terms

### CLI (Command-Line Interface)

A text-based interface where users type commands to interact with software.

**Example:** `prism -i data.csv -c config.toml -o output.csv`

### GUI (Graphical User Interface)

A visual interface with buttons, menus, and windows for interacting with software.

### Pipeline

A series of automated steps for processing data from raw input to final output.

### Batch Processing

Processing multiple files or datasets in sequence using automated scripts.

### Validation

Checking data or configurations for errors before processing (e.g., verifying all items exist in CSV).

---

## File Terms

### Input File

The raw CSV data exported from your survey platform.

### Output File

The processed CSV with scale scores added.

### Summary Statistics File

A text report containing aggregate statistics (M, SD, min, max, N).

### Quality Report File

A text report listing participants flagged for quality issues.

### Config File

A TOML file defining survey structure and processing parameters.

---

## Research Terms

### Construct

An abstract concept measured by a scale (e.g., anxiety, burnout, satisfaction).

### Dimension

An aspect or facet of a construct measured by a subscale (e.g., emotional exhaustion is a dimension of burnout).

### Psychometrics

The field of study concerned with measurement of psychological constructs.

### Internal Consistency

The extent to which items in a scale measure the same construct. Measured by Cronbach's alpha.

### Validity

The degree to which a scale measures what it claims to measure.

### Reliability

The consistency or stability of a measurement.

### Exclusion Criteria

Rules for determining which participants to remove from analyses due to data quality issues.

### Pre-Registration

Specifying analysis plans (including exclusion criteria) before data collection begins.

### Sensitivity Analysis

Re-running analyses with different inclusion criteria to assess robustness of findings.

---

## Software Terms

### Rust

The programming language Prism is written in.

### Cargo

Rust's package manager and build tool.

### Tauri

The framework used for Prism's GUI.

### Binary

The compiled executable file (e.g., `prism.exe`).

### Open-Source

Software whose source code is publicly available and freely modifiable.

### MIT License

A permissive open-source license allowing free use, modification, and distribution.

---

## Acronyms

| Acronym | Full Term                       | Meaning                  |
| ------- | ------------------------------- | ------------------------ |
| CLI     | Command-Line Interface          | Text-based interface     |
| CSV     | Comma-Separated Values          | Data file format         |
| GUI     | Graphical User Interface        | Visual interface         |
| M       | Mean                            | Average value            |
| MBI     | Maslach Burnout Inventory       | Common burnout measure   |
| SD      | Standard Deviation              | Variability measure      |
| TOML    | Tom's Obvious, Minimal Language | Config file format       |
| VAS     | Visual Analog Scale             | Continuous rating scale  |
| WAI     | Working Alliance Inventory      | Therapy alliance measure |

---

## Related Reading

- [Configuration Guide](CONFIGURATION_GUIDE.md) - Understanding config files
- [Quality Checks](QUALITY_CHECKS.md) - Quality terminology in depth
- [How to Use](HOW_TO_USE.md) - Practical application of terms
- [FAQ](FAQ.md) - Common questions about terminology

---

## Wiki Navigation

- [📚 Back to Wiki Home](README.md)
- [📖 How to Use](HOW_TO_USE.md)
- [🔧 Configuration Guide](CONFIGURATION_GUIDE.md)
- [❓ FAQ](FAQ.md)

---

[⬆ Back to Top](#glossary) | [📚 Wiki Home](README.md)
