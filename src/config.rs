use crate::longitudinal::LongitudinalConfig;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct SurveyConfig {
    #[serde(default)]
    pub survey: SurveySettings,
    pub quality: Option<QualitySettings>,
    #[serde(default)]
    pub scales: HashMap<String, ScaleDefinition>,
    #[serde(default)]
    pub column_mappings: Option<HashMap<String, String>>,
    #[serde(default)]
    pub output: Option<OutputSettings>,
    #[serde(default)]
    pub longitudinal: Option<LongitudinalConfig>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SurveySettings {
    pub name: String,
    pub min_score: u32,
    pub max_score: u32,
    #[serde(default)]
    pub participant_id_column: Option<String>,
}

impl Default for SurveySettings {
    fn default() -> Self {
        Self {
            name: "Survey".to_string(),
            min_score: 1,
            max_score: 7,
            participant_id_column: None,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct QualitySettings {
    pub max_missing_percent: f64,
    pub flag_straightlining: bool,
    #[serde(default)]
    pub min_response_variance: Option<f64>,
    #[serde(default)]
    pub max_response_time: Option<f64>,
    #[serde(default)]
    pub min_response_time: Option<f64>,
    #[serde(default)]
    pub careless_responding_threshold: Option<f64>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct OutputSettings {
    #[serde(default = "default_decimal_places")]
    pub decimal_places: usize,
    #[serde(default = "default_date_format")]
    pub date_format: String,
    #[serde(default)]
    pub include_item_scores: bool,
}

fn default_decimal_places() -> usize {
    2
}

fn default_date_format() -> String {
    "%Y-%m-%d".to_string()
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ScaleDefinition {
    pub items: Vec<String>,
    pub reverse_scored: Option<Vec<String>>,
}
