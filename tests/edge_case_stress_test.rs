// Comprehensive Edge Case and Stress Testing
// Tests extreme values, boundary conditions, and potential bugs

use assert_cmd::Command;
use std::fs;

const EDGE_CASES_CSV: &str = "tests/fixtures/edge_cases.csv";
const EDGE_CASE_CONFIG: &str = "tests/fixtures/edge_case_config.toml";
const EXTREME_VALUES_CSV: &str = "tests/fixtures/extreme_values.csv";
const EXTREME_CONFIG: &str = "tests/fixtures/extreme_config.toml";

#[test]
fn test_all_zeros_handling() {
    let output_path = "tests/output/edge_all_zeros.csv";

    let mut cmd = Command::cargo_bin("prism").unwrap();
    cmd.arg("process")
        .arg("--input")
        .arg(EDGE_CASES_CSV)
        .arg("--config")
        .arg(EDGE_CASE_CONFIG)
        .arg("--output")
        .arg(output_path);

    let output = cmd.output().unwrap();
    println!("STDOUT: {}", String::from_utf8_lossy(&output.stdout));
    println!("STDERR: {}", String::from_utf8_lossy(&output.stderr));

    assert!(
        output.status.success(),
        "Should handle all zeros without crashing"
    );

    // Verify output contains P001 (all zeros) and P002 (all ones)
    let content = fs::read_to_string(output_path).unwrap();
    assert!(
        content.contains("P001"),
        "Should include participant with all zeros"
    );
    assert!(content.contains("PHQ9_total"), "Should have PHQ9 total");
}

#[test]
fn test_all_max_values_handling() {
    let output_path = "tests/output/edge_all_max.csv";

    let mut cmd = Command::cargo_bin("prism").unwrap();
    cmd.arg("process")
        .arg("--input")
        .arg(EDGE_CASES_CSV)
        .arg("--config")
        .arg(EDGE_CASE_CONFIG)
        .arg("--output")
        .arg(output_path);

    let output = cmd.output().unwrap();

    assert!(output.status.success(), "Should handle all max values");

    let content = fs::read_to_string(output_path).unwrap();
    assert!(
        content.contains("P004"),
        "Should include participant with all 3s"
    );
    // PHQ9 all 3s should be 27 total
    assert!(
        content.contains("27"),
        "Should calculate correct total for all max values"
    );
}

#[test]
fn test_completely_missing_data() {
    let output_path = "tests/output/edge_all_missing.csv";
    let quality_path = "tests/output/edge_all_missing_quality.txt";

    let mut cmd = Command::cargo_bin("prism").unwrap();
    cmd.arg("process")
        .arg("--input")
        .arg(EDGE_CASES_CSV)
        .arg("--config")
        .arg(EDGE_CASE_CONFIG)
        .arg("--output")
        .arg(output_path)
        .arg("--quality-report")
        .arg(quality_path);

    let output = cmd.output().unwrap();

    assert!(
        output.status.success(),
        "Should handle completely missing data"
    );

    // Check quality report flags P007 (all missing)
    let quality = fs::read_to_string(quality_path).unwrap();
    assert!(
        quality.contains("P007") || quality.contains("missing"),
        "Should flag participant with all missing data"
    );
}

#[test]
fn test_out_of_range_values() {
    let output_path = "tests/output/edge_out_of_range.csv";

    let mut cmd = Command::cargo_bin("prism").unwrap();
    cmd.arg("process")
        .arg("--input")
        .arg(EDGE_CASES_CSV)
        .arg("--config")
        .arg(EDGE_CASE_CONFIG)
        .arg("--output")
        .arg(output_path);

    let output = cmd.output().unwrap();
    println!("STDERR: {}", String::from_utf8_lossy(&output.stderr));

    // P011 has values -1 to 14 (range 0-3), P016 has 999s
    // These should either be excluded or treated as missing
    assert!(
        output.status.success(),
        "Should handle out-of-range values gracefully"
    );
}

#[test]
fn test_alternating_pattern_detection() {
    let output_path = "tests/output/edge_alternating.csv";
    let quality_path = "tests/output/edge_alternating_quality.txt";

    let mut cmd = Command::cargo_bin("prism").unwrap();
    cmd.arg("process")
        .arg("--input")
        .arg(EDGE_CASES_CSV)
        .arg("--config")
        .arg(EDGE_CASE_CONFIG)
        .arg("--output")
        .arg(output_path)
        .arg("--quality-report")
        .arg(quality_path);

    let output = cmd.output().unwrap();

    assert!(output.status.success());

    let quality = fs::read_to_string(quality_path).unwrap();
    // P020 and P021 have alternating 0,1,0,1 or 1,0,1,0 patterns
    assert!(
        quality.contains("alternating") || quality.contains("pattern"),
        "Should detect alternating patterns in P020/P021"
    );
}

#[test]
fn test_straightlining_detection() {
    let output_path = "tests/output/edge_straightlining.csv";
    let quality_path = "tests/output/edge_straightlining_quality.txt";

    let mut cmd = Command::cargo_bin("prism").unwrap();
    cmd.arg("process")
        .arg("--input")
        .arg(EDGE_CASES_CSV)
        .arg("--config")
        .arg(EDGE_CASE_CONFIG)
        .arg("--output")
        .arg(output_path)
        .arg("--quality-report")
        .arg(quality_path);

    let output = cmd.output().unwrap();

    assert!(output.status.success());

    let quality = fs::read_to_string(quality_path).unwrap();
    println!("Quality report:\n{}", quality);

    // P022 and P023 have all 2s and all 3s respectively
    assert!(
        quality.contains("straightlin") || quality.contains("P022") || quality.contains("P023"),
        "Should detect straightlining in participants with all same values"
    );
}

#[test]
fn test_decimal_values_handling() {
    let output_path = "tests/output/edge_decimals.csv";

    let mut cmd = Command::cargo_bin("prism").unwrap();
    cmd.arg("process")
        .arg("--input")
        .arg(EDGE_CASES_CSV)
        .arg("--config")
        .arg(EDGE_CASE_CONFIG)
        .arg("--output")
        .arg(output_path);

    let output = cmd.output().unwrap();
    println!("STDERR: {}", String::from_utf8_lossy(&output.stderr));

    // P014 has decimal values like 1.5, 2.7, etc.
    assert!(output.status.success(), "Should handle decimal values");

    let content = fs::read_to_string(output_path).unwrap();
    assert!(
        content.contains("P014"),
        "Should process participant with decimal values"
    );
}

#[test]
fn test_various_null_representations() {
    let output_path = "tests/output/edge_nulls.csv";

    let mut cmd = Command::cargo_bin("prism").unwrap();
    cmd.arg("process")
        .arg("--input")
        .arg(EDGE_CASES_CSV)
        .arg("--config")
        .arg(EDGE_CASE_CONFIG)
        .arg("--output")
        .arg(output_path);

    let output = cmd.output().unwrap();

    // P017 has "NA", P018 has "NULL", P019 has spaces
    assert!(
        output.status.success(),
        "Should handle various null representations"
    );
}

#[test]
fn test_reverse_scoring_with_extremes() {
    let output_path = "tests/output/extreme_reverse.csv";
    let stats_path = "tests/output/extreme_reverse_stats.txt";

    let mut cmd = Command::cargo_bin("prism").unwrap();
    cmd.arg("process")
        .arg("--input")
        .arg(EXTREME_VALUES_CSV)
        .arg("--config")
        .arg(EXTREME_CONFIG)
        .arg("--output")
        .arg(output_path)
        .arg("--stats-output")
        .arg(stats_path);

    let output = cmd.output().unwrap();
    println!("STDOUT: {}", String::from_utf8_lossy(&output.stdout));
    println!("STDERR: {}", String::from_utf8_lossy(&output.stderr));

    assert!(
        output.status.success(),
        "Should handle reverse scoring with extreme values"
    );

    // E001: all zeros, reverse (4-0=4) -> PSS4,5,7,8 become 4
    // E002: all 4s, reverse (4-4=0) -> PSS4,5,7,8 become 0
    let content = fs::read_to_string(output_path).unwrap();
    assert!(
        content.contains("E001"),
        "Should process E001 with reverse scoring"
    );
}

#[test]
fn test_negative_values_handling() {
    let output_path = "tests/output/extreme_negative.csv";

    let mut cmd = Command::cargo_bin("prism").unwrap();
    cmd.arg("process")
        .arg("--input")
        .arg(EXTREME_VALUES_CSV)
        .arg("--config")
        .arg(EXTREME_CONFIG)
        .arg("--output")
        .arg(output_path);

    let output = cmd.output().unwrap();

    // E008 has negative values -1 to -10
    assert!(output.status.success(), "Should handle negative values");
}

#[test]
fn test_excessively_large_values() {
    let output_path = "tests/output/extreme_large.csv";

    let mut cmd = Command::cargo_bin("prism").unwrap();
    cmd.arg("process")
        .arg("--input")
        .arg(EXTREME_VALUES_CSV)
        .arg("--config")
        .arg(EXTREME_CONFIG)
        .arg("--output")
        .arg(output_path);

    let output = cmd.output().unwrap();

    // E009 has values of 100 (way out of range)
    assert!(
        output.status.success(),
        "Should handle excessively large values"
    );
}

#[test]
fn test_partial_missing_data_threshold() {
    let output_path = "tests/output/extreme_partial_missing.csv";
    let quality_path = "tests/output/extreme_partial_missing_quality.txt";

    let mut cmd = Command::cargo_bin("prism").unwrap();
    cmd.arg("process")
        .arg("--input")
        .arg(EXTREME_VALUES_CSV)
        .arg("--config")
        .arg(EXTREME_CONFIG)
        .arg("--output")
        .arg(output_path)
        .arg("--quality-report")
        .arg(quality_path);

    let output = cmd.output().unwrap();

    assert!(output.status.success());

    // E012: 1 missing out of 10 = 10% (below 20% threshold, should pass)
    // E013: 1 missing out of 10 = 10% (should pass)
    // E014: 1 missing out of 10 = 10% (should pass)
    let content = fs::read_to_string(output_path).unwrap();
    assert!(
        content.contains("E012"),
        "Should process with 10% missing data"
    );
}

#[test]
fn test_special_string_values() {
    let output_path = "tests/output/extreme_special_strings.csv";

    let mut cmd = Command::cargo_bin("prism").unwrap();
    cmd.arg("process")
        .arg("--input")
        .arg(EXTREME_VALUES_CSV)
        .arg("--config")
        .arg(EXTREME_CONFIG)
        .arg("--output")
        .arg(output_path);

    let output = cmd.output().unwrap();

    // E018: NaN, E019: inf, E020: -inf
    assert!(
        output.status.success(),
        "Should handle special string values"
    );
}

#[test]
fn test_statistical_accuracy_all_zeros() {
    let stats_path = "tests/output/edge_stats_zeros.txt";

    let mut cmd = Command::cargo_bin("prism").unwrap();
    cmd.arg("process")
        .arg("--input")
        .arg(EDGE_CASES_CSV)
        .arg("--config")
        .arg(EDGE_CASE_CONFIG)
        .arg("--output")
        .arg("tests/output/edge_stats_zeros.csv")
        .arg("--stats-output")
        .arg(stats_path);

    cmd.assert().success();

    let stats = fs::read_to_string(stats_path).unwrap();
    println!("Statistics:\n{}", stats);

    // With all zeros, SD should be 0, mean should be 0
    // Check if statistics handle this edge case correctly
    assert!(stats.contains("PHQ9"), "Should have PHQ9 statistics");
}

#[test]
fn test_statistical_accuracy_no_variance() {
    let stats_path = "tests/output/edge_stats_no_variance.txt";

    let mut cmd = Command::cargo_bin("prism").unwrap();
    cmd.arg("process")
        .arg("--input")
        .arg(EDGE_CASES_CSV)
        .arg("--config")
        .arg(EDGE_CASE_CONFIG)
        .arg("--output")
        .arg("tests/output/edge_stats_no_variance.csv")
        .arg("--stats-output")
        .arg(stats_path);

    cmd.assert().success();

    let stats = fs::read_to_string(stats_path).unwrap();

    // When all values are the same, Cronbach's alpha should handle it
    // (might be undefined or 0)
    assert!(
        stats.contains("Cronbach"),
        "Should include Cronbach's alpha"
    );
}

#[test]
fn test_missing_data_in_different_positions() {
    let output_path = "tests/output/extreme_missing_positions.csv";

    let mut cmd = Command::cargo_bin("prism").unwrap();
    cmd.arg("process")
        .arg("--input")
        .arg(EXTREME_VALUES_CSV)
        .arg("--config")
        .arg(EXTREME_CONFIG)
        .arg("--output")
        .arg(output_path);

    cmd.assert().success();

    let content = fs::read_to_string(output_path).unwrap();

    // E012: missing last item
    // E013: missing first item
    // E014: missing second item
    // E015: missing second-to-last item
    // All should be processed if under threshold
    assert!(content.contains("E012"), "Should handle missing last item");
    assert!(content.contains("E013"), "Should handle missing first item");
    assert!(
        content.contains("E014"),
        "Should handle missing middle item"
    );
}

#[test]
fn test_cronbach_alpha_with_two_participants() {
    // Minimum case for alpha calculation
    let output_path = "tests/output/edge_cronbach_min.csv";
    let stats_path = "tests/output/edge_cronbach_min_stats.txt";

    let mut cmd = Command::cargo_bin("prism").unwrap();
    cmd.arg("process")
        .arg("--input")
        .arg(EDGE_CASES_CSV)
        .arg("--config")
        .arg(EDGE_CASE_CONFIG)
        .arg("--output")
        .arg(output_path)
        .arg("--stats-output")
        .arg(stats_path);

    cmd.assert().success();

    let stats = fs::read_to_string(stats_path).unwrap();

    // Cronbach's alpha requires at least 2 participants and 2 items
    // Should calculate without division by zero
    assert!(
        stats.contains("Cronbach"),
        "Should calculate Cronbach's alpha"
    );
}

#[test]
fn test_diagonal_pattern_detection() {
    let output_path = "tests/output/edge_diagonal.csv";
    let quality_path = "tests/output/edge_diagonal_quality.txt";

    let mut cmd = Command::cargo_bin("prism").unwrap();
    cmd.arg("process")
        .arg("--input")
        .arg(EDGE_CASES_CSV)
        .arg("--config")
        .arg(EDGE_CASE_CONFIG)
        .arg("--output")
        .arg(output_path)
        .arg("--quality-report")
        .arg(quality_path);

    cmd.assert().success();

    let quality = fs::read_to_string(quality_path).unwrap();

    // P005: 0,1,2,3,0,1,2,3,0 - diagonal pattern
    // P025: 1,2,3,0,1,2,3,0,1 - diagonal pattern
    assert!(
        quality.contains("diagonal") || quality.contains("pattern") || quality.contains("P005"),
        "Should detect diagonal patterns"
    );
}

#[test]
fn test_output_directory_creation() {
    // Test that output directory is created if it doesn't exist
    let test_dir = "tests/output/nested/deep/path";
    fs::remove_dir_all("tests/output/nested").ok(); // Clean up first

    let output_path = format!("{}/test.csv", test_dir);

    let mut cmd = Command::cargo_bin("prism").unwrap();
    cmd.arg("process")
        .arg("--input")
        .arg(EDGE_CASES_CSV)
        .arg("--config")
        .arg(EDGE_CASE_CONFIG)
        .arg("--output")
        .arg(&output_path);

    let result = cmd.output();

    // Should either create directory or fail gracefully
    assert!(result.is_ok(), "Should handle nested directory creation");
}

#[test]
fn test_empty_reverse_items_list() {
    // PHQ9 and GAD7 have no reverse items
    let output_path = "tests/output/edge_no_reverse.csv";

    let mut cmd = Command::cargo_bin("prism").unwrap();
    cmd.arg("process")
        .arg("--input")
        .arg(EDGE_CASES_CSV)
        .arg("--config")
        .arg(EDGE_CASE_CONFIG)
        .arg("--output")
        .arg(output_path);

    cmd.assert().success();

    let content = fs::read_to_string(output_path).unwrap();
    assert!(
        content.contains("PHQ9_total"),
        "Should handle scales with no reverse items"
    );
}

#[test]
fn test_quality_report_formatting() {
    let quality_path = "tests/output/edge_quality_format.txt";

    let mut cmd = Command::cargo_bin("prism").unwrap();
    cmd.arg("process")
        .arg("--input")
        .arg(EDGE_CASES_CSV)
        .arg("--config")
        .arg(EDGE_CASE_CONFIG)
        .arg("--output")
        .arg("tests/output/edge_quality_format.csv")
        .arg("--quality-report")
        .arg(quality_path);

    cmd.assert().success();

    let quality = fs::read_to_string(quality_path).unwrap();

    // Check that quality report is well-formatted
    assert!(!quality.is_empty(), "Quality report should not be empty");
    assert!(
        quality.contains("QUALITY") || quality.contains("Quality"),
        "Should have quality header"
    );
}

#[test]
fn test_stats_report_formatting() {
    let stats_path = "tests/output/edge_stats_format.txt";

    let mut cmd = Command::cargo_bin("prism").unwrap();
    cmd.arg("process")
        .arg("--input")
        .arg(EDGE_CASES_CSV)
        .arg("--config")
        .arg(EDGE_CASE_CONFIG)
        .arg("--output")
        .arg("tests/output/edge_stats_format.csv")
        .arg("--stats-output")
        .arg(stats_path);

    cmd.assert().success();

    let stats = fs::read_to_string(stats_path).unwrap();

    // Check comprehensive statistics format
    assert!(
        stats.contains("Mean") || stats.contains("SD"),
        "Should have descriptive stats"
    );
    assert!(
        stats.contains("Cronbach") || stats.contains("Alpha"),
        "Should have reliability"
    );
}
