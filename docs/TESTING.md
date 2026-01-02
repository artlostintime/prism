# Testing Guide

**[📚 Wiki Home](README.md)** | **[💻 Development](DEVELOPMENT.md)** | **[🤝 Contributing](CONTRIBUTING.md)** | **[🏗️ Architecture](ARCHITECTURE.md)**

---

## Overview

This guide covers testing practices, tools, and guidelines for Prism development.

---

## Table of Contents

1. [Testing Philosophy](#testing-philosophy)
2. [Test Types](#test-types)
3. [Writing Tests](#writing-tests)
4. [Running Tests](#running-tests)
5. [Test Data](#test-data)
6. [Coverage](#coverage)

---

## Testing Philosophy

### Why Test?

- ✅ Catch bugs early
- ✅ Enable refactoring
- ✅ Document behavior
- ✅ Improve design
- ✅ Build confidence

### What to Test

**High Priority:**

- Core algorithms (scale calculations, reverse scoring)
- Data processing logic
- Config validation
- Quality checks
- Error handling

**Medium Priority:**

- Edge cases
- Input validation
- File I/O

**Low Priority:**

- UI interactions
- Print statements
- Trivial getters/setters

---

## Test Types

### Unit Tests

**Test individual functions in isolation.**

**Location:** Same file as code

```rust
// src/main.rs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_scoring() {
        let result = reverse_score(5.0, 1.0, 5.0);
        assert_eq!(result, 1.0);
    }
}
```

### Integration Tests

**Test multiple components together.**

**Location:** `tests/` directory

```rust
// tests/integration_test.rs
use prism::config::SurveyConfig;

#[test]
fn test_full_pipeline() {
    // Test end-to-end processing
}
```

### Property-Based Tests

**Test with generated inputs (optional).**

```rust
#[cfg(test)]
mod tests {
    use quickcheck::quickcheck;

    quickcheck! {
        fn reverse_score_twice_equals_original(val: f64, min: f64, max: f64) -> bool {
            let reversed = reverse_score(val, min, max);
            let restored = reverse_score(reversed, min, max);
            (restored - val).abs() < 1e-6
        }
    }
}
```

---

## Writing Tests

### Test Structure

**AAA Pattern: Arrange, Act, Assert**

```rust
#[test]
fn test_calculate_mean() {
    // Arrange - Set up test data
    let values = vec![1.0, 2.0, 3.0, 4.0, 5.0];

    // Act - Call function under test
    let stats = Stats::calculate(&values);

    // Assert - Verify results
    assert_eq!(stats.mean, 3.0);
}
```

### Naming Conventions

**Pattern:** `test_[function]_[scenario]_[expected]`

```rust
#[test]
fn test_process_scale_with_reverse_scoring_returns_correct_mean() {
    // Test implementation
}

#[test]
fn test_validate_config_with_missing_column_returns_error() {
    // Test implementation
}
```

### Assertions

**Common assertions:**

```rust
// Equality
assert_eq!(actual, expected);
assert_ne!(actual, unexpected);

// Boolean
assert!(condition);
assert!(!condition);

// Floating point (use epsilon)
assert!((actual - expected).abs() < 1e-6);

// Result types
assert!(result.is_ok());
assert!(result.is_err());

// Option types
assert!(option.is_some());
assert!(option.is_none());
```

### Error Testing

**Test error cases:**

```rust
#[test]
fn test_validate_config_with_invalid_range_returns_error() {
    let config = create_invalid_config();
    let result = validate_config(&config, &headers);

    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Invalid range"));
}
```

### Fixture Data

**Use helper functions:**

```rust
fn create_test_config() -> SurveyConfig {
    SurveyConfig {
        survey: SurveySettings {
            name: "Test Survey".to_string(),
        },
        quality: QualitySettings {
            missing_threshold: 10.0,
        },
        scales: HashMap::new(),
    }
}

#[test]
fn test_with_fixture() {
    let config = create_test_config();
    // Use config in test
}
```

---

## Running Tests

### Basic Commands

**Run all tests:**

```bash
cargo test
```

**Run specific test:**

```bash
cargo test test_name
```

**Run tests matching pattern:**

```bash
cargo test reverse_scoring
```

**Run tests in specific file:**

```bash
cargo test --test integration_test
```

### Test Output

**Show print statements:**

```bash
cargo test -- --nocapture
```

**Show all test names:**

```bash
cargo test -- --list
```

**Verbose output:**

```bash
cargo test -- --test-threads=1 --nocapture
```

### Parallel vs Sequential

**Parallel (default):**

```bash
cargo test
```

**Sequential (for debugging):**

```bash
cargo test -- --test-threads=1
```

---

## Test Data

### Test Fixtures

**Structure:**

```
tests/
├── fixtures/
│   ├── test_data.csv
│   ├── test_config.toml
│   ├── expected_output.csv
│   └── invalid_config.toml
├── integration_test.rs
└── README.md
```

### Example Test Data

**tests/fixtures/test_data.csv:**

```csv
participant_id,q1,q2,q3
001,5,4,3
002,3,2,4
003,1,1,1
```

**tests/fixtures/test_config.toml:**

```toml
[survey]
name = "Test Survey"

[quality]
missing_threshold = 10.0

[scales.test_scale]
items = ["q1", "q2", "q3"]
reverse_scored = ["q2"]
min_value = 1.0
max_value = 5.0
```

### Using Fixtures

```rust
#[test]
fn test_with_fixture_files() {
    let config_path = "tests/fixtures/test_config.toml";
    let data_path = "tests/fixtures/test_data.csv";

    let config = SurveyConfig::from_file(config_path).unwrap();
    // Process data and verify
}
```

---

## Coverage

### Measuring Coverage

**Install tarpaulin:**

```bash
cargo install cargo-tarpaulin
```

**Run coverage:**

```bash
cargo tarpaulin --out Html --output-dir coverage
# Open coverage/index.html
```

### Coverage Goals

**Target:**

- Core logic: 90%+
- Utilities: 80%+
- Total: 80%+

**Don't aim for 100%:**

- Some code is hard to test (UI, I/O)
- Focus on critical paths
- Diminishing returns > 90%

---

## Example Test Suite

### Complete Test Example

```rust
// src/main.rs
fn reverse_score(value: f64, min: f64, max: f64) -> f64 {
    (max + min) - value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_score_max_becomes_min() {
        let result = reverse_score(5.0, 1.0, 5.0);
        assert_eq!(result, 1.0);
    }

    #[test]
    fn test_reverse_score_min_becomes_max() {
        let result = reverse_score(1.0, 1.0, 5.0);
        assert_eq!(result, 5.0);
    }

    #[test]
    fn test_reverse_score_middle_stays_middle() {
        let result = reverse_score(3.0, 1.0, 5.0);
        assert_eq!(result, 3.0);
    }

    #[test]
    fn test_reverse_score_twice_equals_original() {
        let original = 4.0;
        let reversed = reverse_score(original, 1.0, 5.0);
        let restored = reverse_score(reversed, 1.0, 5.0);
        assert!((restored - original).abs() < 1e-6);
    }

    #[test]
    fn test_reverse_score_different_range() {
        let result = reverse_score(7.0, 1.0, 7.0);
        assert_eq!(result, 1.0);
    }
}
```

---

## Testing Best Practices

### ✅ DO:

- Write tests before fixing bugs
- Test edge cases (empty, zero, negative)
- Use descriptive test names
- Keep tests simple and focused
- Test one thing per test
- Use fixtures for complex setups
- Test error cases

### ❌ DON'T:

- Test implementation details
- Write flaky tests (non-deterministic)
- Use real file I/O when possible to mock
- Test external dependencies directly
- Skip tests with `#[ignore]` without reason
- Write tests that depend on execution order
- Test trivial code

---

## Test Organization

### Module Organization

```rust
// src/main.rs
pub mod config;
pub mod processing;
pub mod quality;

#[cfg(test)]
mod tests {
    use super::*;

    mod config_tests {
        use super::*;
        // Config tests here
    }

    mod processing_tests {
        use super::*;
        // Processing tests here
    }

    mod quality_tests {
        use super::*;
        // Quality tests here
    }
}
```

### File Organization

```
src/
├── main.rs              # Contains #[cfg(test)] mod tests
├── config.rs            # Contains #[cfg(test)] mod tests
└── lib.rs               # Contains #[cfg(test)] mod tests

tests/
├── integration_test.rs  # Integration tests
├── cli_test.rs          # CLI integration tests
└── fixtures/            # Test data
```

---

## Continuous Integration

### GitHub Actions

**`.github/workflows/test.yml`:**

```yaml
name: Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - name: Run tests
        run: cargo test --verbose
      - name: Run clippy
        run: cargo clippy -- -D warnings
```

---

## Debugging Tests

### Failed Test

**Example output:**

```
test tests::test_reverse_scoring ... FAILED

failures:

---- tests::test_reverse_scoring stdout ----
thread 'tests::test_reverse_scoring' panicked at 'assertion failed: `(left == right)`
  left: `2.0`,
 right: `1.0`', src/main.rs:100:9
```

**Debug steps:**

1. Add print statements
2. Run single test with `--nocapture`
3. Use debugger
4. Check assumptions

```rust
#[test]
fn test_reverse_scoring() {
    let result = reverse_score(5.0, 1.0, 5.0);
    println!("result = {}", result);  // Debug output
    assert_eq!(result, 1.0);
}
```

---

## Wiki Navigation

- [📚 Back to Wiki Home](README.md)
- [💻 Development Guide](DEVELOPMENT.md)
- [🤝 Contributing Guide](CONTRIBUTING.md)
- [🏗️ Architecture](ARCHITECTURE.md)
- [🔍 API Reference](API_REFERENCE.md)

---

[⬆ Back to Top](#testing-guide) | [📚 Wiki Home](README.md)
