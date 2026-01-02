// src/errors.rs
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProcessingError {
    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Data error at row {row}: {message}")]
    DataError { row: usize, message: String },

    #[error("Missing column: {0}")]
    MissingColumn(String),

    #[error("Invalid value in column '{column}': {value}")]
    InvalidValue { column: String, value: String },

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("CSV error: {0}")]
    CsvError(#[from] csv::Error),

    #[error("TOML parsing error: {0}")]
    TomlError(#[from] toml::de::Error),

    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("Excel error: {0}")]
    ExcelError(#[from] rust_xlsxwriter::XlsxError),

    #[error("{0}")]
    Other(#[from] anyhow::Error),
}

pub type Result<T> = std::result::Result<T, ProcessingError>;
