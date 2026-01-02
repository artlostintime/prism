// src/validation.rs
use crate::config::SurveyConfig;
use crate::errors::{ProcessingError, Result};
use log::{info, warn};
use std::collections::HashSet;

/// Validate configuration against CSV headers with enhanced checks
pub fn validate_config(config: &SurveyConfig, headers: &[String]) -> Result<()> {
    // Check scale definitions
    if config.scales.is_empty() {
        return Err(ProcessingError::ConfigError(
            "No scales defined in config".to_string(),
        ));
    }

    info!("Validating {} scales", config.scales.len());

    // Check that all scale items exist in CSV headers
    for (scale_name, scale_def) in &config.scales {
        if scale_def.items.is_empty() {
            return Err(ProcessingError::ConfigError(format!(
                "Scale '{}' has no items defined",
                scale_name
            )));
        }

        // Warn if scale has too few items (low reliability expected)
        if scale_def.items.len() < 3 {
            warn!(
                "Scale '{}' has only {} items - reliability may be low",
                scale_name,
                scale_def.items.len()
            );
        }

        // Check for duplicate items in scale
        let unique_items: HashSet<_> = scale_def.items.iter().collect();
        if unique_items.len() != scale_def.items.len() {
            return Err(ProcessingError::ConfigError(format!(
                "Scale '{}' contains duplicate items",
                scale_name
            )));
        }

        for item in &scale_def.items {
            // Check with column mapping
            let mapped_item = config
                .column_mappings
                .as_ref()
                .and_then(|m| m.get(item))
                .unwrap_or(item);

            if !headers.contains(mapped_item) {
                // Provide suggestions for typos
                let suggestions = find_similar_headers(mapped_item, headers);
                let suggestion_text = if !suggestions.is_empty() {
                    format!(" Did you mean: {}?", suggestions.join(", "))
                } else {
                    String::new()
                };

                return Err(ProcessingError::ConfigError(format!(
                    "Item '{}' from scale '{}' not found in CSV headers.{}",
                    item, scale_name, suggestion_text
                )));
            }
        }

        // Check reverse-scored items are subset of items
        if let Some(reversed) = &scale_def.reverse_scored {
            if reversed.is_empty() {
                warn!(
                    "Scale '{}' has empty reverse_scored array - consider removing",
                    scale_name
                );
            }

            for rev_item in reversed {
                if !scale_def.items.contains(rev_item) {
                    return Err(ProcessingError::ConfigError(format!(
                        "Reverse-scored item '{}' in scale '{}' not in items list",
                        rev_item, scale_name
                    )));
                }
            }
        } else {
            // No reverse-scored items might be suspicious for some scales
            info!(
                "Scale '{}' has no reverse-scored items - verify if intentional",
                scale_name
            );
        }
    }

    // Validate score ranges
    if config.survey.min_score >= config.survey.max_score {
        return Err(ProcessingError::ConfigError(format!(
            "min_score ({}) must be less than max_score ({})",
            config.survey.min_score, config.survey.max_score
        )));
    }

    // Validate quality settings if present
    if let Some(quality) = &config.quality {
        if quality.max_missing_percent < 0.0 || quality.max_missing_percent > 1.0 {
            return Err(ProcessingError::ConfigError(
                "max_missing_percent must be between 0.0 and 1.0".to_string(),
            ));
        }

        if quality.max_missing_percent > 0.3 {
            warn!(
                "max_missing_percent is {:.1}% - this is quite high",
                quality.max_missing_percent * 100.0
            );
        }

        if let Some(min_var) = quality.min_response_variance {
            if min_var < 0.0 {
                return Err(ProcessingError::ConfigError(
                    "min_response_variance cannot be negative".to_string(),
                ));
            }
        }

        if let Some(max_time) = quality.max_response_time {
            if let Some(min_time) = quality.min_response_time {
                if min_time >= max_time {
                    return Err(ProcessingError::ConfigError(
                        "min_response_time must be less than max_response_time".to_string(),
                    ));
                }
            }
        }
    }

    // Validate output settings if present
    if let Some(output) = &config.output {
        if output.decimal_places > 10 {
            warn!(
                "decimal_places is {} - this is unusually high",
                output.decimal_places
            );
        }
    }

    // Check participant ID column exists
    if let Some(id_col) = &config.survey.participant_id_column {
        if !headers.contains(id_col) {
            let suggestions = find_similar_headers(id_col, headers);
            let suggestion_text = if !suggestions.is_empty() {
                format!(" Did you mean: {}?", suggestions.join(", "))
            } else {
                String::new()
            };

            return Err(ProcessingError::ConfigError(format!(
                "Participant ID column '{}' not found in CSV headers.{}",
                id_col, suggestion_text
            )));
        }
    }

    info!("✓ Configuration validation passed");
    Ok(())
}

/// Find similar header names for suggestions (using string similarity)
fn find_similar_headers(target: &str, headers: &[String]) -> Vec<String> {
    let mut similarities: Vec<(String, f64)> = headers
        .iter()
        .map(|h| {
            let similarity = strsim::jaro_winkler(target, h);
            (h.clone(), similarity)
        })
        .collect();

    similarities.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    similarities
        .into_iter()
        .take(3)
        .filter(|(_, sim)| *sim > 0.7) // Only suggest if reasonably similar
        .map(|(name, _)| name)
        .collect()
}

/// Validate a batch file containing list of input files
pub fn validate_batch_file(batch_path: &str) -> Result<Vec<String>> {
    use std::fs;

    let content = fs::read_to_string(batch_path)
        .map_err(|e| ProcessingError::ConfigError(format!("Failed to read batch file: {}", e)))?;

    let files: Vec<String> = content
        .lines()
        .filter(|line| !line.trim().is_empty() && !line.trim().starts_with('#'))
        .map(|line| line.trim().to_string())
        .collect();

    if files.is_empty() {
        return Err(ProcessingError::ConfigError(
            "Batch file contains no valid file paths".to_string(),
        ));
    }

    // Check that all files exist
    for file in &files {
        if !std::path::Path::new(file).exists() {
            return Err(ProcessingError::ConfigError(format!(
                "File '{}' from batch list does not exist",
                file
            )));
        }
    }

    info!("✓ Batch file validated: {} files found", files.len());
    Ok(files)
}

/// Generate a sample configuration template
pub fn generate_config_template() -> String {
    r#"[survey]
name = "My Survey Study"
min_score = 1
max_score = 7
participant_id_column = "ResponseId"  # Optional, defaults to first column

# Optional: Map messy column names to simpler names
[column_mappings]
"Q1_Emotional_Exhaustion_Item1" = "Q1"
"Q2_Emotional_Exhaustion_Item2" = "Q2"

[output]
decimal_places = 2
date_format = "%Y-%m-%d"
include_item_scores = false

[quality]
max_missing_percent = 0.10
flag_straightlining = true
min_response_variance = 0.5  # Optional: Detect low variance
# max_response_time = 300  # Optional: Flag slow responses (seconds)
# min_response_time = 30   # Optional: Flag fast responses (seconds)

[scales.burnout]
items = ["Q1", "Q2", "Q3", "Q4", "Q5"]
reverse_scored = ["Q3", "Q5"]

[scales.engagement]
items = ["Q6", "Q7", "Q8", "Q9"]
reverse_scored = []
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_similar_headers() {
        let headers = vec![
            "ResponseID".to_string(),
            "ResponseId".to_string(),
            "Participant_ID".to_string(),
            "Q1".to_string(),
        ];

        let suggestions = find_similar_headers("ResponseId", &headers);
        assert!(!suggestions.is_empty());
        assert!(suggestions.contains(&"ResponseId".to_string()));
    }

    #[test]
    fn test_generate_template() {
        let template = generate_config_template();
        assert!(template.contains("[survey]"));
        assert!(template.contains("[scales."));
    }
}
