# Prism Test Suite

Comprehensive test suite for the Prism survey data processing tool.

## Overview

The test suite contains **24 tests** across 4 test files:

- **Integration Tests** (9 tests) - End-to-end CLI workflows
- **Calculation Tests** (5 tests) - Mathematical correctness validation
- **Quality Tests** (5 tests) - Quality check algorithm testing
- **Config Validation Tests** (5 tests) - Configuration error handling

## Running Tests

Run all tests:

```bash
cargo test
```

Run specific test suite:

```bash
cargo test --test integration_test
cargo test --test calculation_test
cargo test --test quality_test
cargo test --test config_validation_test
```

Run a single test:

```bash
cargo test test_reverse_scoring_calculation
```

Run with output:

```bash
cargo test -- --nocapture
```

## Test Coverage

### Integration Tests (`integration_test.rs`)

End-to-end testing of CLI workflows:

- ✓ CLI help and version display
- ✓ Basic data processing workflow
- ✓ Statistics output generation
- ✓ Quality report generation
- ✓ Combined outputs (stats + quality)
- ✓ Error handling for missing files
- ✓ Straightlining detection in real data

### Calculation Tests (`calculation_test.rs`)

Validates mathematical correctness:

- ✓ Reverse scoring formula: `(max + min) - value`
- ✓ Scale total calculation (sum of items)
- ✓ Scale mean calculation (average of items)
- ✓ Aggregate statistics (mean, SD, range across participants)
- ✓ Missing data handling (graceful degradation)

### Quality Tests (`quality_test.rs`)

Tests quality check detection algorithms:

- ✓ Straightlining detection (all identical responses)
- ✓ No false positives (varied responses pass)
- ✓ High missing data percentage detection (>10%)
- ✓ Out-of-range value detection
- ✓ Multiple simultaneous quality issues

### Config Validation Tests (`config_validation_test.rs`)

Ensures robust error handling:

- ✓ Invalid column references (item not in CSV)
- ✓ Reverse item validation (must be in items list)
- ✓ Min > max validation (invalid range)
- ✓ Valid configuration acceptance
- ✓ Malformed TOML detection

## Test Fixtures

Tests use automatically created fixtures in `tests/fixtures/`:

- Created temporarily by each test
- Cleaned up automatically after test completion
- No persistent test data required

Test outputs are written to `tests/output/`:

- Generated during test execution
- Can be inspected for debugging
- Cleaned up automatically

## CI/CD Integration

Tests run automatically on:

- Every commit via GitHub Actions
- Pull requests
- Release builds

Ensures code quality and prevents regressions.

## Adding New Tests

1. Choose appropriate test file based on test type
2. Follow existing test structure:
   ```rust
   #[test]
   fn test_your_feature() {
       // Create test fixtures
       let test_csv = "tests/fixtures/test_feature.csv";
       fs::write(test_csv, "...").unwrap();

       // Run command
       let mut cmd = Command::cargo_bin("prism").unwrap();
       cmd.arg("-i").arg(test_csv);
       cmd.assert().success();

       // Verify output
       assert!(result.contains("expected"));

       // Clean up
       let _ = fs::remove_file(test_csv);
   }
   ```
3. Run tests to verify: `cargo test`

## Test Philosophy

- **Comprehensive**: Cover all major features
- **Fast**: All tests complete in <1 second
- **Isolated**: Each test cleans up after itself
- **Readable**: Clear test names and assertions
- **Maintainable**: Follow consistent patterns

## Critical for Research

These tests are especially important for Prism because:

1. **Data Accuracy**: Psychology research requires precise calculations
2. **Reverse Scoring**: Easy to implement incorrectly - tests verify formula
3. **Quality Checks**: Ensures detection algorithms work correctly
4. **Config Validation**: Prevents user errors that could corrupt data
5. **Regression Prevention**: Maintains reliability across changes

Run tests before every commit to maintain data integrity!
