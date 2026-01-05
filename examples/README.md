# Examples

This folder contains example configurations and sample data to help you get started.

## Files

- **`sample_data.csv`** - Example survey data with 2 participants
- **`study_config.toml`** - Example configuration with 5 scales (emotional exhaustion, depersonalization, peer support, supervision rapport, and therapeutic alliance)

## Quick Start

Try processing the sample data:

```bash
# From project root
cargo build --release

./target/release/prism \
  --input examples/sample_data.csv \
  --config examples/study_config.toml \
  --output examples/output_clean.csv \
  --stats-output examples/output_stats.txt \
  --quality-report examples/output_quality.txt
```

## Longitudinal Data Examples (v0.3.0+)

### Example 1: Merge Multiple Waves

Combine data from three time points:

```bash
prism merge \
  --waves T1:examples/wave1_data.csv T2:examples/wave2_data.csv T3:examples/wave3_data.csv \
  --id ParticipantID \
  --join outer \
  -o examples/merged_waves.csv
```

### Example 2: Calculate Reliable Change

Determine clinically significant change between baseline and follow-up:

```bash
prism rci \
  -i examples/merged_waves.csv \
  --baseline depression_T1 \
  --followup depression_T2 \
  --reliability 0.89 \
  --id ParticipantID \
  -o examples/rci_results.csv
```

### Example 3: Reshape to Long Format

Convert for growth curve modeling:

```bash
prism reshape \
  -i examples/merged_waves.csv \
  --format wide-to-long \
  --waves T1 T2 T3 \
  --id ParticipantID \
  -o examples/long_format.csv
```

### Example 4: Complete Longitudinal Workflow

```bash
# Step 1: Process each wave
prism process -i examples/baseline.csv -c examples/study_config.toml -o examples/clean_T1.csv
prism process -i examples/followup.csv -c examples/study_config.toml -o examples/clean_T2.csv

# Step 2: Merge waves
prism merge --waves T1:examples/clean_T1.csv T2:examples/clean_T2.csv \
  --id ParticipantID --join outer -o examples/merged.csv

# Step 3: Calculate RCI
prism rci -i examples/merged.csv --baseline PHQ9_total_T1 --followup PHQ9_total_T2 \
  --reliability 0.89 --id ParticipantID -o examples/rci.csv

# Step 4: Reshape for analysis
prism reshape -i examples/merged.csv --format wide-to-long --waves T1 T2 \
  --id ParticipantID -o examples/long.csv
```

## Creating Your Own Config

Copy `study_config.toml` and modify it for your survey:

```toml
[survey]
name = "Your Study Name"
min_score = 1
max_score = 7

[quality]
max_missing_percent = 0.10
flag_straightlining = true

[scales.your_scale]
items = ["Q1", "Q2", "Q3"]
reverse_scored = ["Q2"]  # Optional
```

See [`docs/HOW_TO_USE.md`](../docs/HOW_TO_USE.md) for detailed instructions.
