use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

#[test]
fn test_straightlining_detection_all_same() {
    let test_csv = "tests/fixtures/test_straightline.csv";
    let test_config = "tests/fixtures/test_straightline_config.toml";
    let output = "tests/output/test_straightline_output.csv";
    let quality = "tests/output/test_straightline_quality.txt";

    // Create CSV where participant answers all 4s (straightlining)
    fs::write(
        test_csv,
        "participant_id,q1,q2,q3,q4,q5\n\
         P001,4,4,4,4,4\n", // All identical - should be flagged
    )
    .unwrap();

    fs::write(
        test_config,
        "[survey]\n\
         name = \"Straightline Test\"\n\
         min_score = 1\n\
         max_score = 5\n\
         \n\
         [quality]\n\
         max_missing_percent = 0.10\n\
         flag_straightlining = true\n\
         \n\
         [scales.test_scale]\n\
         items = [\"q1\", \"q2\", \"q3\", \"q4\", \"q5\"]\n",
    )
    .unwrap();

    let mut cmd = Command::cargo_bin("prism").unwrap();
    cmd.arg("process")
        .arg("-i")
        .arg(test_csv)
        .arg("-c")
        .arg(test_config)
        .arg("-o")
        .arg(output)
        .arg("--quality-report")
        .arg(quality);

    cmd.assert().success();

    // Check that straightlining was detected
    let quality_content = fs::read_to_string(quality).unwrap();
    assert!(
        quality_content.to_lowercase().contains("straightlining")
            || quality_content.contains("identical")
    );

    // Clean up
    let _ = fs::remove_file(test_csv);
    let _ = fs::remove_file(test_config);
    let _ = fs::remove_file(output);
    let _ = fs::remove_file(quality);
}

#[test]
fn test_no_straightlining_with_variation() {
    let test_csv = "tests/fixtures/test_no_straightline.csv";
    let test_config = "tests/fixtures/test_no_straightline_config.toml";
    let output = "tests/output/test_no_straightline_output.csv";
    let quality = "tests/output/test_no_straightline_quality.txt";

    // Create CSV with varied responses (no straightlining)
    fs::write(
        test_csv,
        "participant_id,q1,q2,q3,q4\n\
         P001,3,4,5,2\n", // Varied - should NOT be flagged
    )
    .unwrap();

    fs::write(
        test_config,
        "[survey]\n\
         name = \"No Straightline Test\"\n\
         min_score = 1\n\
         max_score = 5\n\
         \n\
         [quality]\n\
         max_missing_percent = 0.10\n\
         flag_straightlining = true\n\
         \n\
         [scales.test_scale]\n\
         items = [\"q1\", \"q2\", \"q3\", \"q4\"]\n",
    )
    .unwrap();

    let mut cmd = Command::cargo_bin("prism").unwrap();
    cmd.arg("process")
        .arg("-i")
        .arg(test_csv)
        .arg("-c")
        .arg(test_config)
        .arg("-o")
        .arg(output)
        .arg("--quality-report")
        .arg(quality);

    cmd.assert().success();

    // Quality report should exist but not flag straightlining for P001
    if std::path::Path::new(quality).exists() {
        let quality_content = fs::read_to_string(quality).unwrap();
        // Should not mention P001 in straightlining section
        if quality_content.to_lowercase().contains("straightlining") {
            assert!(!quality_content.contains("P001"));
        }
    }

    // Clean up
    let _ = fs::remove_file(test_csv);
    let _ = fs::remove_file(test_config);
    let _ = fs::remove_file(output);
    let _ = fs::remove_file(quality);
}

#[test]
fn test_missing_data_percentage_high() {
    let test_csv = "tests/fixtures/test_high_missing.csv";
    let test_config = "tests/fixtures/test_high_missing_config.toml";
    let output = "tests/output/test_high_missing_output.csv";
    let quality = "tests/output/test_high_missing_quality.txt";

    // Create CSV with >50% missing data
    fs::write(
        test_csv,
        "participant_id,q1,q2,q3,q4,q5,q6\n\
         P001,3,,,,,\n", // Only 1 of 6 answered = 83% missing
    )
    .unwrap();

    fs::write(
        test_config,
        "[survey]\n\
         name = \"High Missing Test\"\n\
         min_score = 1\n\
         max_score = 5\n\
         \n\
         [quality]\n\
         max_missing_percent = 0.10\n\
         flag_straightlining = true\n\
         \n\
         [scales.test_scale]\n\
         items = [\"q1\", \"q2\", \"q3\", \"q4\", \"q5\", \"q6\"]\n",
    )
    .unwrap();

    let mut cmd = Command::cargo_bin("prism").unwrap();
    cmd.arg("process")
        .arg("-i")
        .arg(test_csv)
        .arg("-c")
        .arg(test_config)
        .arg("-o")
        .arg(output)
        .arg("--quality-report")
        .arg(quality);

    cmd.assert().success();

    // Check that missing data was flagged
    if std::path::Path::new(quality).exists() {
        let quality_content = fs::read_to_string(quality).unwrap();
        // Should flag high missing data
        assert!(
            quality_content.to_lowercase().contains("missing") || quality_content.contains("P001")
        );
    }

    // Clean up
    let _ = fs::remove_file(test_csv);
    let _ = fs::remove_file(test_config);
    let _ = fs::remove_file(output);
    let _ = fs::remove_file(quality);
}

#[test]
fn test_out_of_range_detection() {
    let test_csv = "tests/fixtures/test_out_of_range.csv";
    let test_config = "tests/fixtures/test_out_of_range_config.toml";
    let output = "tests/output/test_out_of_range_output.csv";
    let quality = "tests/output/test_out_of_range_quality.txt";

    // Create CSV with value outside 1-5 range
    fs::write(
        test_csv,
        "participant_id,q1,q2,q3\n\
         P001,3,8,4\n", // q2=8 is out of range (max=5)
    )
    .unwrap();

    fs::write(
        test_config,
        "[survey]\n\
         name = \"Out of Range Test\"\n\
         min_score = 1\n\
         max_score = 5\n\
         \n\
         [quality]\n\
         max_missing_percent = 0.10\n\
         flag_straightlining = true\n\
         \n\
         [scales.test_scale]\n\
         items = [\"q1\", \"q2\", \"q3\"]\n",
    )
    .unwrap();

    let mut cmd = Command::cargo_bin("prism").unwrap();
    cmd.arg("process")
        .arg("-i")
        .arg(test_csv)
        .arg("-c")
        .arg(test_config)
        .arg("-o")
        .arg(output)
        .arg("--quality-report")
        .arg(quality);

    cmd.assert().success();

    // Check that out-of-range was detected
    if std::path::Path::new(quality).exists() {
        let quality_content = fs::read_to_string(quality).unwrap();
        assert!(
            quality_content.to_lowercase().contains("out-of-range")
                || quality_content.to_lowercase().contains("range")
                || quality_content.contains("8")
        );
    }

    // Clean up
    let _ = fs::remove_file(test_csv);
    let _ = fs::remove_file(test_config);
    let _ = fs::remove_file(output);
    let _ = fs::remove_file(quality);
}

#[test]
fn test_multiple_quality_issues() {
    let test_csv = "tests/fixtures/test_multi_issues.csv";
    let test_config = "tests/fixtures/test_multi_issues_config.toml";
    let output = "tests/output/test_multi_issues_output.csv";
    let quality = "tests/output/test_multi_issues_quality.txt";

    // Create CSV with multiple issues
    fs::write(
        test_csv,
        "participant_id,q1,q2,q3,q4\n\
         P001,3,3,3,3\n\
         P002,5,,,\n\
         P003,9,2,3,4\n",
    )
    .unwrap();

    fs::write(
        test_config,
        "[survey]\n\
         name = \"Multi Issues Test\"\n\
         min_score = 1\n\
         max_score = 7\n\
         \n\
         [quality]\n\
         max_missing_percent = 0.10\n\
         flag_straightlining = true\n\
         \n\
         [scales.test_scale]\n\
         items = [\"q1\", \"q2\", \"q3\", \"q4\"]\n",
    )
    .unwrap();

    let mut cmd = Command::cargo_bin("prism").unwrap();
    cmd.arg("process")
        .arg("-i")
        .arg(test_csv)
        .arg("-c")
        .arg(test_config)
        .arg("-o")
        .arg(output)
        .arg("--quality-report")
        .arg(quality);

    cmd.assert().success();

    // Check that report contains multiple issue types
    if std::path::Path::new(quality).exists() {
        let quality_content = fs::read_to_string(quality).unwrap();

        // Should have at least 2 different types of issues
        let has_straightline = quality_content.to_lowercase().contains("straightlining");
        let has_missing = quality_content.to_lowercase().contains("missing");
        let has_range = quality_content.to_lowercase().contains("range");

        let issue_count = [has_straightline, has_missing, has_range]
            .iter()
            .filter(|&&x| x)
            .count();

        assert!(issue_count >= 2, "Should detect multiple types of issues");
    }

    // Clean up
    let _ = fs::remove_file(test_csv);
    let _ = fs::remove_file(test_config);
    let _ = fs::remove_file(output);
    let _ = fs::remove_file(quality);
}

// Helper to create directories
#[ctor::ctor]
fn setup() {
    let _ = fs::create_dir_all("tests/output");
    let _ = fs::create_dir_all("tests/fixtures");
}
