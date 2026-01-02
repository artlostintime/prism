# Configuration Guide

**[📚 Wiki Home](README.md)** | **[📖 How to Use](HOW_TO_USE.md)** | **[⚡ Quick Ref](QUICK_REFERENCE.md)** | **[📊 Examples](WORKFLOW_EXAMPLE.md)**

---

## Overview

Prism uses TOML configuration files to define your survey structure. This guide covers everything you need to know about creating and customizing config files.

---

## Table of Contents

1. [Basic Structure](#basic-structure)
2. [Survey Settings](#survey-settings)
3. [Quality Settings](#quality-settings)
4. [Scale Definitions](#scale-definitions)
5. [Reverse Scoring](#reverse-scoring)
6. [Advanced Options](#advanced-options)
7. [Examples](#examples)
8. [Validation](#validation)

---

## Basic Structure

Every config file has three main sections:

```toml
[survey]
# Survey metadata

[quality]
# Quality check thresholds

[scales.scale_name]
# Scale definitions (one per scale)
```

### Minimal Example

```toml
[survey]
name = "My Survey"

[quality]
missing_threshold = 10.0

[scales.depression]
items = ["dep1", "dep2", "dep3"]
```

---

## Survey Settings

### `[survey]` Section

**Required Fields:**

- `name` - Survey title (string)

**Optional Fields:**

- `description` - Survey description (string)
- `version` - Version number (string)
- `author` - Researcher name (string)
- `date` - Creation date (string)

### Example

```toml
[survey]
name = "Clinical Outcomes Study"
description = "Measuring burnout and supervision quality"
version = "2.0"
author = "Dr. Jane Smith"
date = "2026-01-15"
```

---

## Quality Settings

### `[quality]` Section

**Available Settings:**

| Setting                  | Type  | Default | Description                        |
| ------------------------ | ----- | ------- | ---------------------------------- |
| `missing_threshold`      | float | 10.0    | Max % missing data per participant |
| `straightlining_enabled` | bool  | true    | Enable straightlining detection    |
| `out_of_range_enabled`   | bool  | true    | Enable range validation            |

### Example

```toml
[quality]
missing_threshold = 15.0  # Allow up to 15% missing
straightlining_enabled = true
out_of_range_enabled = true
```

### Quality Threshold Guidelines

**Conservative (Clinical Research):**

```toml
missing_threshold = 5.0  # Flag >5% missing
```

**Standard (Academic Research):**

```toml
missing_threshold = 10.0  # Flag >10% missing
```

**Lenient (Exploratory Studies):**

```toml
missing_threshold = 20.0  # Flag >20% missing
```

---

## Scale Definitions

### `[scales.scale_name]` Sections

Each scale needs its own section.

**Required Fields:**

- `items` - Array of column names (strings)

**Optional Fields:**

- `reverse_scored` - Array of items to reverse (strings)
- `min_value` - Minimum valid response (float)
- `max_value` - Maximum valid response (float)

### Basic Scale

```toml
[scales.anxiety]
items = ["anx1", "anx2", "anx3", "anx4", "anx5"]
```

### Scale with Reverse Scoring

```toml
[scales.satisfaction]
items = ["sat1", "sat2", "sat3", "sat4"]
reverse_scored = ["sat2", "sat4"]  # Reverse these items
```

### Scale with Range Validation

```toml
[scales.likert_scale]
items = ["q1", "q2", "q3"]
min_value = 1.0
max_value = 5.0  # Flag values outside 1-5 range
```

### Complete Scale Example

```toml
[scales.burnout]
items = [
    "burn1", "burn2", "burn3",
    "burn4", "burn5", "burn6"
]
reverse_scored = ["burn3", "burn6"]
min_value = 1.0
max_value = 7.0
```

---

## Reverse Scoring

### How It Works

**Formula:** `(max + min) - original_value`

**Example with 1-5 scale:**

- Item response: 5
- Reversed: (5 + 1) - 5 = 1

**Example with 1-7 scale:**

- Item response: 2
- Reversed: (7 + 1) - 2 = 6

### When to Use Reverse Scoring

Use for **negatively worded items** in positively scored scales:

**Example: Self-Esteem Scale**

```toml
[scales.self_esteem]
items = [
    "se1",  # "I feel good about myself" (positive)
    "se2",  # "I am worthless" (negative - needs reversing)
    "se3",  # "I have many good qualities" (positive)
    "se4"   # "I am a failure" (negative - needs reversing)
]
reverse_scored = ["se2", "se4"]
```

### Multiple Scales with Different Ranges

```toml
[scales.scale_a]
items = ["a1", "a2", "a3"]
reverse_scored = ["a2"]
min_value = 1.0
max_value = 5.0

[scales.scale_b]
items = ["b1", "b2", "b3"]
reverse_scored = ["b1", "b3"]
min_value = 1.0
max_value = 7.0
```

---

## Advanced Options

### Multi-Word Scale Names

```toml
[scales."emotional exhaustion"]
items = ["ee1", "ee2", "ee3"]

[scales."peer support"]
items = ["ps1", "ps2", "ps3"]
```

**Note:** Use quotes for names with spaces.

### Mixed Response Formats

```toml
# Scale A: 1-5 Likert
[scales.likert_items]
items = ["lik1", "lik2", "lik3"]
min_value = 1.0
max_value = 5.0

# Scale B: 0-10 VAS
[scales.vas_items]
items = ["vas1", "vas2", "vas3"]
min_value = 0.0
max_value = 10.0

# Scale C: 1-7 scale
[scales.seven_point]
items = ["sp1", "sp2", "sp3"]
min_value = 1.0
max_value = 7.0
```

### Large Scale Example

```toml
[scales.long_scale]
items = [
    "item1", "item2", "item3", "item4", "item5",
    "item6", "item7", "item8", "item9", "item10",
    "item11", "item12", "item13", "item14", "item15",
    "item16", "item17", "item18", "item19", "item20"
]
reverse_scored = [
    "item3", "item7", "item11", "item15", "item19"
]
min_value = 1.0
max_value = 5.0
```

---

## Examples

### Example 1: Burnout Study

```toml
[survey]
name = "Healthcare Worker Burnout"
description = "MBI and support measures"
version = "1.0"

[quality]
missing_threshold = 10.0

[scales.emotional_exhaustion]
items = ["ee1", "ee2", "ee3", "ee4", "ee5"]
min_value = 1.0
max_value = 7.0

[scales.depersonalization]
items = ["dp1", "dp2", "dp3"]
min_value = 1.0
max_value = 7.0

[scales.personal_accomplishment]
items = ["pa1", "pa2", "pa3", "pa4"]
reverse_scored = ["pa1", "pa4"]  # Positive items need reversing
min_value = 1.0
max_value = 7.0
```

### Example 2: Therapy Alliance Study

```toml
[survey]
name = "Working Alliance Study"
description = "WAI and outcome measures"

[quality]
missing_threshold = 5.0  # Strict for clinical data

[scales.alliance_bond]
items = ["wai1", "wai2", "wai3", "wai4"]
reverse_scored = ["wai4"]
min_value = 1.0
max_value = 5.0

[scales.alliance_task]
items = ["wai5", "wai6", "wai7", "wai8"]
min_value = 1.0
max_value = 5.0

[scales.alliance_goal]
items = ["wai9", "wai10", "wai11", "wai12"]
reverse_scored = ["wai10"]
min_value = 1.0
max_value = 5.0
```

### Example 3: Minimal Config

```toml
[survey]
name = "Quick Survey"

[quality]
missing_threshold = 10.0

[scales.total]
items = ["q1", "q2", "q3", "q4", "q5"]
```

---

## Validation

### What Gets Validated

Prism validates your config before processing:

✅ **Structure Checks:**

- Required sections present (`[survey]`, `[quality]`, at least one scale)
- Valid TOML syntax

✅ **Scale Checks:**

- All `items` exist in CSV columns
- `reverse_scored` items exist in `items` list
- `min_value` < `max_value` (if specified)

✅ **Data Checks:**

- All scale items have numeric values
- Values within specified ranges (if min/max set)

### Common Validation Errors

**Error: "Scale 'anxiety' references column 'anx6' which doesn't exist in CSV"**

```toml
# ❌ Wrong: Column name mismatch
[scales.anxiety]
items = ["anx1", "anx2", "anx6"]  # anx6 doesn't exist

# ✅ Fix: Use correct column names
[scales.anxiety]
items = ["anx1", "anx2", "anx3"]
```

**Error: "Reverse scored item 'q5' not in items list"**

```toml
# ❌ Wrong: Reverse item not in items
[scales.test]
items = ["q1", "q2", "q3"]
reverse_scored = ["q5"]  # q5 not in items

# ✅ Fix: Only reverse items in the list
[scales.test]
items = ["q1", "q2", "q3"]
reverse_scored = ["q2"]
```

**Error: "Invalid range: min (5.0) >= max (3.0)"**

```toml
# ❌ Wrong: Min greater than max
[scales.test]
items = ["q1", "q2"]
min_value = 5.0
max_value = 3.0

# ✅ Fix: Min less than max
[scales.test]
items = ["q1", "q2"]
min_value = 1.0
max_value = 5.0
```

---

## Best Practices

### ✅ DO:

- Use descriptive scale names
- Document reverse-scored items
- Set appropriate missing thresholds
- Specify min/max ranges for validation
- Keep configs in version control
- Test with sample data first

### ❌ DON'T:

- Use spaces in scale names (without quotes)
- Forget to list reverse-scored items
- Set missing threshold too strict (< 5%)
- Skip range validation on restricted scales
- Reuse scale names

---

## Template

**Copy this template to get started:**

```toml
[survey]
name = "Your Survey Name"
description = "Brief description"
version = "1.0"

[quality]
missing_threshold = 10.0

[scales.your_scale_name]
items = [
    "item1", "item2", "item3"
]
reverse_scored = ["item2"]  # If any
min_value = 1.0
max_value = 5.0
```

---

## Wiki Navigation

- [📚 Back to Wiki Home](README.md)
- [📖 How to Use](HOW_TO_USE.md)
- [📊 Workflow Examples](WORKFLOW_EXAMPLE.md)
- [✅ Quality Checks](QUALITY_CHECKS.md)
- [❓ FAQ](FAQ.md)

---

**Related:**

- [How to Use →](HOW_TO_USE.md)
- [Examples →](WORKFLOW_EXAMPLE.md)
- [Tutorial →](TUTORIAL.md)

---

[⬆ Back to Top](#configuration-guide) | [📚 Wiki Home](README.md)
