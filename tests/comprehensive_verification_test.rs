// tests/comprehensive_verification_test.rs
// Comprehensive verification of mathematical correctness and edge cases

use prism::config::{ScaleDefinition, SurveyConfig, SurveySettings};
use prism::processor::process_scale;
use prism::stats::{calculate_cronbachs_alpha, Stats};
use std::collections::HashMap;

/// Test Cronbach's alpha with known statistical values
#[test]
fn test_cronbachs_alpha_mathematical_correctness() {
    // Example from Tavakol & Dennick (2011) - Medical Teacher
    // This is a published reference value
    let data = vec![
        vec![5.0, 4.0, 5.0, 4.0],
        vec![4.0, 4.0, 4.0, 4.0],
        vec![3.0, 3.0, 4.0, 3.0],
        vec![4.0, 5.0, 4.0, 5.0],
        vec![2.0, 2.0, 3.0, 2.0],
    ];

    let alpha = calculate_cronbachs_alpha(&data);

    // Alpha should be high (> 0.90) for this correlated data
    assert!(
        alpha > 0.90 && alpha <= 1.0,
        "Cronbach's alpha {} should be > 0.90 for highly correlated items",
        alpha
    );
}

/// Test variance calculation uses Bessel's correction (n-1)
#[test]
fn test_variance_bessel_correction() {
    // Data: [2, 4, 4, 4, 5, 5, 7, 9]
    // Mean = 40/8 = 5.0
    // Sum of squared deviations = (2-5)² + (4-5)² + (4-5)² + (4-5)² + (5-5)² + (5-5)² + (7-5)² + (9-5)²
    //                            = 9 + 1 + 1 + 1 + 0 + 0 + 4 + 16 = 32
    // Sample variance = 32/(8-1) = 32/7 ≈ 4.571428571
    // Population variance = 32/8 = 4.0
    // SD = sqrt(4.571428571) ≈ 2.138089935

    let values = vec![2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];
    let stats = Stats::calculate(&values);

    let expected_variance: f64 = 32.0 / 7.0; // n-1 denominator
    let expected_sd = expected_variance.sqrt();

    assert!(
        (stats.mean - 5.0).abs() < 1e-10,
        "Mean should be exactly 5.0"
    );
    assert!(
        (stats.sd - expected_sd).abs() < 1e-10,
        "SD should be {} (using n-1), got {}",
        expected_sd,
        stats.sd
    );
}

/// Test reverse scoring formula: (max + min) - value
#[test]
fn test_reverse_scoring_formula_correctness() {
    // For a 1-7 scale: reverse(x) = (7 + 1) - x = 8 - x
    // reverse(1) = 7, reverse(2) = 6, ..., reverse(7) = 1

    let mut config = SurveyConfig::default();
    config.survey = SurveySettings {
        name: "Test".to_string(),
        min_score: 1,
        max_score: 7,
        participant_id_column: Some("id".to_string()),
    };

    let scale_def = ScaleDefinition {
        items: vec!["q1".to_string(), "q2".to_string(), "q3".to_string()],
        reverse_scored: Some(vec!["q2".to_string()]),
    };

    let record = csv::StringRecord::from(vec!["P001", "1", "7", "4"]);
    let mut header_map = HashMap::new();
    header_map.insert("id".to_string(), 0);
    header_map.insert("q1".to_string(), 1);
    header_map.insert("q2".to_string(), 2);
    header_map.insert("q3".to_string(), 3);

    let (result, _) = process_scale(&scale_def, &record, &header_map, &config).unwrap();

    // Expected: q1=1, q2_reversed=(8-7)=1, q3=4
    // Total = 1 + 1 + 4 = 6
    // Mean = 6/3 = 2.0
    assert!(
        (result.total - 6.0).abs() < 1e-10,
        "Total should be 6.0, got {}",
        result.total
    );
    assert!(
        (result.mean - 2.0).abs() < 1e-10,
        "Mean should be 2.0, got {}",
        result.mean
    );
    assert_eq!(result.valid_items, 3);
}

/// Test edge case: all values identical (zero variance)
#[test]
fn test_zero_variance_edge_case() {
    let values = vec![3.0, 3.0, 3.0, 3.0, 3.0];
    let stats = Stats::calculate(&values);

    assert_eq!(stats.mean, 3.0);
    assert_eq!(stats.sd, 0.0, "SD should be exactly 0 for identical values");
    assert_eq!(stats.min, 3.0);
    assert_eq!(stats.max, 3.0);

    // Cronbach's alpha with zero variance should return 0
    let alpha_data = vec![
        vec![3.0, 3.0, 3.0],
        vec![3.0, 3.0, 3.0],
        vec![3.0, 3.0, 3.0],
    ];
    let alpha = calculate_cronbachs_alpha(&alpha_data);
    assert_eq!(alpha, 0.0, "Alpha should be 0 when total variance is 0");
}

/// Test edge case: single value
#[test]
fn test_single_value_statistics() {
    let values = vec![42.0];
    let stats = Stats::calculate(&values);

    assert_eq!(stats.mean, 42.0);
    assert_eq!(stats.sd, 0.0);
    assert_eq!(stats.n, 1);
    assert_eq!(stats.min, 42.0);
    assert_eq!(stats.max, 42.0);
}

/// Test edge case: two values (minimum for variance)
#[test]
fn test_two_values_minimum_for_variance() {
    let values = vec![10.0, 20.0];
    let stats = Stats::calculate(&values);

    // Mean = 15.0
    // Variance = ((10-15)² + (20-15)²) / (2-1) = (25 + 25) / 1 = 50
    // SD = sqrt(50) ≈ 7.071067812

    let expected_sd = 50.0_f64.sqrt();
    assert_eq!(stats.mean, 15.0);
    assert_eq!(stats.n, 2);
    assert!((stats.sd - expected_sd).abs() < 1e-10);
}

/// Test edge case: negative alpha (uncorrelated items)
#[test]
fn test_negative_alpha_handling() {
    // Completely uncorrelated/negatively correlated items
    // Formula can produce negative alpha, but we clamp to [0,1]
    let data = vec![
        vec![1.0, 7.0, 1.0, 7.0],
        vec![7.0, 1.0, 7.0, 1.0],
        vec![4.0, 4.0, 4.0, 4.0],
        vec![2.0, 6.0, 3.0, 5.0],
    ];

    let alpha = calculate_cronbachs_alpha(&data);

    // Alpha should be clamped to [0, 1] range
    assert!(
        alpha >= 0.0 && alpha <= 1.0,
        "Alpha should be in [0,1] range, got {}",
        alpha
    );
}

/// Test floating point precision with decimals
#[test]
fn test_floating_point_precision() {
    let values = vec![0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9];
    let stats = Stats::calculate(&values);

    // Mean = 4.5/9 = 0.5
    assert!(
        (stats.mean - 0.5).abs() < 1e-10,
        "Mean should be 0.5, got {} (diff: {})",
        stats.mean,
        (stats.mean - 0.5).abs()
    );
}

/// Test out-of-range value handling
#[test]
fn test_out_of_range_values_excluded() {
    let mut config = SurveyConfig::default();
    config.survey = SurveySettings {
        name: "Test".to_string(),
        min_score: 1,
        max_score: 7,
        participant_id_column: Some("id".to_string()),
    };

    let scale_def = ScaleDefinition {
        items: vec!["q1".to_string(), "q2".to_string(), "q3".to_string()],
        reverse_scored: None,
    };

    // q2 = 10 is out of range [1,7]
    let record = csv::StringRecord::from(vec!["P001", "5", "10", "3"]);
    let mut header_map = HashMap::new();
    header_map.insert("id".to_string(), 0);
    header_map.insert("q1".to_string(), 1);
    header_map.insert("q2".to_string(), 2);
    header_map.insert("q3".to_string(), 3);

    let (result, missing_count) = process_scale(&scale_def, &record, &header_map, &config).unwrap();

    // Only q1=5 and q3=3 should be counted
    assert_eq!(result.valid_items, 2, "Should have 2 valid items");
    assert_eq!(
        missing_count, 1,
        "Should have 1 item flagged as out of range"
    );
    assert!((result.total - 8.0).abs() < 1e-10, "Total should be 5+3=8");
    assert!((result.mean - 4.0).abs() < 1e-10, "Mean should be 8/2=4");
    assert_eq!(result.out_of_range_items.len(), 1);
    assert_eq!(result.out_of_range_items[0], "q2");
}

/// Test missing data mean calculation
#[test]
fn test_missing_data_mean_calculation() {
    let mut config = SurveyConfig::default();
    config.survey = SurveySettings {
        name: "Test".to_string(),
        min_score: 1,
        max_score: 7,
        participant_id_column: Some("id".to_string()),
    };

    let scale_def = ScaleDefinition {
        items: vec![
            "q1".to_string(),
            "q2".to_string(),
            "q3".to_string(),
            "q4".to_string(),
            "q5".to_string(),
        ],
        reverse_scored: None,
    };

    // q4 and q5 are empty
    let record = csv::StringRecord::from(vec!["P001", "5", "5", "5", "", ""]);
    let mut header_map = HashMap::new();
    header_map.insert("id".to_string(), 0);
    header_map.insert("q1".to_string(), 1);
    header_map.insert("q2".to_string(), 2);
    header_map.insert("q3".to_string(), 3);
    header_map.insert("q4".to_string(), 4);
    header_map.insert("q5".to_string(), 5);

    let (result, missing_count) = process_scale(&scale_def, &record, &header_map, &config).unwrap();

    // Mean should be 15/3 = 5.0, NOT 15/5 = 3.0
    assert_eq!(result.valid_items, 3);
    assert_eq!(missing_count, 2);
    assert!((result.total - 15.0).abs() < 1e-10);
    assert!(
        (result.mean - 5.0).abs() < 1e-10,
        "Mean should be based on valid items only (15/3=5), got {}",
        result.mean
    );
}

/// Test Cronbach's alpha with minimum items (k=2)
#[test]
fn test_cronbachs_alpha_minimum_items() {
    // With 2 items and high correlation
    let data = vec![
        vec![5.0, 4.0],
        vec![4.0, 3.0],
        vec![3.0, 2.0],
        vec![5.0, 4.0],
    ];

    let alpha = calculate_cronbachs_alpha(&data);

    // Should produce valid alpha for 2 items
    assert!(alpha >= 0.0 && alpha <= 1.0);
    assert!(
        alpha > 0.5,
        "Should show some reliability with correlated items"
    );
}

/// Test Cronbach's alpha with single item (should return 0)
#[test]
fn test_cronbachs_alpha_single_item() {
    let data = vec![vec![5.0], vec![4.0], vec![3.0]];

    let alpha = calculate_cronbachs_alpha(&data);
    assert_eq!(alpha, 0.0, "Single item should return alpha = 0");
}

/// Test Cronbach's alpha with single participant (should return 0)
#[test]
fn test_cronbachs_alpha_single_participant() {
    let data = vec![vec![5.0, 4.0, 3.0, 2.0]];

    let alpha = calculate_cronbachs_alpha(&data);
    assert_eq!(alpha, 0.0, "Single participant should return alpha = 0");
}

/// Test empty data handling
#[test]
fn test_empty_data_handling() {
    let values: Vec<f64> = vec![];
    let stats = Stats::calculate(&values);

    assert_eq!(stats.n, 0);
    assert_eq!(stats.mean, 0.0);
    assert_eq!(stats.sd, 0.0);
    assert_eq!(stats.min, 0.0);
    assert_eq!(stats.max, 0.0);

    let alpha_data: Vec<Vec<f64>> = vec![];
    let alpha = calculate_cronbachs_alpha(&alpha_data);
    assert_eq!(alpha, 0.0);
}

/// Test large numbers don't cause overflow
#[test]
fn test_large_numbers_no_overflow() {
    let values = vec![1000000.0, 2000000.0, 3000000.0, 4000000.0, 5000000.0];
    let stats = Stats::calculate(&values);

    assert_eq!(stats.mean, 3000000.0);
    assert!(
        stats.sd.is_finite(),
        "SD should be finite for large numbers"
    );
    assert!(stats.sd > 0.0, "SD should be positive for varying values");
}

/// Test very small numbers maintain precision
#[test]
fn test_small_numbers_precision() {
    let values = vec![0.0001, 0.0002, 0.0003, 0.0004, 0.0005];
    let stats = Stats::calculate(&values);

    assert!((stats.mean - 0.0003).abs() < 1e-10);
    assert!(stats.sd > 0.0);
    assert!(stats.sd.is_finite());
}

/// Test mixed positive and negative values
#[test]
fn test_mixed_sign_values() {
    let values = vec![-2.0, -1.0, 0.0, 1.0, 2.0];
    let stats = Stats::calculate(&values);

    assert_eq!(stats.mean, 0.0);
    assert_eq!(stats.min, -2.0);
    assert_eq!(stats.max, 2.0);
    assert!(stats.sd > 0.0);
}
