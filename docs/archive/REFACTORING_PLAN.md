# Code Quality Analysis & Refactoring Plan

## Issues Identified

### 1. **Excessive `.clone()` and `.to_string()` calls** (Performance Impact: Medium)

**Location:** Throughout codebase - 50+ unnecessary allocations

- `quality.rs`: `issue.clone()` used 9 times when pushing to vectors
- `scales.rs`: 40+ `.to_string()` calls for static data
- `validation.rs`: `.clone()` in similarity scoring

**Impact:** Unnecessary heap allocations, increased memory usage

### 2. **Code Duplication in Quality Checks** (Maintainability: High)

**Location:** `quality.rs` Lines 18-430

- All quality check functions follow identical pattern:
  1. Format issue string
  2. Push to quality_flags (with clone)
  3. Push QualityIssue to quality_issues
- 9 functions with ~90% identical structure

**Impact:** Hard to maintain, error-prone, violates DRY

### 3. **String Literals Should Be Constants** (Maintainability: Medium)

**Location:** Multiple files

- Issue type strings: "MissingData", "Straightlining", "DiagonalPattern", etc.
- Magic strings scattered throughout code
- Risk of typos causing bugs

### 4. **Unclear Variable Names** (Readability: Medium)

**Location:** Various files

- `score_range` in `processor.rs` Line 42 (actually sum, not range)
- `n` used for counts (not descriptive)
- Single-letter variables in statistical functions

### 5. **Missing Const for Epsilon** (Maintainability: Low)

**Location:** `quality.rs` Line 6

- `FLOAT_EPSILON` defined but could be shared across modules
- Should be in a central constants module

### 6. **Inefficient Percentage Calculations** (Performance: Low)

**Location:** Multiple files

- Pattern: `(count as f64 / total as f64) * 100.0` repeated 20+ times
- Should be a utility function

### 7. **Complex Boolean Logic** (Readability: Medium)

**Location:** `quality.rs` Line 60-64

```rust
if config.quality.as_ref().is_some_and(|q| !q.flag_straightlining) {
    return;
}
```

Could be more readable with early return pattern

### 8. **Magic Numbers** (Maintainability: Low)

**Location:** Various files

- Histogram bins: 15
- Careless score weights: 0.3, 0.5, 0.2
- Pattern detection thresholds: 4, 6 items

### 9. **Long Functions** (Maintainability: High)

**Location:** Multiple files

- `generate_html_report()`: 632 lines
- `process_record` in main.rs: 200+ lines
- Should be broken into smaller, testable units

### 10. **Error Messages Could Be More Descriptive** (UX: Medium)

**Location:** Throughout error handling

- Generic "Error:" prefixes
- Missing context about what operation failed
- Could include suggestions for fixes

## Refactoring Recommendations

### Priority 1: High Impact, Low Risk

#### A. Create Utility Functions Module

```rust
// src/utils.rs
pub fn calculate_percentage(count: usize, total: usize) -> f64 {
    if total == 0 {
        0.0
    } else {
        (count as f64 / total as f64) * 100.0
    }
}

pub fn format_percentage(count: usize, total: usize, decimals: usize) -> String {
    format!("{:.decimals$}%", calculate_percentage(count, total), decimals = decimals)
}
```

#### B. Create Constants Module

```rust
// src/constants.rs
// Quality check issue types
pub const ISSUE_MISSING_DATA: &str = "MissingData";
pub const ISSUE_STRAIGHTLINING: &str = "Straightlining";
pub const ISSUE_DIAGONAL_PATTERN: &str = "DiagonalPattern";
pub const ISSUE_ALTERNATING_PATTERN: &str = "AlternatingPattern";
pub const ISSUE_BLOCK_PATTERN: &str = "BlockPattern";
pub const ISSUE_LOW_VARIANCE: &str = "LowVariance";
pub const ISSUE_FAST_RESPONSE: &str = "FastResponse";
pub const ISSUE_SLOW_RESPONSE: &str = "SlowResponse";
pub const ISSUE_SEMANTIC_INCONSISTENCY: &str = "SemanticInconsistency";

// Numerical constants
pub const FLOAT_EPSILON: f64 = 1e-10;
pub const DEFAULT_HISTOGRAM_BINS: usize = 15;
pub const MIN_PATTERN_ITEMS: usize = 4;
pub const MIN_BLOCK_ITEMS: usize = 6;

// Careless responding weights
pub const WEIGHT_MISSING_DATA: f64 = 0.3;
pub const WEIGHT_STRAIGHTLINING: f64 = 0.5;
pub const WEIGHT_LOW_VARIANCE: f64 = 0.2;
```

#### C. Refactor Quality Check Pattern

```rust
// Helper to reduce duplication
fn add_quality_issue(
    participant_id: &str,
    issue_type: &str,
    description: String,
    quality_flags: &mut Vec<String>,
    quality_issues: &mut Vec<QualityIssue>,
) {
    quality_flags.push(description.clone());
    quality_issues.push(QualityIssue::new(participant_id, issue_type, description));
}

// Then all check functions become:
pub fn check_straightlining(...) {
    if should_skip_check(config) {
        return;
    }

    if is_straightlining(item_values) {
        let desc = format!("Straightlining: {}", scale_name);
        add_quality_issue(participant_id, ISSUE_STRAIGHTLINING, desc, quality_flags, quality_issues);
    }
}
```

### Priority 2: Medium Impact, Medium Risk

#### D. Fix Variable Naming

```rust
// BEFORE (processor.rs Line 42)
let score_range = max_score + min_score;

// AFTER
let score_sum_for_reversal = max_score + min_score;
// Or even better: inline it where used
let reversed_score = (max_score + min_score) - original_score;
```

#### E. Simplify Boolean Logic

```rust
// BEFORE
if config.quality.as_ref().is_some_and(|q| !q.flag_straightlining) {
    return;
}

// AFTER
let should_check = config.quality
    .as_ref()
    .map(|q| q.flag_straightlining)
    .unwrap_or(true); // Check by default

if !should_check {
    return;
}
```

#### F. Use Lazy Static for Scale Metadata

```rust
// Instead of recreating strings on every call
use once_cell::sync::Lazy;

static SCALE_NAMES: Lazy<Vec<&'static str>> = Lazy::new(|| {
    vec!["PHQ-9", "GAD-7", "PSS-10", "PSS-14", "PANAS", "BDI-II", "BAI", "SWLS"]
});

pub fn list_available_scales() -> Vec<&'static str> {
    SCALE_NAMES.to_vec()
}
```

### Priority 3: Low Risk Improvements

#### G. Add Type Aliases

```rust
// src/types.rs
pub type ParticipantId = String;
pub type ScaleName = String;
pub type ItemName = String;
pub type HeaderMap = HashMap<String, usize>;
```

#### H. Extract Magic Numbers to Config

```rust
#[derive(Debug, Deserialize, Serialize)]
pub struct VisualizationSettings {
    #[serde(default = "default_histogram_bins")]
    pub histogram_bins: usize,
}

fn default_histogram_bins() -> usize {
    15
}
```

## Implementation Plan

### Phase 1: Foundation (Low Risk)

1. ✅ Create `src/constants.rs` module
2. ✅ Create `src/utils.rs` with percentage calculations
3. ✅ Add type aliases to `src/types.rs`
4. ✅ Update imports across codebase

### Phase 2: Quality Module Refactor (Medium Risk)

5. ✅ Create helper function `add_quality_issue()`
6. ✅ Refactor all 9 quality check functions
7. ✅ Remove unnecessary `.clone()` calls
8. ✅ Run full test suite (171 tests must pass)

### Phase 3: Performance Optimizations (Low Risk)

9. ✅ Replace hardcoded strings with constants
10. ✅ Use lazy static for scale metadata
11. ✅ Inline small functions where beneficial
12. ✅ Benchmark before/after

### Phase 4: Readability Improvements (Low Risk)

13. ✅ Fix variable names
14. ✅ Simplify complex boolean expressions
15. ✅ Add inline documentation for unclear logic
16. ✅ Code review

## Expected Benefits

### Performance

- **Memory:** Reduce allocations by ~30-40% (fewer clones)
- **CPU:** Faster string operations with constants
- **Compile time:** Slightly faster with better code organization

### Maintainability

- **DRY:** 90% duplication removed in quality checks
- **Constants:** No more magic strings/numbers
- **Type safety:** Clear type aliases prevent bugs

### Readability

- **Clarity:** Better variable names
- **Structure:** Smaller, focused functions
- **Documentation:** Clear intent with constants

## Risk Assessment

### Low Risk (Safe to implement immediately)

- Constants module
- Utility functions
- Type aliases
- Variable renaming

### Medium Risk (Requires testing)

- Quality check refactor
- Lazy static implementation
- Boolean logic simplification

### High Risk (Deferred to separate PR)

- Breaking up large functions (HTML generation)
- API changes
- Config structure changes

## Testing Strategy

After each phase:

1. ✅ Run `cargo test` (all 171 tests must pass)
2. ✅ Run `cargo clippy` (no new warnings)
3. ✅ Run `cargo build --release` (verify compilation)
4. ✅ Manual smoke test with example data
5. ✅ Check performance with benchmark suite

## Timeline

- Phase 1: 1 hour (foundation)
- Phase 2: 2 hours (quality refactor)
- Phase 3: 1 hour (optimizations)
- Phase 4: 1 hour (readability)
- Testing: 1 hour throughout
- **Total: 6 hours**

## Metrics

### Before Refactoring

- `.clone()` calls: 50+
- `.to_string()` calls: 90+
- Duplicated code: ~300 lines
- Magic strings: 25+
- Magic numbers: 15+

### After Refactoring (Target)

- `.clone()` calls: <10 (only where necessary)
- `.to_string()` calls: <30 (only for dynamic data)
- Duplicated code: <50 lines
- Magic strings: 0
- Magic numbers: 0

## Conclusion

This refactoring plan addresses 10 major code quality issues without changing behavior. All improvements are backward compatible and maintain the existing API. The changes will make the codebase more maintainable, performant, and easier to understand for future contributors.

**Recommendation:** Implement Phase 1-3, defer Phase 4 readability improvements to a follow-up PR to keep changes focused and reviewable.
