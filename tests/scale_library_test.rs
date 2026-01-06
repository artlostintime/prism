// tests/scale_library_test.rs
//! Tests for the pre-built scale library

use prism::scales;

#[test]
fn test_list_available_scales() {
    let scales_list = scales::list_available_scales();

    assert!(!scales_list.is_empty(), "Scale list should not be empty");
    assert!(scales_list.contains(&"PHQ-9".to_string()));
    assert!(scales_list.contains(&"GAD-7".to_string()));
    assert!(scales_list.contains(&"PSS-10".to_string()));
    assert!(scales_list.contains(&"PSS-14".to_string()));
    assert!(scales_list.contains(&"PANAS".to_string()));
    assert!(scales_list.contains(&"BDI-II".to_string()));
    assert!(scales_list.contains(&"BAI".to_string()));
    assert!(scales_list.contains(&"SWLS".to_string()));

    assert_eq!(scales_list.len(), 8, "Should have 8 pre-built scales");
}

#[test]
fn test_generate_phq9_config() {
    let config = scales::generate_scale_config("PHQ-9").unwrap();

    assert!(config.contains("PHQ-9"));
    assert!(config.contains("Patient Health Questionnaire"));
    assert!(config.contains("Kroenke"));
    assert!(config.contains("[scales.phq9_total]"));
    assert!(config.contains("PHQ1"));
    assert!(config.contains("PHQ9"));
    assert!(config.contains("min_score = 0"));
    assert!(config.contains("max_score = 3"));
}

#[test]
fn test_generate_phq9_config_case_insensitive() {
    let config1 = scales::generate_scale_config("PHQ-9").unwrap();
    let config2 = scales::generate_scale_config("phq-9").unwrap();
    let config3 = scales::generate_scale_config("PHQ9").unwrap();

    assert_eq!(config1, config2);
    assert_eq!(config1, config3);
}

#[test]
fn test_generate_gad7_config() {
    let config = scales::generate_scale_config("GAD-7").unwrap();

    assert!(config.contains("GAD-7"));
    assert!(config.contains("Generalized Anxiety Disorder"));
    assert!(config.contains("Spitzer"));
    assert!(config.contains("[scales.gad7_total]"));
    assert!(config.contains("GAD1"));
    assert!(config.contains("GAD7"));
    assert!(config.contains("min_score = 0"));
    assert!(config.contains("max_score = 3"));
}

#[test]
fn test_generate_pss10_config() {
    let config = scales::generate_scale_config("PSS-10").unwrap();

    assert!(config.contains("PSS-10"));
    assert!(config.contains("Perceived Stress Scale"));
    assert!(config.contains("Cohen"));
    assert!(config.contains("[scales.pss10_total]"));
    assert!(config.contains("PSS1"));
    assert!(config.contains("PSS10"));
    assert!(config.contains("reverse_scored = [\"PSS4\", \"PSS5\", \"PSS7\", \"PSS8\"]"));
    assert!(config.contains("min_score = 0"));
    assert!(config.contains("max_score = 4"));
}

#[test]
fn test_generate_pss14_config() {
    let config = scales::generate_scale_config("PSS-14").unwrap();

    assert!(config.contains("PSS-14"));
    assert!(config.contains("14-item"));
    assert!(config.contains("[scales.pss14_total]"));
    assert!(config.contains("PSS1"));
    assert!(config.contains("PSS14"));
    assert!(config.contains("reverse_scored"));
    assert!(config.contains("PSS4"));
    assert!(config.contains("PSS13"));
}

#[test]
fn test_generate_panas_config() {
    let config = scales::generate_scale_config("PANAS").unwrap();

    assert!(config.contains("PANAS"));
    assert!(config.contains("Positive and Negative Affect"));
    assert!(config.contains("Watson"));
    assert!(config.contains("[scales.panas_positive]"));
    assert!(config.contains("[scales.panas_negative]"));
    assert!(config.contains("PANAS_Interested"));
    assert!(config.contains("PANAS_Distressed"));
    assert!(config.contains("min_score = 1"));
    assert!(config.contains("max_score = 5"));
}

#[test]
fn test_generate_bdi_ii_config() {
    let config = scales::generate_scale_config("BDI-II").unwrap();

    assert!(config.contains("BDI-II"));
    assert!(config.contains("Beck Depression Inventory"));
    assert!(config.contains("Beck"));
    assert!(config.contains("[scales.bdi_ii_total]"));
    assert!(config.contains("BDI1"));
    assert!(config.contains("BDI21"));
    assert!(config.contains("min_score = 0"));
    assert!(config.contains("max_score = 3"));
}

#[test]
fn test_generate_bai_config() {
    let config = scales::generate_scale_config("BAI").unwrap();

    assert!(config.contains("BAI"));
    assert!(config.contains("Beck Anxiety Inventory"));
    assert!(config.contains("[scales.bai_total]"));
    assert!(config.contains("BAI1"));
    assert!(config.contains("BAI21"));
}

#[test]
fn test_generate_swls_config() {
    let config = scales::generate_scale_config("SWLS").unwrap();

    assert!(config.contains("SWLS"));
    assert!(config.contains("Satisfaction With Life"));
    assert!(config.contains("Diener"));
    assert!(config.contains("[scales.swls_total]"));
    assert!(config.contains("SWLS1"));
    assert!(config.contains("SWLS5"));
    assert!(config.contains("min_score = 1"));
    assert!(config.contains("max_score = 7"));
}

#[test]
fn test_generate_unknown_scale() {
    let result = scales::generate_scale_config("UNKNOWN");

    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Unknown scale"));
}

#[test]
fn test_get_phq9_metadata() {
    let metadata = scales::get_scale_metadata("PHQ-9").unwrap();

    assert_eq!(metadata.name, "PHQ-9");
    assert_eq!(metadata.full_name, "Patient Health Questionnaire-9");
    assert_eq!(metadata.num_items, 9);
    assert_eq!(metadata.min_score, 0);
    assert_eq!(metadata.max_score, 3);
    assert!(metadata.citation.contains("Kroenke"));
    assert!(metadata.normative_data.is_some());

    let norm_data = metadata.normative_data.unwrap();
    assert_eq!(norm_data.clinical_cutoff, Some(10.0));
    assert_eq!(norm_data.severity_ranges.len(), 5);
}

#[test]
fn test_get_gad7_metadata() {
    let metadata = scales::get_scale_metadata("GAD-7").unwrap();

    assert_eq!(metadata.name, "GAD-7");
    assert_eq!(metadata.num_items, 7);
    assert_eq!(metadata.min_score, 0);
    assert_eq!(metadata.max_score, 3);
    assert!(metadata.citation.contains("Spitzer"));
}

#[test]
fn test_get_pss10_metadata() {
    let metadata = scales::get_scale_metadata("PSS-10").unwrap();

    assert_eq!(metadata.name, "PSS-10");
    assert_eq!(metadata.num_items, 10);
    assert_eq!(metadata.min_score, 0);
    assert_eq!(metadata.max_score, 4);
    assert!(metadata.description.contains("reverse-scored"));
}

#[test]
fn test_get_panas_metadata() {
    let metadata = scales::get_scale_metadata("PANAS").unwrap();

    assert_eq!(metadata.name, "PANAS");
    assert_eq!(metadata.num_items, 20);
    assert_eq!(metadata.min_score, 1);
    assert_eq!(metadata.max_score, 5);
    assert!(metadata.description.contains("subscales"));
}

#[test]
fn test_get_bdi_ii_metadata() {
    let metadata = scales::get_scale_metadata("BDI-II").unwrap();

    assert_eq!(metadata.name, "BDI-II");
    assert_eq!(metadata.num_items, 21);
    assert!(metadata.citation.contains("Beck"));
}

#[test]
fn test_get_bai_metadata() {
    let metadata = scales::get_scale_metadata("BAI").unwrap();

    assert_eq!(metadata.name, "BAI");
    assert_eq!(metadata.num_items, 21);
    assert!(metadata.description.contains("somatic"));
}

#[test]
fn test_get_swls_metadata() {
    let metadata = scales::get_scale_metadata("SWLS").unwrap();

    assert_eq!(metadata.name, "SWLS");
    assert_eq!(metadata.num_items, 5);
    assert_eq!(metadata.min_score, 1);
    assert_eq!(metadata.max_score, 7);
    assert!(metadata.description.contains("life satisfaction"));
}

#[test]
fn test_metadata_has_interpretation() {
    for scale_name in scales::list_available_scales() {
        let metadata = scales::get_scale_metadata(&scale_name).unwrap();
        assert!(
            !metadata.interpretation.is_empty(),
            "{} should have interpretation",
            scale_name
        );
    }
}

#[test]
fn test_metadata_has_citation() {
    for scale_name in scales::list_available_scales() {
        let metadata = scales::get_scale_metadata(&scale_name).unwrap();
        assert!(
            !metadata.citation.is_empty(),
            "{} should have citation",
            scale_name
        );
    }
}

#[test]
fn test_all_configs_are_valid_toml() {
    for scale_name in scales::list_available_scales() {
        let config = scales::generate_scale_config(&scale_name).unwrap();
        let parse_result: Result<toml::Value, _> = toml::from_str(&config);
        assert!(
            parse_result.is_ok(),
            "{} config should be valid TOML: {:?}",
            scale_name,
            parse_result
        );
    }
}

#[test]
fn test_configs_have_required_sections() {
    for scale_name in scales::list_available_scales() {
        let config = scales::generate_scale_config(&scale_name).unwrap();

        assert!(
            config.contains("[survey]"),
            "{} should have [survey] section",
            scale_name
        );
        assert!(
            config.contains("[quality]"),
            "{} should have [quality] section",
            scale_name
        );
        assert!(
            config.contains("[output]") || config.contains("decimal_places"),
            "{} should have output configuration",
            scale_name
        );
        assert!(
            config.contains("[scales."),
            "{} should have at least one scale definition",
            scale_name
        );
    }
}
