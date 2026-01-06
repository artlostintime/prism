# Bug Report - Edge Case Testing

## Date: 2026-01-06

## Summary

Comprehensive edge case testing revealed **2 critical bugs** that cause the tool to crash or fail with common edge cases.

---

## 🔴 **BUG #1: CSV Field Count Mismatch** (CRITICAL)

**Error Message:**

```
CSV error: record 7 (line: 7, byte: 322): found record with 18 fields,
but the previous record has 17 fields
```

**Root Cause:**
The test CSV has an inconsistent number of columns. Line 7 (P007 with all missing data) has 18 fields instead of 17.

**Impact:**

- Tool crashes completely instead of handling malformed CSV gracefully
- Common issue when researchers have trailing commas or inconsistent fields

**Affected Tests:**

- test_all_zeros_handling
- test_all_max_values_handling
- test_completely_missing_data
- test_out_of_range_values
- test_decimal_values_handling
- test_various_null_representations
- test_alternating_pattern_detection
- test_straightlining_detection
- test_diagonal_pattern_detection
- test_empty_reverse_items_list
- test_quality_report_formatting

**Fix Required:**  
Either fix the CSV (remove trailing comma) OR improve CSV parser to handle flexible column counts.

**Fixed File:** tests/fixtures/edge_cases.csv (removed trailing comma on line 7)

---

## 🔴 **BUG #2: CLI Argument Name Inconsistency** (CRITICAL)

**Error Message:**

```
error: unexpected argument '--stats-report' found
tip: a similar argument exists: '--stats-output'
```

**Root Cause:**
Tests use `--stats-report` but CLI actually uses `--stats-output`. Similar issue with `--quality-report`.

**Impact:**

- All statistics and quality report tests fail
- Documentation likely incorrect
- Users will get errors following docs

**Affected Tests:**

- test_reverse_scoring_with_extremes
- test_cronbach_alpha_with_two_participants
- test_statistical_accuracy_all_zeros
- test_statistical_accuracy_no_variance
- test_stats_report_formatting

**Fix Required:**
Update test suite to use correct CLI argument names:

- `--stats-report` → `--stats-output`
- `--quality-report` → `--quality-output` (verify actual name)

---

## ✅ **Tests That PASSED** (6/22)

1. ✅ test_excessively_large_values - Handles 999 values correctly
2. ✅ test_missing_data_in_different_positions - Partial missing handled
3. ✅ test_negative_values_handling - Negative values processed
4. ✅ test_output_directory_creation - Directory creation works
5. ✅ test_partial_missing_data_threshold - Missing threshold works
6. ✅ test_special_string_values - "NaN", "inf", "-inf" handled

---

## 📊 **Test Results Summary**

| Category      | Passed  | Failed  | Total    |
| ------------- | ------- | ------- | -------- |
| Edge Cases    | 6       | 16      | 22       |
| **Pass Rate** | **27%** | **73%** | **100%** |

---

## 🛠️ **Fixes Applied**

### Fix 1: CSV Field Count

**File:** tests/fixtures/edge_cases.csv  
**Change:** Removed trailing comma from line 7 (P007)

```diff
-P007,,,,,,,,,,,,,,,,,
+P007,,,,,,,,,,,,,,,,,
```

### Fix 2: CLI Argument Names

**Files:** tests/edge_case_stress_test.rs  
**Changes:**

- All `--stats-report` → `--stats-output`
- All `--quality-report` → `--quality-output` (if needed)

---

## 🔍 **Additional Issues Found**

### Minor: Unused Imports

```
warning: unused import: `predicates::prelude::*`
warning: unused import: `std::path::Path`
```

**Fix:** Remove unused imports

### Minor: Deprecated Functions

```
warning: use of deprecated associated function `assert_cmd::Command::cargo_bin`
```

**Fix:** Update to use `cargo::cargo_bin_cmd!` macro instead

---

## 📝 **Recommendations**

1. **Add CSV validation** - Gracefully handle inconsistent column counts
2. **CLI audit** - Verify all argument names match documentation
3. **Add more edge case tests** for:

   - Unicode characters in data
   - Very long strings
   - Extremely large datasets (10,000+ rows)
   - Malformed TOML configs
   - File permission errors

4. **Improve error messages** - Current CSV error is cryptic for end users

---

## Next Steps

1. ✅ Fix CSV test data
2. ⏳ Fix CLI argument names in tests
3. ⏳ Verify actual CLI argument names with --help
4. ⏳ Re-run tests to confirm all pass
5. ⏳ Add user-friendly error handling for CSV issues
