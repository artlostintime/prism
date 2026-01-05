// src/longitudinal.rs
//! Longitudinal data processing for multi-wave/repeated measures studies.
//!
//! This module provides functionality for:
//! - Merging data from multiple time points
//! - Converting between wide and long formats
//! - Calculating reliable change indices (RCI)
//! - Time-based quality checks

use crate::errors::{ProcessingError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Configuration for longitudinal data processing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LongitudinalConfig {
    /// Column name containing participant IDs
    pub id_column: String,

    /// Column name containing time/wave information (for long format)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_column: Option<String>,

    /// List of wave identifiers (e.g., ["T1", "T2", "T3"])
    pub waves: Vec<String>,

    /// Data format: "wide" or "long"
    #[serde(default = "default_format")]
    pub format: DataFormat,
}

fn default_format() -> DataFormat {
    DataFormat::Wide
}

/// Data format for longitudinal data
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DataFormat {
    /// Wide format: one row per participant, multiple columns per variable
    Wide,
    /// Long format: multiple rows per participant, one column per variable
    Long,
}

/// Parameters for merging multiple waves of data
#[derive(Debug, Clone)]
pub struct MergeParams {
    /// Path to each wave's data file
    pub wave_files: Vec<(String, String)>, // (wave_name, file_path)

    /// Column name to use for matching participants across waves
    pub id_column: String,

    /// Output file path
    pub output_path: String,

    /// Whether to include only participants present in all waves
    pub inner_join: bool,
}

/// Parameters for reshaping data between wide and long formats
#[derive(Debug, Clone)]
pub struct ReshapeParams {
    /// Input file path
    pub input_path: String,

    /// Output file path
    pub output_path: String,

    /// Target format
    pub target_format: DataFormat,

    /// ID column name
    pub id_column: String,

    /// Time/wave column name (used in long format)
    pub time_column: String,

    /// Variable names to reshape (if empty, reshape all except id and time)
    pub variables: Vec<String>,

    /// Wave names (for wide-to-long conversion)
    pub waves: Vec<String>,
}

/// Parameters for calculating Reliable Change Index
#[derive(Debug, Clone)]
pub struct RCIParams {
    /// Baseline (T1) data file
    pub baseline_path: String,

    /// Follow-up (T2) data file
    pub followup_path: String,

    /// Scale/variable name to analyze
    pub scale_name: String,

    /// ID column name
    pub id_column: String,

    /// Test-retest reliability coefficient
    pub reliability: f64,

    /// Standard deviation at baseline (if known, otherwise calculated)
    pub baseline_sd: Option<f64>,

    /// Output file path
    pub output_path: String,
}

/// Result of RCI calculation for one participant
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RCIResult {
    /// Participant ID
    pub id: String,

    /// Baseline score
    pub baseline: f64,

    /// Follow-up score
    pub followup: f64,

    /// Raw difference (followup - baseline)
    pub difference: f64,

    /// Reliable Change Index value
    pub rci: f64,

    /// Whether change is statistically reliable
    pub is_reliable: bool,

    /// Direction of change: "improved", "worsened", or "no change"
    pub direction: String,
}

/// Merge multiple waves of data into a single wide-format dataset
///
/// # Arguments
/// * `params` - Merge parameters including file paths and join type
///
/// # Returns
/// Number of participants in the merged dataset
///
/// # Example
/// ```no_run
/// use prism::longitudinal::{MergeParams, merge_waves};
///
/// let params = MergeParams {
///     wave_files: vec![
///         ("T1".to_string(), "data_t1.csv".to_string()),
///         ("T2".to_string(), "data_t2.csv".to_string()),
///     ],
///     id_column: "ParticipantID".to_string(),
///     output_path: "merged.csv".to_string(),
///     inner_join: false,
/// };
///
/// let n = merge_waves(params)?;
/// println!("Merged {} participants", n);
/// # Ok::<(), prism::errors::ProcessingError>(())
/// ```
pub fn merge_waves(params: MergeParams) -> Result<usize> {
    use csv::{Reader, Writer};
    use std::fs::File;

    if params.wave_files.is_empty() {
        return Err(ProcessingError::Config(
            "At least one wave file must be specified".to_string(),
        ));
    }

    // Step 1: Read all waves and store in HashMaps
    let mut wave_data: HashMap<String, HashMap<String, csv::StringRecord>> = HashMap::new();
    let mut all_headers: HashMap<String, Vec<String>> = HashMap::new();
    let mut id_column_indices: HashMap<String, usize> = HashMap::new();
    let mut all_participant_ids: std::collections::HashSet<String> =
        std::collections::HashSet::new();

    for (wave_name, file_path) in &params.wave_files {
        let mut reader = Reader::from_path(file_path)
            .map_err(|e| ProcessingError::Config(format!("Failed to read {}: {}", file_path, e)))?;

        let headers = reader
            .headers()
            .map_err(|e| ProcessingError::Config(format!("Failed to read headers: {}", e)))?
            .clone();

        // Find ID column index
        let id_column_index = headers
            .iter()
            .position(|h| h == params.id_column)
            .ok_or_else(|| {
                ProcessingError::MissingColumn(format!(
                    "ID column '{}' not found in {}",
                    params.id_column, file_path
                ))
            })?;

        // Store ID column index for this wave
        id_column_indices.insert(wave_name.clone(), id_column_index);

        // Store headers for this wave (excluding ID column as it won't be renamed)
        let wave_headers: Vec<String> = headers
            .iter()
            .enumerate()
            .filter_map(|(i, h)| {
                if i != id_column_index {
                    Some(h.to_string())
                } else {
                    None
                }
            })
            .collect();
        all_headers.insert(wave_name.clone(), wave_headers);

        // Read all records for this wave
        let mut wave_records: HashMap<String, csv::StringRecord> = HashMap::new();
        for result in reader.records() {
            let record = result?;
            let id = record
                .get(id_column_index)
                .ok_or_else(|| {
                    ProcessingError::Config(format!("Missing ID in record: {:?}", record))
                })?
                .to_string();

            all_participant_ids.insert(id.clone());
            wave_records.insert(id, record);
        }

        wave_data.insert(wave_name.clone(), wave_records);
    }

    // Step 2: Determine which participants to include
    let participants_to_include: Vec<String> = if params.inner_join {
        // Inner join: only participants present in ALL waves
        all_participant_ids
            .into_iter()
            .filter(|id| {
                params.wave_files.iter().all(|(wave, _)| {
                    wave_data
                        .get(wave)
                        .map(|records| records.contains_key(id))
                        .unwrap_or(false)
                })
            })
            .collect()
    } else {
        // Full outer join: include all participants
        all_participant_ids.into_iter().collect()
    };

    // Step 3: Build output headers
    let mut output_headers = vec![params.id_column.clone()];
    for (wave_name, _) in &params.wave_files {
        if let Some(headers) = all_headers.get(wave_name) {
            for header in headers {
                output_headers.push(format!("{}_{}", header, wave_name));
            }
        }
    }

    // Step 4: Write merged data
    let output_file = File::create(&params.output_path)
        .map_err(|e| ProcessingError::Config(format!("Failed to create output file: {}", e)))?;
    let mut writer = Writer::from_writer(output_file);

    // Write header
    writer.write_record(&output_headers)?;

    // Write data rows
    for participant_id in &participants_to_include {
        let mut output_row = vec![participant_id.clone()];

        for (wave_name, _) in &params.wave_files {
            let id_col_idx = *id_column_indices.get(wave_name).unwrap_or(&0);

            if let Some(wave_records) = wave_data.get(wave_name) {
                if let Some(record) = wave_records.get(participant_id) {
                    // Add all fields except ID column
                    for (i, field) in record.iter().enumerate() {
                        if i != id_col_idx {
                            output_row.push(field.to_string());
                        }
                    }
                } else {
                    // Participant not in this wave - add empty values
                    let expected_cols = all_headers.get(wave_name).map(|h| h.len()).unwrap_or(0);
                    for _ in 0..expected_cols {
                        output_row.push(String::new());
                    }
                }
            }
        }

        writer.write_record(&output_row)?;
    }

    writer.flush()?;
    Ok(participants_to_include.len())
}

/// Convert data between wide and long formats
///
/// # Arguments
/// * `params` - Reshape parameters including format direction and variable names
///
/// # Returns
/// Number of rows in the reshaped dataset
pub fn reshape_data(params: ReshapeParams) -> Result<usize> {
    match params.target_format {
        DataFormat::Long => wide_to_long(params),
        DataFormat::Wide => long_to_wide(params),
    }
}

/// Convert wide format to long format
/// Wide: ID, anxiety_T1, anxiety_T2, depression_T1, depression_T2
/// Long: ID, Time, anxiety, depression
fn wide_to_long(params: ReshapeParams) -> Result<usize> {
    use csv::{Reader, Writer};
    use std::fs::File;

    let mut reader = Reader::from_path(&params.input_path)?;
    let headers = reader.headers()?.clone();

    // Find ID column index
    let id_col_idx = headers
        .iter()
        .position(|h| h == params.id_column)
        .ok_or_else(|| ProcessingError::MissingColumn(params.id_column.clone()))?;

    // Parse wave-specific columns
    // Expected format: variable_wave (e.g., "anxiety_T1", "depression_T2")
    let mut variable_map: HashMap<String, HashMap<String, usize>> = HashMap::new();

    for (col_idx, header) in headers.iter().enumerate() {
        if col_idx == id_col_idx {
            continue;
        }

        // Try to split by underscore to find wave suffix
        if let Some(last_underscore) = header.rfind('_') {
            let var_name = &header[..last_underscore];
            let wave_name = &header[last_underscore + 1..];

            // Check if this wave is in our expected waves
            if params.waves.iter().any(|w| w == wave_name) {
                variable_map
                    .entry(var_name.to_string())
                    .or_default()
                    .insert(wave_name.to_string(), col_idx);
            }
        }
    }

    if variable_map.is_empty() {
        return Err(ProcessingError::Config(
            "No wave-specific columns found with expected format (variable_wave)".to_string(),
        ));
    }

    // Get variable names (sorted for consistent output)
    let mut variable_names: Vec<String> = if params.variables.is_empty() {
        variable_map.keys().cloned().collect()
    } else {
        params.variables.clone()
    };
    variable_names.sort();

    // Create output file
    let output_file = File::create(&params.output_path)?;
    let mut writer = Writer::from_writer(output_file);

    // Write header: ID, Time, var1, var2, ...
    let mut output_headers = vec![params.id_column.clone(), params.time_column.clone()];
    output_headers.extend(variable_names.iter().cloned());
    writer.write_record(&output_headers)?;

    // Process each record
    let mut row_count = 0;
    for result in reader.records() {
        let record = result?;
        let id = record.get(id_col_idx).unwrap_or("");

        // Create one row per wave
        for wave in &params.waves {
            let mut output_row = vec![id.to_string(), wave.clone()];

            // Add each variable's value for this wave
            for var_name in &variable_names {
                if let Some(wave_cols) = variable_map.get(var_name) {
                    if let Some(&col_idx) = wave_cols.get(wave) {
                        output_row.push(record.get(col_idx).unwrap_or("").to_string());
                    } else {
                        output_row.push(String::new());
                    }
                } else {
                    output_row.push(String::new());
                }
            }

            writer.write_record(&output_row)?;
            row_count += 1;
        }
    }

    writer.flush()?;
    Ok(row_count)
}

/// Convert long format to wide format
/// Long: ID, Time, anxiety, depression
/// Wide: ID, anxiety_T1, anxiety_T2, depression_T1, depression_T2
fn long_to_wide(params: ReshapeParams) -> Result<usize> {
    use csv::{Reader, Writer};
    use std::fs::File;

    let mut reader = Reader::from_path(&params.input_path)?;
    let headers = reader.headers()?.clone();

    // Find ID and time column indices
    let id_col_idx = headers
        .iter()
        .position(|h| h == params.id_column)
        .ok_or_else(|| ProcessingError::MissingColumn(params.id_column.clone()))?;

    let time_col_idx = headers
        .iter()
        .position(|h| h == params.time_column)
        .ok_or_else(|| ProcessingError::MissingColumn(params.time_column.clone()))?;

    // Get variable column indices (all except ID and Time)
    let variable_indices: Vec<(String, usize)> = headers
        .iter()
        .enumerate()
        .filter_map(|(idx, header)| {
            if idx != id_col_idx && idx != time_col_idx {
                Some((header.to_string(), idx))
            } else {
                None
            }
        })
        .collect();

    if variable_indices.is_empty() {
        return Err(ProcessingError::Config(
            "No variable columns found (all columns are ID or Time)".to_string(),
        ));
    }

    // Read all data and organize by participant
    let mut participant_data: HashMap<String, HashMap<String, csv::StringRecord>> = HashMap::new();
    let mut all_waves: std::collections::HashSet<String> = std::collections::HashSet::new();

    for result in reader.records() {
        let record = result?;
        let id = record.get(id_col_idx).unwrap_or("").to_string();
        let wave = record.get(time_col_idx).unwrap_or("").to_string();

        all_waves.insert(wave.clone());
        participant_data.entry(id).or_default().insert(wave, record);
    }

    // Sort waves for consistent column order
    let mut waves: Vec<String> = if params.waves.is_empty() {
        all_waves.into_iter().collect()
    } else {
        params.waves.clone()
    };
    waves.sort();

    // Build output headers: ID, var1_wave1, var1_wave2, var2_wave1, var2_wave2, ...
    let mut output_headers = vec![params.id_column.clone()];
    for (var_name, _) in &variable_indices {
        for wave in &waves {
            output_headers.push(format!("{}_{}", var_name, wave));
        }
    }

    // Write output
    let output_file = File::create(&params.output_path)?;
    let mut writer = Writer::from_writer(output_file);
    writer.write_record(&output_headers)?;

    // Write data rows
    let mut row_count = 0;
    for (id, wave_records) in &participant_data {
        let mut output_row = vec![id.clone()];

        for (_var_name, var_idx) in &variable_indices {
            for wave in &waves {
                if let Some(record) = wave_records.get(wave) {
                    output_row.push(record.get(*var_idx).unwrap_or("").to_string());
                } else {
                    output_row.push(String::new());
                }
            }
        }

        writer.write_record(&output_row)?;
        row_count += 1;
    }

    writer.flush()?;
    Ok(row_count)
}

/// Calculate Reliable Change Index (RCI) for longitudinal data
///
/// RCI = (X2 - X1) / SE_diff
/// where SE_diff = SD1 * sqrt(2 * (1 - r))
///
/// RCI > 1.96 indicates statistically reliable change (p < .05)
///
/// # Arguments
/// * `params` - RCI calculation parameters including reliability and file paths
///
/// # Returns
/// Vector of RCI results for all matched participants
pub fn calculate_rci(params: RCIParams) -> Result<Vec<RCIResult>> {
    use csv::Reader;

    // Validate parameters
    if params.reliability < 0.0 || params.reliability > 1.0 {
        return Err(ProcessingError::Config(
            "Reliability coefficient must be between 0 and 1".to_string(),
        ));
    }

    // Read baseline data
    let mut baseline_reader = Reader::from_path(&params.baseline_path)?;
    let baseline_headers = baseline_reader.headers()?.clone();

    let baseline_id_idx = baseline_headers
        .iter()
        .position(|h| h == params.id_column)
        .ok_or_else(|| ProcessingError::MissingColumn(params.id_column.clone()))?;

    let baseline_scale_idx = baseline_headers
        .iter()
        .position(|h| h == params.scale_name)
        .ok_or_else(|| ProcessingError::MissingColumn(params.scale_name.clone()))?;

    // Store baseline scores
    let mut baseline_scores: HashMap<String, f64> = HashMap::new();
    let mut baseline_values: Vec<f64> = Vec::new();

    for result in baseline_reader.records() {
        let record = result?;
        let id = record.get(baseline_id_idx).unwrap_or("").to_string();
        let score_str = record.get(baseline_scale_idx).unwrap_or("");

        if let Ok(score) = score_str.parse::<f64>() {
            baseline_scores.insert(id, score);
            baseline_values.push(score);
        }
    }

    if baseline_scores.is_empty() {
        return Err(ProcessingError::Config(
            "No valid baseline scores found".to_string(),
        ));
    }

    // Calculate baseline SD if not provided
    let baseline_sd = if let Some(sd) = params.baseline_sd {
        sd
    } else {
        calculate_sd(&baseline_values)
    };

    // Calculate SE_diff = SD * sqrt(2 * (1 - reliability))
    let se_diff = baseline_sd * (2.0 * (1.0 - params.reliability)).sqrt();

    if se_diff == 0.0 {
        return Err(ProcessingError::Config(
            "Standard error of difference is zero - cannot calculate RCI".to_string(),
        ));
    }

    // Read follow-up data
    let mut followup_reader = Reader::from_path(&params.followup_path)?;
    let followup_headers = followup_reader.headers()?.clone();

    let followup_id_idx = followup_headers
        .iter()
        .position(|h| h == params.id_column)
        .ok_or_else(|| ProcessingError::MissingColumn(params.id_column.clone()))?;

    let followup_scale_idx = followup_headers
        .iter()
        .position(|h| h == params.scale_name)
        .ok_or_else(|| ProcessingError::MissingColumn(params.scale_name.clone()))?;

    // Calculate RCI for each matched participant
    let mut results = Vec::new();

    for result in followup_reader.records() {
        let record = result?;
        let id = record.get(followup_id_idx).unwrap_or("").to_string();
        let followup_str = record.get(followup_scale_idx).unwrap_or("");

        if let Ok(followup_score) = followup_str.parse::<f64>() {
            if let Some(&baseline_score) = baseline_scores.get(&id) {
                let difference = followup_score - baseline_score;
                let rci = difference / se_diff;
                let is_reliable = rci.abs() >= 1.96; // p < .05 (two-tailed)

                let direction = if !is_reliable {
                    "no change".to_string()
                } else if rci > 0.0 {
                    "increased".to_string()
                } else {
                    "decreased".to_string()
                };

                results.push(RCIResult {
                    id,
                    baseline: baseline_score,
                    followup: followup_score,
                    difference,
                    rci,
                    is_reliable,
                    direction,
                });
            }
        }
    }

    // Write results to output file
    if !results.is_empty() {
        write_rci_results(&results, &params.output_path)?;
    }

    Ok(results)
}

/// Helper function to calculate standard deviation using sample variance
/// Uses Bessel's correction (n-1) for unbiased estimation
fn calculate_sd(values: &[f64]) -> f64 {
    let n = values.len();
    if n < 2 {
        return 0.0; // Cannot calculate sample SD with n < 2
    }

    let mean = values.iter().sum::<f64>() / n as f64;

    // Use sample variance (n-1) for unbiased estimation
    // This matches the variance calculation in stats.rs and quality.rs
    let variance = values
        .iter()
        .map(|&x| {
            let diff = x - mean;
            diff * diff
        })
        .sum::<f64>()
        / (n - 1) as f64; // ✅ FIXED: Sample variance (Bessel's correction)

    variance.sqrt()
}

/// Write RCI results to CSV file
fn write_rci_results(results: &[RCIResult], output_path: &str) -> Result<()> {
    use csv::Writer;
    use std::fs::File;

    let output_file = File::create(output_path)?;
    let mut writer = Writer::from_writer(output_file);

    // Write header
    writer.write_record([
        "ParticipantID",
        "Baseline",
        "Followup",
        "Difference",
        "RCI",
        "IsReliable",
        "Direction",
    ])?;

    // Write data
    for result in results {
        writer.write_record([
            &result.id,
            &format!("{:.2}", result.baseline),
            &format!("{:.2}", result.followup),
            &format!("{:.2}", result.difference),
            &format!("{:.2}", result.rci),
            &result.is_reliable.to_string(),
            &result.direction,
        ])?;
    }

    writer.flush()?;
    Ok(())
}

/// Validate longitudinal configuration
pub fn validate_longitudinal_config(config: &LongitudinalConfig) -> Result<()> {
    if config.id_column.is_empty() {
        return Err(ProcessingError::Config(
            "Longitudinal config must specify id_column".to_string(),
        ));
    }

    if config.waves.is_empty() {
        return Err(ProcessingError::Config(
            "Longitudinal config must specify at least one wave".to_string(),
        ));
    }

    if config.format == DataFormat::Long && config.time_column.is_none() {
        return Err(ProcessingError::Config(
            "Long format requires time_column to be specified".to_string(),
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_longitudinal_config() {
        let valid_config = LongitudinalConfig {
            id_column: "ID".to_string(),
            time_column: Some("Wave".to_string()),
            waves: vec!["T1".to_string(), "T2".to_string()],
            format: DataFormat::Wide,
        };
        assert!(validate_longitudinal_config(&valid_config).is_ok());

        let invalid_config = LongitudinalConfig {
            id_column: "".to_string(),
            time_column: None,
            waves: vec![],
            format: DataFormat::Wide,
        };
        assert!(validate_longitudinal_config(&invalid_config).is_err());
    }

    #[test]
    fn test_calculate_sd_uses_sample_variance() {
        // Test data: [10, 12, 14, 16, 18]
        // Mean = 14.0, Sample variance = 10.0, Sample SD = √10.0 ≈ 3.162
        let values = vec![10.0, 12.0, 14.0, 16.0, 18.0];
        let sd = calculate_sd(&values);

        // Should use sample variance (n-1), not population variance (n)
        assert!(
            (sd - 3.162277660168).abs() < 1e-9,
            "SD should be ~3.162, got {}",
            sd
        );
    }

    #[test]
    fn test_calculate_sd_consistency_with_stats_module() {
        use crate::stats::Stats;

        // Verify calculate_sd produces same result as stats.rs
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];

        let sd_longitudinal = calculate_sd(&values);
        let stats = Stats::calculate(&values);

        // Should match exactly (both use sample variance with n-1)
        assert!(
            (sd_longitudinal - stats.sd).abs() < 1e-10,
            "SD mismatch: longitudinal={}, stats.rs={}",
            sd_longitudinal,
            stats.sd
        );
    }

    #[test]
    fn test_data_format_serde() {
        let wide = DataFormat::Wide;
        let serialized = serde_json::to_string(&wide).unwrap();
        assert_eq!(serialized, "\"wide\"");

        let long = DataFormat::Long;
        let serialized = serde_json::to_string(&long).unwrap();
        assert_eq!(serialized, "\"long\"");
    }
}
