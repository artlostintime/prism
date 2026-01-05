// src/processor.rs
use crate::config::{ScaleDefinition, SurveyConfig};
use crate::errors::{ProcessingError, Result};
use crate::quality::{
    check_alternating_pattern, check_block_pattern, check_diagonal_pattern, check_low_variance,
    check_missing_data, check_straightlining,
};
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
            let final_val = if has_reverse.is_some_and(|rev| rev.contains(item_name)) {
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

/// Parameters for quality check processing
pub struct QualityCheckParams<'a> {
    pub scale_name: &'a str,
    pub scale_result: &'a ScaleResult,
    pub missing_count: usize,
    pub total_items: usize,
    pub participant_id: &'a str,
    pub config: &'a SurveyConfig,
    pub quality_flags: &'a mut Vec<String>,
    pub quality_issues: &'a mut Vec<QualityIssue>,
}

/// Process all quality checks for a scale
pub fn process_quality_checks(params: QualityCheckParams) {
    // Check missing data
    check_missing_data(
        params.scale_name,
        params.missing_count,
        params.total_items,
        params.participant_id,
        params.config,
        params.quality_flags,
        params.quality_issues,
    );

    // Check straightlining
    if params.scale_result.valid_items > 0 {
        check_straightlining(
            params.scale_name,
            &params.scale_result.item_values,
            params.participant_id,
            params.config,
            params.quality_flags,
            params.quality_issues,
        );

        // Check low variance
        check_low_variance(
            params.scale_name,
            &params.scale_result.item_values,
            params.participant_id,
            params.config,
            params.quality_flags,
            params.quality_issues,
        );

        // Check for careless response patterns
        check_diagonal_pattern(
            params.scale_name,
            &params.scale_result.item_values,
            params.participant_id,
            params.quality_flags,
            params.quality_issues,
        );

        check_alternating_pattern(
            params.scale_name,
            &params.scale_result.item_values,
            params.participant_id,
            params.quality_flags,
            params.quality_issues,
        );

        check_block_pattern(
            params.scale_name,
            &params.scale_result.item_values,
            params.participant_id,
            params.quality_flags,
            params.quality_issues,
        );
    }
}

/// Get participant ID from record
#[inline] // Inline small hot function
pub fn get_participant_id(
    record: &csv::StringRecord,
    header_map: &HashMap<String, usize>,
    config: &SurveyConfig,
    row_number: usize,
) -> String {
    // Try configured column first
    if let Some(id_column) = &config.survey.participant_id_column {
        if let Some(&idx) = header_map.get(id_column) {
            if let Some(value) = record.get(idx) {
                if !value.is_empty() {
                    return value.to_string();
                }
            }
        }
    }

    // Try common column names
    let common_names = [
        "ResponseId",
        "id",
        "ID",
        "participant_id",
        "ParticipantID",
        "response_id",
        "SubjectID",
        "subject_id",
    ];

    for name in &common_names {
        if let Some(&idx) = header_map.get(*name) {
            if let Some(value) = record.get(idx) {
                if !value.is_empty() {
                    return value.to_string();
                }
            }
        }
    }

    // If still not found, use row number as fallback
    format!("Row_{}", row_number)
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
        let id = get_participant_id(&record, &header_map, &config, 1);
        assert_eq!(id, "P001");
    }
}
