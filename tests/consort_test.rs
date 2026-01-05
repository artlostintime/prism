// tests/consort_test.rs
use prism::output::{generate_consort_json, generate_consort_report};
use prism::types::QualityIssue;
use std::fs;

#[test]
fn test_consort_report_generation() {
    let issues = vec![
        QualityIssue::new("P001", "MissingData", "High missing data"),
        QualityIssue::new("P001", "Straightlining", "All same response"),
        QualityIssue::new("P002", "LowVariance", "Low variance"),
        QualityIssue::new("P003", "DiagonalPattern", "Diagonal pattern detected"),
        QualityIssue::new("P003", "ResponseTimeFast", "Too fast"),
    ];

    let output_path = "tests/output/consort_report.txt";
    let result = generate_consort_report(100, &issues, output_path);
    assert!(result.is_ok());

    let content = fs::read_to_string(output_path).unwrap();
    assert!(content.contains("CONSORT Participant Flow Report"));
    assert!(content.contains("n = 100"));
    assert!(content.contains("n = 3")); // 3 unique participants
    assert!(content.contains("n = 97")); // 97 analyzed
    assert!(content.contains("Missing data"));
    assert!(content.contains("Straightlining"));

    // Cleanup
    let _ = fs::remove_file(output_path);
}

#[test]
fn test_consort_json_generation() {
    let issues = vec![
        QualityIssue::new("P001", "MissingData", "High missing data"),
        QualityIssue::new("P002", "Straightlining", "All same response"),
        QualityIssue::new("P003", "DiagonalPattern", "Diagonal pattern"),
    ];

    let output_path = "tests/output/consort_data.json";
    let result = generate_consort_json(50, &issues, output_path);
    assert!(result.is_ok());

    let content = fs::read_to_string(output_path).unwrap();
    let json: serde_json::Value = serde_json::from_str(&content).unwrap();

    assert_eq!(json["study_flow"]["screened"]["n"], 50);
    assert_eq!(json["study_flow"]["excluded"]["n"], 3);
    assert_eq!(json["study_flow"]["analyzed"]["n"], 47);
    assert_eq!(json["summary"]["participants_with_issues"], 3);

    // Cleanup
    let _ = fs::remove_file(output_path);
}

#[test]
fn test_consort_no_exclusions() {
    let issues: Vec<QualityIssue> = vec![];

    let output_path = "tests/output/consort_clean.txt";
    let result = generate_consort_report(100, &issues, output_path);
    assert!(result.is_ok());

    let content = fs::read_to_string(output_path).unwrap();
    assert!(content.contains("n = 0")); // 0 excluded
    assert!(content.contains("n = 100")); // All analyzed

    // Cleanup
    let _ = fs::remove_file(output_path);
}

#[test]
fn test_consort_multiple_issues_per_participant() {
    // One participant with multiple issues should only count once
    let issues = vec![
        QualityIssue::new("P001", "MissingData", "Missing data"),
        QualityIssue::new("P001", "Straightlining", "Straightlining"),
        QualityIssue::new("P001", "LowVariance", "Low variance"),
        QualityIssue::new("P002", "DiagonalPattern", "Diagonal"),
    ];

    let output_path = "tests/output/consort_multi.txt";
    let result = generate_consort_report(100, &issues, output_path);
    assert!(result.is_ok());

    let content = fs::read_to_string(output_path).unwrap();
    assert!(content.contains("n = 2")); // 2 unique participants excluded
    assert!(content.contains("n = 98")); // 98 analyzed

    // Cleanup
    let _ = fs::remove_file(output_path);
}

#[test]
fn test_consort_exclusion_percentages() {
    let issues = vec![
        QualityIssue::new("P001", "MissingData", "Missing"),
        QualityIssue::new("P002", "MissingData", "Missing"),
        QualityIssue::new("P003", "MissingData", "Missing"),
        QualityIssue::new("P004", "MissingData", "Missing"),
        QualityIssue::new("P005", "MissingData", "Missing"),
    ];

    let output_path = "tests/output/consort_percent.txt";
    let result = generate_consort_report(100, &issues, output_path);
    assert!(result.is_ok());

    let content = fs::read_to_string(output_path).unwrap();
    assert!(content.contains("5.0%")); // 5% excluded
    assert!(content.contains("95.0%")); // 95% analyzed

    // Cleanup
    let _ = fs::remove_file(output_path);
}

#[test]
fn test_consort_json_structure() {
    let issues = vec![
        QualityIssue::new("P001", "MissingData", "Missing"),
        QualityIssue::new("P002", "Straightlining", "Straightlining"),
    ];

    let output_path = "tests/output/consort_structure.json";
    let result = generate_consort_json(20, &issues, output_path);
    assert!(result.is_ok());

    let content = fs::read_to_string(output_path).unwrap();
    let json: serde_json::Value = serde_json::from_str(&content).unwrap();

    // Check structure
    assert!(json["study_flow"]["screened"].is_object());
    assert!(json["study_flow"]["excluded"].is_object());
    assert!(json["study_flow"]["analyzed"].is_object());
    assert!(json["summary"].is_object());
    assert!(json["excluded_participants"].is_array());

    // Check excluded participants
    let excluded = json["excluded_participants"].as_array().unwrap();
    assert_eq!(excluded.len(), 2);

    // Cleanup
    let _ = fs::remove_file(output_path);
}

#[test]
fn test_consort_issue_breakdown() {
    let issues = vec![
        QualityIssue::new("P001", "MissingData", "Missing"),
        QualityIssue::new("P002", "MissingData", "Missing"),
        QualityIssue::new("P003", "Straightlining", "Straightlining"),
        QualityIssue::new("P004", "DiagonalPattern", "Diagonal"),
        QualityIssue::new("P005", "AlternatingPattern", "Alternating"),
        QualityIssue::new("P006", "BlockPattern", "Block"),
    ];

    let output_path = "tests/output/consort_breakdown.txt";
    let result = generate_consort_report(100, &issues, output_path);
    assert!(result.is_ok());

    let content = fs::read_to_string(output_path).unwrap();

    // Check all issue types are mentioned
    assert!(content.contains("Missing data"));
    assert!(content.contains("Straightlining"));
    assert!(content.contains("Diagonal pattern"));
    assert!(content.contains("Alternating pattern"));
    assert!(content.contains("Block pattern"));

    // Cleanup
    let _ = fs::remove_file(output_path);
}

#[test]
fn test_consort_semantic_inconsistency() {
    let issues = vec![
        QualityIssue::new("P001", "SemanticInconsistency", "Contradictory responses"),
        QualityIssue::new("P002", "ResponseTimeFast", "Too fast"),
        QualityIssue::new("P003", "ResponseTimeSlow", "Too slow"),
    ];

    let output_path = "tests/output/consort_semantic.txt";
    let result = generate_consort_report(50, &issues, output_path);
    assert!(result.is_ok());

    let content = fs::read_to_string(output_path).unwrap();
    assert!(content.contains("Semantic inconsistency"));
    assert!(content.contains("Response time too fast"));
    assert!(content.contains("Response time too slow"));

    // Cleanup
    let _ = fs::remove_file(output_path);
}
