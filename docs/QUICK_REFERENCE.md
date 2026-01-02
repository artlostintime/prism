# Prism Quick Reference

**[📚 Wiki Home](README.md)** | **[📖 Full Guide](HOW_TO_USE.md)** | **[🔧 Config](CONFIGURATION_GUIDE.md)** | **[❓ FAQ](FAQ.md)**

---

## Installation

```bash
cargo build --release
```

## Basic Commands

### Process Survey Data

```bash
prism --input data.csv --config survey.toml --output clean.csv
```

### With Reports

```bash
prism \
  -i data.csv \
  -c survey.toml \
  -o clean.csv \
  --stats-output summary.txt \
  --quality-report quality.txt
```

### Help

```bash
prism --help
```

## Config Template

```toml
[survey]
name = "Study Name"
min_score = 1
max_score = 7

[quality]
max_missing_percent = 0.10
flag_straightlining = true

[scales.scale_name]
items = ["Q1", "Q2", "Q3"]
reverse_scored = ["Q2"]
```

## Output Files

| File                 | Contains                                |
| -------------------- | --------------------------------------- |
| `clean_data.csv`     | Original + scale scores + quality flags |
| `summary_stats.txt`  | M, SD, min, max, N per scale            |
| `quality_report.txt` | Detailed quality issues                 |

## Quality Flags

| Flag                | Meaning                  |
| ------------------- | ------------------------ |
| `OK`                | No issues detected       |
| `Straightlining`    | All identical responses  |
| `Missing`           | Scale completely missing |
| `High missing data` | >threshold% missing      |
| `Out-of-range`      | Value outside min/max    |

## Common Issues

### "Item not found"

→ Check CSV headers match config item names

### "Could not parse CSV"

→ Verify CSV is properly formatted with headers

### "NA" in all scale columns

→ Column name mismatch between CSV and config

## Tips

✓ Keep raw data separate from processed  
✓ Version control your config files  
✓ Always review quality reports  
✓ Spot-check a few participants manually  
✓ Document reverse-scoring decisions

## Example Workflow

1. Export survey data to CSV
2. Create config file describing scales
3. Run: `prism -i data.csv -c config.toml -o clean.csv --stats-output stats.txt`
4. Review quality report
5. Import clean.csv to R/SPSS/Python
6. Analyze!

## Architecture

```
GUI → calls → CLI → processes → outputs
```

- **GUI**: Simple file picker
- **CLI**: All processing logic
- **Zero code duplication**

## Files

```
prism/
├── src/main.rs           # CLI implementation
├── src/config.rs         # Config parsing
├── src-tauri/            # GUI wrapper
├── data/raw/             # Input files
├── data/processed/       # Output files
├── study_config.toml     # Survey definition
└── README.md             # Full documentation
```

## Performance

- 2 participants: <1 second
- 50 participants: <2 seconds
- 500 participants: <10 seconds

## Support

---

## Related Documentation

- [📚 Wiki Home](README.md)
- [📖 Complete Guide](HOW_TO_USE.md)
- [🏗️ Architecture](ARCHITECTURE.md)
- [📊 Examples](WORKFLOW_EXAMPLE.md)
- [❓ FAQ](FAQ.md)

---

[⬆ Back to Top](#prism-quick-reference) | [📚 Wiki Home](README.md)
