// src/types.rs
use serde::{Deserialize, Serialize};

/// Newtype for score values to prevent mixing with other numeric types
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ScoreValue(pub f64);

impl ScoreValue {
    pub fn new(value: f64) -> Self {
        ScoreValue(value)
    }

    pub fn get(&self) -> f64 {
        self.0
    }
}

impl From<f64> for ScoreValue {
    fn from(value: f64) -> Self {
        ScoreValue(value)
    }
}

impl std::ops::Add for ScoreValue {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        ScoreValue(self.0 + rhs.0)
    }
}

impl std::ops::Sub for ScoreValue {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        ScoreValue(self.0 - rhs.0)
    }
}

/// Newtype for missing data percentage
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct MissingPercent(pub f64);

impl MissingPercent {
    pub fn new(value: f64) -> Self {
        assert!(
            (0.0..=1.0).contains(&value),
            "MissingPercent must be between 0.0 and 1.0"
        );
        MissingPercent(value)
    }

    pub fn get(&self) -> f64 {
        self.0
    }

    pub fn as_percentage(&self) -> f64 {
        self.0 * 100.0
    }
}

impl From<f64> for MissingPercent {
    fn from(value: f64) -> Self {
        MissingPercent::new(value)
    }
}

/// Helper struct for scale processing results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScaleResult {
    pub total: f64,
    pub mean: f64,
    pub valid_items: usize,
    pub item_values: Vec<f64>,
    pub missing_items: Vec<String>,
    pub out_of_range_items: Vec<String>,
}

impl ScaleResult {
    pub fn new(
        total: f64,
        mean: f64,
        valid_items: usize,
        item_values: Vec<f64>,
        missing_items: Vec<String>,
        out_of_range_items: Vec<String>,
    ) -> Self {
        Self {
            total,
            mean,
            valid_items,
            item_values,
            missing_items,
            out_of_range_items,
        }
    }

    pub fn has_issues(&self) -> bool {
        !self.missing_items.is_empty() || !self.out_of_range_items.is_empty()
    }
}

/// Quality issue tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityIssue {
    pub participant_id: String,
    pub issue_type: String,
    pub details: String,
}

impl QualityIssue {
    pub fn new(
        participant_id: impl Into<String>,
        issue_type: impl Into<String>,
        details: impl Into<String>,
    ) -> Self {
        Self {
            participant_id: participant_id.into(),
            issue_type: issue_type.into(),
            details: details.into(),
        }
    }
}

/// Output format options
#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum, Serialize, Deserialize)]
pub enum OutputFormat {
    Csv,
    Excel,
    Json,
    Spss,
    R,
    Python,
    HtmlReport,
}

impl OutputFormat {
    pub fn extension(&self) -> &str {
        match self {
            OutputFormat::Csv => "csv",
            OutputFormat::Excel => "xlsx",
            OutputFormat::Json => "json",
            OutputFormat::Spss => "sps",
            OutputFormat::R => "R",
            OutputFormat::Python => "py",
            OutputFormat::HtmlReport => "html",
        }
    }
}
