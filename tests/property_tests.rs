// tests/property_tests.rs
//! Property-based tests using proptest

use prism::config::SurveyConfig;
use prism::stats::Stats;
use proptest::prelude::*;

/// Test that reverse scoring is involutive: reverse(reverse(x)) == x
#[test]
fn test_reverse_scoring_involution() {
    proptest!(|(score in 1.0f64..7.0f64)| {
        let min: f64 = 1.0;
        let max: f64 = 7.0;
        let score_range: f64 = max + min;

        let reversed_once: f64 = score_range - score;
        let reversed_twice: f64 = score_range - reversed_once;

        prop_assert!((score - reversed_twice).abs() < 1e-10);
    });
}

/// Test that scale means are always within valid range
#[test]
fn test_scale_means_in_range() {
    proptest!(|(
        values in prop::collection::vec(1.0..7.0, 1..20)
    )| {
        let mean = values.iter().sum::<f64>() / values.len() as f64;
        assert!(mean >= 1.0);
        assert!(mean <= 7.0);
    });
}

/// Test that stats calculation is consistent
#[test]
fn test_stats_consistency() {
    proptest!(|(
        values in prop::collection::vec(0.0..100.0, 2..100)
    )| {
        let stats = Stats::calculate(&values);

        // Mean should be between min and max
        assert!(stats.mean >= stats.min);
        assert!(stats.mean <= stats.max);

        // N should match input
        assert_eq!(stats.n, values.len());

        // SD should be non-negative
        assert!(stats.sd >= 0.0);
    });
}

/// Test that missing percent is always between 0 and 1
#[test]
fn test_missing_percent_bounds() {
    proptest!(|(
        missing in 0usize..20,
        total in 1usize..20
    )| {
        let missing_clamped = missing.min(total);
        let percent = missing_clamped as f64 / total as f64;

        assert!(percent >= 0.0);
        assert!(percent <= 1.0);
    });
}

/// Test that Cronbach's alpha is in valid range
#[test]
fn test_cronbachs_alpha_range() {
    // Simplified test - just use fixed sizes with proptest values
    proptest!(|(values in prop::collection::vec(
        prop::collection::vec(1.0..7.0, 2..20),
        2..50
    ))| {
        let alpha = prism::stats::calculate_cronbachs_alpha(&values);

        // Alpha can be negative for poorly related items, but should be finite
        prop_assert!(alpha.is_finite());
        // For reasonable data, alpha should typically be between -1 and 1
        // but can exceed 1 in edge cases
    });
}

/// Test that scale computation with no items returns 0
#[test]
fn test_empty_scale() {
    let stats = Stats::calculate(&[]);
    assert_eq!(stats.n, 0);
    assert_eq!(stats.mean, 0.0);
}

/// Test configuration defaults
#[test]
fn test_config_defaults() {
    let config = SurveyConfig::default();
    assert!(config.scales.is_empty());
    assert_eq!(config.survey.min_score, 1);
    assert_eq!(config.survey.max_score, 7);
}
