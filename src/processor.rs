// src/processor.rs
use crate::config::{ScaleDefinition, SurveyConfig};
use crate::errors::{ProcessingError, Result};
use crate::quality::{check_low_variance, check_missing_data, check_straightlining};
use crate::types::{QualityIssue, ScaleResult};
use std::collections::HashMap;

/// Process a single scale for a participant
///
/// # Arguments
/// * `scale_def` - Scale definition from configuration
/// * `record` - CSV record for this participant
/// * `header_map` - Map from column names to indices
/// * `config` - Survey configuration
///
/// # Returns
/// Tuple of (ScaleResult, missing_count)
///
/// # Example
/// ```no_run
/// use prism::processor::process_scale;
/// // let result = process_scale(&scale_def, &record, &header_map, &config)?;
/// ```
pub fn process_scale(
    scale_def: &ScaleDefinition,
    record: &csv::StringRecord,
    header_map: &HashMap<String, usize>,
    config: &SurveyConfig,
) -> Result<(ScaleResult, usize)> {
    let mut total_score = 0.0;
    let mut valid_items = 0;
    let mut item_values = Vec::with_capacity(scale_def.items.len()); // Pre-allocate capacity
    let mut missing_count = 0;
    let mut missing_items = Vec::new();
    let mut out_of_range_items = Vec::new();

    let min_score = config.survey.min_score as f64;
    let max_score = config.survey.max_score as f64;
    let score_range = max_score + min_score;

    // Pre-check if reverse scoring is needed
    let has_reverse = scale_def.reverse_scored.as_ref();

    for item_name in &scale_def.items {
        // Check for column mappings
        let mapped_name = config
            .column_mappings
            .as_ref()
            .and_then(|m| m.get(item_name))
            .unwrap_or(item_name);

        let idx = *header_map
            .get(mapped_name)
            .ok_or_else(|| ProcessingError::MissingColumn(item_name.to_string()))?; // Avoid clone

        let val_str = &record[idx];

        if let Ok(val) = val_str.parse::<f64>() {
            // Check for out-of-range values
            if val < min_score || val > max_score {
                missing_count += 1;
                out_of_range_items.push(item_name.to_string());
                continue;
            }

            // Reverse scoring if needed - use reference to avoid clone
            let final_val = if has_reverse.map_or(false, |rev| rev.contains(item_name)) {
                score_range - val
            } else {
                val
            };

            total_score += final_val;
            valid_items += 1;
            item_values.push(final_val);
        } else {
            missing_count += 1;
            missing_items.push(item_name.to_string());
        }
    }

    let mean = if valid_items > 0 {
        total_score / valid_items as f64
    } else {
        0.0
    };

    Ok((
        ScaleResult::new(
            total_score,
            mean,
            valid_items,
            item_values,
            missing_items,
            out_of_range_items,
        ),
        missing_count,
    ))
}

/// Process all quality checks for a scale
pub fn process_quality_checks(
    scale_name: &str,
    scale_result: &ScaleResult,
    missing_count: usize,
    total_items: usize,
    participant_id: &str,
    config: &SurveyConfig,
    quality_flags: &mut Vec<String>,
    quality_issues: &mut Vec<QualityIssue>,
) {
    // Check missing data
    check_missing_data(
        scale_name,
        missing_count,
        total_items,
        participant_id,
        config,
        quality_flags,
        quality_issues,
    );

    // Check straightlining
    if scale_result.valid_items > 0 {
        check_straightlining(
            scale_name,
            &scale_result.item_values,
            participant_id,
            config,
            quality_flags,
            quality_issues,
        );

        // Check low variance
        check_low_variance(
            scale_name,
            &scale_result.item_values,
            participant_id,
            config,
            quality_flags,
            quality_issues,
        );
    }
}

/// Get participant ID from record
#[inline] // Inline small hot function
pub fn get_participant_id(
    record: &csv::StringRecord,
    header_map: &HashMap<String, usize>,
    config: &SurveyConfig,
) -> String {
    let id_column = config
        .survey
        .participant_id_column
        .as_ref()
        .map(|s| s.as_str())
        .unwrap_or("ResponseId");

    header_map
        .get(id_column)
        .and_then(|&idx| record.get(idx))
        .unwrap_or("Unknown")
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_participant_id_default() {
        let record = csv::StringRecord::from(vec!["P001", "5", "4", "3"]);
        let mut header_map = HashMap::new();
        header_map.insert("ResponseId".to_string(), 0);

        let config = SurveyConfig::default();
        let id = get_participant_id(&record, &header_map, &config);
        assert_eq!(id, "P001");
    }
}
