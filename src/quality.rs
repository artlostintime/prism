// src/quality.rs
use crate::config::SurveyConfig;
use crate::types::{MissingPercent, QualityIssue};

/// Constants for quality checks
const FLOAT_EPSILON: f64 = 1e-10;

/// Check for missing data issues
///
/// # Arguments
/// * `scale_name` - Name of the scale being checked
/// * `missing_count` - Number of missing items
/// * `total_items` - Total number of items in scale
/// * `participant_id` - ID of the participant
/// * `config` - Survey configuration
/// * `quality_flags` - Mutable vector to add flags to
/// * `quality_issues` - Mutable vector to add issues to
pub fn check_missing_data(
    scale_name: &str,
    missing_count: usize,
    total_items: usize,
    participant_id: &str,
    config: &SurveyConfig,
    quality_flags: &mut Vec<String>,
    quality_issues: &mut Vec<QualityIssue>,
) {
    let missing_percent = MissingPercent::new(missing_count as f64 / total_items as f64);
    
    if let Some(quality_settings) = &config.quality {
        if missing_percent.get() > quality_settings.max_missing_percent {
            let issue = format!(
                "High missing data: {} ({:.1}% missing)",
                scale_name,
                missing_percent.as_percentage()
            );
            quality_flags.push(issue.clone());
            quality_issues.push(QualityIssue::new(participant_id, "MissingData", issue));
        }
    }
}

/// Check for straightlining (giving the same response to all items)
///
/// # Arguments
/// * `scale_name` - Name of the scale being checked
/// * `item_values` - Vector of item response values
/// * `participant_id` - ID of the participant
/// * `config` - Survey configuration
/// * `quality_flags` - Mutable vector to add flags to
/// * `quality_issues` - Mutable vector to add issues to
pub fn check_straightlining(
    scale_name: &str,
    item_values: &[f64],
    participant_id: &str,
    config: &SurveyConfig,
    quality_flags: &mut Vec<String>,
    quality_issues: &mut Vec<QualityIssue>,
) {
    if !config
        .quality
        .as_ref()
        .map_or(true, |q| q.flag_straightlining)
    {
        return;
    }

    if item_values.len() > 1 {
        let first = item_values[0];
        if item_values
            .iter()
            .all(|&x| (x - first).abs() < FLOAT_EPSILON)
        {
            let issue = format!("Straightlining: {}", scale_name);
            quality_flags.push(issue.clone());
            quality_issues.push(QualityIssue::new(participant_id, "Straightlining", issue));
        }
    }
}

/// Check for low response variance
///
/// # Arguments
/// * `scale_name` - Name of the scale being checked
/// * `item_values` - Vector of item response values
/// * `participant_id` - ID of the participant
/// * `config` - Survey configuration
/// * `quality_flags` - Mutable vector to add flags to
/// * `quality_issues` - Mutable vector to add issues to
pub fn check_low_variance(
    scale_name: &str,
    item_values: &[f64],
    participant_id: &str,
    config: &SurveyConfig,
    quality_flags: &mut Vec<String>,
    quality_issues: &mut Vec<QualityIssue>,
) {
    if item_values.len() < 2 {
        return;
    }

    if let Some(quality_settings) = &config.quality {
        if let Some(min_variance) = quality_settings.min_response_variance {
            // Calculate variance inline to avoid extra function call
            let n = item_values.len() as f64;
            let mean = item_values.iter().sum::<f64>() / n;
            let variance = item_values
                .iter()
                .map(|&x| {
                    let diff = x - mean;
                    diff * diff // Faster than powi(2)
                })
                .sum::<f64>()
                / (n - 1.0);

            if variance < min_variance {
                let issue = format!(
                    "Low variance: {} (variance={:.3})",
                    scale_name, variance
                );
                quality_flags.push(issue.clone());
                quality_issues.push(QualityIssue::new(participant_id, "LowVariance", issue));
            }
        }
    }
}

/// Check if response time is suspiciously fast (if timing data available)
///
/// # Arguments
/// * `response_time` - Response time in seconds
/// * `participant_id` - ID of the participant
/// * `config` - Survey configuration
/// * `quality_flags` - Mutable vector to add flags to
/// * `quality_issues` - Mutable vector to add issues to
pub fn check_response_time(
    response_time: f64,
    participant_id: &str,
    config: &SurveyConfig,
    quality_flags: &mut Vec<String>,
    quality_issues: &mut Vec<QualityIssue>,
) {
    if let Some(quality_settings) = &config.quality {
        if let Some(max_time) = quality_settings.max_response_time {
            if response_time > max_time {
                let issue = format!(
                    "Slow response time: {:.1} seconds (max: {})",
                    response_time, max_time
                );
                quality_flags.push(issue.clone());
                quality_issues.push(QualityIssue::new(participant_id, "SlowResponse", issue));
            }
        }

        if let Some(min_time) = quality_settings.min_response_time {
            if response_time < min_time {
                let issue = format!(
                    "Fast response time: {:.1} seconds (min: {})",
                    response_time, min_time
                );
                quality_flags.push(issue.clone());
                quality_issues.push(QualityIssue::new(participant_id, "FastResponse", issue));
            }
        }
    }
}

/// Calculate overall careless responding score
/// Combines multiple indicators into a single metric
#[inline]
pub fn calculate_careless_score(
    missing_percent: f64,
    has_straightlining: bool,
    variance: f64,
) -> f64 {
    let mut score = 0.0;

    // Weight missing data
    score += missing_percent * 0.3;

    // Weight straightlining
    if has_straightlining {
        score += 0.5;
    }

    // Weight low variance (normalize to 0-1, assuming typical variance is around 1.0)
    let variance_score = (1.0 - variance.min(1.0)).max(0.0);
    score += variance_score * 0.2;

    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_careless_score() {
        // High careless responding
        let score = calculate_careless_score(0.5, true, 0.1);
        assert!(score > 0.5);

        // Low careless responding
        let score = calculate_careless_score(0.0, false, 1.0);
        assert!(score < 0.3);
    }
}
