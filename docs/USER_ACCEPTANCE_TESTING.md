# User Acceptance Testing Guide

## Overview

This guide helps testers validate that Prism meets real-world research requirements.

## Test Environment Setup

### Prerequisites

- [ ] Windows 10/11, macOS 10.15+, or Linux
- [ ] Test datasets prepared (small, medium, large)
- [ ] Multiple configuration scenarios ready

### Installation Test

- [ ] Install from release package
- [ ] Verify executable permissions
- [ ] Check all files extracted correctly
- [ ] Launch application successfully

---

## Functional Testing

### 1. CLI Interface Tests

#### Basic Processing

```bash
# Test 1: Standard processing
prism process -i test_data.csv -c config.toml -o output.csv
```

- [ ] Processes without errors
- [ ] Creates output file
- [ ] Output has correct columns
- [ ] Scale scores calculated correctly

#### Quality Checks

```bash
# Test 2: Quality reporting
prism process -i test_data.csv -c config.toml --quality-report quality.txt
```

- [ ] Identifies missing data correctly
- [ ] Flags straightlining responses
- [ ] Detects low variance patterns
- [ ] Report is readable and accurate

#### Statistical Output

```bash
# Test 3: Statistics generation
prism process -i test_data.csv -c config.toml --stats-output stats.txt
```

- [ ] Calculates correct means and SDs
- [ ] Cronbach's alpha is accurate
- [ ] All scales included in report
- [ ] Formatting is clear

#### Multi-format Export

```bash
# Test 4: All output formats
prism process -i test_data.csv -c config.toml --all-outputs
```

- [ ] CSV output is valid
- [ ] Excel file opens correctly
- [ ] JSON structure is valid
- [ ] SPSS syntax runs in SPSS
- [ ] R script executes in R

#### Validation

```bash
# Test 5: Config validation
prism validate -c config.toml -i test_data.csv
```

- [ ] Detects missing columns
- [ ] Identifies invalid items
- [ ] Flags configuration errors
- [ ] Provides helpful error messages

---

### 2. GUI Interface Tests

#### File Selection

- [ ] CSV file picker opens
- [ ] Shows file name after selection
- [ ] Displays row/column count
- [ ] "Choose Different" button works
- [ ] Cannot process without CSV

#### Configuration

**Existing Config:**

- [ ] Config file picker opens
- [ ] Shows selected config name
- [ ] Validates TOML syntax
- [ ] Handles invalid configs gracefully

**Edit Config:**

- [ ] Editor opens on click
- [ ] Can paste config text
- [ ] Validate button works
- [ ] Clear button resets editor
- [ ] Proper error messages for invalid syntax

**Examples:**

- [ ] Examples modal opens
- [ ] All 4 templates load
- [ ] Clicking example fills editor
- [ ] Templates are valid TOML

#### Processing

- [ ] Process button enables when ready
- [ ] Shows progress bar during processing
- [ ] Displays status updates
- [ ] Success message appears
- [ ] Error messages are clear
- [ ] Can process multiple files sequentially

#### Output Management

- [ ] Output path displays correctly
- [ ] Open folder button works
- [ ] Opens correct directory
- [ ] All output files present
- [ ] "Process Another" resets interface

#### Keyboard Shortcuts

- [ ] Ctrl+O opens CSV picker
- [ ] Ctrl+K opens config picker
- [ ] Ctrl+Enter starts processing
- [ ] Shortcuts work consistently

#### Visual Feedback

- [ ] Tooltips appear on hover
- [ ] Help dialog shows information
- [ ] Animations are smooth
- [ ] Status colors are correct (green=success, red=error)
- [ ] Progress bar animates

---

## Data Quality Tests

### Test Datasets

#### Dataset 1: Perfect Data

- All responses complete
- No missing values
- Varied responses
- **Expected:** Clean output, high Cronbach's alpha

#### Dataset 2: Missing Data

- 5% missing randomly
- 15% missing in one participant
- **Expected:** Flags participant with >10% missing

#### Dataset 3: Straightlining

- One participant: all 4s
- One participant: all 7s
- **Expected:** Both flagged in quality report

#### Dataset 4: Reverse Scoring

- Config with reverse-scored items
- **Expected:** Correct reverse calculation [(max+min)-value]

#### Dataset 5: Large Scale

- 10,000 participants
- 100 items
- **Expected:** Processes in <10 seconds

---

## Configuration Testing

### Config Scenarios

#### Scenario 1: Simple Scale

```toml
[scales.satisfaction]
items = ["Q1", "Q2", "Q3", "Q4", "Q5"]
reverse_scored = []
```

- [ ] Calculates total correctly
- [ ] Calculates mean correctly
- [ ] Statistics accurate

#### Scenario 2: Multiple Scales

- 5+ different scales
- Various item counts
- **Expected:** All scales computed independently

#### Scenario 3: Reverse Scoring

```toml
[scales.test]
items = ["Q1", "Q2", "Q3"]
reverse_scored = ["Q2"]
```

- [ ] Q2 reversed: (7+1)-value = 8-value
- [ ] Other items unchanged
- [ ] Total reflects reversed score

#### Scenario 4: Edge Cases

- Scale with 1 item
- Scale with 50 items
- Empty reverse_scored list
- All items reverse_scored
- **Expected:** Handles all gracefully

---

## Performance Testing

### Benchmarks

#### Small Dataset (100 rows × 50 cols)

- [ ] Processes in <1 second
- [ ] Memory usage <50MB
- [ ] CPU usage reasonable

#### Medium Dataset (1,000 rows × 100 cols)

- [ ] Processes in <3 seconds
- [ ] Memory usage <100MB
- [ ] Parallel processing utilized

#### Large Dataset (10,000 rows × 100 cols)

- [ ] Processes in <10 seconds
- [ ] Memory usage <500MB
- [ ] No performance degradation

#### Very Large Dataset (50,000 rows × 100 cols)

- [ ] Processes in <60 seconds
- [ ] Memory usage <2GB
- [ ] System remains responsive

---

## Error Handling Tests

### Expected Errors

#### Missing Files

```bash
prism process -i nonexistent.csv -c config.toml
```

- [ ] Clear error message
- [ ] Doesn't crash
- [ ] Suggests solution

#### Invalid CSV

- Malformed CSV (missing commas)
- Empty file
- No header row
- **Expected:** Descriptive error message

#### Invalid Config

- Missing [survey] section
- Invalid TOML syntax
- Items not in CSV
- **Expected:** Validation error with line number

#### Permission Errors

- Read-only input file
- No write permission for output
- **Expected:** Clear permission error

---

## Cross-Platform Testing

### Windows

- [ ] CLI runs in PowerShell
- [ ] CLI runs in CMD
- [ ] GUI launches from installer
- [ ] File paths with spaces work
- [ ] Explorer opens output folder

### macOS

- [ ] CLI runs in Terminal
- [ ] GUI launches as .app
- [ ] File picker works
- [ ] Finder opens output folder

### Linux

- [ ] CLI runs in bash/zsh
- [ ] GUI launches from AppImage
- [ ] File picker works
- [ ] File manager opens folder

---

## Regression Testing

### After Each Update

- [ ] Run all example datasets
- [ ] Verify output matches expected
- [ ] Check no new errors introduced
- [ ] Performance hasn't degraded
- [ ] UI still responsive

---

## User Scenarios

### Scenario A: New User

1. First time using Prism
2. Has CSV and wants to score
3. No existing config

**Steps:**

- [ ] Opens GUI
- [ ] Selects CSV file
- [ ] Clicks "Examples"
- [ ] Chooses relevant template
- [ ] Modifies for their items
- [ ] Processes successfully

**Success Criteria:**

- Completes in <10 minutes
- No errors encountered
- Output is usable

### Scenario B: Regular User

1. Has existing config
2. Processing weekly data

**Steps:**

- [ ] Uses keyboard shortcuts (Ctrl+O, Ctrl+K)
- [ ] Selects files quickly
- [ ] Processes with Ctrl+Enter
- [ ] Opens output folder
- [ ] Moves to next file

**Success Criteria:**

- Workflow takes <2 minutes
- No mouse needed
- Efficient and fast

### Scenario C: Advanced User

1. Large dataset
2. Custom quality thresholds
3. Needs all output formats

**Steps:**

- [ ] Uses CLI for automation
- [ ] Adjusts quality settings
- [ ] Exports to multiple formats
- [ ] Imports to SPSS/R

**Success Criteria:**

- Can automate with scripts
- All formats work correctly
- Integrates with existing workflow

---

## Documentation Testing

### User Documentation

- [ ] Installation guide is clear
- [ ] Examples are accurate
- [ ] Screenshots match current UI
- [ ] Troubleshooting helps resolve issues
- [ ] FAQ answers common questions

### Developer Documentation

- [ ] API docs are accurate
- [ ] Code examples work
- [ ] Architecture diagrams up-to-date
- [ ] Contributing guide is complete

---

## Sign-Off Checklist

### Critical Path

- [ ] All basic processing tests pass
- [ ] Quality checks work correctly
- [ ] GUI is usable and responsive
- [ ] Common errors handled gracefully
- [ ] Performance meets requirements

### Nice-to-Have

- [ ] All keyboard shortcuts work
- [ ] All output formats valid
- [ ] Cross-platform consistency
- [ ] Documentation complete

### Release Criteria

- [ ] No critical bugs
- [ ] All P0/P1 issues resolved
- [ ] Performance benchmarks met
- [ ] User feedback addressed
- [ ] Documentation reviewed

---

## Feedback Collection

### During Testing

**For each issue found:**

- Severity: Critical / High / Medium / Low
- Steps to reproduce
- Expected vs actual behavior
- Screenshots/logs
- Environment details

### After Testing

**Overall impression:**

- Ease of use: 1-5 ⭐
- Performance: 1-5 ⭐
- Reliability: 1-5 ⭐
- Would you recommend? Yes/No
- Top 3 improvements needed

---

## Test Data Sets

### Provided Test Files

1. `test_small.csv` - 100 participants, perfect data
2. `test_missing.csv` - Various missing data patterns
3. `test_quality.csv` - Quality issues for detection
4. `test_large.csv` - 10k participants for performance
5. Corresponding configs for each dataset

### How to Run Complete Test Suite

```bash
# Run all CLI tests
cargo test --release

# Run performance benchmarks
cargo run --release --bin performance_benchmark

# Manual GUI testing with test data
cargo tauri dev
```

---

## Success Criteria

✅ **Ready for Release When:**

- All critical tests pass
- No P0/P1 bugs remaining
- Performance benchmarks met
- At least 3 external testers approve
- Documentation complete and accurate
- Cross-platform testing successful

---

**Tester Name:** ******\_\_\_******  
**Date:** ******\_\_\_******  
**Version Tested:** ******\_\_\_******  
**Overall Status:** ⬜ PASS ⬜ PASS WITH MINOR ISSUES ⬜ FAIL
