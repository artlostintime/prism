use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_power_a_priori_independent_t() {
    let mut cmd = Command::cargo_bin("prism").unwrap();
    cmd.arg("power")
        .arg("--test")
        .arg("independent-t")
        .arg("--effect-size")
        .arg("0.5")
        .arg("--power")
        .arg("0.80")
        .arg("--alpha")
        .arg("0.05");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Power Analysis Results"))
        .stdout(predicate::str::contains("Test Type:       IndependentT"))
        .stdout(predicate::str::contains("Effect Size:     0.500 (Medium)"))
        .stdout(predicate::str::contains("Sample Size:     63"))
        .stdout(predicate::str::contains("Power:           0.800"));
}

#[test]
fn test_power_post_hoc_correlation() {
    let mut cmd = Command::cargo_bin("prism").unwrap();
    cmd.arg("power")
        .arg("--test")
        .arg("correlation")
        .arg("--effect-size")
        .arg("0.3")
        .arg("--sample-size")
        .arg("100")
        .arg("--alpha")
        .arg("0.05");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Power Analysis Results"))
        .stdout(predicate::str::contains("Test Type:       Correlation"))
        .stdout(predicate::str::contains("Effect Size:     0.300 (Medium)"))
        .stdout(predicate::str::contains("Sample Size:     100"))
        .stdout(predicate::str::contains("Adequate power"));
}

#[test]
fn test_power_paired_t_small_effect() {
    let mut cmd = Command::cargo_bin("prism").unwrap();
    cmd.arg("power")
        .arg("--test")
        .arg("paired-t")
        .arg("--effect-size")
        .arg("0.2")
        .arg("--power")
        .arg("0.80")
        .arg("--alpha")
        .arg("0.05");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Effect Size:     0.200 (Small)"))
        .stdout(predicate::str::contains("Sample Size:"))
        .stdout(predicate::str::contains("Power:           0.800"));
}

#[test]
fn test_power_one_tailed_test() {
    let mut cmd = Command::cargo_bin("prism").unwrap();
    cmd.arg("power")
        .arg("--test")
        .arg("one-sample-t")
        .arg("--effect-size")
        .arg("0.5")
        .arg("--power")
        .arg("0.80")
        .arg("--alpha")
        .arg("0.05")
        .arg("--tails")
        .arg("1");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Tails:           1"))
        .stdout(predicate::str::contains("Power Analysis Results"));
}

#[test]
fn test_power_large_effect() {
    let mut cmd = Command::cargo_bin("prism").unwrap();
    cmd.arg("power")
        .arg("--test")
        .arg("independent-t")
        .arg("--effect-size")
        .arg("0.8")
        .arg("--power")
        .arg("0.80")
        .arg("--alpha")
        .arg("0.05");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Effect Size:     0.800 (Large)"))
        .stdout(predicate::str::contains("Sample Size:"));
}

#[test]
fn test_power_low_observed_power() {
    let mut cmd = Command::cargo_bin("prism").unwrap();
    cmd.arg("power")
        .arg("--test")
        .arg("correlation")
        .arg("--effect-size")
        .arg("0.1")
        .arg("--sample-size")
        .arg("50")
        .arg("--alpha")
        .arg("0.05");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Effect Size:     0.100 (Small)"))
        .stdout(
            predicate::str::contains("Low power").or(predicate::str::contains("Very low power")),
        );
}

#[test]
fn test_power_strict_alpha() {
    let mut cmd = Command::cargo_bin("prism").unwrap();
    cmd.arg("power")
        .arg("--test")
        .arg("independent-t")
        .arg("--effect-size")
        .arg("0.5")
        .arg("--power")
        .arg("0.80")
        .arg("--alpha")
        .arg("0.01");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Alpha:           0.010"))
        .stdout(predicate::str::contains("Sample Size:"));
}

#[test]
fn test_power_invalid_effect_size() {
    let mut cmd = Command::cargo_bin("prism").unwrap();
    cmd.arg("power")
        .arg("--test")
        .arg("independent-t")
        .arg("--effect-size")
        .arg("0")
        .arg("--power")
        .arg("0.80");

    cmd.assert().failure().stderr(predicate::str::contains(
        "Effect size must be greater than 0",
    ));
}

#[test]
fn test_power_invalid_alpha() {
    let mut cmd = Command::cargo_bin("prism").unwrap();
    cmd.arg("power")
        .arg("--test")
        .arg("independent-t")
        .arg("--effect-size")
        .arg("0.5")
        .arg("--power")
        .arg("0.80")
        .arg("--alpha")
        .arg("1.5");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Alpha must be between 0 and 1"));
}

#[test]
fn test_power_missing_required_params() {
    let mut cmd = Command::cargo_bin("prism").unwrap();
    cmd.arg("power")
        .arg("--test")
        .arg("independent-t")
        .arg("--effect-size")
        .arg("0.5");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Must specify either --power"));
}

#[test]
fn test_power_invalid_test_type() {
    let mut cmd = Command::cargo_bin("prism").unwrap();
    cmd.arg("power")
        .arg("--test")
        .arg("invalid-test")
        .arg("--effect-size")
        .arg("0.5")
        .arg("--power")
        .arg("0.80");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Unknown test type"));
}

#[test]
fn test_power_output_file() {
    use std::fs;
    use tempfile::tempdir;

    let temp_dir = tempdir().unwrap();
    let output_path = temp_dir.path().join("power_results.txt");

    let mut cmd = Command::cargo_bin("prism").unwrap();
    cmd.arg("power")
        .arg("--test")
        .arg("paired-t")
        .arg("--effect-size")
        .arg("0.5")
        .arg("--power")
        .arg("0.80")
        .arg("--output")
        .arg(output_path.to_str().unwrap());

    cmd.assert().success();

    // Verify file was created and contains expected content
    let contents = fs::read_to_string(&output_path).unwrap();
    assert!(contents.contains("Power Analysis Results"));
    assert!(contents.contains("Test Type:       PairedT"));
    assert!(contents.contains("Effect Size:     0.500"));
}

#[test]
fn test_power_correlation_small_effect() {
    let mut cmd = Command::cargo_bin("prism").unwrap();
    cmd.arg("power")
        .arg("--test")
        .arg("correlation")
        .arg("--effect-size")
        .arg("0.1")
        .arg("--power")
        .arg("0.80")
        .arg("--alpha")
        .arg("0.05");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Effect Size:     0.100 (Small)"))
        .stdout(predicate::str::contains("Sample Size:"));
}

#[test]
fn test_power_high_desired_power() {
    let mut cmd = Command::cargo_bin("prism").unwrap();
    cmd.arg("power")
        .arg("--test")
        .arg("independent-t")
        .arg("--effect-size")
        .arg("0.5")
        .arg("--power")
        .arg("0.95")
        .arg("--alpha")
        .arg("0.05");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Power:           0.950"))
        .stdout(predicate::str::contains("Sample Size:"));
}
