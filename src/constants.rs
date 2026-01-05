// src/constants.rs
//! Central repository for all constants used throughout the application

/// Floating point comparison epsilon for quality checks
pub const FLOAT_EPSILON: f64 = 1e-10;

/// Default number of bins for histogram generation
pub const DEFAULT_HISTOGRAM_BINS: usize = 15;

/// Minimum items required for pattern detection
pub const MIN_PATTERN_ITEMS: usize = 4;

/// Minimum items required for block pattern detection
pub const MIN_BLOCK_ITEMS: usize = 6;

// Quality Issue Type Constants
/// Missing data quality issue identifier
pub const ISSUE_MISSING_DATA: &str = "MissingData";

/// Straightlining quality issue identifier
pub const ISSUE_STRAIGHTLINING: &str = "Straightlining";

/// Diagonal pattern quality issue identifier
pub const ISSUE_DIAGONAL_PATTERN: &str = "DiagonalPattern";

/// Alternating pattern quality issue identifier
pub const ISSUE_ALTERNATING_PATTERN: &str = "AlternatingPattern";

/// Block pattern quality issue identifier
pub const ISSUE_BLOCK_PATTERN: &str = "BlockPattern";

/// Low variance quality issue identifier
pub const ISSUE_LOW_VARIANCE: &str = "LowVariance";

/// Fast response time quality issue identifier
pub const ISSUE_FAST_RESPONSE: &str = "FastResponse";

/// Slow response time quality issue identifier
pub const ISSUE_SLOW_RESPONSE: &str = "SlowResponse";

/// Semantic inconsistency quality issue identifier
pub const ISSUE_SEMANTIC_INCONSISTENCY: &str = "SemanticInconsistency";

// Pattern Descriptions (for user-friendly display)
/// Description for diagonal pattern detection
pub const DESC_DIAGONAL_PATTERN: &str = "Sequential patterns (e.g., 1,2,3,4,5)";

/// Description for alternating pattern detection
pub const DESC_ALTERNATING_PATTERN: &str = "Alternating responses (e.g., 1,5,1,5)";

/// Description for block pattern detection
pub const DESC_BLOCK_PATTERN: &str = "Response blocks (e.g., all 1s then all 5s)";

/// Description for straightlining detection
pub const DESC_STRAIGHTLINING: &str = "Identical responses to all items";

// Careless Responding Score Weights
/// Weight for missing data in careless responding calculation
pub const WEIGHT_MISSING_DATA: f64 = 0.3;

/// Weight for straightlining in careless responding calculation
pub const WEIGHT_STRAIGHTLINING: f64 = 0.5;

/// Weight for low variance in careless responding calculation
pub const WEIGHT_LOW_VARIANCE: f64 = 0.2;

// Output and Processing Constants
/// Quality flag indicating no issues detected
pub const QUALITY_FLAG_OK: &str = "OK";

/// Separator for multiple quality flags
pub const QUALITY_FLAG_SEPARATOR: &str = "; ";

/// Progress bar update interval (in records processed)
pub const PROGRESS_INTERVAL: usize = 100;

/// Default statistics output filename
pub const DEFAULT_STATS_FILE: &str = "summary_stats.txt";

/// Default quality report filename
pub const DEFAULT_QUALITY_FILE: &str = "quality_report.txt";
