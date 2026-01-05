// src/stats.rs
use serde::{Deserialize, Serialize};

/// Statistics structure for aggregate calculations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stats {
    pub mean: f64,
    pub sd: f64,
    pub min: f64,
    pub max: f64,
    pub n: usize,
}

impl Stats {
    /// Calculate descriptive statistics from a set of values
    ///
    /// # Example
    /// ```
    /// use prism::stats::Stats;
    /// let values = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    /// let stats = Stats::calculate(&values);
    /// assert_eq!(stats.mean, 3.0);
    /// assert_eq!(stats.n, 5);
    /// ```
    pub fn calculate(values: &[f64]) -> Self {
        let n = values.len();
        if n == 0 {
            return Stats {
                mean: 0.0,
                sd: 0.0,
                min: 0.0,
                max: 0.0,
                n: 0,
            };
        }

        // Single pass for sum, min, max
        let mut sum = 0.0;
        let mut min = f64::INFINITY;
        let mut max = f64::NEG_INFINITY;

        for &v in values {
            sum += v;
            if v < min {
                min = v;
            }
            if v > max {
                max = v;
            }
        }

        let mean = sum / n as f64;

        // Second pass for variance (unavoidable)
        let variance = if n > 1 {
            let sum_squared_diff: f64 = values
                .iter()
                .map(|v| {
                    let diff = v - mean;
                    diff * diff // Faster than .powi(2)
                })
                .sum();
            sum_squared_diff / (n - 1) as f64
        } else {
            0.0
        };

        let sd = variance.sqrt();

        Stats {
            mean,
            sd,
            min,
            max,
            n,
        }
    }
}

/// Calculate Cronbach's alpha for internal consistency reliability
///
/// # Arguments
/// * `item_matrix` - A matrix where each inner vector represents one participant's responses to all items
///
/// # Returns
/// Cronbach's alpha coefficient (typically ranges from 0 to 1, with > 0.7 considered acceptable)
///
/// # Example
/// ```
/// use prism::stats::calculate_cronbachs_alpha;
/// // 3 participants, 4 items each
/// let data = vec![
///     vec![5.0, 4.0, 5.0, 4.0],
///     vec![3.0, 3.0, 4.0, 3.0],
///     vec![4.0, 5.0, 4.0, 5.0],
/// ];
/// let alpha = calculate_cronbachs_alpha(&data);
/// assert!(alpha > 0.0 && alpha <= 1.0);
/// ```
pub fn calculate_cronbachs_alpha(item_matrix: &[Vec<f64>]) -> f64 {
    if item_matrix.is_empty() {
        return 0.0;
    }

    let n_items = item_matrix[0].len();
    if n_items < 2 {
        return 0.0; // Need at least 2 items
    }

    let n_participants = item_matrix.len();
    if n_participants < 2 {
        return 0.0;
    }

    // Validate all rows have same length
    if !item_matrix.iter().all(|row| row.len() == n_items) {
        return 0.0; // Cannot calculate with jagged array
    }

    // Pre-allocate for total scores
    let mut total_scores = Vec::with_capacity(n_participants);
    for items in item_matrix {
        total_scores.push(items.iter().sum::<f64>());
    }

    // Calculate variance of total scores
    let total_variance = calculate_variance(&total_scores);

    // Handle edge case: if total variance is zero, reliability is undefined
    if total_variance == 0.0 || total_variance.is_nan() {
        return 0.0; // No variability means alpha is undefined, return 0
    }

    // Calculate variance of each item
    let mut sum_item_variances = 0.0;
    for item_idx in 0..n_items {
        // Inline calculation without allocation
        let mut sum = 0.0;
        for row in item_matrix {
            sum += row[item_idx];
        }
        let mean = sum / n_participants as f64;

        let variance = if n_participants > 1 {
            let sum_sq: f64 = item_matrix
                .iter()
                .map(|row| {
                    let diff = row[item_idx] - mean;
                    diff * diff
                })
                .sum();
            sum_sq / (n_participants - 1) as f64
        } else {
            0.0
        };

        sum_item_variances += variance;
    }

    // Cronbach's alpha formula: α = (k/(k-1)) * (1 - Σvar_i/var_total)
    // Ensure result is bounded [0, 1] as negative alphas are theoretically possible but not meaningful
    let k = n_items as f64;
    let alpha = (k / (k - 1.0)) * (1.0 - (sum_item_variances / total_variance));

    // Clamp to [0, 1] range - negative alpha suggests measurement issues
    alpha.clamp(0.0, 1.0)
}

/// Calculate variance of a set of values
#[inline]
fn calculate_variance(values: &[f64]) -> f64 {
    let n = values.len();
    if n < 2 {
        return 0.0;
    }

    let mean = values.iter().sum::<f64>() / n as f64;
    let sum_squared_diff: f64 = values
        .iter()
        .map(|v| {
            let diff = v - mean;
            diff * diff // Faster than powi(2)
        })
        .sum();
    sum_squared_diff / (n - 1) as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stats_calculate() {
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let stats = Stats::calculate(&values);
        assert_eq!(stats.mean, 3.0);
        assert_eq!(stats.n, 5);
        assert_eq!(stats.min, 1.0);
        assert_eq!(stats.max, 5.0);
    }

    #[test]
    fn test_stats_empty() {
        let values: Vec<f64> = vec![];
        let stats = Stats::calculate(&values);
        assert_eq!(stats.n, 0);
    }

    #[test]
    fn test_cronbachs_alpha() {
        // Perfect consistency
        let perfect = vec![
            vec![5.0, 5.0, 5.0],
            vec![3.0, 3.0, 3.0],
            vec![4.0, 4.0, 4.0],
        ];
        let alpha = calculate_cronbachs_alpha(&perfect);
        assert!(alpha > 0.99); // Should be very close to 1.0

        // Some variance
        let varied = vec![
            vec![5.0, 4.0, 5.0],
            vec![3.0, 3.0, 4.0],
            vec![4.0, 5.0, 4.0],
        ];
        let alpha = calculate_cronbachs_alpha(&varied);
        assert!(alpha > 0.0 && alpha < 1.0);
    }
}
