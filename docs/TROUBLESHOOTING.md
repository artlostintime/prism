# Troubleshooting Guide

**[📚 Wiki Home](README.md)** | **[❓ FAQ](FAQ.md)** | **[📖 How to Use](HOW_TO_USE.md)** | **[🔧 Configuration](CONFIGURATION_GUIDE.md)**

---

## Quick Diagnostic

**Select your issue:**

- [Installation Problems](#installation-problems)
- [Configuration Errors](#configuration-errors)
- [CSV/Data Issues](#csvdata-issues)
- [Processing Errors](#processing-errors)
- [Output Problems](#output-problems)
- [Quality Check Issues](#quality-check-issues)
- [Performance Issues](#performance-issues)

---

## Installation Problems

### "cargo: command not found"

**Error:**

```
'cargo' is not recognized as an internal or external command
```

**Cause:** Rust/Cargo not installed or not in PATH.

**Solution:**

**1. Install Rust:**

```bash
# Visit https://rustup.rs/ or:
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

**2. Restart terminal:**

```bash
# Close and reopen terminal, then verify:
cargo --version
```

**3. If still not working (Windows):**

- Restart computer
- Check PATH includes: `%USERPROFILE%\.cargo\bin`

---

### Build Fails on Windows

**Error:**

```
error: linker `link.exe` not found
```

**Cause:** Missing Visual Studio Build Tools.

**Solution:**

**1. Download Visual Studio Build Tools:**

- Visit: https://visualstudio.microsoft.com/downloads/
- Select "Build Tools for Visual Studio 2022"

**2. During installation, select:**

- ✅ Desktop development with C++
- ✅ Windows 10/11 SDK

**3. Restart and rebuild:**

```bash
cargo build --release
```

---

### "linking with `cc` failed" (Linux)

**Error:**

```
error: linking with `cc` failed: exit status: 1
```

**Cause:** Missing build dependencies.

**Solution:**

**Ubuntu/Debian:**

```bash
sudo apt update
sudo apt install build-essential pkg-config libssl-dev
```

**Fedora/RHEL:**

```bash
sudo dnf install gcc pkg-config openssl-devel
```

**Then rebuild:**

```bash
cargo build --release
```

---

### GUI Build Fails

**Error:**

```
error: failed to run custom build command for `webkit2gtk-sys`
```

**Cause:** Missing Tauri dependencies.

**Solution:**

**Windows:** Install Visual Studio with C++ Desktop Development

**macOS:**

```bash
xcode-select --install
```

**Linux:**

```bash
sudo apt install libwebkit2gtk-4.0-dev \
  libgtk-3-dev \
  libayatana-appindicator3-dev
```

---

## Configuration Errors

### "Failed to parse config file"

**Error:**

```
Error: Failed to parse config file: TOML parse error
```

**Cause:** Invalid TOML syntax.

**Common mistakes:**

**❌ Missing quotes:**

```toml
name = My Survey  # Wrong!
```

✅ **Fix:**

```toml
name = "My Survey"
```

**❌ Wrong array syntax:**

```toml
items = [q1, q2, q3]  # Wrong!
```

✅ **Fix:**

```toml
items = ["q1", "q2", "q3"]
```

**❌ Missing section brackets:**

```toml
scales.anxiety  # Wrong!
```

✅ **Fix:**

```toml
[scales.anxiety]
```

**❌ Extra commas:**

```toml
items = ["q1", "q2",]  # Wrong!
```

✅ **Fix:**

```toml
items = ["q1", "q2"]
```

**Validation tool:**

```bash
# Test your TOML online:
# https://www.toml-lint.com/
```

---

### "Scale references non-existent column"

**Error:**

```
Error: Scale 'anxiety' references column 'anx6' which doesn't exist in CSV
```

**Cause:** Config item name doesn't match CSV column name.

**Solution:**

**1. Check CSV headers (case-sensitive):**

```csv
participant_id,anx1,anx2,anx3  ← Actual columns
```

**2. Match exactly in config:**

```toml
[scales.anxiety]
items = ["anx1", "anx2", "anx3"]  # Must match CSV
```

**Common mistakes:**

- Case mismatch: `ANX1` vs `anx1`
- Spaces: `anx 1` vs `anx1`
- Underscores: `anx_1` vs `anx1`

**Debug tip:**

```bash
# View CSV headers:
head -1 your_data.csv
```

---

### "Reverse scored item not in items list"

**Error:**

```
Error: Reverse scored item 'q5' not found in items list for scale 'test'
```

**Cause:** Item listed in `reverse_scored` but not in `items`.

**Solution:**

**❌ Wrong:**

```toml
[scales.test]
items = ["q1", "q2", "q3"]
reverse_scored = ["q5"]  # q5 not in items!
```

**✅ Fix:**

```toml
[scales.test]
items = ["q1", "q2", "q3", "q5"]
reverse_scored = ["q5"]  # Now it's in items
```

---

### "Invalid range: min >= max"

**Error:**

```
Error: Invalid range for scale 'test': min (5.0) >= max (3.0)
```

**Cause:** Minimum value not less than maximum.

**Solution:**

**❌ Wrong:**

```toml
min_value = 5.0
max_value = 3.0
```

**✅ Fix:**

```toml
min_value = 1.0
max_value = 5.0
```

---

## CSV/Data Issues

### "Failed to read CSV file"

**Error:**

```
Error: Failed to read CSV file: permission denied
```

**Cause:** File is open in another program or lacks permissions.

**Solution:**

**1. Close the file in Excel/other programs**

**2. Check permissions:**

```bash
# Windows
icacls your_file.csv

# macOS/Linux
ls -l your_file.csv
chmod 644 your_file.csv  # If needed
```

**3. Check file path:**

```bash
# Use absolute path:
prism -i "C:\full\path\to\data.csv" -c config.toml -o output.csv
```

---

### "No valid numeric data found"

**Error:**

```
Error: No valid numeric data found for scale 'anxiety'
```

**Cause:** Non-numeric values in scale columns.

**Common issues:**

**❌ Text in numeric columns:**

```csv
participant_id,q1,q2,q3
001,5,4,NA        # "NA" is text!
002,3,Yes,2       # "Yes" is text!
```

**✅ Fix:**

```csv
participant_id,q1,q2,q3
001,5,4,          # Empty for missing
002,3,,2          # Empty for missing
```

**❌ Text responses:**

```csv
participant_id,q1,q2
001,Agree,Strongly Agree  # Text scales!
```

**✅ Fix:** Recode to numbers:

```csv
participant_id,q1,q2
001,4,5  # Agree=4, Strongly Agree=5
```

---

### "CSV has no data rows"

**Error:**

```
Error: CSV file has no data rows (only headers)
```

**Cause:** CSV only contains header row, no actual data.

**Solution:**

**Check your file:**

```csv
participant_id,q1,q2,q3  ← Header only, no data!
```

**Should be:**

```csv
participant_id,q1,q2,q3  ← Header
001,5,4,3                ← Data rows
002,3,2,4
```

---

### "Participant ID column missing"

**Error:**

```
Error: First column must be participant ID
```

**Cause:** CSV doesn't have ID column as first column.

**Solution:**

**❌ Wrong:**

```csv
q1,q2,q3,participant_id  # ID not first!
```

**✅ Fix:**

```csv
participant_id,q1,q2,q3  # ID first
001,5,4,3
```

---

## Processing Errors

### Processing Hangs/Freezes

**Symptom:** Prism runs but never completes.

**Possible causes:**

**1. Very large file:**

- Files > 100MB may take time
- Check progress in task manager

**2. Infinite loop (bug):**

- Report as issue with data sample

**Solution:**

**Test with subset:**

```bash
# Create small test file (first 100 rows)
head -101 large_file.csv > test.csv  # Unix
# Or use Excel to save first 100 rows

# Test processing:
prism -i test.csv -c config.toml -o test_output.csv
```

---

### "Thread panicked" Error

**Error:**

```
thread 'main' panicked at 'attempt to divide by zero'
```

**Cause:** Bug or edge case in code.

**Solution:**

**1. Report the issue with:**

- Config file
- Sample data (2-3 rows)
- Exact command used

**2. Workaround:**

- Try without optional flags
- Check for zero-variance scales

---

## Output Problems

### Output File is Empty

**Symptom:** Output CSV is created but has no content.

**Cause:** Processing failed silently.

**Solution:**

**1. Check console output for errors**

**2. Verify config:**

```bash
# Try verbose mode (if available):
prism -i data.csv -c config.toml -o output.csv -v
```

**3. Check config has scales:**

```toml
[scales.test]
items = ["q1", "q2"]  # Must have at least one scale
```

---

### "Permission denied" When Saving

**Error:**

```
Error: Permission denied: output.csv
```

**Cause:** File is open or folder is read-only.

**Solution:**

**1. Close output file** in Excel/other programs

**2. Use different filename:**

```bash
prism -i data.csv -c config.toml -o output_new.csv
```

**3. Check folder permissions:**

```bash
# Windows: Run as administrator if needed
# macOS/Linux:
chmod +w /path/to/folder
```

---

### Missing Columns in Output

**Symptom:** Expected scale columns not in output.

**Cause:** Scale name doesn't match expectations.

**Solution:**

Scale columns are named: `scalename_total` and `scalename_mean`

**Config:**

```toml
[scales.anxiety]
items = ["q1", "q2"]
```

**Output columns:**

```
anxiety_total
anxiety_mean
```

**If scale has spaces:**

```toml
[scales."emotional exhaustion"]
```

**Output:**

```
emotional exhaustion_total
emotional exhaustion_mean
```

---

## Quality Check Issues

### Too Many Straightlining Flags

**Symptom:** Many participants flagged for straightlining.

**Possible causes:**

**1. Floor/ceiling effects** (legitimate):

```
Scale: pain (1-10 scale)
Many participants report "1" (no pain) → Valid!
```

**2. Poor survey design:**

- All items worded similarly
- Response sets encouraged

**Solutions:**

**For floor/ceiling:**

```toml
[quality]
straightlining_enabled = false  # Disable if expected
```

**For survey issues:**

- Add reverse-coded items
- Vary item wording
- Add attention checks

---

### Missing Data Threshold Too Strict

**Symptom:** Too many participants flagged for missing data.

**Solution:**

**Adjust threshold:**

```toml
[quality]
# Was:
missing_threshold = 5.0   # Very strict

# Try:
missing_threshold = 15.0  # More lenient
```

**Or analyze patterns:**

- Is missing random or systematic?
- Are specific items problematic?
- Consider survey length

---

### Out-of-Range False Positives

**Symptom:** Valid responses flagged as out-of-range.

**Cause:** Incorrect min/max specified.

**Solution:**

**Check your scale range:**

```toml
# If scale is actually 0-10:
[scales.vas]
items = ["vas1", "vas2"]
min_value = 0.0   # Include 0!
max_value = 10.0
```

**Common mistakes:**

- Forgetting 0 in 0-10 scales
- Wrong range for some items

---

## Performance Issues

### Processing is Slow

**Symptom:** Large files take a long time.

**Expected performance:**

- 1,000 rows: 1-2 seconds
- 10,000 rows: 5-10 seconds
- 100,000 rows: 30-60 seconds

**If slower:**

**1. Check file size:**

```bash
# Windows
dir data.csv

# macOS/Linux
ls -lh data.csv
```

**2. Reduce columns:**

- Remove unnecessary columns from CSV
- Only include needed items

**3. Use release build:**

```bash
cargo build --release  # Not --debug
./target/release/prism  # Not ./target/debug/prism
```

---

## Getting More Help

### Enable Verbose Output

```bash
# If available:
prism -i data.csv -c config.toml -o output.csv --verbose
```

### Check Logs

Look for error details in console output.

### Create Minimal Example

**1. Create small test file (3 rows):**

```csv
participant_id,q1,q2,q3
001,5,4,3
002,3,2,4
```

**2. Create minimal config:**

```toml
[survey]
name = "Test"

[quality]
missing_threshold = 10.0

[scales.test]
items = ["q1", "q2", "q3"]
```

**3. Test:**

```bash
prism -i test.csv -c test_config.toml -o test_output.csv
```

---

## Reporting Bugs

If you can't resolve the issue:

**Include:**

1. ✅ Prism version (`prism --version`)
2. ✅ Operating system
3. ✅ Exact error message
4. ✅ Config file (sanitized)
5. ✅ Sample data (2-3 rows)
6. ✅ Command used

**Don't include:**

- ❌ Real participant data
- ❌ Identifiable information

**Report at:** GitHub Issues

---

## Wiki Navigation

- [📚 Back to Wiki Home](README.md)
- [❓ FAQ](FAQ.md)
- [📖 How to Use](HOW_TO_USE.md)
- [🔧 Configuration Guide](CONFIGURATION_GUIDE.md)
- [🎓 Tutorial](TUTORIAL.md)

---

[⬆ Back to Top](#troubleshooting-guide) | [📚 Wiki Home](README.md)
