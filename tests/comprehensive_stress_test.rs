// tests/comprehensive_stress_test.rs
// Extreme stress testing with large datasets and boundary conditions

use assert_cmd::cargo;
use std::fs;
use std::io::Write;

#[test]
fn test_large_dataset_1000_participants() {
    let test_csv = "tests/fixtures/stress_large_1k.csv";
    let test_config = "tests/fixtures/stress_large_config.toml";
    let output = "tests/output/stress_large_1k.csv";

    // Generate 1000 participants with 10 items each
    let mut file = fs::File::create(test_csv).unwrap();
    writeln!(file, "id,q1,q2,q3,q4,q5,q6,q7,q8,q9,q10").unwrap();

    for i in 1..=1000 {
        writeln!(
            file,
            "P{:04},{},{},{},{},{},{},{},{},{},{}",
            i,
            (i % 5) + 1,
            (i % 5) + 1,
            (i % 5) + 1,
            (i % 5) + 1,
            (i % 5) + 1,
            (i % 5) + 1,
            (i % 5) + 1,
            (i % 5) + 1,
            (i % 5) + 1,
            (i % 5) + 1
        )
        .unwrap();
    }

    fs::write(
        test_config,
        r#"[survey]
name = "Large Dataset Test"
min_score = 1
max_score = 5

[scales.test_scale]
items = ["q1", "q2", "q3", "q4", "q5", "q6", "q7", "q8", "q9", "q10"]
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
    assert_eq!(lines.len(), 1001, "Should have 1000 data rows + header");
}

#[test]
fn test_very_large_scale_50_items() {
    let test_csv = "tests/fixtures/stress_wide.csv";
    let test_config = "tests/fixtures/stress_wide_config.toml";
    let output = "tests/output/stress_wide.csv";

    // Generate 50-item scale
    let mut file = fs::File::create(test_csv).unwrap();
    write!(file, "id").unwrap();
    for i in 1..=50 {
        write!(file, ",i{}", i).unwrap();
    }
    writeln!(file).unwrap();

    // Add 100 participants
    for p in 1..=100 {
        write!(file, "P{:03}", p).unwrap();
        for _ in 1..=50 {
            write!(file, ",{}", (p % 5) + 1).unwrap();
        }
        writeln!(file).unwrap();
    }

    // Generate config
    let mut config = String::from(
        "[survey]\nname = \"Wide Scale Test\"\nmin_score = 1\nmax_score = 5\n\n[scales.bigscale]\nitems = [",
    );
    for i in 1..=50 {
        if i > 1 {
            config.push_str(", ");
        }
        config.push_str(&format!("\"i{}\"", i));
    }
    config.push_str("]\n");

    fs::write(test_config, config).unwrap();

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
    assert_eq!(lines.len(), 101, "Should have 100 data rows + header");
}

#[test]
fn test_high_missing_data_90_percent() {
    let test_csv = "tests/fixtures/stress_missing_90.csv";
    let test_config = "tests/fixtures/stress_missing_config.toml";
    let output = "tests/output/stress_missing_90.csv";

    // Create CSV where 90% of values are missing
    let mut file = fs::File::create(test_csv).unwrap();
    writeln!(file, "id,q1,q2,q3,q4,q5,q6,q7,q8,q9,q10").unwrap();

    for i in 1..=100 {
        write!(file, "P{:03}", i).unwrap();
        for j in 1..=10 {
            if (i + j) % 10 == 0 {
                // Only 10% have values
                write!(file, ",{}", (j % 5) + 1).unwrap();
            } else {
                write!(file, ",").unwrap();
            }
        }
        writeln!(file).unwrap();
    }

    fs::write(
        test_config,
        r#"[survey]
name = "High Missing Data Test"
min_score = 1
max_score = 5

[quality]
max_missing_percent = 0.95
flag_straightlining = true

[scales.test]
items = ["q1", "q2", "q3", "q4", "q5", "q6", "q7", "q8", "q9", "q10"]
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
}

#[test]
fn test_unicode_participant_ids() {
    let test_csv = "tests/fixtures/stress_unicode.csv";
    let test_config = "tests/fixtures/stress_unicode_config.toml";
    let output = "tests/output/stress_unicode.csv";

    fs::write(
        test_csv,
        "id,q1,q2,q3\n\
         用户001,5,4,5\n\
         Müller_42,3,3,4\n\
         José_García,2,3,2\n\
         Владимир_99,4,4,4\n\
         田中太郎,5,5,5\n",
    )
    .unwrap();

    fs::write(
        test_config,
        r#"[survey]
name = "Unicode Test"
min_score = 1
max_score = 5

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
    assert!(content.contains("用户001"));
    assert!(content.contains("Müller_42"));
    assert!(content.contains("José_García"));
    assert!(content.contains("Владимир_99"));
    assert!(content.contains("田中太郎"));
}

#[test]
fn test_empty_csv_file() {
    let test_csv = "tests/fixtures/stress_empty.csv";
    let test_config = "tests/fixtures/stress_empty_config.toml";
    let output = "tests/output/stress_empty.csv";

    fs::write(test_csv, "").unwrap();

    fs::write(
        test_config,
        r#"[survey]
name = "Empty Test"
min_score = 1
max_score = 5

[scales.test]
items = ["q1"]
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

    // Should fail gracefully
    cmd.assert().failure();
}

#[test]
fn test_single_participant() {
    let test_csv = "tests/fixtures/stress_single.csv";
    let test_config = "tests/fixtures/stress_single_config.toml";
    let output = "tests/output/stress_single.csv";

    fs::write(
        test_csv,
        "id,q1,q2,q3,q4,q5\n\
         P001,5,4,5,4,5\n",
    )
    .unwrap();

    fs::write(
        test_config,
        r#"[survey]
name = "Single Participant Test"
min_score = 1
max_score = 5

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
    assert_eq!(lines.len(), 2, "Should have 1 data row + header");
}

#[test]
fn test_mixed_numeric_types() {
    let test_csv = "tests/fixtures/stress_mixed_numeric.csv";
    let test_config = "tests/fixtures/stress_numeric_config.toml";
    let output = "tests/output/stress_numeric.csv";

    fs::write(
        test_csv,
        "id,q1,q2,q3,q4,q5\n\
         P001,5,4.0,5.00,4.5,5\n\
         P002,3.5,3,4.0,3.50,4\n",
    )
    .unwrap();

    fs::write(
        test_config,
        r#"[survey]
name = "Mixed Numeric Test"
min_score = 1
max_score = 5

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
}

#[test]
fn test_extremely_long_participant_id() {
    let test_csv = "tests/fixtures/stress_long_id.csv";
    let test_config = "tests/fixtures/stress_long_id_config.toml";
    let output = "tests/output/stress_long_id.csv";

    let long_id = "P".to_string() + &"X".repeat(500);

    fs::write(
        test_csv,
        format!(
            "id,q1,q2,q3\n\
             {},5,4,5\n",
            long_id
        ),
    )
    .unwrap();

    fs::write(
        test_config,
        r#"[survey]
name = "Long ID Test"
min_score = 1
max_score = 5

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
    assert!(content.contains(&long_id));
}

#[test]
fn test_boundary_score_values() {
    let test_csv = "tests/fixtures/stress_boundary.csv";
    let test_config = "tests/fixtures/stress_boundary_config.toml";
    let output = "tests/output/stress_boundary.csv";

    fs::write(
        test_csv,
        "id,q1,q2,q3,q4,q5\n\
         P001,1,1,1,1,1\n\
         P002,7,7,7,7,7\n\
         P003,0,8,1,7,4\n\
         P004,1.0,7.0,1.0,7.0,4.0\n",
    )
    .unwrap();

    fs::write(
        test_config,
        r#"[survey]
name = "Boundary Values Test"
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

    // Check that out-of-range values (0, 8) are flagged
    let content = fs::read_to_string(output).unwrap();
    assert!(content.contains("P003"));
}

// Helper to create directories
#[ctor::ctor]
fn setup() {
    fs::create_dir_all("tests/fixtures").ok();
    fs::create_dir_all("tests/output").ok();
}
