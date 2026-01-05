// src/lib.rs
//! # Prism - Psychology Survey Data Pipeline
//!
//! Prism is a comprehensive tool for processing psychology survey data with
//! automated scoring, quality control, and statistical reporting.
//!
//! ## Features
//!
//! - Automated reverse-scoring and scale computation
//! - Quality checks (missing data, straightlining, low variance)
//! - Statistical reporting with Cronbach's alpha
//! - Multiple output formats (CSV, Excel, JSON, SPSS, R)
//! - Parallel processing for large datasets
//! - Configurable via TOML files
//!
//! ## Example
//!
//! ```no_run
//! use prism::config::SurveyConfig;
//! use prism::processor::process_scale;
//! use std::fs;
//!
//! let config_content = fs::read_to_string("config.toml").unwrap();
//! let config: SurveyConfig = toml::from_str(&config_content).unwrap();
//! ```

pub mod config;
pub mod constants;
pub mod errors;
pub mod longitudinal;
pub mod output;
pub mod power;
pub mod processor;
pub mod quality;
pub mod scales;
pub mod stats;
pub mod types;
pub mod utils;
pub mod validation;
pub mod visualization;

// Re-export commonly used types
pub use config::SurveyConfig;
pub use errors::{ProcessingError, Result};
pub use scales::{NormativeData, ScaleMetadata};
pub use types::{MissingPercent, OutputFormat, QualityIssue, ScaleResult, ScoreValue};

// Re-export constants from central constants module
pub use constants::{
    DEFAULT_QUALITY_FILE, DEFAULT_STATS_FILE, FLOAT_EPSILON, PROGRESS_INTERVAL, QUALITY_FLAG_OK,
    QUALITY_FLAG_SEPARATOR,
};

/// Version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
pub const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
