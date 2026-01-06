// tests/mathematical_validation_test.rs
// Comprehensive mathematical correctness tests against known reference values

use assert_cmd::cargo;
use prism::stats::{calculate_cronbachs_alpha, Stats};
use std::fs;

#[test]
fn test_reverse_scoring_formula() {
    // Test reverse scoring formula: (max + min) - value
    // For 1-7 scale: reverse(1) = 8-1 = 7, reverse(7) = 8-7 = 1

    let test_csv = "tests/fixtures/math_reverse.csv";
    let test_config = "tests/fixtures/math_reverse_config.toml";
    let output = "tests/output/math_reverse.csv";

    // Create CSV with known values
    fs::write(
        test_csv,
        "id,item1,item2,item3\n\
         P001,1,4,7\n\
         P002,2,5,6\n\
         P003,3,3,3\n",
    )
    .unwrap();

    // Config with item2 reverse scored
    fs::write(
        test_config,
        r#"[survey]
name = "Reverse Score Test"
min_score = 1
max_score = 7

[scales.test_scale]
items = ["item1", "item2", "item3"]
reverse_scored = ["item2"]
"#,
    )
    .unwrap();

    let mut cmd = cargo::cargo_bin_cmd!("prism");
    cmd.arg("process")
        .arg("-i")
        .arg(test_csv)
        .arg("-c")
        .arg(test_config)
        .arg("-o")
        .arg(output);

    cmd.assert().success();

    let content = fs::read_to_string(output).unwrap();
    let lines: Vec<&str> = content.lines().collect();

    // P001: item1=1, item2_reversed=8-4=4, item3=7 => total=12, mean=4.0
    assert!(lines[1].contains("12.00"), "P001 total should be 12.00");
    assert!(lines[1].contains("4.00"), "P001 mean should be 4.00");

    // P002: item1=2, item2_reversed=8-5=3, item3=6 => total=11, mean=3.67
    assert!(lines[2].contains("11.00"), "P002 total should be 11.00");
    assert!(lines[2].contains("3.67"), "P002 mean should be 3.67");

    // P003: item1=3, item2_reversed=8-3=5, item3=3 => total=11, mean=3.67
    assert!(lines[3].contains("11.00"), "P003 total should be 11.00");
    assert!(lines[3].contains("3.67"), "P003 mean should be 3.67");
}

#[test]
fn test_variance_uses_n_minus_1() {
    // Verify sample variance uses n-1 denominator (Bessel's correction)
    // Data: [2, 4, 4, 4, 5, 5, 7, 9]
    // Mean = 5.0
    // Sum of squared deviations = 9 + 1 + 1 + 1 + 0 + 0 + 4 + 16 = 32
    // Variance (sample) = 32 / (8-1) = 32/7 = 4.571428...
    // SD = sqrt(4.571428) = 2.138...

    let values = vec![2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];
    let stats = Stats::calculate(&values);

    assert_eq!(stats.mean, 5.0, "Mean should be 5.0");
    assert_eq!(stats.n, 8, "N should be 8");

    // Expected variance = 32/7 = 4.571428...
    let expected_variance: f64 = 32.0 / 7.0;
    let expected_sd = expected_variance.sqrt();

    // Allow small floating point tolerance
    let tolerance = 0.001;
    assert!(
        (stats.sd - expected_sd).abs() < tolerance,
        "SD should be {} but got {}",
        expected_sd,
        stats.sd
    );
}

#[test]
fn test_cronbachs_alpha_known_value() {
    // Test against a known Cronbach's alpha calculation
    // Using example from Tavakol & Dennick (2011)
    // 5 participants, 4 items
    let data = vec![
        vec![5.0, 4.0, 5.0, 4.0],
        vec![4.0, 4.0, 4.0, 4.0],
        vec![3.0, 3.0, 4.0, 3.0],
        vec![4.0, 5.0, 4.0, 5.0],
        vec![2.0, 2.0, 3.0, 2.0],
    ];

    let alpha = calculate_cronbachs_alpha(&data);

    // Expected alpha ≈ 0.93 (based on actual calculation)
    // Formula: α = (k/(k-1)) * (1 - Σvar_items/var_total)
    assert!(
        alpha > 0.90 && alpha < 0.96,
        "Cronbach's alpha should be around 0.93, got {}",
        alpha
    );
}

#[test]
fn test_cronbachs_alpha_perfect_reliability() {
    // All participants give identical pattern
    // Alpha should be very high (approaching 1.0)
    let data = vec![
        vec![5.0, 4.0, 3.0],
        vec![5.0, 4.0, 3.0],
        vec![5.0, 4.0, 3.0],
        vec![5.0, 4.0, 3.0],
    ];

    let alpha = calculate_cronbachs_alpha(&data);

    // With identical responses, variance is zero, alpha becomes 0 or undefined
    // This is mathematically correct - no variance means no covariance
    assert!(
        alpha == 0.0 || alpha.is_nan(),
        "Identical responses (zero variance) should give alpha = 0 or NaN, got {}",
        alpha
    );
}

#[test]
fn test_cronbachs_alpha_no_reliability() {
    // Random uncorrelated responses
    // Alpha should be low or negative
    let data = vec![
        vec![1.0, 7.0, 1.0, 7.0],
        vec![7.0, 1.0, 7.0, 1.0],
        vec![4.0, 4.0, 4.0, 4.0],
        vec![2.0, 6.0, 3.0, 5.0],
    ];

    let alpha = calculate_cronbachs_alpha(&data);

    // Low/negative alpha indicates no internal consistency
    assert!(
        alpha < 0.5,
        "Uncorrelated items should have low alpha, got {}",
        alpha
    );
}

#[test]
fn test_descriptive_statistics_against_reference() {
    // Values: [10, 15, 20, 25, 30]
    // Mean = 20
    // Variance = ((10-20)² + (15-20)² + (20-20)² + (25-20)² + (30-20)²) / (5-1)
    //          = (100 + 25 + 0 + 25 + 100) / 4
    //          = 250 / 4 = 62.5
    // SD = sqrt(62.5) = 7.9056...

    let values = vec![10.0, 15.0, 20.0, 25.0, 30.0];
    let stats = Stats::calculate(&values);

    assert_eq!(stats.mean, 20.0);
    assert_eq!(stats.min, 10.0);
    assert_eq!(stats.max, 30.0);
    assert_eq!(stats.n, 5);

    let expected_sd = (62.5_f64).sqrt();
    let tolerance = 0.001;
    assert!(
        (stats.sd - expected_sd).abs() < tolerance,
        "SD should be {} but got {}",
        expected_sd,
        stats.sd
    );
}

#[test]
fn test_floating_point_precision() {
    // Test with values that could expose floating point errors
    let values = vec![0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9];

    let stats = Stats::calculate(&values);

    // Mean should be 0.5
    let expected_mean = 0.5;
    assert!(
        (stats.mean - expected_mean).abs() < 0.0001,
        "Mean should be {} but got {}",
        expected_mean,
        stats.mean
    );
}

#[test]
fn test_scale_calculation_with_missing_data() {
    // Test mean calculation with missing items
    // If 2 out of 5 items are missing, mean should be based on valid items

    let test_csv = "tests/fixtures/math_missing.csv";
    let test_config = "tests/fixtures/math_missing_config.toml";
    let output = "tests/output/math_missing.csv";

    fs::write(
        test_csv,
        "id,q1,q2,q3,q4,q5\n\
         P001,5,5,5,,\n", // 3 items: total=15, mean=5
    )
    .unwrap();

    fs::write(
        test_config,
        r#"[survey]
name = "Missing Data Test"
min_score = 1
max_score = 7

[scales.test]
items = ["q1", "q2", "q3", "q4", "q5"]
"#,
    )
    .unwrap();

    let mut cmd = cargo::cargo_bin_cmd!("prism");
    cmd.arg("process")
        .arg("-i")
        .arg(test_csv)
        .arg("-c")
        .arg(test_config)
        .arg("-o")
        .arg(output);

    cmd.assert().success();

    let content = fs::read_to_string(output).unwrap();
    let lines: Vec<&str> = content.lines().collect();

    // Total should be 15 (sum of valid items)
    // Mean should be 5.0 (15/3, not 15/5)
    assert!(lines[1].contains("15.00"), "Total should be 15.00");
    assert!(lines[1].contains("5.00"), "Mean should be 5.00");
}

#[test]
fn test_zero_variance_handling() {
    // All values are the same - SD should be 0
    let values = vec![3.0, 3.0, 3.0, 3.0, 3.0];
    let stats = Stats::calculate(&values);

    assert_eq!(stats.mean, 3.0);
    assert_eq!(stats.sd, 0.0);
    assert_eq!(stats.min, 3.0);
    assert_eq!(stats.max, 3.0);
}

#[test]
fn test_single_value_statistics() {
    // With n=1, SD should be 0
    let values = vec![42.0];
    let stats = Stats::calculate(&values);

    assert_eq!(stats.mean, 42.0);
    assert_eq!(stats.sd, 0.0);
    assert_eq!(stats.n, 1);
}

#[test]
fn test_two_values_statistics() {
    // Minimum for SD calculation with Bessel's correction
    // Values: [10, 20]
    // Mean = 15
    // Variance = ((10-15)² + (20-15)²) / (2-1) = (25 + 25) / 1 = 50
    // SD = sqrt(50) = 7.071...

    let values = vec![10.0, 20.0];
    let stats = Stats::calculate(&values);

    assert_eq!(stats.mean, 15.0);
    assert_eq!(stats.n, 2);

    let expected_sd = 50.0_f64.sqrt();
    assert!((stats.sd - expected_sd).abs() < 0.001);
}

// Helper to create directories
#[ctor::ctor]
fn setup() {
    fs::create_dir_all("tests/fixtures").ok();
    fs::create_dir_all("tests/output").ok();
}
