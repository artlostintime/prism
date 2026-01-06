// tests/bug_hunt_test.rs
// Additional edge case testing to find potential bugs

use assert_cmd::cargo;
use std::fs;

#[test]
fn test_division_by_zero_scenario() {
    // Test with all missing data - could cause division by zero
    let test_csv = "tests/fixtures/bug_all_missing.csv";
    let test_config = "tests/fixtures/bug_all_missing_config.toml";
    let output = "tests/output/bug_all_missing.csv";

    fs::write(
        test_csv,
        "id,q1,q2,q3\n\
         P001,,,\n\
         P002,,,\n",
    )
    .unwrap();

    fs::write(
        test_config,
        r#"[survey]
name = "Division by Zero Test"
min_score = 1
max_score = 7

[quality]
max_missing_percent = 1.0
flag_straightlining = true

[scales.test]
items = ["q1", "q2", "q3"]
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
    // Should output NA for all-missing data, not crash or divide by zero
    assert!(content.contains(",NA,NA,"));
}

#[test]
fn test_negative_cronbach_alpha() {
    // Test case where items are negatively correlated
    // This should produce negative alpha but be clamped to 0
    let test_csv = "tests/fixtures/bug_negative_alpha.csv";
    let test_config = "tests/fixtures/bug_negative_alpha_config.toml";
    let output = "tests/output/bug_negative_alpha.csv";

    // Create data where items are negatively correlated
    fs::write(
        test_csv,
        "id,q1,q2,q3,q4\n\
         P001,7,1,7,1\n\
         P002,1,7,1,7\n\
         P003,7,1,7,1\n\
         P004,1,7,1,7\n\
         P005,7,1,7,1\n",
    )
    .unwrap();

    fs::write(
        test_config,
        r#"[survey]
name = "Negative Alpha Test"
min_score = 1
max_score = 7

[scales.test]
items = ["q1", "q2", "q3", "q4"]
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
        .arg(output)
        .arg("--stats-output")
        .arg("tests/output/bug_negative_alpha_stats.txt");

    cmd.assert().success();

    let stats = fs::read_to_string("tests/output/bug_negative_alpha_stats.txt").unwrap();
    // Alpha should be clamped to 0.0, not negative
    assert!(stats.contains("Cronbach's Alpha"));
    // Should not show negative alpha
    assert!(!stats.contains("α)  = -"));
}

#[test]
fn test_inf_nan_handling() {
    // Test with values that could cause Inf or NaN
    let test_csv = "tests/fixtures/bug_extreme_calc.csv";
    let test_config = "tests/fixtures/bug_extreme_calc_config.toml";
    let output = "tests/output/bug_extreme_calc.csv";

    // Single unique value (zero variance)
    fs::write(
        test_csv,
        "id,q1,q2,q3,q4,q5\n\
         P001,5,5,5,5,5\n\
         P002,5,5,5,5,5\n\
         P003,5,5,5,5,5\n",
    )
    .unwrap();

    fs::write(
        test_config,
        r#"[survey]
name = "Zero Variance Test"
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
        .arg(output)
        .arg("--stats-output")
        .arg("tests/output/bug_extreme_calc_stats.txt");

    cmd.assert().success();

    let content = fs::read_to_string(output).unwrap();
    // Should not contain NaN or Inf
    assert!(!content.contains("NaN"));
    assert!(!content.contains("Inf"));
    assert!(!content.contains("inf"));

    let stats = fs::read_to_string("tests/output/bug_extreme_calc_stats.txt").unwrap();
    assert!(!stats.contains("NaN"));
    assert!(!stats.contains("Inf"));
}

#[test]
fn test_reverse_scoring_boundary() {
    // Test reverse scoring at boundaries
    let test_csv = "tests/fixtures/bug_reverse_boundary.csv";
    let test_config = "tests/fixtures/bug_reverse_boundary_config.toml";
    let output = "tests/output/bug_reverse_boundary.csv";

    fs::write(
        test_csv,
        "id,q1,q2,q3\n\
         P001,1,7,4\n\
         P002,7,1,4\n",
    )
    .unwrap();

    fs::write(
        test_config,
        r#"[survey]
name = "Reverse Boundary Test"
min_score = 1
max_score = 7

[scales.test]
items = ["q1", "q2", "q3"]
reverse_scored = ["q2"]
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

    // P001: q1=1, q2_reversed=(7+1)-7=1, q3=4 => total=6, mean=2.0
    assert!(lines[1].contains("6.00"));
    assert!(lines[1].contains("2.00"));

    // P002: q1=7, q2_reversed=(7+1)-1=7, q3=4 => total=18, mean=6.0
    assert!(lines[2].contains("18.00"));
    assert!(lines[2].contains("6.00"));
}

#[test]
fn test_float_precision_accumulation() {
    // Test with many items to see if floating point errors accumulate
    let test_csv = "tests/fixtures/bug_float_precision.csv";
    let test_config = "tests/fixtures/bug_float_precision_config.toml";
    let output = "tests/output/bug_float_precision.csv";

    let mut csv_content = String::from("id");
    for i in 1..=100 {
        csv_content.push_str(&format!(",q{}", i));
    }
    csv_content.push_str("\nP001");
    for _ in 1..=100 {
        csv_content.push_str(",3.33");
    }
    csv_content.push('\n');

    fs::write(test_csv, csv_content).unwrap();

    let mut config_content = String::from(
        "[survey]\nname = \"Float Precision Test\"\nmin_score = 1\nmax_score = 5\n\n[scales.test]\nitems = ["
    );
    for i in 1..=100 {
        if i > 1 {
            config_content.push_str(", ");
        }
        config_content.push_str(&format!("\"q{}\"", i));
    }
    config_content.push_str("]\n");

    fs::write(test_config, config_content).unwrap();

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
    // Total should be 333.00 (100 * 3.33)
    // Mean should be 3.33
    assert!(content.contains("333.00") || content.contains("333.0"));
    assert!(content.contains("3.33"));
}

#[test]
fn test_empty_scale_definition() {
    // Test with scale that has no items (shouldn't happen but might)
    let test_csv = "tests/fixtures/bug_empty_scale.csv";
    let test_config = "tests/fixtures/bug_empty_scale_config.toml";
    let output = "tests/output/bug_empty_scale.csv";

    fs::write(
        test_csv,
        "id,q1,q2\n\
         P001,5,4\n",
    )
    .unwrap();

    fs::write(
        test_config,
        r#"[survey]
name = "Empty Scale Test"
min_score = 1
max_score = 7

[scales.test]
items = []
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

    // Should fail with configuration error for empty scale
    cmd.assert().failure();
}

#[test]
fn test_whitespace_in_csv_values() {
    // Test handling of whitespace in CSV values
    let test_csv = "tests/fixtures/bug_whitespace.csv";
    let test_config = "tests/fixtures/bug_whitespace_config.toml";
    let output = "tests/output/bug_whitespace.csv";

    fs::write(
        test_csv,
        "id,q1,q2,q3\n\
         P001, 5 ,4, 5\n\
         P002,  3,  3  ,4\n",
    )
    .unwrap();

    fs::write(
        test_config,
        r#"[survey]
name = "Whitespace Test"
min_score = 1
max_score = 7

[scales.test]
items = ["q1", "q2", "q3"]
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
    // Should parse " 5 " as 5, etc.
    assert!(content.contains("P001"));
    assert!(content.contains("P002"));
}

// Helper to create directories
#[ctor::ctor]
fn setup() {
    fs::create_dir_all("tests/fixtures").ok();
    fs::create_dir_all("tests/output").ok();
}
