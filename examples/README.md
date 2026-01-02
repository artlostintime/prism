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
