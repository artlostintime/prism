// tests/semantic_inconsistency_test.rs
use prism::quality::check_semantic_inconsistency;
use prism::types::QualityIssue;

#[test]
fn test_negative_correlation_both_high() {
    // Test case: stress and well-being both high (should be flagged)
    let mut flags = Vec::new();
    let mut issues = Vec::new();

    check_semantic_inconsistency(
        "stress",
        6.5,
        "wellbeing",
        6.8,
        "P001",
        "negative",
        1.0,
        7.0,
        0.7,
        &mut flags,
        &mut issues,
    );

    assert_eq!(flags.len(), 1);
    assert!(flags[0].contains("Semantic inconsistency"));
    assert!(flags[0].contains("stress"));
    assert!(flags[0].contains("wellbeing"));
    assert!(flags[0].contains("negative"));
    assert_eq!(issues.len(), 1);
    assert_eq!(issues[0].issue_type, "SemanticInconsistency");
}

#[test]
fn test_negative_correlation_one_high_one_low() {
    // Test case: stress high, well-being low (consistent, should NOT be flagged)
    let mut flags = Vec::new();
    let mut issues = Vec::new();

    check_semantic_inconsistency(
        "stress",
        6.5,
        "wellbeing",
        2.0,
        "P002",
        "negative",
        1.0,
        7.0,
        0.7,
        &mut flags,
        &mut issues,
    );

    assert_eq!(flags.len(), 0);
    assert_eq!(issues.len(), 0);
}

#[test]
fn test_negative_correlation_both_low() {
    // Test case: both scales low (consistent, should NOT be flagged)
    let mut flags = Vec::new();
    let mut issues = Vec::new();

    check_semantic_inconsistency(
        "stress",
        2.0,
        "wellbeing",
        2.5,
        "P003",
        "negative",
        1.0,
        7.0,
        0.7,
        &mut flags,
        &mut issues,
    );

    assert_eq!(flags.len(), 0);
    assert_eq!(issues.len(), 0);
}

#[test]
fn test_positive_correlation_one_high_one_low() {
    // Test case: engagement high but satisfaction low (should be flagged)
    let mut flags = Vec::new();
    let mut issues = Vec::new();

    check_semantic_inconsistency(
        "engagement",
        6.5,
        "satisfaction",
        1.5,
        "P004",
        "positive",
        1.0,
        7.0,
        0.7,
        &mut flags,
        &mut issues,
    );

    assert_eq!(flags.len(), 1);
    assert!(flags[0].contains("Semantic inconsistency"));
    assert!(flags[0].contains("positive"));
    assert_eq!(issues.len(), 1);
}

#[test]
fn test_positive_correlation_both_high() {
    // Test case: both scales high (consistent, should NOT be flagged)
    let mut flags = Vec::new();
    let mut issues = Vec::new();

    check_semantic_inconsistency(
        "engagement",
        6.5,
        "satisfaction",
        6.2,
        "P005",
        "positive",
        1.0,
        7.0,
        0.7,
        &mut flags,
        &mut issues,
    );

    assert_eq!(flags.len(), 0);
    assert_eq!(issues.len(), 0);
}

#[test]
fn test_positive_correlation_both_low() {
    // Test case: both scales low (consistent, should NOT be flagged)
    let mut flags = Vec::new();
    let mut issues = Vec::new();

    check_semantic_inconsistency(
        "engagement",
        2.0,
        "satisfaction",
        2.5,
        "P006",
        "positive",
        1.0,
        7.0,
        0.7,
        &mut flags,
        &mut issues,
    );

    assert_eq!(flags.len(), 0);
    assert_eq!(issues.len(), 0);
}

#[test]
fn test_threshold_sensitivity() {
    // Test with different threshold (0.8) - stricter
    let mut flags = Vec::new();
    let mut issues = Vec::new();

    // Score of 6.0 on 1-7 scale is ~83% (normalized: 0.83)
    // With threshold 0.8, should be flagged
    check_semantic_inconsistency(
        "stress",
        6.0,
        "wellbeing",
        6.0,
        "P007",
        "negative",
        1.0,
        7.0,
        0.8,
        &mut flags,
        &mut issues,
    );

    assert_eq!(flags.len(), 1);
}

#[test]
fn test_threshold_not_met() {
    // Test with scores just below threshold
    let mut flags = Vec::new();
    let mut issues = Vec::new();

    // Score of 5.0 on 1-7 scale is ~67% (normalized: 0.67)
    // With threshold 0.7, should NOT be flagged
    check_semantic_inconsistency(
        "stress",
        5.0,
        "wellbeing",
        5.0,
        "P008",
        "negative",
        1.0,
        7.0,
        0.7,
        &mut flags,
        &mut issues,
    );

    assert_eq!(flags.len(), 0);
}

#[test]
fn test_different_scale_range() {
    // Test with 1-5 scale
    let mut flags = Vec::new();
    let mut issues = Vec::new();

    // Both scores high on 1-5 scale
    check_semantic_inconsistency(
        "burnout",
        4.8,
        "wellbeing",
        4.5,
        "P009",
        "negative",
        1.0,
        5.0,
        0.7,
        &mut flags,
        &mut issues,
    );

    assert_eq!(flags.len(), 1);
    assert!(flags[0].contains("4.80"));
    assert!(flags[0].contains("4.50"));
}

#[test]
fn test_participant_id_tracking() {
    // Verify participant ID is correctly tracked
    let mut flags = Vec::new();
    let mut issues = Vec::new();

    check_semantic_inconsistency(
        "stress",
        6.5,
        "wellbeing",
        6.5,
        "PARTICIPANT_123",
        "negative",
        1.0,
        7.0,
        0.7,
        &mut flags,
        &mut issues,
    );

    assert_eq!(issues.len(), 1);
    assert_eq!(issues[0].participant_id, "PARTICIPANT_123");
    assert_eq!(issues[0].issue_type, "SemanticInconsistency");
}

#[test]
fn test_invalid_correlation_type() {
    // Test with invalid correlation type (should not flag)
    let mut flags = Vec::new();
    let mut issues = Vec::new();

    check_semantic_inconsistency(
        "stress",
        6.5,
        "wellbeing",
        6.5,
        "P010",
        "invalid_type",
        1.0,
        7.0,
        0.7,
        &mut flags,
        &mut issues,
    );

    // Should not flag with invalid correlation type
    assert_eq!(flags.len(), 0);
    assert_eq!(issues.len(), 0);
}

#[test]
fn test_edge_case_exact_threshold() {
    // Test when scores are exactly at threshold
    let mut flags = Vec::new();
    let mut issues = Vec::new();

    // Threshold 0.7 on 1-7 scale = score of 5.2
    // Test with score slightly above threshold
    check_semantic_inconsistency(
        "scale1",
        5.21,
        "scale2",
        5.21,
        "P011",
        "negative",
        1.0,
        7.0,
        0.7,
        &mut flags,
        &mut issues,
    );

    assert_eq!(flags.len(), 1);
}

#[test]
fn test_positive_correlation_reverse_order() {
    // Test positive correlation with scale2 high, scale1 low
    let mut flags = Vec::new();
    let mut issues = Vec::new();

    check_semantic_inconsistency(
        "satisfaction",
        1.5,
        "engagement",
        6.5,
        "P012",
        "positive",
        1.0,
        7.0,
        0.7,
        &mut flags,
        &mut issues,
    );

    assert_eq!(flags.len(), 1);
    assert!(flags[0].contains("positive"));
}
