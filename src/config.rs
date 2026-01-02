use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct SurveyConfig {
    pub survey: SurveySettings,
    pub quality: Option<QualitySettings>,
    pub scales: HashMap<String, ScaleDefinition>,
}

#[derive(Debug, Deserialize)]
pub struct SurveySettings {
    pub name: String,
    pub min_score: u32,
    pub max_score: u32,
}

#[derive(Debug, Deserialize)]
pub struct QualitySettings {
    pub max_missing_percent: f64,
    pub flag_straightlining: bool,
}

#[derive(Debug, Deserialize)]
pub struct ScaleDefinition {
    pub items: Vec<String>,
    pub reverse_scored: Option<Vec<String>>,
}
