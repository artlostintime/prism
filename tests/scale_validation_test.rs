// Scale Library Validation Tests
// Tests to ensure pre-built scales generate valid configurations

use prism::scales::{generate_scale_config, get_scale_metadata, list_available_scales};

#[test]
fn test_all_scales_available() {
    let scales = list_available_scales();
    assert!(!scales.is_empty(), "Should have pre-built scales");

    // Verify expected scales exist
    let expected = vec![
        "PHQ-9", "GAD-7", "PSS-10", "PSS-14", "PANAS", "BDI-II", "BAI", "SWLS",
    ];
    for scale_name in expected {
        assert!(
            scales.contains(&scale_name.to_string()),
            "Missing expected scale: {}",
            scale_name
        );
    }
}

#[test]
fn test_phq9_scale_config() {
    let config = generate_scale_config("PHQ-9").expect("PHQ-9 should exist");

    // Verify scale structure
    assert!(
        config.contains("[scales.phq9_total]"),
        "Should have PHQ9 scale section"
    );
    assert!(config.contains("items = ["), "Should have items list");

    // Verify all 9 items
    for i in 1..=9 {
        assert!(
            config.contains(&format!("\"PHQ{}\"", i)),
            "Should have item PHQ{}",
            i
        );
    }

    // PHQ-9 has no reverse items
    assert!(
        config.contains("reverse_scored = []"),
        "PHQ-9 should have no reverse scored items"
    );
}

#[test]
fn test_gad7_scale_config() {
    let config = generate_scale_config("GAD-7").expect("GAD-7 should exist");

    // Verify scale structure
    assert!(
        config.contains("[scales.gad7_total]"),
        "Should have GAD7 scale section"
    );

    // Verify all 7 items
    for i in 1..=7 {
        assert!(
            config.contains(&format!("\"GAD{}\"", i)),
            "Should have item GAD{}",
            i
        );
    }

    // GAD-7 has no reverse items
    assert!(
        config.contains("reverse_scored = []"),
        "GAD-7 should have no reverse scored items"
    );
}

#[test]
fn test_pss10_scale_config() {
    let config = generate_scale_config("PSS-10").expect("PSS-10 should exist");

    // Verify scale structure
    assert!(
        config.contains("[scales.pss10_total]"),
        "Should have PSS10 scale section"
    );

    // Verify all 10 items
    for i in 1..=10 {
        assert!(
            config.contains(&format!("\"PSS{}\"", i)),
            "Should have item PSS{}",
            i
        );
    }

    // PSS-10 has reverse scored items (4, 5, 7, 8)
    assert!(
        config.contains("reverse_scored = ["),
        "PSS-10 should have reverse scored items"
    );
    assert!(config.contains("\"PSS4\""), "Should reverse PSS4");
    assert!(config.contains("\"PSS5\""), "Should reverse PSS5");
    assert!(config.contains("\"PSS7\""), "Should reverse PSS7");
    assert!(config.contains("\"PSS8\""), "Should reverse PSS8");
}

#[test]
fn test_panas_scale_config() {
    let config = generate_scale_config("PANAS").expect("PANAS should exist");

    // PANAS has two subscales
    assert!(
        config.contains("[scales.panas_positive]"),
        "Should have positive affect subscale"
    );
    assert!(
        config.contains("[scales.panas_negative]"),
        "Should have negative affect subscale"
    );

    // Each subscale should have 10 items
    let positive_items = [
        "Interested",
        "Excited",
        "Strong",
        "Enthusiastic",
        "Proud",
        "Alert",
        "Inspired",
        "Determined",
        "Attentive",
        "Active",
    ];
    let negative_items = [
        "Distressed",
        "Upset",
        "Guilty",
        "Scared",
        "Hostile",
        "Irritable",
        "Ashamed",
        "Nervous",
        "Jittery",
        "Afraid",
    ];

    for item in &positive_items {
        assert!(
            config.contains(&format!("\"PANAS_{}\"", item)),
            "Should have positive item: {}",
            item
        );
    }

    for item in &negative_items {
        assert!(
            config.contains(&format!("\"PANAS_{}\"", item)),
            "Should have negative item: {}",
            item
        );
    }
}

#[test]
fn test_scale_info_contains_metadata() {
    let scales = list_available_scales();

    for scale_name in &scales {
        let info = get_scale_metadata(scale_name)
            .unwrap_or_else(|_| panic!("Should get info for {}", scale_name));

        // Every scale should have basic metadata
        assert!(
            !info.full_name.is_empty(),
            "Should have name for {}",
            scale_name
        );
        assert!(
            info.num_items > 0,
            "Should have item count for {}",
            scale_name
        );
        assert!(
            !info.description.is_empty(),
            "Should have description for {}",
            scale_name
        );

        // Most scales should have citations (except maybe custom ones)
        if scale_name != "Custom" {
            assert!(
                !info.citation.is_empty(),
                "Should have citation for {}",
                scale_name
            );
        }
    }
}

#[test]
fn test_scale_config_is_valid_toml() {
    let scales = list_available_scales();

    for scale_name in &scales {
        let config = generate_scale_config(scale_name)
            .unwrap_or_else(|_| panic!("Should get config for {}", scale_name));

        // Try to parse as TOML
        let parsed: Result<toml::Value, _> = toml::from_str(&config);
        assert!(
            parsed.is_ok(),
            "Config for {} should be valid TOML: {:?}",
            scale_name,
            parsed.err()
        );

        // Verify has scales section
        let toml_value = parsed.unwrap();
        assert!(
            toml_value.get("scales").is_some(),
            "Config for {} should have 'scales' section",
            scale_name
        );
    }
}

#[test]
fn test_scale_items_format_consistency() {
    let scales = list_available_scales();

    for scale_name in &scales {
        let config = generate_scale_config(scale_name)
            .unwrap_or_else(|_| panic!("Should get config for {}", scale_name));
        let parsed: toml::Value = toml::from_str(&config).expect("Should parse TOML");

        if let Some(scales_section) = parsed.get("scales").and_then(|v| v.as_table()) {
            for (subscale_name, subscale_data) in scales_section {
                let subscale_table = subscale_data
                    .as_table()
                    .unwrap_or_else(|| panic!("Subscale {} should be a table", subscale_name));

                // Verify items field exists and is an array
                assert!(
                    subscale_table.get("items").is_some(),
                    "Subscale {} should have 'items' field",
                    subscale_name
                );

                let items = subscale_table
                    .get("items")
                    .and_then(|v| v.as_array())
                    .unwrap_or_else(|| panic!("Items for {} should be an array", subscale_name));

                assert!(
                    !items.is_empty(),
                    "Subscale {} should have at least one item",
                    subscale_name
                );

                // All items should be strings
                for item in items {
                    assert!(
                        item.is_str(),
                        "All items in {} should be strings, found: {:?}",
                        subscale_name,
                        item
                    );
                }

                // If reverse_items exists, it should be an array
                if let Some(reverse_items) = subscale_table.get("reverse_items") {
                    assert!(
                        reverse_items.is_array(),
                        "reverse_items for {} should be an array",
                        subscale_name
                    );
                }
            }
        }
    }
}

#[test]
fn test_scale_response_scale_format() {
    let config = generate_scale_config("PHQ-9").expect("Should get PHQ-9");
    let parsed: toml::Value = toml::from_str(&config).expect("Should parse TOML");

    // Check response scale format if present
    if let Some(scales) = parsed.get("scales").and_then(|v| v.as_table()) {
        for (_, subscale_data) in scales {
            if let Some(table) = subscale_data.as_table() {
                if let Some(response_scale) = table.get("response_scale") {
                    let scale_table = response_scale
                        .as_table()
                        .expect("response_scale should be a table");

                    // Should have min and max
                    assert!(
                        scale_table.get("min").is_some(),
                        "response_scale should have min"
                    );
                    assert!(
                        scale_table.get("max").is_some(),
                        "response_scale should have max"
                    );

                    // Min should be less than max
                    let min = scale_table.get("min").unwrap().as_integer().unwrap();
                    let max = scale_table.get("max").unwrap().as_integer().unwrap();
                    assert!(min < max, "Min should be less than max");
                }
            }
        }
    }
}
