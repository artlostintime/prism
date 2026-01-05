// tests/longitudinal_test.rs
use assert_cmd::cargo::CommandCargoExt;
use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_merge_waves_outer_join() {
    let temp_dir = TempDir::new().unwrap();
    let t1_path = temp_dir.path().join("wave_t1.csv");
    let t2_path = temp_dir.path().join("wave_t2.csv");
    let output_path = temp_dir.path().join("merged.csv");

    // Create test data for T1
    fs::write(
        &t1_path,
        "ParticipantID,anxiety,depression\n\
         P001,15,20\n\
         P002,10,12\n\
         P003,25,30\n",
    )
    .unwrap();

    // Create test data for T2 (P003 missing, P004 added)
    fs::write(
        &t2_path,
        "ParticipantID,anxiety,depression\n\
         P001,12,18\n\
         P002,8,10\n\
         P004,20,25\n",
    )
    .unwrap();

    let mut cmd = Command::cargo_bin("prism").unwrap();
    cmd.arg("merge")
        .arg("--waves")
        .arg(format!("T1:{}", t1_path.display()))
        .arg("--waves")
        .arg(format!("T2:{}", t2_path.display()))
        .arg("--output")
        .arg(output_path.as_path())
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Successfully merged 4 participants",
        ));

    // Verify output file exists and has correct structure
    let output_content = fs::read_to_string(&output_path).unwrap();
    assert!(output_content.contains("ParticipantID"));
    assert!(output_content.contains("anxiety_T1"));
    assert!(output_content.contains("anxiety_T2"));
    assert!(output_content.contains("depression_T1"));
    assert!(output_content.contains("depression_T2"));
    assert!(output_content.contains("P001"));
    assert!(output_content.contains("P002"));
    assert!(output_content.contains("P003"));
    assert!(output_content.contains("P004"));
}

#[test]
fn test_merge_waves_inner_join() {
    let temp_dir = TempDir::new().unwrap();
    let t1_path = temp_dir.path().join("wave_t1.csv");
    let t2_path = temp_dir.path().join("wave_t2.csv");
    let output_path = temp_dir.path().join("merged.csv");

    // Create test data
    fs::write(
        &t1_path,
        "ParticipantID,score\n\
         P001,15\n\
         P002,10\n\
         P003,25\n",
    )
    .unwrap();

    fs::write(
        &t2_path,
        "ParticipantID,score\n\
         P001,12\n\
         P002,8\n\
         P004,20\n",
    )
    .unwrap();

    let mut cmd = Command::cargo_bin("prism").unwrap();
    cmd.arg("merge")
        .arg("--waves")
        .arg(format!("T1:{}", t1_path.display()))
        .arg("--waves")
        .arg(format!("T2:{}", t2_path.display()))
        .arg("--output")
        .arg(output_path.as_path())
        .arg("--inner-join")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Successfully merged 2 participants",
        ));

    // Verify only P001 and P002 are in output (both present in all waves)
    let output_content = fs::read_to_string(&output_path).unwrap();
    assert!(output_content.contains("P001"));
    assert!(output_content.contains("P002"));
    assert!(!output_content.contains("P003"));
    assert!(!output_content.contains("P004"));
}

#[test]
fn test_reshape_wide_to_long() {
    let temp_dir = TempDir::new().unwrap();
    let input_path = temp_dir.path().join("wide_data.csv");
    let output_path = temp_dir.path().join("long_data.csv");

    // Create wide format data
    fs::write(
        &input_path,
        "ParticipantID,anxiety_T1,anxiety_T2,depression_T1,depression_T2\n\
         P001,15,12,20,18\n\
         P002,10,8,12,10\n",
    )
    .unwrap();

    let mut cmd = Command::cargo_bin("prism").unwrap();
    cmd.arg("reshape")
        .arg("--input")
        .arg(input_path.as_path())
        .arg("--output")
        .arg(output_path.as_path())
        .arg("--format")
        .arg("long")
        .arg("--waves")
        .arg("T1,T2")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Successfully reshaped data (4 rows)",
        ));

    // Verify long format structure
    let output_content = fs::read_to_string(&output_path).unwrap();
    assert!(output_content.contains("ParticipantID,Wave,anxiety,depression"));
    assert!(output_content.contains("P001,T1,15,20"));
    assert!(output_content.contains("P001,T2,12,18"));
    assert!(output_content.contains("P002,T1,10,12"));
    assert!(output_content.contains("P002,T2,8,10"));
}

#[test]
fn test_reshape_long_to_wide() {
    let temp_dir = TempDir::new().unwrap();
    let input_path = temp_dir.path().join("long_data.csv");
    let output_path = temp_dir.path().join("wide_data.csv");

    // Create long format data
    fs::write(
        &input_path,
        "ParticipantID,Wave,anxiety,depression\n\
         P001,T1,15,20\n\
         P001,T2,12,18\n\
         P002,T1,10,12\n\
         P002,T2,8,10\n",
    )
    .unwrap();

    let mut cmd = Command::cargo_bin("prism").unwrap();
    cmd.arg("reshape")
        .arg("--input")
        .arg(input_path.as_path())
        .arg("--output")
        .arg(output_path.as_path())
        .arg("--format")
        .arg("wide")
        .arg("--waves")
        .arg("T1,T2")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Successfully reshaped data (2 rows)",
        ));

    // Verify wide format structure
    let output_content = fs::read_to_string(&output_path).unwrap();
    assert!(output_content.contains("ParticipantID"));
    assert!(output_content.contains("anxiety_T1"));
    assert!(output_content.contains("anxiety_T2"));
    assert!(output_content.contains("depression_T1"));
    assert!(output_content.contains("depression_T2"));
    assert!(output_content.contains("P001"));
    assert!(output_content.contains("P002"));
}

#[test]
fn test_rci_calculation() {
    let temp_dir = TempDir::new().unwrap();
    let baseline_path = temp_dir.path().join("baseline.csv");
    let followup_path = temp_dir.path().join("followup.csv");
    let output_path = temp_dir.path().join("rci_results.csv");

    // Create baseline data
    fs::write(
        &baseline_path,
        "ParticipantID,anxiety\n\
         P001,20\n\
         P002,15\n\
         P003,25\n\
         P004,18\n",
    )
    .unwrap();

    // Create follow-up data with various changes
    fs::write(
        &followup_path,
        "ParticipantID,anxiety\n\
         P001,10\n\
         P002,14\n\
         P003,26\n\
         P004,12\n",
    )
    .unwrap();

    let mut cmd = Command::cargo_bin("prism").unwrap();
    cmd.arg("rci")
        .arg("--baseline")
        .arg(baseline_path.as_path())
        .arg("--followup")
        .arg(followup_path.as_path())
        .arg("--scale")
        .arg("anxiety")
        .arg("--reliability")
        .arg("0.85")
        .arg("--output")
        .arg(output_path.as_path())
        .assert()
        .success()
        .stdout(predicate::str::contains("RCI Analysis Complete"))
        .stdout(predicate::str::contains("Total participants: 4"));

    // Verify output file contains expected columns
    let output_content = fs::read_to_string(&output_path).unwrap();
    assert!(output_content.contains("ParticipantID"));
    assert!(output_content.contains("Baseline"));
    assert!(output_content.contains("Followup"));
    assert!(output_content.contains("RCI"));
    assert!(output_content.contains("IsReliable"));
    assert!(output_content.contains("Direction"));
    assert!(output_content.contains("P001"));
    assert!(output_content.contains("P002"));
    assert!(output_content.contains("P003"));
    assert!(output_content.contains("P004"));
}

#[test]
fn test_rci_with_baseline_sd() {
    let temp_dir = TempDir::new().unwrap();
    let baseline_path = temp_dir.path().join("baseline.csv");
    let followup_path = temp_dir.path().join("followup.csv");
    let output_path = temp_dir.path().join("rci_results.csv");

    // Create test data
    fs::write(
        &baseline_path,
        "ParticipantID,depression\n\
         P001,30\n\
         P002,25\n",
    )
    .unwrap();

    fs::write(
        &followup_path,
        "ParticipantID,depression\n\
         P001,15\n\
         P002,24\n",
    )
    .unwrap();

    let mut cmd = Command::cargo_bin("prism").unwrap();
    cmd.arg("rci")
        .arg("--baseline")
        .arg(baseline_path.as_path())
        .arg("--followup")
        .arg(followup_path.as_path())
        .arg("--scale")
        .arg("depression")
        .arg("--reliability")
        .arg("0.90")
        .arg("--baseline-sd")
        .arg("5.0")
        .arg("--output")
        .arg(output_path.as_path())
        .assert()
        .success();

    assert!(output_path.exists());
}

#[test]
fn test_merge_missing_wave_file() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("merged.csv");

    let mut cmd = Command::cargo_bin("prism").unwrap();
    cmd.arg("merge")
        .arg("--waves")
        .arg("T1:nonexistent.csv")
        .arg("--output")
        .arg(output_path.as_path())
        .assert()
        .failure()
        .stderr(predicate::str::contains("Error merging waves"));
}

#[test]
fn test_reshape_invalid_format() {
    let temp_dir = TempDir::new().unwrap();
    let input_path = temp_dir.path().join("data.csv");
    let output_path = temp_dir.path().join("reshaped.csv");

    fs::write(&input_path, "ID,value\nP001,10\n").unwrap();

    let mut cmd = Command::cargo_bin("prism").unwrap();
    cmd.arg("reshape")
        .arg("--input")
        .arg(input_path.as_path())
        .arg("--output")
        .arg(output_path.as_path())
        .arg("--format")
        .arg("invalid")
        .arg("--waves")
        .arg("T1,T2")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Format must be 'wide' or 'long'"));
}

#[test]
fn test_rci_invalid_reliability() {
    let temp_dir = TempDir::new().unwrap();
    let baseline_path = temp_dir.path().join("baseline.csv");
    let followup_path = temp_dir.path().join("followup.csv");
    let output_path = temp_dir.path().join("rci.csv");

    fs::write(&baseline_path, "ID,score\nP001,10\n").unwrap();
    fs::write(&followup_path, "ID,score\nP001,8\n").unwrap();

    let mut cmd = Command::cargo_bin("prism").unwrap();
    cmd.arg("rci")
        .arg("--baseline")
        .arg(baseline_path.as_path())
        .arg("--followup")
        .arg(followup_path.as_path())
        .arg("--scale")
        .arg("score")
        .arg("--reliability")
        .arg("1.5")
        .arg("--output")
        .arg(output_path.as_path())
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "Reliability must be between 0 and 1",
        ));
}

#[test]
fn test_merge_three_waves() {
    let temp_dir = TempDir::new().unwrap();
    let t1_path = temp_dir.path().join("wave_t1.csv");
    let t2_path = temp_dir.path().join("wave_t2.csv");
    let t3_path = temp_dir.path().join("wave_t3.csv");
    let output_path = temp_dir.path().join("merged.csv");

    // Create test data for three time points
    fs::write(&t1_path, "ParticipantID,score\nP001,10\nP002,15\n").unwrap();
    fs::write(&t2_path, "ParticipantID,score\nP001,12\nP002,14\n").unwrap();
    fs::write(&t3_path, "ParticipantID,score\nP001,8\nP002,16\n").unwrap();

    let mut cmd = Command::cargo_bin("prism").unwrap();
    cmd.arg("merge")
        .arg("--waves")
        .arg(format!("T1:{}", t1_path.display()))
        .arg("--waves")
        .arg(format!("T2:{}", t2_path.display()))
        .arg("--waves")
        .arg(format!("T3:{}", t3_path.display()))
        .arg("--output")
        .arg(output_path.as_path())
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Successfully merged 2 participants",
        ));

    // Verify all three waves are in the output
    let output_content = fs::read_to_string(&output_path).unwrap();
    assert!(output_content.contains("score_T1"));
    assert!(output_content.contains("score_T2"));
    assert!(output_content.contains("score_T3"));
}

#[test]
fn test_reshape_missing_waves() {
    let temp_dir = TempDir::new().unwrap();
    let input_path = temp_dir.path().join("data.csv");
    let output_path = temp_dir.path().join("reshaped.csv");

    fs::write(&input_path, "ID,value\nP001,10\n").unwrap();

    let mut cmd = Command::cargo_bin("prism").unwrap();
    cmd.arg("reshape")
        .arg("--input")
        .arg(input_path.as_path())
        .arg("--output")
        .arg(output_path.as_path())
        .arg("--format")
        .arg("long")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Wave names"));
}
