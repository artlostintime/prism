// tests/spss_syntax_test.rs

use prism::config::{QualitySettings, ScaleDefinition, SurveyConfig, SurveySettings};
use prism::output::generate_spss_syntax;
use std::collections::HashMap;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_spss_syntax_basic_structure() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("test.sps");
    let csv_path = "test_data.csv";

    let mut scales = HashMap::new();
    scales.insert(
        "anxiety".to_string(),
        ScaleDefinition {
            items: vec!["Q1".to_string(), "Q2".to_string(), "Q3".to_string()],
            reverse_scored: None,
        },
    );

    let config = SurveyConfig {
        survey: SurveySettings {
            name: "Test Survey".to_string(),
            min_score: 1,
            max_score: 5,
            participant_id_column: None,
        },
        quality: None,
        scales,
        column_mappings: None,
        output: None,
        longitudinal: None,
    };

    generate_spss_syntax(csv_path, &config, output_path.to_str().unwrap()).unwrap();

    let content = fs::read_to_string(&output_path).unwrap();

    // Check for major sections
    assert!(content.contains("SECTION 1: DATA IMPORT"));
    assert!(content.contains("SECTION 2: VARIABLE LABELS"));
    assert!(content.contains("SECTION 3: VALUE LABELS"));
    assert!(content.contains("SECTION 4: MISSING VALUE DECLARATIONS"));
    assert!(content.contains("SECTION 5: REVERSE SCORING"));
    assert!(content.contains("SECTION 6: SCALE SCORE COMPUTATION"));

    // Check GET DATA command
    assert!(content.contains("GET DATA"));
    assert!(content.contains("/TYPE=TXT"));
    assert!(content.contains("/FILE='test_data.csv'"));
    assert!(content.contains("/ENCODING='UTF8'"));
    assert!(content.contains("/FIRSTCASE=2"));
}

#[test]
fn test_spss_syntax_variable_labels() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("test.sps");

    let mut scales = HashMap::new();
    scales.insert(
        "depression".to_string(),
        ScaleDefinition {
            items: vec!["PHQ1".to_string(), "PHQ2".to_string()],
            reverse_scored: None,
        },
    );

    let config = SurveyConfig {
        survey: SurveySettings {
            name: "PHQ-2 Test".to_string(),
            min_score: 0,
            max_score: 3,
            participant_id_column: Some("ID".to_string()),
        },
        quality: None,
        scales,
        column_mappings: None,
        output: None,
        longitudinal: None,
    };

    generate_spss_syntax("data.csv", &config, output_path.to_str().unwrap()).unwrap();

    let content = fs::read_to_string(&output_path).unwrap();

    // Check variable labels
    assert!(content.contains("VARIABLE LABELS"));
    assert!(content.contains("ID 'Participant ID'"));
    assert!(content.contains("PHQ1 'depression - Item 1'"));
    assert!(content.contains("PHQ2 'depression - Item 2'"));
    assert!(content.contains("depression_total 'depression Total Score"));
    assert!(content.contains("depression_mean 'depression Mean Score"));
}

#[test]
fn test_spss_syntax_value_labels_5_point() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("test.sps");

    let mut scales = HashMap::new();
    scales.insert(
        "scale1".to_string(),
        ScaleDefinition {
            items: vec!["Q1".to_string()],
            reverse_scored: None,
        },
    );

    let config = SurveyConfig {
        survey: SurveySettings {
            name: "5-Point Test".to_string(),
            min_score: 1,
            max_score: 5,
            participant_id_column: None,
        },
        quality: None,
        scales,
        column_mappings: None,
        output: None,
        longitudinal: None,
    };

    generate_spss_syntax("data.csv", &config, output_path.to_str().unwrap()).unwrap();

    let content = fs::read_to_string(&output_path).unwrap();

    // Check 5-point Likert labels
    assert!(content.contains("VALUE LABELS"));
    assert!(content.contains("1 'Strongly Disagree'"));
    assert!(content.contains("2 'Disagree'"));
    assert!(content.contains("3 'Neutral'"));
    assert!(content.contains("4 'Agree'"));
    assert!(content.contains("5 'Strongly Agree'"));
}

#[test]
fn test_spss_syntax_value_labels_7_point() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("test.sps");

    let mut scales = HashMap::new();
    scales.insert(
        "scale1".to_string(),
        ScaleDefinition {
            items: vec!["Q1".to_string()],
            reverse_scored: None,
        },
    );

    let config = SurveyConfig {
        survey: SurveySettings {
            name: "7-Point Test".to_string(),
            min_score: 1,
            max_score: 7,
            participant_id_column: None,
        },
        quality: None,
        scales,
        column_mappings: None,
        output: None,
        longitudinal: None,
    };

    generate_spss_syntax("data.csv", &config, output_path.to_str().unwrap()).unwrap();

    let content = fs::read_to_string(&output_path).unwrap();

    // Check 7-point Likert labels
    assert!(content.contains("1 'Strongly Disagree'"));
    assert!(content.contains("4 'Neutral'"));
    assert!(content.contains("7 'Strongly Agree'"));
    assert!(content.contains("3 'Somewhat Disagree'"));
    assert!(content.contains("5 'Somewhat Agree'"));
}

#[test]
fn test_spss_syntax_reverse_scoring() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("test.sps");

    let mut scales = HashMap::new();
    scales.insert(
        "optimism".to_string(),
        ScaleDefinition {
            items: vec!["OPT1".to_string(), "OPT2".to_string(), "OPT3".to_string()],
            reverse_scored: Some(vec!["OPT2".to_string()]),
        },
    );

    let config = SurveyConfig {
        survey: SurveySettings {
            name: "Optimism Test".to_string(),
            min_score: 1,
            max_score: 7,
            participant_id_column: None,
        },
        quality: None,
        scales,
        column_mappings: None,
        output: None,
        longitudinal: None,
    };

    generate_spss_syntax("data.csv", &config, output_path.to_str().unwrap()).unwrap();

    let content = fs::read_to_string(&output_path).unwrap();

    // Check reverse scoring section
    assert!(content.contains("SECTION 5: REVERSE SCORING"));
    assert!(content.contains("Reverse scoring for optimism scale"));
    assert!(content.contains("RECODE OPT2 (1 = 7) (7 = 1)"));
    assert!(content.contains("(2 = 6)"));
    assert!(content.contains("(3 = 5)"));
    assert!(content.contains("(4 = 4)"));

    // Check variable label indicates reverse scoring
    assert!(content.contains("OPT2 'optimism - Item 2 (reverse scored)'"));
}

#[test]
fn test_spss_syntax_no_reverse_scoring() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("test.sps");

    let mut scales = HashMap::new();
    scales.insert(
        "scale1".to_string(),
        ScaleDefinition {
            items: vec!["Q1".to_string(), "Q2".to_string()],
            reverse_scored: None,
        },
    );

    let config = SurveyConfig {
        survey: SurveySettings {
            name: "No Reverse Test".to_string(),
            min_score: 1,
            max_score: 5,
            participant_id_column: None,
        },
        quality: None,
        scales,
        column_mappings: None,
        output: None,
        longitudinal: None,
    };

    generate_spss_syntax("data.csv", &config, output_path.to_str().unwrap()).unwrap();

    let content = fs::read_to_string(&output_path).unwrap();

    // Should note no reverse scoring
    assert!(content.contains("No reverse-scored items in this survey"));
}

#[test]
fn test_spss_syntax_compute_statements() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("test.sps");

    let mut scales = HashMap::new();
    scales.insert(
        "stress".to_string(),
        ScaleDefinition {
            items: vec!["PSS1".to_string(), "PSS2".to_string(), "PSS3".to_string()],
            reverse_scored: None,
        },
    );

    let config = SurveyConfig {
        survey: SurveySettings {
            name: "PSS Test".to_string(),
            min_score: 0,
            max_score: 4,
            participant_id_column: None,
        },
        quality: None,
        scales,
        column_mappings: None,
        output: None,
        longitudinal: None,
    };

    generate_spss_syntax("data.csv", &config, output_path.to_str().unwrap()).unwrap();

    let content = fs::read_to_string(&output_path).unwrap();

    // Check COMPUTE statements
    assert!(content.contains("SECTION 6: SCALE SCORE COMPUTATION"));
    assert!(content.contains("Compute stress scale (3 items)"));
    assert!(content.contains("COMPUTE stress_total = PSS1 + PSS2 + PSS3"));
    assert!(content.contains("COMPUTE stress_mean = MEAN(PSS1, PSS2, PSS3)"));
    assert!(content.contains("EXECUTE."));
}

#[test]
fn test_spss_syntax_missing_values() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("test.sps");

    let mut scales = HashMap::new();
    scales.insert(
        "scale1".to_string(),
        ScaleDefinition {
            items: vec!["Q1".to_string(), "Q2".to_string()],
            reverse_scored: None,
        },
    );

    let config = SurveyConfig {
        survey: SurveySettings {
            name: "Missing Test".to_string(),
            min_score: 1,
            max_score: 7,
            participant_id_column: None,
        },
        quality: None,
        scales,
        column_mappings: None,
        output: None,
        longitudinal: None,
    };

    generate_spss_syntax("data.csv", &config, output_path.to_str().unwrap()).unwrap();

    let content = fs::read_to_string(&output_path).unwrap();

    // Check missing value handling
    assert!(content.contains("SECTION 4: MISSING VALUE DECLARATIONS"));
    assert!(content.contains("IF (Q1 < 1 OR Q1 > 7) Q1 = $SYSMIS"));
    assert!(content.contains("IF (Q2 < 1 OR Q2 > 7) Q2 = $SYSMIS"));
}

#[test]
fn test_spss_syntax_quality_flags() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("test.sps");

    let mut scales = HashMap::new();
    scales.insert(
        "scale1".to_string(),
        ScaleDefinition {
            items: vec!["Q1".to_string()],
            reverse_scored: None,
        },
    );

    let config = SurveyConfig {
        survey: SurveySettings {
            name: "Quality Test".to_string(),
            min_score: 1,
            max_score: 5,
            participant_id_column: None,
        },
        quality: Some(QualitySettings {
            max_missing_percent: 20.0,
            flag_straightlining: true,
            min_response_variance: Some(0.5),
            max_response_time: None,
            min_response_time: None,
            careless_responding_threshold: None,
        }),
        scales,
        column_mappings: None,
        output: None,
        longitudinal: None,
    };

    generate_spss_syntax("data.csv", &config, output_path.to_str().unwrap()).unwrap();

    let content = fs::read_to_string(&output_path).unwrap();

    // Check quality variables
    assert!(content.contains("quality_flag A50"));
    assert!(content.contains("quality_reason A255"));
    assert!(content.contains("quality_flag 'Data Quality Flag'"));
    assert!(content.contains("quality_reason 'Quality Issue Description'"));
    assert!(content.contains("VALUE LABELS quality_flag"));
    assert!(content.contains("'OK' 'Passed all quality checks'"));
    assert!(content.contains("'FLAGGED' 'Quality issues detected'"));
}

#[test]
fn test_spss_syntax_reliability_example() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("test.sps");

    let mut scales = HashMap::new();
    scales.insert(
        "anxiety".to_string(),
        ScaleDefinition {
            items: vec!["GAD1".to_string(), "GAD2".to_string(), "GAD3".to_string()],
            reverse_scored: None,
        },
    );

    let config = SurveyConfig {
        survey: SurveySettings {
            name: "GAD Test".to_string(),
            min_score: 0,
            max_score: 3,
            participant_id_column: None,
        },
        quality: None,
        scales,
        column_mappings: None,
        output: None,
        longitudinal: None,
    };

    generate_spss_syntax("data.csv", &config, output_path.to_str().unwrap()).unwrap();

    let content = fs::read_to_string(&output_path).unwrap();

    // Check reliability analysis example at end
    assert!(content.contains("Reliability analysis example:"));
    assert!(content.contains("RELIABILITY"));
    assert!(content.contains("/VARIABLES="));
    assert!(content.contains("GAD1 GAD2 GAD3"));
    assert!(content.contains("/SCALE('anxiety') ALL"));
    assert!(content.contains("/MODEL=ALPHA"));
}

#[test]
fn test_spss_syntax_descriptives_example() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("test.sps");

    let mut scales = HashMap::new();
    scales.insert(
        "scale1".to_string(),
        ScaleDefinition {
            items: vec!["Q1".to_string()],
            reverse_scored: None,
        },
    );

    let config = SurveyConfig {
        survey: SurveySettings {
            name: "Descriptives Test".to_string(),
            min_score: 1,
            max_score: 5,
            participant_id_column: None,
        },
        quality: None,
        scales,
        column_mappings: None,
        output: None,
        longitudinal: None,
    };

    generate_spss_syntax("data.csv", &config, output_path.to_str().unwrap()).unwrap();

    let content = fs::read_to_string(&output_path).unwrap();

    // Check descriptives example
    assert!(content.contains("To verify data:"));
    assert!(content.contains("DESCRIPTIVES VARIABLES="));
    assert!(content.contains("scale1_mean"));
    assert!(content.contains("/STATISTICS=MEAN STDDEV MIN MAX"));
}

#[test]
fn test_spss_syntax_multiple_scales() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("test.sps");

    let mut scales = HashMap::new();
    scales.insert(
        "anxiety".to_string(),
        ScaleDefinition {
            items: vec!["GAD1".to_string(), "GAD2".to_string()],
            reverse_scored: None,
        },
    );
    scales.insert(
        "depression".to_string(),
        ScaleDefinition {
            items: vec!["PHQ1".to_string(), "PHQ2".to_string()],
            reverse_scored: Some(vec!["PHQ2".to_string()]),
        },
    );

    let config = SurveyConfig {
        survey: SurveySettings {
            name: "Multi-Scale Test".to_string(),
            min_score: 0,
            max_score: 3,
            participant_id_column: None,
        },
        quality: None,
        scales,
        column_mappings: None,
        output: None,
        longitudinal: None,
    };

    generate_spss_syntax("data.csv", &config, output_path.to_str().unwrap()).unwrap();

    let content = fs::read_to_string(&output_path).unwrap();

    // Check both scales are present
    assert!(content.contains("anxiety"));
    assert!(content.contains("depression"));
    assert!(content.contains("GAD1"));
    assert!(content.contains("GAD2"));
    assert!(content.contains("PHQ1"));
    assert!(content.contains("PHQ2"));
    assert!(content.contains("anxiety_total"));
    assert!(content.contains("depression_total"));
    assert!(content.contains("Reverse scoring for depression scale"));
    assert!(content.contains("RECODE PHQ2"));
}
