# Prism v0.8.5 - Critical Statistical Correctness Release

**Release Date**: January 6, 2026  
**Type**: Critical Bug Fix + Production Refactoring  
**Status**: Production-Ready ✅

---

## 🔴 CRITICAL: Statistical Correctness Fix

### The Issue

A **critical statistical error** was discovered in the longitudinal RCI (Reliable Change Index) analysis module. The `calculate_sd()` function was using **population variance** (÷n) instead of **sample variance** (÷n-1), violating standard statistical practice for sample-based research.

### Impact

**Severity**: 🔴 **CRITICAL** - Affects validity of all RCI calculations

- **Magnitude**: 5-12% underestimation of standard deviation for typical sample sizes

  - n=5: 11.8% too low
  - n=10: 5.4% too low
  - n=20: 2.6% too low
  - n=50: 1.0% too low

- **Effect**: Smaller SD → Inflated RCI values → Higher false positive rate for "reliable change" detection
- **RCI Formula**: `RCI = (X₂ - X₁) / (SD × √(2(1-r)))`
  - Underestimated SD causes smaller denominator → larger RCI → more false positives

### The Fix

```rust
// BEFORE (INCORRECT - Population Variance)
let variance = values.iter()
    .map(|&x| (x - mean).powi(2))
    .sum::<f64>() / values.len() as f64;  // ❌ Wrong!

// AFTER (CORRECT - Sample Variance with Bessel's Correction)
let variance = values.iter()
    .map(|&x| {
        let diff = x - mean;
        diff * diff
    })
    .sum::<f64>() / (n - 1) as f64;  // ✅ Correct!
```

### Mathematical Justification

**Bessel's Correction**: When calculating variance from a sample (not entire population), dividing by `n-1` provides an **unbiased estimator** of the population variance. This is:

- Standard practice in psychological research (Jacobson & Truax, 1991)
- Required for valid inference from sample to population
- Implemented correctly in `stats.rs` and `quality.rs` - NOW consistent across entire codebase

### Validation

✅ **3 New Regression Tests Added**:

1. `test_calculate_sd_uses_sample_variance` - Validates correct formula with known values
2. `test_calculate_sd_consistency_with_stats_module` - Cross-validates with stats.rs
3. Extended edge case coverage (n<2 scenarios)

✅ **Cross-Module Consistency**:

- `stats.rs`: ✅ Already correct (uses n-1)
- `quality.rs`: ✅ Already correct (uses n-1)
- `longitudinal.rs`: ✅ NOW CORRECT (fixed to use n-1)

✅ **All Tests Passing**: 18/18 library unit tests

---

## 📊 Comprehensive Statistical Validation

### New Documentation

**STATISTICAL_CORRECTNESS_REVIEW.md** (2500+ lines)

- Complete mathematical validation of all statistical functions
- Formula verification against peer-reviewed literature:
  - Cohen, J. (1988). _Statistical Power Analysis for Behavioral Sciences_
  - Jacobson & Truax (1991). Clinical significance statistical approach
- Computational efficiency analysis
- Impact analysis with quantified error magnitudes
- Complete references for reproducibility

### Modules Validated

#### ✅ stats.rs - ALL CORRECT

- **Variance/Standard Deviation**: Correctly uses sample variance (n-1)
- **Cronbach's Alpha**: Matches Cronbach (1951) formula exactly
- **Implementation**: Efficient inline calculation, no unnecessary allocations

#### ✅ power.rs - ALL CORRECT

- **Independent t-test**: Cohen's d = (μ₁ - μ₂) / σpooled ✅
- **Paired t-test**: Cohen's d = μdiff / σdiff ✅
- **Correlation**: Uses Fisher's Z transformation ✅
- **Sample Size**: Exact formulas from Cohen (1988) ✅
- **Observed Power**: Proper post-hoc calculation ✅

#### ✅ quality.rs - ALL CORRECT

- **Pattern Detection**: Optimal algorithms for straightlining, alternating, outliers
- **Variance Calculation**: Correctly uses sample variance (n-1)
- **Careless Response Detection**: Methodologically sound
- **Efficiency**: Uses efficient `windows()` iterator, O(n) complexity

#### ✅ processor.rs - ALL CORRECT

- **Reverse Scoring**: `(max + min) - value` formula is correct
- **Missing Data**: Proper handling with `Option<f64>`
- **Scale Calculations**: Mean-based scoring appropriate for Likert scales

#### ✅ longitudinal.rs - NOW CORRECT

- **RCI Calculation**: Formula matches Jacobson & Truax (1991)
- **Variance**: NOW USES CORRECT SAMPLE VARIANCE (n-1) ✅
- **Standard Error**: SE_diff = SD₁ × √(2(1 - reliability))
- **Clinical Significance**: Proper z-score based thresholds

---

## 🛠️ Production Refactoring (v0.8.5)

### Code Quality Improvements

**Eliminated Duplicate Constants**:

- Consolidated `DEFAULT_RELIABILITY` across modules
- Created single source of truth in `constants.rs`
- Prevents drift and inconsistencies

**Improved Error Handling**:

- Better error messages in CLI commands
- Consistent error propagation patterns
- More informative error context

**Test Infrastructure**:

- Updated to modern `assert_cmd` patterns
- Added `CommandCargoExt` trait imports
- Resolved deprecation warnings in test suite

---

## 📈 Quality Metrics

### Test Coverage

```
✅ 173/173 Total Tests Passing (100%)
✅ 18/18 Library Unit Tests
✅ 155/155 Integration Tests
✅ Zero Clippy Warnings (main codebase)
```

### Performance

- No performance regressions
- All algorithms remain O(n) or O(n log n)
- Efficient implementations validated

### Code Quality

- **~15,000 lines** of Rust code
- **Zero unsafe blocks**
- **Comprehensive documentation** with mathematical references
- **Production-grade error handling**

---

## 🎯 Why This Release Matters

### For Researchers

1. **Research Validity**: RCI calculations now produce statistically correct results
2. **False Positive Reduction**: Proper variance estimation reduces spurious findings
3. **Methodological Rigor**: All formulas validated against peer-reviewed literature
4. **Reproducibility**: Complete documentation of all mathematical assumptions

### For Statistical Reviewers

1. **Transparent Formulas**: All calculations documented with references
2. **Bessel's Correction**: Proper unbiased estimation throughout
3. **Literature Support**: Cohen (1988), Jacobson & Truax (1991) compliance
4. **Computational Correctness**: Verified against known test cases

### For Production Use

1. **Stability**: Comprehensive test coverage prevents regressions
2. **Maintainability**: Code consolidated, duplicates eliminated
3. **Reliability**: Critical bug fixed with regression tests
4. **Trust**: Complete statistical validation documented

---

## 📚 References

### Statistical Literature

1. **Cronbach, L. J. (1951)**. Coefficient alpha and the internal structure of tests. _Psychometrika_, 16(3), 297-334.

2. **Cohen, J. (1988)**. _Statistical Power Analysis for the Behavioral Sciences_ (2nd ed.). Lawrence Erlbaum Associates.

3. **Jacobson, N. S., & Truax, P. (1991)**. Clinical significance: A statistical approach to defining meaningful change in psychotherapy research. _Journal of Consulting and Clinical Psychology_, 59(1), 12-19.

### Documentation

- **STATISTICAL_CORRECTNESS_REVIEW.md**: Complete mathematical validation (2500+ lines)
- **PRODUCTION_REFACTORING_v0.8.5.md**: Code quality improvements
- **BUG_AUDIT_REPORT_v0.8.4.md**: Previous bug fixes (14 critical bugs)
- **PERFORMANCE_OPTIMIZATION_REPORT.md**: 15-40% performance improvements

---

## ⬆️ Upgrade Guide

### For Existing Users

**IMPORTANT**: If you have used longitudinal RCI analysis in previous versions (< 0.8.5):

1. **Re-run Your Analyses**: RCI values will be slightly lower (more conservative) with correct variance
2. **Review Flagged Cases**: Some previously flagged "reliable change" cases may no longer meet threshold
3. **Update Citations**: Reference the corrected methodology in your papers
4. **Benefit**: More valid, conservative, and defensible results

### Breaking Changes

**None** - This is a bug fix that makes results _more correct_, not a breaking API change.

### What You Get

```bash
# Update to latest version
git pull origin main

# All your existing configs work
cargo build --release

# Run with confidence
prism process -i data.csv -c config.toml -o output.csv
```

---

## 🚀 Installation

### From Source

```bash
git clone https://github.com/artlostintime/prism.git
cd prism
git checkout v0.8.5
cargo build --release
./target/release/prism --version  # Should show: prism 0.8.5
```

### Verify Correctness

```bash
# Run the full test suite
cargo test --all

# Should see:
# ✅ test longitudinal::tests::test_calculate_sd_uses_sample_variance ... ok
# ✅ test longitudinal::tests::test_calculate_sd_consistency_with_stats_module ... ok
# ✅ 173 tests passed
```

---

## 🙏 Acknowledgments

This critical bug fix demonstrates the importance of:

- **Peer review** in statistical software
- **Comprehensive testing** with known values
- **Mathematical validation** against literature
- **Transparent documentation** of assumptions

Special thanks to the psychology research community for establishing clear statistical standards that make validation possible.

---

## 📞 Support

- **Issues**: https://github.com/artlostintime/prism/issues
- **Documentation**: https://github.com/artlostintime/prism/tree/main/docs
- **Statistical Questions**: See STATISTICAL_CORRECTNESS_REVIEW.md

---

## 🔜 Next Steps

v0.9.0 will focus on:

- Additional validated scale libraries
- GUI enhancements for longitudinal analysis
- Extended power analysis options
- More comprehensive reporting features

---

**Bottom Line**: This release ensures Prism produces statistically valid, defensible results for psychological research. If you're using RCI analysis, **update immediately**.

---

_Released with ❤️ for valid, reproducible psychological research_
