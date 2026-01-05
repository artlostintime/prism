//! Pre-built scale library for common psychology instruments
//!
//! This module provides ready-to-use configurations for widely used psychological scales,
//! including citations, scoring rules, and normative data where applicable.

use crate::errors::{ProcessingError, Result};

/// Metadata for a pre-built scale
#[derive(Debug, Clone)]
pub struct ScaleMetadata {
    pub name: String,
    pub full_name: String,
    pub citation: String,
    pub description: String,
    pub num_items: usize,
    pub min_score: u32,
    pub max_score: u32,
    pub interpretation: String,
    pub normative_data: Option<NormativeData>,
}

/// Normative data for scale interpretation
#[derive(Debug, Clone)]
pub struct NormativeData {
    pub population: String,
    pub mean: f64,
    pub sd: f64,
    pub clinical_cutoff: Option<f64>,
    pub severity_ranges: Vec<(String, f64, f64)>, // (label, min, max)
}

/// List all available pre-built scales
pub fn list_available_scales() -> Vec<String> {
    vec![
        "PHQ-9".to_string(),
        "GAD-7".to_string(),
        "PSS-10".to_string(),
        "PSS-14".to_string(),
        "PANAS".to_string(),
        "BDI-II".to_string(),
        "BAI".to_string(),
        "SWLS".to_string(),
    ]
}

/// Generate a config for a specific pre-built scale
pub fn generate_scale_config(scale_id: &str) -> Result<String> {
    let scale_id_upper = scale_id.to_uppercase();

    match scale_id_upper.as_str() {
        "PHQ-9" | "PHQ9" => Ok(generate_phq9_config()),
        "GAD-7" | "GAD7" => Ok(generate_gad7_config()),
        "PSS-10" | "PSS10" => Ok(generate_pss10_config()),
        "PSS-14" | "PSS14" => Ok(generate_pss14_config()),
        "PANAS" => Ok(generate_panas_config()),
        "BDI-II" | "BDIII" | "BDI2" => Ok(generate_bdi_ii_config()),
        "BAI" => Ok(generate_bai_config()),
        "SWLS" => Ok(generate_swls_config()),
        _ => Err(ProcessingError::ConfigError(format!(
            "Unknown scale: '{}'. Available scales: PHQ-9, GAD-7, PSS-10, PSS-14, PANAS, BDI-II, BAI, SWLS",
            scale_id
        ))),
    }
}

/// Get metadata for a specific scale
pub fn get_scale_metadata(scale_id: &str) -> Result<ScaleMetadata> {
    let scale_id_upper = scale_id.to_uppercase();

    match scale_id_upper.as_str() {
        "PHQ-9" | "PHQ9" => Ok(phq9_metadata()),
        "GAD-7" | "GAD7" => Ok(gad7_metadata()),
        "PSS-10" | "PSS10" => Ok(pss10_metadata()),
        "PSS-14" | "PSS14" => Ok(pss14_metadata()),
        "PANAS" => Ok(panas_metadata()),
        "BDI-II" | "BDIII" | "BDI2" => Ok(bdi_ii_metadata()),
        "BAI" => Ok(bai_metadata()),
        "SWLS" => Ok(swls_metadata()),
        _ => Err(ProcessingError::ConfigError(format!(
            "Unknown scale: '{}'",
            scale_id
        ))),
    }
}

// ============================================================================
// PHQ-9: Patient Health Questionnaire-9
// ============================================================================

fn generate_phq9_config() -> String {
    r#"# PHQ-9: Patient Health Questionnaire-9
# Depression screening and severity measure
#
# Citation: Kroenke, K., Spitzer, R. L., & Williams, J. B. (2001).
#           The PHQ-9: validity of a brief depression severity measure.
#           Journal of General Internal Medicine, 16(9), 606-613.
#
# Scoring: 0-3 scale (0=Not at all, 1=Several days, 2=More than half the days, 3=Nearly every day)
# Interpretation:
#   0-4:   Minimal depression
#   5-9:   Mild depression
#   10-14: Moderate depression
#   15-19: Moderately severe depression
#   20-27: Severe depression

[survey]
name = "PHQ-9 Depression Screening"
min_score = 0
max_score = 3
participant_id_column = "ResponseId"

[quality]
max_missing_percent = 0.10
flag_straightlining = true

[output]
decimal_places = 2

[scales.phq9_total]
items = [
    "PHQ1",  # Little interest or pleasure in doing things
    "PHQ2",  # Feeling down, depressed, or hopeless
    "PHQ3",  # Trouble falling/staying asleep, sleeping too much
    "PHQ4",  # Feeling tired or having little energy
    "PHQ5",  # Poor appetite or overeating
    "PHQ6",  # Feeling bad about yourself
    "PHQ7",  # Trouble concentrating
    "PHQ8",  # Moving or speaking slowly/being fidgety
    "PHQ9"   # Thoughts of self-harm
]
reverse_scored = []

# Note: Ensure your CSV columns match the item names above (PHQ1, PHQ2, etc.)
# You can use [column_mappings] to rename columns if needed
"#
    .to_string()
}

fn phq9_metadata() -> ScaleMetadata {
    ScaleMetadata {
        name: "PHQ-9".to_string(),
        full_name: "Patient Health Questionnaire-9".to_string(),
        citation: "Kroenke, K., Spitzer, R. L., & Williams, J. B. (2001). The PHQ-9: validity of a brief depression severity measure. Journal of General Internal Medicine, 16(9), 606-613.".to_string(),
        description: "9-item self-report measure of depression severity based on DSM-IV criteria".to_string(),
        num_items: 9,
        min_score: 0,
        max_score: 3,
        interpretation: "0-4: Minimal, 5-9: Mild, 10-14: Moderate, 15-19: Moderately severe, 20-27: Severe depression".to_string(),
        normative_data: Some(NormativeData {
            population: "Primary care patients".to_string(),
            mean: 5.06,
            sd: 5.75,
            clinical_cutoff: Some(10.0),
            severity_ranges: vec![
                ("Minimal depression".to_string(), 0.0, 4.0),
                ("Mild depression".to_string(), 5.0, 9.0),
                ("Moderate depression".to_string(), 10.0, 14.0),
                ("Moderately severe depression".to_string(), 15.0, 19.0),
                ("Severe depression".to_string(), 20.0, 27.0),
            ],
        }),
    }
}

// ============================================================================
// GAD-7: Generalized Anxiety Disorder-7
// ============================================================================

fn generate_gad7_config() -> String {
    r#"# GAD-7: Generalized Anxiety Disorder-7
# Anxiety screening and severity measure
#
# Citation: Spitzer, R. L., Kroenke, K., Williams, J. B., & Löwe, B. (2006).
#           A brief measure for assessing generalized anxiety disorder: the GAD-7.
#           Archives of Internal Medicine, 166(10), 1092-1097.
#
# Scoring: 0-3 scale (0=Not at all, 1=Several days, 2=More than half the days, 3=Nearly every day)
# Interpretation:
#   0-4:   Minimal anxiety
#   5-9:   Mild anxiety
#   10-14: Moderate anxiety
#   15-21: Severe anxiety

[survey]
name = "GAD-7 Anxiety Screening"
min_score = 0
max_score = 3
participant_id_column = "ResponseId"

[quality]
max_missing_percent = 0.10
flag_straightlining = true

[output]
decimal_places = 2

[scales.gad7_total]
items = [
    "GAD1",  # Feeling nervous, anxious, or on edge
    "GAD2",  # Not being able to stop or control worrying
    "GAD3",  # Worrying too much about different things
    "GAD4",  # Trouble relaxing
    "GAD5",  # Being so restless that it's hard to sit still
    "GAD6",  # Becoming easily annoyed or irritable
    "GAD7"   # Feeling afraid as if something awful might happen
]
reverse_scored = []

# Note: Ensure your CSV columns match the item names above (GAD1, GAD2, etc.)
"#
    .to_string()
}

fn gad7_metadata() -> ScaleMetadata {
    ScaleMetadata {
        name: "GAD-7".to_string(),
        full_name: "Generalized Anxiety Disorder-7".to_string(),
        citation: "Spitzer, R. L., Kroenke, K., Williams, J. B., & Löwe, B. (2006). A brief measure for assessing generalized anxiety disorder: the GAD-7. Archives of Internal Medicine, 166(10), 1092-1097.".to_string(),
        description: "7-item self-report measure of generalized anxiety disorder severity".to_string(),
        num_items: 7,
        min_score: 0,
        max_score: 3,
        interpretation: "0-4: Minimal, 5-9: Mild, 10-14: Moderate, 15-21: Severe anxiety".to_string(),
        normative_data: Some(NormativeData {
            population: "Primary care patients".to_string(),
            mean: 4.90,
            sd: 5.40,
            clinical_cutoff: Some(10.0),
            severity_ranges: vec![
                ("Minimal anxiety".to_string(), 0.0, 4.0),
                ("Mild anxiety".to_string(), 5.0, 9.0),
                ("Moderate anxiety".to_string(), 10.0, 14.0),
                ("Severe anxiety".to_string(), 15.0, 21.0),
            ],
        }),
    }
}

// ============================================================================
// PSS-10: Perceived Stress Scale (10-item)
// ============================================================================

fn generate_pss10_config() -> String {
    r#"# PSS-10: Perceived Stress Scale (10-item version)
# Measure of perceived stress over the past month
#
# Citation: Cohen, S., Kamarck, T., & Mermelstein, R. (1983).
#           A global measure of perceived stress.
#           Journal of Health and Social Behavior, 24(4), 385-396.
#
# Scoring: 0-4 scale (0=Never, 1=Almost Never, 2=Sometimes, 3=Fairly Often, 4=Very Often)
# Items 4, 5, 7, 8 are REVERSE SCORED
# Higher scores indicate greater perceived stress

[survey]
name = "PSS-10 Perceived Stress Scale"
min_score = 0
max_score = 4
participant_id_column = "ResponseId"

[quality]
max_missing_percent = 0.10
flag_straightlining = true

[output]
decimal_places = 2

[scales.pss10_total]
items = [
    "PSS1",   # Upset because of unexpected event
    "PSS2",   # Unable to control important things
    "PSS3",   # Felt nervous and stressed
    "PSS4",   # Felt confident about handling problems (R)
    "PSS5",   # Felt things were going your way (R)
    "PSS6",   # Could not cope with all the things to do
    "PSS7",   # Able to control irritations (R)
    "PSS8",   # Felt on top of things (R)
    "PSS9",   # Angered by things outside your control
    "PSS10"   # Felt difficulties were piling up
]
reverse_scored = ["PSS4", "PSS5", "PSS7", "PSS8"]

# Note: Ensure your CSV columns match the item names above (PSS1, PSS2, etc.)
"#
    .to_string()
}

fn pss10_metadata() -> ScaleMetadata {
    ScaleMetadata {
        name: "PSS-10".to_string(),
        full_name: "Perceived Stress Scale (10-item)".to_string(),
        citation: "Cohen, S., Kamarck, T., & Mermelstein, R. (1983). A global measure of perceived stress. Journal of Health and Social Behavior, 24(4), 385-396.".to_string(),
        description: "10-item measure of perceived stress over the past month with 4 reverse-scored items".to_string(),
        num_items: 10,
        min_score: 0,
        max_score: 4,
        interpretation: "0-13: Low stress, 14-26: Moderate stress, 27-40: High perceived stress".to_string(),
        normative_data: Some(NormativeData {
            population: "U.S. adult population".to_string(),
            mean: 16.0,
            sd: 7.5,
            clinical_cutoff: None,
            severity_ranges: vec![
                ("Low stress".to_string(), 0.0, 13.0),
                ("Moderate stress".to_string(), 14.0, 26.0),
                ("High stress".to_string(), 27.0, 40.0),
            ],
        }),
    }
}

// ============================================================================
// PSS-14: Perceived Stress Scale (14-item)
// ============================================================================

fn generate_pss14_config() -> String {
    r#"# PSS-14: Perceived Stress Scale (14-item original version)
# Measure of perceived stress over the past month
#
# Citation: Cohen, S., Kamarck, T., & Mermelstein, R. (1983).
#           A global measure of perceived stress.
#           Journal of Health and Social Behavior, 24(4), 385-396.
#
# Scoring: 0-4 scale (0=Never, 1=Almost Never, 2=Sometimes, 3=Fairly Often, 4=Very Often)
# Items 4, 5, 6, 7, 9, 10, 13 are REVERSE SCORED
# Higher scores indicate greater perceived stress

[survey]
name = "PSS-14 Perceived Stress Scale"
min_score = 0
max_score = 4
participant_id_column = "ResponseId"

[quality]
max_missing_percent = 0.10
flag_straightlining = true

[output]
decimal_places = 2

[scales.pss14_total]
items = [
    "PSS1",   # Upset because of unexpected event
    "PSS2",   # Unable to control important things
    "PSS3",   # Felt nervous and stressed
    "PSS4",   # Dealt successfully with day to day problems (R)
    "PSS5",   # Felt effective in handling changes (R)
    "PSS6",   # Felt confident about handling problems (R)
    "PSS7",   # Felt things were going your way (R)
    "PSS8",   # Could not cope with all the things to do
    "PSS9",   # Able to control irritations (R)
    "PSS10",  # Felt on top of things (R)
    "PSS11",  # Angered by things outside your control
    "PSS12",  # Thought about things to accomplish
    "PSS13",  # Able to control time management (R)
    "PSS14"   # Felt difficulties were piling up
]
reverse_scored = ["PSS4", "PSS5", "PSS6", "PSS7", "PSS9", "PSS10", "PSS13"]

# Note: Ensure your CSV columns match the item names above (PSS1, PSS2, etc.)
"#
    .to_string()
}

fn pss14_metadata() -> ScaleMetadata {
    ScaleMetadata {
        name: "PSS-14".to_string(),
        full_name: "Perceived Stress Scale (14-item)".to_string(),
        citation: "Cohen, S., Kamarck, T., & Mermelstein, R. (1983). A global measure of perceived stress. Journal of Health and Social Behavior, 24(4), 385-396.".to_string(),
        description: "Original 14-item measure of perceived stress with 7 reverse-scored items".to_string(),
        num_items: 14,
        min_score: 0,
        max_score: 4,
        interpretation: "0-18: Low stress, 19-37: Moderate stress, 38-56: High perceived stress".to_string(),
        normative_data: Some(NormativeData {
            population: "U.S. adult population".to_string(),
            mean: 24.0,
            sd: 9.0,
            clinical_cutoff: None,
            severity_ranges: vec![
                ("Low stress".to_string(), 0.0, 18.0),
                ("Moderate stress".to_string(), 19.0, 37.0),
                ("High stress".to_string(), 38.0, 56.0),
            ],
        }),
    }
}

// ============================================================================
// PANAS: Positive and Negative Affect Schedule
// ============================================================================

fn generate_panas_config() -> String {
    r#"# PANAS: Positive and Negative Affect Schedule
# Measure of positive and negative affect
#
# Citation: Watson, D., Clark, L. A., & Tellegen, A. (1988).
#           Development and validation of brief measures of positive and negative affect: the PANAS scales.
#           Journal of Personality and Social Psychology, 54(6), 1063-1070.
#
# Scoring: 1-5 scale (1=Very slightly or not at all, 2=A little, 3=Moderately, 4=Quite a bit, 5=Extremely)
# Two subscales: Positive Affect (PA) and Negative Affect (NA)
# Higher scores on each subscale indicate higher levels of that affect

[survey]
name = "PANAS Affect Schedule"
min_score = 1
max_score = 5
participant_id_column = "ResponseId"

[quality]
max_missing_percent = 0.10
flag_straightlining = true

[output]
decimal_places = 2

[scales.panas_positive]
items = [
    "PANAS_Interested",
    "PANAS_Excited",
    "PANAS_Strong",
    "PANAS_Enthusiastic",
    "PANAS_Proud",
    "PANAS_Alert",
    "PANAS_Inspired",
    "PANAS_Determined",
    "PANAS_Attentive",
    "PANAS_Active"
]
reverse_scored = []

[scales.panas_negative]
items = [
    "PANAS_Distressed",
    "PANAS_Upset",
    "PANAS_Guilty",
    "PANAS_Scared",
    "PANAS_Hostile",
    "PANAS_Irritable",
    "PANAS_Ashamed",
    "PANAS_Nervous",
    "PANAS_Jittery",
    "PANAS_Afraid"
]
reverse_scored = []

# Note: Ensure your CSV columns match the item names above (PANAS_Interested, etc.)
# Positive Affect score range: 10-50
# Negative Affect score range: 10-50
"#.to_string()
}

fn panas_metadata() -> ScaleMetadata {
    ScaleMetadata {
        name: "PANAS".to_string(),
        full_name: "Positive and Negative Affect Schedule".to_string(),
        citation: "Watson, D., Clark, L. A., & Tellegen, A. (1988). Development and validation of brief measures of positive and negative affect: the PANAS scales. Journal of Personality and Social Psychology, 54(6), 1063-1070.".to_string(),
        description: "20-item measure with two 10-item subscales: Positive Affect and Negative Affect".to_string(),
        num_items: 20,
        min_score: 1,
        max_score: 5,
        interpretation: "PA: 31-50=High, 24-30=Moderate, 10-23=Low positive affect. NA: 31-50=High, 24-30=Moderate, 10-23=Low negative affect".to_string(),
        normative_data: Some(NormativeData {
            population: "College students".to_string(),
            mean: 29.7, // PA mean
            sd: 7.9,
            clinical_cutoff: None,
            severity_ranges: vec![
                ("Low PA".to_string(), 10.0, 23.0),
                ("Moderate PA".to_string(), 24.0, 30.0),
                ("High PA".to_string(), 31.0, 50.0),
            ],
        }),
    }
}

// ============================================================================
// BDI-II: Beck Depression Inventory-II
// ============================================================================

fn generate_bdi_ii_config() -> String {
    r#"# BDI-II: Beck Depression Inventory-II
# Depression severity measure
#
# Citation: Beck, A. T., Steer, R. A., & Brown, G. K. (1996).
#           Manual for the Beck Depression Inventory-II.
#           San Antonio, TX: Psychological Corporation.
#
# Scoring: 0-3 scale for each of 21 items
# Interpretation:
#   0-13:  Minimal depression
#   14-19: Mild depression
#   20-28: Moderate depression
#   29-63: Severe depression

[survey]
name = "BDI-II Depression Inventory"
min_score = 0
max_score = 3
participant_id_column = "ResponseId"

[quality]
max_missing_percent = 0.10
flag_straightlining = true

[output]
decimal_places = 2

[scales.bdi_ii_total]
items = [
    "BDI1",   # Sadness
    "BDI2",   # Pessimism
    "BDI3",   # Past Failure
    "BDI4",   # Loss of Pleasure
    "BDI5",   # Guilty Feelings
    "BDI6",   # Punishment Feelings
    "BDI7",   # Self-Dislike
    "BDI8",   # Self-Criticalness
    "BDI9",   # Suicidal Thoughts
    "BDI10",  # Crying
    "BDI11",  # Agitation
    "BDI12",  # Loss of Interest
    "BDI13",  # Indecisiveness
    "BDI14",  # Worthlessness
    "BDI15",  # Loss of Energy
    "BDI16",  # Changes in Sleep Pattern
    "BDI17",  # Irritability
    "BDI18",  # Changes in Appetite
    "BDI19",  # Concentration Difficulty
    "BDI20",  # Tiredness or Fatigue
    "BDI21"   # Loss of Interest in Sex
]
reverse_scored = []

# Note: Ensure your CSV columns match the item names above (BDI1, BDI2, etc.)
"#
    .to_string()
}

fn bdi_ii_metadata() -> ScaleMetadata {
    ScaleMetadata {
        name: "BDI-II".to_string(),
        full_name: "Beck Depression Inventory-II".to_string(),
        citation: "Beck, A. T., Steer, R. A., & Brown, G. K. (1996). Manual for the Beck Depression Inventory-II. San Antonio, TX: Psychological Corporation.".to_string(),
        description: "21-item self-report measure of depression severity aligned with DSM-IV criteria".to_string(),
        num_items: 21,
        min_score: 0,
        max_score: 3,
        interpretation: "0-13: Minimal, 14-19: Mild, 20-28: Moderate, 29-63: Severe depression".to_string(),
        normative_data: Some(NormativeData {
            population: "Clinical outpatients".to_string(),
            mean: 22.45,
            sd: 12.75,
            clinical_cutoff: Some(14.0),
            severity_ranges: vec![
                ("Minimal depression".to_string(), 0.0, 13.0),
                ("Mild depression".to_string(), 14.0, 19.0),
                ("Moderate depression".to_string(), 20.0, 28.0),
                ("Severe depression".to_string(), 29.0, 63.0),
            ],
        }),
    }
}

// ============================================================================
// BAI: Beck Anxiety Inventory
// ============================================================================

fn generate_bai_config() -> String {
    r#"# BAI: Beck Anxiety Inventory
# Anxiety severity measure
#
# Citation: Beck, A. T., Epstein, N., Brown, G., & Steer, R. A. (1988).
#           An inventory for measuring clinical anxiety: psychometric properties.
#           Journal of Consulting and Clinical Psychology, 56(6), 893-897.
#
# Scoring: 0-3 scale (0=Not at all, 1=Mildly, 2=Moderately, 3=Severely)
# Interpretation:
#   0-7:   Minimal anxiety
#   8-15:  Mild anxiety
#   16-25: Moderate anxiety
#   26-63: Severe anxiety

[survey]
name = "BAI Anxiety Inventory"
min_score = 0
max_score = 3
participant_id_column = "ResponseId"

[quality]
max_missing_percent = 0.10
flag_straightlining = true

[output]
decimal_places = 2

[scales.bai_total]
items = [
    "BAI1",   # Numbness or tingling
    "BAI2",   # Feeling hot
    "BAI3",   # Wobbliness in legs
    "BAI4",   # Unable to relax
    "BAI5",   # Fear of worst happening
    "BAI6",   # Dizzy or lightheaded
    "BAI7",   # Heart pounding/racing
    "BAI8",   # Unsteady
    "BAI9",   # Terrified or afraid
    "BAI10",  # Nervous
    "BAI11",  # Feeling of choking
    "BAI12",  # Hands trembling
    "BAI13",  # Shaky/unsteady
    "BAI14",  # Fear of losing control
    "BAI15",  # Difficulty in breathing
    "BAI16",  # Fear of dying
    "BAI17",  # Scared
    "BAI18",  # Indigestion
    "BAI19",  # Faint/lightheaded
    "BAI20",  # Face flushed
    "BAI21"   # Hot/cold sweats
]
reverse_scored = []

# Note: Ensure your CSV columns match the item names above (BAI1, BAI2, etc.)
"#
    .to_string()
}

fn bai_metadata() -> ScaleMetadata {
    ScaleMetadata {
        name: "BAI".to_string(),
        full_name: "Beck Anxiety Inventory".to_string(),
        citation: "Beck, A. T., Epstein, N., Brown, G., & Steer, R. A. (1988). An inventory for measuring clinical anxiety: psychometric properties. Journal of Consulting and Clinical Psychology, 56(6), 893-897.".to_string(),
        description: "21-item self-report measure of anxiety severity focusing on somatic symptoms".to_string(),
        num_items: 21,
        min_score: 0,
        max_score: 3,
        interpretation: "0-7: Minimal, 8-15: Mild, 16-25: Moderate, 26-63: Severe anxiety".to_string(),
        normative_data: Some(NormativeData {
            population: "Clinical outpatients".to_string(),
            mean: 15.8,
            sd: 11.9,
            clinical_cutoff: Some(16.0),
            severity_ranges: vec![
                ("Minimal anxiety".to_string(), 0.0, 7.0),
                ("Mild anxiety".to_string(), 8.0, 15.0),
                ("Moderate anxiety".to_string(), 16.0, 25.0),
                ("Severe anxiety".to_string(), 26.0, 63.0),
            ],
        }),
    }
}

// ============================================================================
// SWLS: Satisfaction With Life Scale
// ============================================================================

fn generate_swls_config() -> String {
    r#"# SWLS: Satisfaction With Life Scale
# Global life satisfaction measure
#
# Citation: Diener, E., Emmons, R. A., Larsen, R. J., & Griffin, S. (1985).
#           The Satisfaction With Life Scale.
#           Journal of Personality Assessment, 49(1), 71-75.
#
# Scoring: 1-7 scale (1=Strongly disagree, 2=Disagree, 3=Slightly disagree, 
#                     4=Neither agree nor disagree, 5=Slightly agree, 6=Agree, 7=Strongly agree)
# Interpretation:
#   31-35: Extremely satisfied
#   26-30: Satisfied
#   21-25: Slightly satisfied
#   20:    Neutral
#   15-19: Slightly dissatisfied
#   10-14: Dissatisfied
#   5-9:   Extremely dissatisfied

[survey]
name = "SWLS Satisfaction With Life"
min_score = 1
max_score = 7
participant_id_column = "ResponseId"

[quality]
max_missing_percent = 0.10
flag_straightlining = true

[output]
decimal_places = 2

[scales.swls_total]
items = [
    "SWLS1",  # In most ways my life is close to my ideal
    "SWLS2",  # The conditions of my life are excellent
    "SWLS3",  # I am satisfied with my life
    "SWLS4",  # So far I have gotten the important things I want in life
    "SWLS5"   # If I could live my life over, I would change almost nothing
]
reverse_scored = []

# Note: Ensure your CSV columns match the item names above (SWLS1, SWLS2, etc.)
"#
    .to_string()
}

fn swls_metadata() -> ScaleMetadata {
    ScaleMetadata {
        name: "SWLS".to_string(),
        full_name: "Satisfaction With Life Scale".to_string(),
        citation: "Diener, E., Emmons, R. A., Larsen, R. J., & Griffin, S. (1985). The Satisfaction With Life Scale. Journal of Personality Assessment, 49(1), 71-75.".to_string(),
        description: "5-item measure of global cognitive judgments of one's life satisfaction".to_string(),
        num_items: 5,
        min_score: 1,
        max_score: 7,
        interpretation: "31-35: Extremely satisfied, 26-30: Satisfied, 21-25: Slightly satisfied, 20: Neutral, 15-19: Slightly dissatisfied, 10-14: Dissatisfied, 5-9: Extremely dissatisfied".to_string(),
        normative_data: Some(NormativeData {
            population: "General adult population".to_string(),
            mean: 23.5,
            sd: 6.4,
            clinical_cutoff: None,
            severity_ranges: vec![
                ("Extremely dissatisfied".to_string(), 5.0, 9.0),
                ("Dissatisfied".to_string(), 10.0, 14.0),
                ("Slightly dissatisfied".to_string(), 15.0, 19.0),
                ("Neutral".to_string(), 20.0, 20.0),
                ("Slightly satisfied".to_string(), 21.0, 25.0),
                ("Satisfied".to_string(), 26.0, 30.0),
                ("Extremely satisfied".to_string(), 31.0, 35.0),
            ],
        }),
    }
}
