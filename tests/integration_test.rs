use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use std::path::Path;

#[test]
fn test_cli_help() {
    let mut cmd = Command::cargo_bin("prism").unwrap();
    cmd.arg("--help");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Prism transforms raw survey data"));
}

#[test]
fn test_cli_version() {
    let mut cmd = Command::cargo_bin("prism").unwrap();
    cmd.arg("--version");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("prism"));
}

#[test]
fn test_process_sample_data() {
    let output_path = "tests/output/test_clean.csv";

    // Clean up any existing output
    let _ = fs::remove_file(output_path);

    let mut cmd = Command::cargo_bin("prism").unwrap();
    cmd.arg("process")
        .arg("-i")
        .arg("examples/sample_data.csv")
        .arg("-c")
        .arg("examples/study_config.toml")
        .arg("-o")
        .arg(output_path);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("✓ Processing Complete"));

    // Verify output file exists
    assert!(Path::new(output_path).exists());

    // Verify output contains scale columns
    let content = fs::read_to_string(output_path).unwrap();
    assert!(content.contains("emotional_exhaustion_total"));
    assert!(content.contains("emotional_exhaustion_mean"));
    assert!(content.contains("depersonalization_total"));

    // Clean up
    let _ = fs::remove_file(output_path);
}

#[test]
fn test_process_with_stats_output() {
    let output_path = "tests/output/test_clean_stats.csv";
    let stats_path = "tests/output/test_stats.txt";

    // Clean up any existing output
    let _ = fs::remove_file(output_path);
    let _ = fs::remove_file(stats_path);

    let mut cmd = Command::cargo_bin("prism").unwrap();
    cmd.arg("process")
        .arg("-i")
        .arg("examples/sample_data.csv")
        .arg("-c")
        .arg("examples/study_config.toml")
        .arg("-o")
        .arg(output_path)
        .arg("--stats-output")
        .arg(stats_path);

    cmd.assert().success();

    // Verify stats file exists
    assert!(Path::new(stats_path).exists());

    // Verify stats content
    let stats_content = fs::read_to_string(stats_path).unwrap();
    assert!(stats_content.contains("Summary Statistics"));
    assert!(stats_content.contains("Mean") || stats_content.contains("M ="));
    assert!(stats_content.contains("Standard Deviation") || stats_content.contains("SD ="));
    assert!(stats_content.contains("N"));

    // Clean up
    let _ = fs::remove_file(output_path);
    let _ = fs::remove_file(stats_path);
}

#[test]
fn test_process_with_quality_report() {
    let output_path = "tests/output/test_clean_quality.csv";
    let quality_path = "tests/output/test_quality.txt";

    // Clean up any existing output
    let _ = fs::remove_file(output_path);
    let _ = fs::remove_file(quality_path);

    let mut cmd = Command::cargo_bin("prism").unwrap();
    cmd.arg("process")
        .arg("-i")
        .arg("examples/sample_data.csv")
        .arg("-c")
        .arg("examples/study_config.toml")
        .arg("-o")
        .arg(output_path)
        .arg("--quality-report")
        .arg(quality_path);

    cmd.assert().success();

    // Verify quality report exists
    assert!(Path::new(quality_path).exists());

    // Verify quality report content
    let quality_content = fs::read_to_string(quality_path).unwrap();
    assert!(quality_content.contains("QUALITY REPORT") || quality_content.contains("Quality"));

    // Clean up
    let _ = fs::remove_file(output_path);
    let _ = fs::remove_file(quality_path);
}

#[test]
fn test_missing_input_file() {
    let mut cmd = Command::cargo_bin("prism").unwrap();
    cmd.arg("process")
        .arg("-i")
        .arg("nonexistent.csv")
        .arg("-c")
        .arg("examples/study_config.toml")
        .arg("-o")
        .arg("output.csv");

    cmd.assert().failure();
}

#[test]
fn test_missing_config_file() {
    let mut cmd = Command::cargo_bin("prism").unwrap();
    cmd.arg("process")
        .arg("-i")
        .arg("examples/sample_data.csv")
        .arg("-c")
        .arg("nonexistent.toml")
        .arg("-o")
        .arg("output.csv");

    cmd.assert().failure();
}

#[test]
fn test_straightlining_detection() {
    let output_path = "tests/output/test_straightlining.csv";
    let quality_path = "tests/output/test_straightlining_quality.txt";

    // Clean up any existing output
    let _ = fs::remove_file(output_path);
    let _ = fs::remove_file(quality_path);

    let mut cmd = Command::cargo_bin("prism").unwrap();
    cmd.arg("process")
        .arg("-i")
        .arg("tests/fixtures/test_bad.csv")
        .arg("-c")
        .arg("examples/study_config.toml")
        .arg("-o")
        .arg(output_path)
        .arg("--quality-report")
        .arg(quality_path);

    cmd.assert().success();

    // Verify straightlining is detected
    if Path::new(quality_path).exists() {
        let quality_content = fs::read_to_string(quality_path).unwrap();
        assert!(
            quality_content.contains("Straightlining")
                || quality_content.contains("STRAIGHTLINING")
        );
    }

    // Clean up
    let _ = fs::remove_file(output_path);
    let _ = fs::remove_file(quality_path);
}

#[test]
fn test_all_outputs_together() {
    let output_path = "tests/output/test_all_clean.csv";
    let stats_path = "tests/output/test_all_stats.txt";
    let quality_path = "tests/output/test_all_quality.txt";

    // Clean up any existing output
    let _ = fs::remove_file(output_path);
    let _ = fs::remove_file(stats_path);
    let _ = fs::remove_file(quality_path);

    let mut cmd = Command::cargo_bin("prism").unwrap();
    cmd.arg("process")
        .arg("-i")
        .arg("examples/sample_data.csv")
        .arg("-c")
        .arg("examples/study_config.toml")
        .arg("-o")
        .arg(output_path)
        .arg("--stats-output")
        .arg(stats_path)
        .arg("--quality-report")
        .arg(quality_path);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("✓ Processing Complete"));
    // Note: file save messages are in stderr (INFO logs), not stdout

    // Verify all outputs exist
    assert!(Path::new(output_path).exists());
    assert!(Path::new(stats_path).exists());
    assert!(Path::new(quality_path).exists());

    // Clean up
    let _ = fs::remove_file(output_path);
    let _ = fs::remove_file(stats_path);
    let _ = fs::remove_file(quality_path);
}

// Helper to create output directory
#[ctor::ctor]
fn setup() {
    let _ = fs::create_dir_all("tests/output");
}
