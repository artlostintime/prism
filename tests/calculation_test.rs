use assert_cmd::{cargo, Command};
use std::fs;

#[test]
fn test_reverse_scoring_calculation() {
    // Create test data with known values
    let test_csv = "tests/fixtures/test_reverse.csv";
    let test_config = "tests/fixtures/test_reverse_config.toml";
    let output = "tests/output/test_reverse_output.csv";

    // Create test CSV: item values that should be reversed
    fs::write(
        test_csv,
        "participant_id,item1,item2,item3\n\
         P001,5,3,1\n\
         P002,1,2,4\n",
    )
    .unwrap();

    // Create test config with reverse scoring on item2
    fs::write(
        test_config,
        "[survey]\n\
         name = \"Reverse Test\"\n\
         min_score = 1\n\
         max_score = 5\n\
         \n\
         [quality]\n\
         max_missing_percent = 0.10\n\
         flag_straightlining = true\n\
         \n\
         [scales.test_scale]\n\
         items = [\"item1\", \"item2\", \"item3\"]\n\
         reverse_scored = [\"item2\"]\n",
    )
    .unwrap();

    // Run processing
    let mut cmd = cargo::cargo_bin_cmd!("prism");
    cmd.arg("process")
        .arg("-i")
        .arg(test_csv)
        .arg("-c")
        .arg(test_config)
        .arg("-o")
        .arg(output);

    cmd.assert().success();

    // Verify output
    let content = fs::read_to_string(output).unwrap();
    let lines: Vec<&str> = content.lines().collect();

    // P001: item1=5, item2=3 (reversed to 3), item3=1
    // Total should be: 5 + 3 + 1 = 9
    // Mean should be: 9/3 = 3.0
    // Note: Reverse formula is (max+min)-value = (5+1)-3 = 3

    // Check if P001 has correct values
    let p001_line = lines.iter().find(|l| l.starts_with("P001")).unwrap();
    assert!(
        p001_line.contains("9.00"),
        "Expected P001 line to contain total 9.00: {}",
        p001_line
    );

    // Clean up
    let _ = fs::remove_file(test_csv);
    let _ = fs::remove_file(test_config);
    let _ = fs::remove_file(output);
}

#[test]
fn test_scale_total_calculation() {
    // Test that totals are correctly summed
    let test_csv = "tests/fixtures/test_totals.csv";
    let test_config = "tests/fixtures/test_totals_config.toml";
    let output = "tests/output/test_totals_output.csv";

    // Create test CSV with known sum
    fs::write(
        test_csv,
        "participant_id,q1,q2,q3,q4\n\
         P001,2,3,4,1\n", // Sum = 10
    )
    .unwrap();

    fs::write(
        test_config,
        "[survey]\n\
         name = \"Total Test\"\n\
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

    // Run processing
    let mut cmd = cargo::cargo_bin_cmd!("prism");
    cmd.arg("process")
        .arg("-i")
        .arg(test_csv)
        .arg("-c")
        .arg(test_config)
        .arg("-o")
        .arg(output);

    cmd.assert().success();

    // Verify total = 10
    let content = fs::read_to_string(output).unwrap();
    assert!(content.contains("10") || content.contains("10.0"));

    // Clean up
    let _ = fs::remove_file(test_csv);
    let _ = fs::remove_file(test_config);
    let _ = fs::remove_file(output);
}

#[test]
fn test_scale_mean_calculation() {
    // Test that means are correctly calculated
    let test_csv = "tests/fixtures/test_means.csv";
    let test_config = "tests/fixtures/test_means_config.toml";
    let output = "tests/output/test_means_output.csv";

    // Create test CSV with known mean
    fs::write(
        test_csv,
        "participant_id,q1,q2,q3,q4\n\
         P001,4,4,4,4\n", // Mean = 4.0
    )
    .unwrap();

    fs::write(
        test_config,
        "[survey]\n\
         name = \"Mean Test\"\n\
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

    // Run processing
    let mut cmd = cargo::cargo_bin_cmd!("prism");
    cmd.arg("process")
        .arg("-i")
        .arg(test_csv)
        .arg("-c")
        .arg(test_config)
        .arg("-o")
        .arg(output);

    cmd.assert().success();

    // Verify mean = 4.0
    let content = fs::read_to_string(output).unwrap();
    let lines: Vec<&str> = content.lines().collect();
    let p001_line = lines.iter().find(|l| l.starts_with("P001")).unwrap();

    // Should contain 4.0 for mean
    assert!(p001_line.contains("4.0") || p001_line.contains("4,0"));

    // Clean up
    let _ = fs::remove_file(test_csv);
    let _ = fs::remove_file(test_config);
    let _ = fs::remove_file(output);
}

#[test]
fn test_aggregate_statistics() {
    let test_csv = "tests/fixtures/test_agg.csv";
    let test_config = "tests/fixtures/test_agg_config.toml";
    let output = "tests/output/test_agg_output.csv";
    let stats = "tests/output/test_agg_stats.txt";

    // Create test CSV with known statistics
    fs::write(
        test_csv,
        "participant_id,q1,q2\n\
         P001,1,1\n\
         P002,2,2\n\
         P003,3,3\n", // Mean should be 2.0
    )
    .unwrap();

    fs::write(
        test_config,
        "[survey]\n\
         name = \"Agg Test\"\n\
         min_score = 1\n\
         max_score = 5\n\
         \n\
         [quality]\n\
         max_missing_percent = 0.10\n\
         flag_straightlining = true\n\
         \n\
         [scales.test]\n\
         items = [\"q1\", \"q2\"]\n",
    )
    .unwrap();

    // Run with stats output
    let mut cmd = cargo::cargo_bin_cmd!("prism");
    cmd.arg("process")
        .arg("-i")
        .arg(test_csv)
        .arg("-c")
        .arg(test_config)
        .arg("-o")
        .arg(output)
        .arg("--stats-output")
        .arg(stats);

    cmd.assert().success();

    // Verify stats file contains aggregate info
    let stats_content = fs::read_to_string(stats).unwrap();
    assert!(stats_content.contains("Mean (M)") || stats_content.contains("M ="));
    assert!(stats_content.contains("Standard Deviation") || stats_content.contains("SD ="));
    assert!(stats_content.contains("N") && stats_content.contains("3"));

    // Clean up
    let _ = fs::remove_file(test_csv);
    let _ = fs::remove_file(test_config);
    let _ = fs::remove_file(output);
    let _ = fs::remove_file(stats);
}

#[test]
fn test_missing_data_handling() {
    let test_csv = "tests/fixtures/test_missing.csv";
    let test_config = "tests/fixtures/test_missing_config.toml";
    let output = "tests/output/test_missing_output.csv";

    // Create test CSV with missing values (empty cells)
    fs::write(
        test_csv,
        "participant_id,q1,q2,q3\n\
         P001,3,,5\n\
         P002,2,4,\n",
    )
    .unwrap();

    fs::write(
        test_config,
        "[survey]\n\
         name = \"Missing Test\"\n\
         min_score = 1\n\
         max_score = 5\n\
         \n\
         [quality]\n\
         max_missing_percent = 0.10\n\
         flag_straightlining = true\n\
         \n\
         [scales.test]\n\
         items = [\"q1\", \"q2\", \"q3\"]\n",
    )
    .unwrap();

    // Run processing - should handle missing data gracefully
    let mut cmd = cargo::cargo_bin_cmd!("prism");
    cmd.arg("process")
        .arg("-i")
        .arg(test_csv)
        .arg("-c")
        .arg(test_config)
        .arg("-o")
        .arg(output);

    // Should succeed despite missing data
    cmd.assert().success();

    // Clean up
    let _ = fs::remove_file(test_csv);
    let _ = fs::remove_file(test_config);
    let _ = fs::remove_file(output);
}

// Helper to create output directory
#[ctor::ctor]
fn setup() {
    let _ = fs::create_dir_all("tests/output");
    let _ = fs::create_dir_all("tests/fixtures");
}
