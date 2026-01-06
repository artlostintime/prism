// src/utils.rs
//! Utility functions for common operations

/// Calculate percentage safely, handling division by zero
///
/// # Arguments
/// * `count` - Numerator value
/// * `total` - Denominator value
///
/// # Returns
/// Percentage as f64, or 0.0 if total is 0
///
/// # Examples
/// ```
/// use prism::utils::calculate_percentage;
///
/// assert_eq!(calculate_percentage(50, 100), 50.0);
/// assert_eq!(calculate_percentage(0, 0), 0.0);
/// ```
#[inline]
pub fn calculate_percentage(count: usize, total: usize) -> f64 {
    if total == 0 {
        0.0
    } else {
        (count as f64 / total as f64) * 100.0
    }
}

/// Format a percentage with specified decimal places
///
/// # Arguments
/// * `count` - Numerator value
/// * `total` - Denominator value
/// * `decimals` - Number of decimal places
///
/// # Returns
/// Formatted percentage string with '%' suffix
///
/// # Examples
/// ```
/// use prism::utils::format_percentage;
///
/// assert_eq!(format_percentage(1, 4, 1), "25.0%");
/// assert_eq!(format_percentage(0, 0, 1), "0.0%");
/// ```
pub fn format_percentage(count: usize, total: usize, decimals: usize) -> String {
    format!("{:.*}%", decimals, calculate_percentage(count, total))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_percentage() {
        assert_eq!(calculate_percentage(50, 100), 50.0);
        assert_eq!(calculate_percentage(1, 4), 25.0);
        assert_eq!(calculate_percentage(0, 100), 0.0);
        assert_eq!(calculate_percentage(0, 0), 0.0); // Edge case
    }

    #[test]
    fn test_format_percentage() {
        assert_eq!(format_percentage(50, 100, 1), "50.0%");
        assert_eq!(format_percentage(1, 3, 2), "33.33%");
        assert_eq!(format_percentage(0, 0, 1), "0.0%");
    }
}
