//! Power Analysis Module
//!
//! Provides statistical power analysis functions for study planning and evaluation.
//! Supports a priori power calculations (sample size determination) and post-hoc
//! power analysis (observed power from collected data).

use crate::errors::ProcessingError;
use serde::{Deserialize, Serialize};

/// Test types supported for power analysis
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TestType {
    /// Independent samples t-test
    IndependentT,
    /// Paired samples t-test
    PairedT,
    /// One-sample t-test
    OneSampleT,
    /// Pearson correlation
    Correlation,
    /// One-way ANOVA
    OneWayAnova,
    /// Linear regression
    LinearRegression,
}

/// Parameters for a priori power analysis
#[derive(Debug, Clone)]
pub struct APrioriParams {
    pub test_type: TestType,
    pub effect_size: f64,
    pub alpha: f64,
    pub power: f64,
    pub tails: u8, // 1 or 2
}

/// Parameters for post-hoc power analysis
#[derive(Debug, Clone)]
pub struct PostHocParams {
    pub test_type: TestType,
    pub effect_size: f64,
    pub sample_size: usize,
    pub alpha: f64,
    pub tails: u8,
}

/// Result of a power analysis calculation
#[derive(Debug, Clone, Serialize)]
pub struct PowerResult {
    pub test_type: String,
    pub effect_size: f64,
    pub alpha: f64,
    pub power: f64,
    pub sample_size: usize,
    pub critical_value: f64,
    pub interpretation: String,
}

/// Calculate required sample size for desired power (a priori)
pub fn calculate_sample_size(params: &APrioriParams) -> Result<PowerResult, ProcessingError> {
    validate_params(params.effect_size, params.alpha, params.power, params.tails)?;

    let n = match params.test_type {
        TestType::IndependentT => {
            calculate_n_independent_t(params.effect_size, params.alpha, params.power, params.tails)
        }
        TestType::PairedT => {
            calculate_n_paired_t(params.effect_size, params.alpha, params.power, params.tails)
        }
        TestType::OneSampleT => {
            calculate_n_one_sample_t(params.effect_size, params.alpha, params.power, params.tails)
        }
        TestType::Correlation => {
            calculate_n_correlation(params.effect_size, params.alpha, params.power, params.tails)
        }
        TestType::OneWayAnova | TestType::LinearRegression => {
            return Err(ProcessingError::Custom(format!(
                "{:?} not yet implemented for a priori analysis",
                params.test_type
            )));
        }
    };

    let critical_value = calculate_critical_value(params.alpha, params.tails);
    let interpretation = interpret_power(params.power);

    Ok(PowerResult {
        test_type: format!("{:?}", params.test_type),
        effect_size: params.effect_size,
        alpha: params.alpha,
        power: params.power,
        sample_size: n.ceil() as usize,
        critical_value,
        interpretation,
    })
}

/// Calculate observed power from collected data (post-hoc)
pub fn calculate_observed_power(params: &PostHocParams) -> Result<PowerResult, ProcessingError> {
    validate_params(params.effect_size, params.alpha, 0.5, params.tails)?;

    if params.sample_size == 0 {
        return Err(ProcessingError::Custom(
            "Sample size must be greater than 0".to_string(),
        ));
    }

    let power = match params.test_type {
        TestType::IndependentT => calculate_power_independent_t(
            params.effect_size,
            params.sample_size,
            params.alpha,
            params.tails,
        ),
        TestType::PairedT => calculate_power_paired_t(
            params.effect_size,
            params.sample_size,
            params.alpha,
            params.tails,
        ),
        TestType::OneSampleT => calculate_power_one_sample_t(
            params.effect_size,
            params.sample_size,
            params.alpha,
            params.tails,
        ),
        TestType::Correlation => calculate_power_correlation(
            params.effect_size,
            params.sample_size,
            params.alpha,
            params.tails,
        ),
        TestType::OneWayAnova | TestType::LinearRegression => {
            return Err(ProcessingError::Custom(format!(
                "{:?} not yet implemented for post-hoc analysis",
                params.test_type
            )));
        }
    };

    let critical_value = calculate_critical_value(params.alpha, params.tails);
    let interpretation = interpret_power(power);

    Ok(PowerResult {
        test_type: format!("{:?}", params.test_type),
        effect_size: params.effect_size,
        alpha: params.alpha,
        power,
        sample_size: params.sample_size,
        critical_value,
        interpretation,
    })
}

// ============================================================================
// Helper Functions - Sample Size Calculations
// ============================================================================

fn calculate_n_independent_t(d: f64, alpha: f64, power: f64, tails: u8) -> f64 {
    let z_alpha = inverse_normal_cdf(1.0 - alpha / (tails as f64));
    let z_beta = inverse_normal_cdf(power);
    2.0 * ((z_alpha + z_beta) / d).powi(2)
}

fn calculate_n_paired_t(d: f64, alpha: f64, power: f64, tails: u8) -> f64 {
    let z_alpha = inverse_normal_cdf(1.0 - alpha / (tails as f64));
    let z_beta = inverse_normal_cdf(power);
    ((z_alpha + z_beta) / d).powi(2)
}

fn calculate_n_one_sample_t(d: f64, alpha: f64, power: f64, tails: u8) -> f64 {
    // Similar to paired t-test
    calculate_n_paired_t(d, alpha, power, tails)
}

fn calculate_n_correlation(r: f64, alpha: f64, power: f64, tails: u8) -> f64 {
    // Fisher's z transformation
    let z_r = 0.5 * ((1.0 + r) / (1.0 - r)).ln();
    let z_alpha = inverse_normal_cdf(1.0 - alpha / (tails as f64));
    let z_beta = inverse_normal_cdf(power);
    ((z_alpha + z_beta) / z_r).powi(2) + 3.0
}

// ============================================================================
// Helper Functions - Power Calculations
// ============================================================================

fn calculate_power_independent_t(d: f64, n: usize, alpha: f64, tails: u8) -> f64 {
    let z_alpha = inverse_normal_cdf(1.0 - alpha / (tails as f64));
    let ncp = d * (n as f64 / 2.0).sqrt(); // non-centrality parameter
    let power = 1.0 - normal_cdf(z_alpha - ncp);
    power.clamp(0.0, 1.0)
}

fn calculate_power_paired_t(d: f64, n: usize, alpha: f64, tails: u8) -> f64 {
    let z_alpha = inverse_normal_cdf(1.0 - alpha / (tails as f64));
    let ncp = d * (n as f64).sqrt();
    let power = 1.0 - normal_cdf(z_alpha - ncp);
    power.clamp(0.0, 1.0)
}

fn calculate_power_one_sample_t(d: f64, n: usize, alpha: f64, tails: u8) -> f64 {
    calculate_power_paired_t(d, n, alpha, tails)
}

fn calculate_power_correlation(r: f64, n: usize, alpha: f64, tails: u8) -> f64 {
    let z_r = 0.5 * ((1.0 + r) / (1.0 - r)).ln();
    let z_alpha = inverse_normal_cdf(1.0 - alpha / (tails as f64));
    let se = 1.0 / ((n as f64 - 3.0).sqrt());
    let ncp = z_r / se;
    let power = 1.0 - normal_cdf(z_alpha - ncp);
    power.clamp(0.0, 1.0)
}

// ============================================================================
// Statistical Functions
// ============================================================================

/// Standard normal cumulative distribution function
fn normal_cdf(x: f64) -> f64 {
    0.5 * (1.0 + erf(x / 2.0_f64.sqrt()))
}

/// Inverse normal CDF (quantile function) using Beasley-Springer-Moro algorithm
fn inverse_normal_cdf(p: f64) -> f64 {
    if p <= 0.0 {
        return f64::NEG_INFINITY;
    }
    if p >= 1.0 {
        return f64::INFINITY;
    }
    if (p - 0.5).abs() < 1e-10 {
        return 0.0;
    }

    // Coefficients for Beasley-Springer-Moro algorithm
    #[allow(clippy::excessive_precision)]
    let a = [
        -3.969683028665376e+01,
        2.209460984245205e+02,
        -2.759285104469687e+02,
        1.383577518672690e+02,
        -3.066479806614716e+01,
        2.506628277459239e+00,
    ];

    let b = [
        -5.447609879822406e+01,
        1.615858368580409e+02,
        -1.556989798598866e+02,
        6.680131188771972e+01,
        -1.328068155288572e+01,
    ];

    let c = [
        -7.784894002430293e-03,
        -3.223964580411365e-01,
        -2.400758277161838e+00,
        -2.549732539343734e+00,
        4.374664141464968e+00,
        2.938163982698783e+00,
    ];

    let d = [
        7.784695709041462e-03,
        3.224671290700398e-01,
        2.445134137142996e+00,
        3.754408661907416e+00,
    ];

    let p_low = 0.02425;
    let p_high = 1.0 - p_low;

    let x: f64;
    let r: f64;

    if p < p_low {
        // Rational approximation for lower region
        let q = (-2.0 * p.ln()).sqrt();
        x = (((((c[0] * q + c[1]) * q + c[2]) * q + c[3]) * q + c[4]) * q + c[5])
            / ((((d[0] * q + d[1]) * q + d[2]) * q + d[3]) * q + 1.0);
    } else if p <= p_high {
        // Rational approximation for central region
        let q = p - 0.5;
        r = q * q;
        x = (((((a[0] * r + a[1]) * r + a[2]) * r + a[3]) * r + a[4]) * r + a[5]) * q
            / (((((b[0] * r + b[1]) * r + b[2]) * r + b[3]) * r + b[4]) * r + 1.0);
    } else {
        // Rational approximation for upper region
        let q = (-2.0 * (1.0 - p).ln()).sqrt();
        x = -(((((c[0] * q + c[1]) * q + c[2]) * q + c[3]) * q + c[4]) * q + c[5])
            / ((((d[0] * q + d[1]) * q + d[2]) * q + d[3]) * q + 1.0);
    }

    x
}

/// Error function approximation
fn erf(x: f64) -> f64 {
    let a1 = 0.254829592;
    let a2 = -0.284496736;
    let a3 = 1.421413741;
    let a4 = -1.453152027;
    let a5 = 1.061405429;
    let p = 0.3275911;

    let sign = if x < 0.0 { -1.0 } else { 1.0 };
    let x = x.abs();

    let t = 1.0 / (1.0 + p * x);
    let y = 1.0 - (((((a5 * t + a4) * t) + a3) * t + a2) * t + a1) * t * (-x * x).exp();

    sign * y
}

/// Calculate critical value for hypothesis test
fn calculate_critical_value(alpha: f64, tails: u8) -> f64 {
    inverse_normal_cdf(1.0 - alpha / (tails as f64))
}

// ============================================================================
// Validation and Interpretation
// ============================================================================

fn validate_params(
    effect_size: f64,
    alpha: f64,
    power: f64,
    tails: u8,
) -> Result<(), ProcessingError> {
    // Check for NaN or infinity in all float parameters
    if !effect_size.is_finite() {
        return Err(ProcessingError::Custom(
            "Effect size must be a finite number".to_string(),
        ));
    }
    if !alpha.is_finite() {
        return Err(ProcessingError::Custom(
            "Alpha must be a finite number".to_string(),
        ));
    }
    if !power.is_finite() {
        return Err(ProcessingError::Custom(
            "Power must be a finite number".to_string(),
        ));
    }

    if effect_size <= 0.0 {
        return Err(ProcessingError::Custom(
            "Effect size must be greater than 0".to_string(),
        ));
    }

    if !(0.0..=1.0).contains(&alpha) {
        return Err(ProcessingError::Custom(
            "Alpha must be between 0 and 1".to_string(),
        ));
    }

    if !(0.0..=1.0).contains(&power) {
        return Err(ProcessingError::Custom(
            "Power must be between 0 and 1".to_string(),
        ));
    }

    if tails != 1 && tails != 2 {
        return Err(ProcessingError::Custom("Tails must be 1 or 2".to_string()));
    }

    Ok(())
}

fn interpret_power(power: f64) -> String {
    if power >= 0.80 {
        format!("Adequate power (≥ 0.80): {:.2}%", power * 100.0)
    } else if power >= 0.50 {
        format!("Low power (< 0.80): {:.2}%", power * 100.0)
    } else {
        format!("Very low power (< 0.50): {:.2}%", power * 100.0)
    }
}

/// Effect size interpretation guidelines (Cohen, 1988)
pub fn interpret_effect_size(test_type: &TestType, effect_size: f64) -> String {
    match test_type {
        TestType::IndependentT | TestType::PairedT | TestType::OneSampleT => {
            // Cohen's d
            if effect_size < 0.2 {
                "Negligible".to_string()
            } else if effect_size < 0.5 {
                "Small".to_string()
            } else if effect_size < 0.8 {
                "Medium".to_string()
            } else {
                "Large".to_string()
            }
        }
        TestType::Correlation => {
            // Pearson's r
            let abs_r = effect_size.abs();
            if abs_r < 0.1 {
                "Negligible".to_string()
            } else if abs_r < 0.3 {
                "Small".to_string()
            } else if abs_r < 0.5 {
                "Medium".to_string()
            } else {
                "Large".to_string()
            }
        }
        _ => "N/A".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cdf() {
        assert!((normal_cdf(0.0) - 0.5).abs() < 0.001);
        assert!((normal_cdf(1.96) - 0.975).abs() < 0.001);
        assert!((normal_cdf(-1.96) - 0.025).abs() < 0.001);
    }

    #[test]
    fn test_inverse_normal_cdf() {
        assert!((inverse_normal_cdf(0.5) - 0.0).abs() < 0.001);
        assert!((inverse_normal_cdf(0.975) - 1.96).abs() < 0.01);
        assert!((inverse_normal_cdf(0.025) - (-1.96)).abs() < 0.01);
    }

    #[test]
    fn test_sample_size_independent_t() {
        let params = APrioriParams {
            test_type: TestType::IndependentT,
            effect_size: 0.5, // medium effect
            alpha: 0.05,
            power: 0.80,
            tails: 2,
        };
        let result = calculate_sample_size(&params).unwrap();
        // Expected n per group ≈ 64
        assert!(result.sample_size >= 60 && result.sample_size <= 70);
    }

    #[test]
    fn test_observed_power_correlation() {
        let params = PostHocParams {
            test_type: TestType::Correlation,
            effect_size: 0.3, // small-medium correlation
            sample_size: 100,
            alpha: 0.05,
            tails: 2,
        };
        let result = calculate_observed_power(&params).unwrap();
        assert!(result.power > 0.80); // Should have adequate power
    }

    #[test]
    fn test_effect_size_interpretation() {
        assert_eq!(interpret_effect_size(&TestType::IndependentT, 0.3), "Small");
        assert_eq!(
            interpret_effect_size(&TestType::IndependentT, 0.6),
            "Medium"
        );
        assert_eq!(interpret_effect_size(&TestType::IndependentT, 0.9), "Large");
    }
}
