// tests/reproducibility_test.rs

use prism::config::{QualitySettings, ScaleDefinition, SurveyConfig, SurveySettings};
use prism::output::{generate_python_script, generate_r_script};
use std::collections::HashMap;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_r_script_basic_structure() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("test.R");
    let csv_path = "test_data.csv";

    let mut scales = HashMap::new();
    scales.insert(
        "anxiety".to_string(),
        ScaleDefinition {
            items: vec!["Q1".to_string(), "Q2".to_string(), "Q3".to_string()],
            reverse_scored: None,
        },
    );

    let config = SurveyConfig {
        survey: SurveySettings {
            name: "Test Survey".to_string(),
            min_score: 1,
            max_score: 5,
            participant_id_column: None,
        },
        quality: None,
        scales,
        column_mappings: None,
        output: None,
        longitudinal: None,
    };

    generate_r_script(csv_path, &config, output_path.to_str().unwrap()).unwrap();

    let content = fs::read_to_string(&output_path).unwrap();

    // Check for major sections
    assert!(content.contains("# R Analysis Script for:"));
    assert!(content.contains("SETUP: Install Required Packages"));
    assert!(content.contains("DATA IMPORT"));
    assert!(content.contains("QUALITY FILTERING"));
    assert!(content.contains("DESCRIPTIVE STATISTICS"));
    assert!(content.contains("RELIABILITY ANALYSIS"));
    assert!(content.contains("DATA VISUALIZATION"));
    assert!(content.contains("EXPORT RESULTS"));
}

#[test]
fn test_r_script_library_imports() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("test.R");

    let mut scales = HashMap::new();
    scales.insert(
        "scale1".to_string(),
        ScaleDefinition {
            items: vec!["Q1".to_string()],
            reverse_scored: None,
        },
    );

    let config = SurveyConfig {
        survey: SurveySettings {
            name: "Test".to_string(),
            min_score: 1,
            max_score: 5,
            participant_id_column: None,
        },
        quality: None,
        scales,
        column_mappings: None,
        output: None,
        longitudinal: None,
    };

    generate_r_script("data.csv", &config, output_path.to_str().unwrap()).unwrap();

    let content = fs::read_to_string(&output_path).unwrap();

    // Check required libraries
    assert!(content.contains("library(tidyverse)"));
    assert!(content.contains("library(psych)"));
    assert!(content.contains("library(ggplot2)"));
    assert!(content.contains("library(patchwork)"));
}

#[test]
fn test_r_script_data_import() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("test.R");

    let mut scales = HashMap::new();
    scales.insert(
        "scale1".to_string(),
        ScaleDefinition {
            items: vec!["Q1".to_string()],
            reverse_scored: None,
        },
    );

    let config = SurveyConfig {
        survey: SurveySettings {
            name: "Test".to_string(),
            min_score: 1,
            max_score: 5,
            participant_id_column: None,
        },
        quality: None,
        scales,
        column_mappings: None,
        output: None,
        longitudinal: None,
    };

    generate_r_script("my_data.csv", &config, output_path.to_str().unwrap()).unwrap();

    let content = fs::read_to_string(&output_path).unwrap();

    // Check data import
    assert!(content.contains("data <- read_csv('my_data.csv')"));
    assert!(content.contains("glimpse(data)"));
}

#[test]
fn test_r_script_quality_filtering() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("test.R");

    let mut scales = HashMap::new();
    scales.insert(
        "scale1".to_string(),
        ScaleDefinition {
            items: vec!["Q1".to_string()],
            reverse_scored: None,
        },
    );

    let config = SurveyConfig {
        survey: SurveySettings {
            name: "Test".to_string(),
            min_score: 1,
            max_score: 5,
            participant_id_column: None,
        },
        quality: Some(QualitySettings {
            max_missing_percent: 20.0,
            flag_straightlining: true,
            min_response_variance: Some(0.5),
            max_response_time: None,
            min_response_time: None,
            careless_responding_threshold: None,
        }),
        scales,
        column_mappings: None,
        output: None,
        longitudinal: None,
    };

    generate_r_script("data.csv", &config, output_path.to_str().unwrap()).unwrap();

    let content = fs::read_to_string(&output_path).unwrap();

    // Check quality filtering
    assert!(content.contains("table(data$quality_flag)"));
    assert!(content.contains("filter(quality_flag == 'FLAGGED')"));
    assert!(content.contains("clean_data <- data %>% filter(quality_flag == 'OK')"));
}

#[test]
fn test_r_script_reliability_analysis() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("test.R");

    let mut scales = HashMap::new();
    scales.insert(
        "anxiety".to_string(),
        ScaleDefinition {
            items: vec!["GAD1".to_string(), "GAD2".to_string(), "GAD3".to_string()],
            reverse_scored: None,
        },
    );

    let config = SurveyConfig {
        survey: SurveySettings {
            name: "GAD Test".to_string(),
            min_score: 0,
            max_score: 3,
            participant_id_column: None,
        },
        quality: None,
        scales,
        column_mappings: None,
        output: None,
        longitudinal: None,
    };

    generate_r_script("data.csv", &config, output_path.to_str().unwrap()).unwrap();

    let content = fs::read_to_string(&output_path).unwrap();

    // Check reliability analysis
    assert!(content.contains("anxiety_items <- clean_data %>% select(GAD1, GAD2, GAD3)"));
    assert!(content.contains("anxiety_alpha <- alpha(anxiety_items)"));
    assert!(content.contains("print(anxiety_alpha, digits = 3)"));
}

#[test]
fn test_r_script_visualizations() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("test.R");

    let mut scales = HashMap::new();
    scales.insert(
        "depression".to_string(),
        ScaleDefinition {
            items: vec!["PHQ1".to_string(), "PHQ2".to_string()],
            reverse_scored: None,
        },
    );

    let config = SurveyConfig {
        survey: SurveySettings {
            name: "PHQ Test".to_string(),
            min_score: 0,
            max_score: 3,
            participant_id_column: None,
        },
        quality: None,
        scales,
        column_mappings: None,
        output: None,
        longitudinal: None,
    };

    generate_r_script("data.csv", &config, output_path.to_str().unwrap()).unwrap();

    let content = fs::read_to_string(&output_path).unwrap();

    // Check visualizations
    assert!(content.contains("ggplot(clean_data, aes(x = depression_mean))"));
    assert!(content.contains("geom_histogram"));
    assert!(content.contains("geom_vline"));
    assert!(content.contains("ggsave('scale_distribution"));
}

#[test]
fn test_r_script_correlation_matrix() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("test.R");

    let mut scales = HashMap::new();
    scales.insert(
        "anxiety".to_string(),
        ScaleDefinition {
            items: vec!["Q1".to_string()],
            reverse_scored: None,
        },
    );
    scales.insert(
        "depression".to_string(),
        ScaleDefinition {
            items: vec!["Q2".to_string()],
            reverse_scored: None,
        },
    );

    let config = SurveyConfig {
        survey: SurveySettings {
            name: "Multi-Scale Test".to_string(),
            min_score: 1,
            max_score: 5,
            participant_id_column: None,
        },
        quality: None,
        scales,
        column_mappings: None,
        output: None,
        longitudinal: None,
    };

    generate_r_script("data.csv", &config, output_path.to_str().unwrap()).unwrap();

    let content = fs::read_to_string(&output_path).unwrap();

    // Check correlation matrix for multiple scales
    assert!(content.contains("CORRELATION MATRIX"));
    assert!(content.contains("scale_means <- clean_data %>% select("));
    assert!(content.contains("cor_matrix <- cor(scale_means"));
    assert!(content.contains("corrplot(cor_matrix"));
}

#[test]
fn test_r_script_export_results() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("test.R");

    let mut scales = HashMap::new();
    scales.insert(
        "scale1".to_string(),
        ScaleDefinition {
            items: vec!["Q1".to_string()],
            reverse_scored: None,
        },
    );

    let config = SurveyConfig {
        survey: SurveySettings {
            name: "Test".to_string(),
            min_score: 1,
            max_score: 5,
            participant_id_column: None,
        },
        quality: None,
        scales,
        column_mappings: None,
        output: None,
        longitudinal: None,
    };

    generate_r_script("data.csv", &config, output_path.to_str().unwrap()).unwrap();

    let content = fs::read_to_string(&output_path).unwrap();

    // Check export commands
    assert!(content.contains("write_csv(clean_data, 'clean_data_r.csv')"));
    assert!(content.contains("write_csv(summary_stats, 'summary_statistics.csv')"));
}

#[test]
fn test_python_script_basic_structure() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("test.py");
    let csv_path = "test_data.csv";

    let mut scales = HashMap::new();
    scales.insert(
        "anxiety".to_string(),
        ScaleDefinition {
            items: vec!["Q1".to_string(), "Q2".to_string(), "Q3".to_string()],
            reverse_scored: None,
        },
    );

    let config = SurveyConfig {
        survey: SurveySettings {
            name: "Test Survey".to_string(),
            min_score: 1,
            max_score: 5,
            participant_id_column: None,
        },
        quality: None,
        scales,
        column_mappings: None,
        output: None,
        longitudinal: None,
    };

    generate_python_script(csv_path, &config, output_path.to_str().unwrap()).unwrap();

    let content = fs::read_to_string(&output_path).unwrap();

    // Check for major sections
    assert!(content.contains("# Python Analysis Script for:"));
    assert!(content.contains("SETUP: Install Required Packages"));
    assert!(content.contains("DATA IMPORT"));
    assert!(content.contains("QUALITY FILTERING"));
    assert!(content.contains("DESCRIPTIVE STATISTICS"));
    assert!(content.contains("RELIABILITY ANALYSIS"));
    assert!(content.contains("DATA VISUALIZATION"));
    assert!(content.contains("EXPORT RESULTS"));
}

#[test]
fn test_python_script_imports() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("test.py");

    let mut scales = HashMap::new();
    scales.insert(
        "scale1".to_string(),
        ScaleDefinition {
            items: vec!["Q1".to_string()],
            reverse_scored: None,
        },
    );

    let config = SurveyConfig {
        survey: SurveySettings {
            name: "Test".to_string(),
            min_score: 1,
            max_score: 5,
            participant_id_column: None,
        },
        quality: None,
        scales,
        column_mappings: None,
        output: None,
        longitudinal: None,
    };

    generate_python_script("data.csv", &config, output_path.to_str().unwrap()).unwrap();

    let content = fs::read_to_string(&output_path).unwrap();

    // Check required imports
    assert!(content.contains("import pandas as pd"));
    assert!(content.contains("import numpy as np"));
    assert!(content.contains("import matplotlib.pyplot as plt"));
    assert!(content.contains("import seaborn as sns"));
    assert!(content.contains("import pingouin as pg"));
    assert!(content.contains("from scipy import stats"));
}

#[test]
fn test_python_script_data_import() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("test.py");

    let mut scales = HashMap::new();
    scales.insert(
        "scale1".to_string(),
        ScaleDefinition {
            items: vec!["Q1".to_string()],
            reverse_scored: None,
        },
    );

    let config = SurveyConfig {
        survey: SurveySettings {
            name: "Test".to_string(),
            min_score: 1,
            max_score: 5,
            participant_id_column: None,
        },
        quality: None,
        scales,
        column_mappings: None,
        output: None,
        longitudinal: None,
    };

    generate_python_script("my_data.csv", &config, output_path.to_str().unwrap()).unwrap();

    let content = fs::read_to_string(&output_path).unwrap();

    // Check data import
    assert!(content.contains("data = pd.read_csv('my_data.csv')"));
    assert!(content.contains("print(data.info())"));
}

#[test]
fn test_python_script_quality_filtering() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("test.py");

    let mut scales = HashMap::new();
    scales.insert(
        "scale1".to_string(),
        ScaleDefinition {
            items: vec!["Q1".to_string()],
            reverse_scored: None,
        },
    );

    let config = SurveyConfig {
        survey: SurveySettings {
            name: "Test".to_string(),
            min_score: 1,
            max_score: 5,
            participant_id_column: None,
        },
        quality: Some(QualitySettings {
            max_missing_percent: 20.0,
            flag_straightlining: true,
            min_response_variance: Some(0.5),
            max_response_time: None,
            min_response_time: None,
            careless_responding_threshold: None,
        }),
        scales,
        column_mappings: None,
        output: None,
        longitudinal: None,
    };

    generate_python_script("data.csv", &config, output_path.to_str().unwrap()).unwrap();

    let content = fs::read_to_string(&output_path).unwrap();

    // Check quality filtering
    assert!(content.contains("print(data['quality_flag'].value_counts())"));
    assert!(content.contains("flagged = data[data['quality_flag'] == 'FLAGGED']"));
    assert!(content.contains("clean_data = data[data['quality_flag'] == 'OK'].copy()"));
}

#[test]
fn test_python_script_reliability_analysis() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("test.py");

    let mut scales = HashMap::new();
    scales.insert(
        "anxiety".to_string(),
        ScaleDefinition {
            items: vec!["GAD1".to_string(), "GAD2".to_string(), "GAD3".to_string()],
            reverse_scored: None,
        },
    );

    let config = SurveyConfig {
        survey: SurveySettings {
            name: "GAD Test".to_string(),
            min_score: 0,
            max_score: 3,
            participant_id_column: None,
        },
        quality: None,
        scales,
        column_mappings: None,
        output: None,
        longitudinal: None,
    };

    generate_python_script("data.csv", &config, output_path.to_str().unwrap()).unwrap();

    let content = fs::read_to_string(&output_path).unwrap();

    // Check reliability analysis with pingouin
    assert!(content.contains("anxiety_items = clean_data[['GAD1', 'GAD2', 'GAD3']].dropna()"));
    assert!(content.contains("anxiety_alpha = pg.cronbach_alpha(data=anxiety_items)"));
    assert!(content.contains("print(f'Cronbach\\'s Alpha:"));
    assert!(content.contains("print(f'95% CI:"));
}

#[test]
fn test_python_script_visualizations() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("test.py");

    let mut scales = HashMap::new();
    scales.insert(
        "depression".to_string(),
        ScaleDefinition {
            items: vec!["PHQ1".to_string(), "PHQ2".to_string()],
            reverse_scored: None,
        },
    );

    let config = SurveyConfig {
        survey: SurveySettings {
            name: "PHQ Test".to_string(),
            min_score: 0,
            max_score: 3,
            participant_id_column: None,
        },
        quality: None,
        scales,
        column_mappings: None,
        output: None,
        longitudinal: None,
    };

    generate_python_script("data.csv", &config, output_path.to_str().unwrap()).unwrap();

    let content = fs::read_to_string(&output_path).unwrap();

    // Check visualizations
    assert!(content.contains("fig, axes = plt.subplots"));
    assert!(content.contains(".hist(clean_data['depression_mean']"));
    assert!(content.contains(".axvline(clean_data['depression_mean'].mean()"));
    assert!(content.contains("plt.savefig('scale_distributions.png'"));
}

#[test]
fn test_python_script_correlation_matrix() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("test.py");

    let mut scales = HashMap::new();
    scales.insert(
        "anxiety".to_string(),
        ScaleDefinition {
            items: vec!["Q1".to_string()],
            reverse_scored: None,
        },
    );
    scales.insert(
        "depression".to_string(),
        ScaleDefinition {
            items: vec!["Q2".to_string()],
            reverse_scored: None,
        },
    );

    let config = SurveyConfig {
        survey: SurveySettings {
            name: "Multi-Scale Test".to_string(),
            min_score: 1,
            max_score: 5,
            participant_id_column: None,
        },
        quality: None,
        scales,
        column_mappings: None,
        output: None,
        longitudinal: None,
    };

    generate_python_script("data.csv", &config, output_path.to_str().unwrap()).unwrap();

    let content = fs::read_to_string(&output_path).unwrap();

    // Check correlation matrix for multiple scales
    assert!(content.contains("CORRELATION MATRIX"));
    assert!(content.contains("scale_means = clean_data[["));
    assert!(content.contains("cor_matrix = scale_means.corr()"));
    assert!(content.contains("sns.heatmap(cor_matrix"));
}

#[test]
fn test_python_script_export_results() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("test.py");

    let mut scales = HashMap::new();
    scales.insert(
        "scale1".to_string(),
        ScaleDefinition {
            items: vec!["Q1".to_string()],
            reverse_scored: None,
        },
    );

    let config = SurveyConfig {
        survey: SurveySettings {
            name: "Test".to_string(),
            min_score: 1,
            max_score: 5,
            participant_id_column: None,
        },
        quality: None,
        scales,
        column_mappings: None,
        output: None,
        longitudinal: None,
    };

    generate_python_script("data.csv", &config, output_path.to_str().unwrap()).unwrap();

    let content = fs::read_to_string(&output_path).unwrap();

    // Check export commands
    assert!(content.contains("clean_data.to_csv('clean_data_python.csv', index=False)"));
    assert!(content.contains("summary_stats.to_csv('summary_statistics.csv')"));
}

#[test]
fn test_python_script_summary_table() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("test.py");

    let mut scales = HashMap::new();
    scales.insert(
        "anxiety".to_string(),
        ScaleDefinition {
            items: vec!["Q1".to_string()],
            reverse_scored: None,
        },
    );

    let config = SurveyConfig {
        survey: SurveySettings {
            name: "Test".to_string(),
            min_score: 1,
            max_score: 5,
            participant_id_column: None,
        },
        quality: None,
        scales,
        column_mappings: None,
        output: None,
        longitudinal: None,
    };

    generate_python_script("data.csv", &config, output_path.to_str().unwrap()).unwrap();

    let content = fs::read_to_string(&output_path).unwrap();

    // Check summary table creation
    assert!(content.contains("summary_stats = pd.DataFrame({"));
    assert!(content.contains("index=['Mean', 'SD', 'Min', 'Max']"));
    assert!(content.contains("clean_data['anxiety_mean'].mean()"));
    assert!(content.contains("clean_data['anxiety_mean'].std()"));
}
