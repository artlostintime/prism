use assert_cmd::Command;
use std::fs;

#[test]
fn test_invalid_config_missing_items() {
    let test_csv = "tests/fixtures/test_validation.csv";
    let test_config = "tests/fixtures/test_invalid_config.toml";
    let output = "tests/output/test_invalid_output.csv";

    fs::write(
        test_csv,
        "participant_id,q1,q2,q3\n\
         P001,3,4,5\n",
    )
    .unwrap();

    // Config references q4 which doesn't exist
    fs::write(
        test_config,
        "[survey]\n\
         name = \"Invalid Test\"\n\
         min_score = 1\n\
         max_score = 5\n\
         \n\
         [quality]\n\
         max_missing_percent = 0.10\n\
         flag_straightlining = true\n\
         \n\
         [scales.test]\n\
         items = [\"q1\", \"q2\", \"q4\"]\n", // q4 doesn't exist!
    )
    .unwrap();

    let mut cmd = Command::cargo_bin("prism").unwrap();
    cmd.arg("process")
        .arg("-i")
        .arg(test_csv)
        .arg("-c")
        .arg(test_config)
        .arg("-o")
        .arg(output);

    // Should fail with validation error
    cmd.assert().failure();

    // Clean up
    let _ = fs::remove_file(test_csv);
    let _ = fs::remove_file(test_config);
    let _ = fs::remove_file(output);
}

#[test]
fn test_invalid_config_reverse_item_not_in_list() {
    let test_csv = "tests/fixtures/test_reverse_validation.csv";
    let test_config = "tests/fixtures/test_reverse_invalid.toml";
    let output = "tests/output/test_reverse_invalid_output.csv";

    fs::write(
        test_csv,
        "participant_id,q1,q2,q3\n\
         P001,3,4,5\n",
    )
    .unwrap();

    // Reverse item q4 not in items list
    fs::write(
        test_config,
        "[survey]\n\
         name = \"Reverse Invalid\"\n\
         min_score = 1\n\
         max_score = 5\n\
         \n\
         [quality]\n\
         max_missing_percent = 0.10\n\
         flag_straightlining = true\n\
         \n\
         [scales.test]\n\
         items = [\"q1\", \"q2\", \"q3\"]\n\
         reverse_scored = [\"q4\"]\n", // q4 not in items!
    )
    .unwrap();

    let mut cmd = Command::cargo_bin("prism").unwrap();
    cmd.arg("process")
        .arg("-i")
        .arg(test_csv)
        .arg("-c")
        .arg(test_config)
        .arg("-o")
        .arg(output);

    // Should fail
    cmd.assert().failure();

    // Clean up
    let _ = fs::remove_file(test_csv);
    let _ = fs::remove_file(test_config);
    let _ = fs::remove_file(output);
}

#[test]
fn test_invalid_config_min_greater_than_max() {
    let test_csv = "tests/fixtures/test_minmax.csv";
    let test_config = "tests/fixtures/test_minmax_invalid.toml";
    let output = "tests/output/test_minmax_output.csv";

    fs::write(
        test_csv,
        "participant_id,q1,q2\n\
         P001,3,4\n",
    )
    .unwrap();

    // min > max (invalid)
    fs::write(
        test_config,
        "[survey]\n\
         name = \"MinMax Invalid\"\n\
         min_score = 5\n\
         max_score = 1\n\
         \n\
         [quality]\n\
         max_missing_percent = 0.10\n\
         flag_straightlining = true\n\
         \n\
         [scales.test]\n\
         items = [\"q1\", \"q2\"]\n", // 5 > 1 - invalid!
    )
    .unwrap();

    let mut cmd = Command::cargo_bin("prism").unwrap();
    cmd.arg("process")
        .arg("-i")
        .arg(test_csv)
        .arg("-c")
        .arg(test_config)
        .arg("-o")
        .arg(output);

    // Should fail
    cmd.assert().failure();

    // Clean up
    let _ = fs::remove_file(test_csv);
    let _ = fs::remove_file(test_config);
    let _ = fs::remove_file(output);
}

#[test]
fn test_valid_config_with_all_options() {
    let test_csv = "tests/fixtures/test_valid_full.csv";
    let test_config = "tests/fixtures/test_valid_full.toml";
    let output = "tests/output/test_valid_full_output.csv";

    fs::write(
        test_csv,
        "participant_id,q1,q2,q3,q4\n\
         P001,5,3,2,4\n",
    )
    .unwrap();

    // Valid config with all options
    fs::write(
        test_config,
        "[survey]\n\
         name = \"Valid Full Test\"\n\
         min_score = 1\n\
         max_score = 7\n\
         \n\
         [quality]\n\
         max_missing_percent = 0.15\n\
         flag_straightlining = true\n\
         \n\
         [scales.scale1]\n\
         items = [\"q1\", \"q2\"]\n\
         reverse_scored = [\"q2\"]\n\
         \n\
         [scales.scale2]\n\
         items = [\"q3\", \"q4\"]\n",
    )
    .unwrap();

    let mut cmd = Command::cargo_bin("prism").unwrap();
    cmd.arg("process")
        .arg("-i")
        .arg(test_csv)
        .arg("-c")
        .arg(test_config)
        .arg("-o")
        .arg(output);

    // Should succeed
    cmd.assert().success();

    // Verify output has both scales
    let content = fs::read_to_string(output).unwrap();
    assert!(content.contains("scale1_total"));
    assert!(content.contains("scale1_mean"));
    assert!(content.contains("scale2_total"));
    assert!(content.contains("scale2_mean"));

    // Clean up
    let _ = fs::remove_file(test_csv);
    let _ = fs::remove_file(test_config);
    let _ = fs::remove_file(output);
}

#[test]
fn test_malformed_toml() {
    let test_csv = "tests/fixtures/test_malformed.csv";
    let test_config = "tests/fixtures/test_malformed.toml";
    let output = "tests/output/test_malformed_output.csv";

    fs::write(
        test_csv,
        "participant_id,q1\n\
         P001,3\n",
    )
    .unwrap();

    // Invalid TOML syntax
    fs::write(
        test_config,
        "[survey\n\
         name = \"Test\"\n", // Missing closing bracket
    )
    .unwrap();

    let mut cmd = Command::cargo_bin("prism").unwrap();
    cmd.arg("-i")
        .arg(test_csv)
        .arg("-c")
        .arg(test_config)
        .arg("-o")
        .arg(output);

    // Should fail with parse error
    cmd.assert().failure();

    // Clean up
    let _ = fs::remove_file(test_csv);
    let _ = fs::remove_file(test_config);
    let _ = fs::remove_file(output);
}

// Helper to create directories
#[ctor::ctor]
fn setup() {
    let _ = fs::create_dir_all("tests/output");
    let _ = fs::create_dir_all("tests/fixtures");
}
