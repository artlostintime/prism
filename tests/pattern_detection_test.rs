// tests/pattern_detection_test.rs
use prism::config::{QualitySettings, SurveyConfig, SurveySettings};
use prism::quality::{
    check_alternating_pattern, check_block_pattern, check_diagonal_pattern, check_response_time,
};
use std::collections::HashMap;

fn create_test_config() -> SurveyConfig {
    SurveyConfig {
        survey: SurveySettings {
            name: "Test Survey".to_string(),
            min_score: 1,
            max_score: 5,
            participant_id_column: Some("ID".to_string()),
        },
        quality: Some(QualitySettings {
            max_missing_percent: 0.2,
            flag_straightlining: true,
            min_response_variance: Some(0.5),
            max_response_time: Some(300.0),
            min_response_time: Some(30.0),
            careless_responding_threshold: Some(0.5),
        }),
        scales: HashMap::new(),
        column_mappings: None,
        output: None,
        longitudinal: None,
    }
}

#[test]
fn test_diagonal_pattern_ascending() {
    let config = create_test_config();
    let mut quality_flags = Vec::new();
    let mut quality_issues = Vec::new();

    // Ascending diagonal: 1,2,3,4,5
    let values = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    check_diagonal_pattern(
        "test_scale",
        &values,
        "P001",
        &mut quality_flags,
        &mut quality_issues,
    );

    assert_eq!(quality_issues.len(), 1);
    assert_eq!(quality_issues[0].issue_type, "DiagonalPattern");
    assert!(quality_issues[0].details.contains("ascending"));
}

#[test]
fn test_diagonal_pattern_descending() {
    let config = create_test_config();
    let mut quality_flags = Vec::new();
    let mut quality_issues = Vec::new();

    // Descending diagonal: 5,4,3,2,1
    let values = vec![5.0, 4.0, 3.0, 2.0, 1.0];
    check_diagonal_pattern(
        "test_scale",
        &values,
        "P002",
        &mut quality_flags,
        &mut quality_issues,
    );

    assert_eq!(quality_issues.len(), 1);
    assert_eq!(quality_issues[0].issue_type, "DiagonalPattern");
    assert!(quality_issues[0].details.contains("descending"));
}

#[test]
fn test_no_diagonal_pattern() {
    let config = create_test_config();
    let mut quality_flags = Vec::new();
    let mut quality_issues = Vec::new();

    // Not a diagonal pattern
    let values = vec![2.0, 3.0, 2.0, 4.0, 3.0];
    check_diagonal_pattern(
        "test_scale",
        &values,
        "P003",
        &mut quality_flags,
        &mut quality_issues,
    );

    assert_eq!(quality_issues.len(), 0);
}

#[test]
fn test_alternating_pattern() {
    let config = create_test_config();
    let mut quality_flags = Vec::new();
    let mut quality_issues = Vec::new();

    // Alternating: 1,5,1,5,1,5
    let values = vec![1.0, 5.0, 1.0, 5.0, 1.0, 5.0];
    check_alternating_pattern(
        "test_scale",
        &values,
        "P004",
        &mut quality_flags,
        &mut quality_issues,
    );

    assert_eq!(quality_issues.len(), 1);
    assert_eq!(quality_issues[0].issue_type, "AlternatingPattern");
    assert!(quality_issues[0].details.contains("Alternating pattern"));
}

#[test]
fn test_no_alternating_pattern() {
    let config = create_test_config();
    let mut quality_flags = Vec::new();
    let mut quality_issues = Vec::new();

    // More than 2 unique values
    let values = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    check_alternating_pattern(
        "test_scale",
        &values,
        "P005",
        &mut quality_flags,
        &mut quality_issues,
    );

    assert_eq!(quality_issues.len(), 0);
}

#[test]
fn test_block_pattern() {
    let config = create_test_config();
    let mut quality_flags = Vec::new();
    let mut quality_issues = Vec::new();

    // Block pattern: 1,1,1,5,5,5
    let values = vec![1.0, 1.0, 1.0, 5.0, 5.0, 5.0];
    check_block_pattern(
        "test_scale",
        &values,
        "P006",
        &mut quality_flags,
        &mut quality_issues,
    );

    assert_eq!(quality_issues.len(), 1);
    assert_eq!(quality_issues[0].issue_type, "BlockPattern");
    assert!(quality_issues[0].details.contains("Block pattern"));
}

#[test]
fn test_no_block_pattern_varied() {
    let config = create_test_config();
    let mut quality_flags = Vec::new();
    let mut quality_issues = Vec::new();

    // Not a block pattern - varied responses
    let values = vec![1.0, 2.0, 3.0, 2.0, 4.0, 5.0];
    check_block_pattern(
        "test_scale",
        &values,
        "P007",
        &mut quality_flags,
        &mut quality_issues,
    );

    assert_eq!(quality_issues.len(), 0);
}

#[test]
fn test_no_block_pattern_same_value() {
    let config = create_test_config();
    let mut quality_flags = Vec::new();
    let mut quality_issues = Vec::new();

    // Both halves are the same - not a block pattern (this is straightlining)
    let values = vec![3.0, 3.0, 3.0, 3.0, 3.0, 3.0];
    check_block_pattern(
        "test_scale",
        &values,
        "P008",
        &mut quality_flags,
        &mut quality_issues,
    );

    assert_eq!(quality_issues.len(), 0);
}

#[test]
fn test_response_time_too_fast() {
    let config = create_test_config();
    let mut quality_flags = Vec::new();
    let mut quality_issues = Vec::new();

    // 15 seconds - too fast (min is 30)
    check_response_time(
        15.0,
        "P009",
        &config,
        &mut quality_flags,
        &mut quality_issues,
    );

    assert_eq!(quality_issues.len(), 1);
    assert_eq!(quality_issues[0].issue_type, "FastResponse");
    assert!(quality_issues[0].details.contains("15"));
}

#[test]
fn test_response_time_too_slow() {
    let config = create_test_config();
    let mut quality_flags = Vec::new();
    let mut quality_issues = Vec::new();

    // 500 seconds - too slow (max is 300)
    check_response_time(
        500.0,
        "P010",
        &config,
        &mut quality_flags,
        &mut quality_issues,
    );

    assert_eq!(quality_issues.len(), 1);
    assert_eq!(quality_issues[0].issue_type, "SlowResponse");
    assert!(quality_issues[0].details.contains("500"));
}

#[test]
fn test_response_time_acceptable() {
    let config = create_test_config();
    let mut quality_flags = Vec::new();
    let mut quality_issues = Vec::new();

    // 120 seconds - acceptable (between 30 and 300)
    check_response_time(
        120.0,
        "P011",
        &config,
        &mut quality_flags,
        &mut quality_issues,
    );

    assert_eq!(quality_issues.len(), 0);
}

#[test]
fn test_short_sequence_no_diagonal() {
    let config = create_test_config();
    let mut quality_flags = Vec::new();
    let mut quality_issues = Vec::new();

    // Too short for pattern detection (< 4 items)
    let values = vec![1.0, 2.0, 3.0];
    check_diagonal_pattern(
        "test_scale",
        &values,
        "P012",
        &mut quality_flags,
        &mut quality_issues,
    );

    assert_eq!(quality_issues.len(), 0);
}

#[test]
fn test_short_sequence_no_alternating() {
    let config = create_test_config();
    let mut quality_flags = Vec::new();
    let mut quality_issues = Vec::new();

    // Too short for alternating pattern (< 4 items)
    let values = vec![1.0, 5.0, 1.0];
    check_alternating_pattern(
        "test_scale",
        &values,
        "P013",
        &mut quality_flags,
        &mut quality_issues,
    );

    assert_eq!(quality_issues.len(), 0);
}

#[test]
fn test_short_sequence_no_block() {
    let config = create_test_config();
    let mut quality_flags = Vec::new();
    let mut quality_issues = Vec::new();

    // Too short for block pattern (< 6 items)
    let values = vec![1.0, 1.0, 5.0, 5.0];
    check_block_pattern(
        "test_scale",
        &values,
        "P014",
        &mut quality_flags,
        &mut quality_issues,
    );

    assert_eq!(quality_issues.len(), 0);
}
