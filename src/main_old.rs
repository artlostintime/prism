// src/main.rs
mod config;

use anyhow::{anyhow, Context, Result};
use clap::Parser;
use config::SurveyConfig;
use std::collections::HashMap;
use std::fs;

// Constants
const FLOAT_EPSILON: f64 = 1e-10;
const QUALITY_FLAG_OK: &str = "OK";
const QUALITY_FLAG_SEPARATOR: &str = "; ";
const PROGRESS_INTERVAL: usize = 100;
const DEFAULT_STATS_FILE: &str = "summary_stats.txt";
const DEFAULT_QUALITY_FILE: &str = "quality_report.txt";

// Helper struct for scale processing results
#[derive(Debug)]
struct ScaleResult {
    total: f64,
    mean: f64,
    valid_items: usize,
    item_values: Vec<f64>,
}

/// Prism - Psychology Survey Data Pipeline
#[derive(Parser)]
#[command(
    author,
    version,
    about = "Psychology survey data processing with automated scoring and quality control",
    long_about = "Prism transforms raw survey data into analysis-ready datasets with automated \nreverse-scoring, scale computation, quality checks, and statistical reporting. \nDesigned for psychology researchers who need accurate results fast."
)]
struct Cli {
    /// Path to the raw CSV file
    #[arg(short, long)]
    input: String,

    /// Path to the TOML configuration file
    #[arg(short, long)]
    config: String,

    /// Path to output the cleaned CSV
    #[arg(short, long, default_value = "clean_data.csv")]
    output: String,

    /// Path to output summary statistics (optional)
    #[arg(long)]
    stats_output: Option<String>,

    /// Path to output quality report (optional)
    #[arg(long)]
    quality_report: Option<String>,

    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,

    /// Dry run - validate config and show preview without writing output
    #[arg(long)]
    dry_run: bool,

    /// Generate all output files (stats and quality report)
    #[arg(long)]
    all_outputs: bool,
}

// Statistics structure for aggregate calculations
#[derive(Debug)]
struct Stats {
    mean: f64,
    sd: f64,
    min: f64,
    max: f64,
    n: usize,
}

impl Stats {
    fn calculate(values: &[f64]) -> Self {
        let n = values.len();
        if n == 0 {
            return Stats {
                mean: 0.0,
                sd: 0.0,
                min: 0.0,
                max: 0.0,
                n: 0,
            };
        }

        let mean = values.iter().sum::<f64>() / n as f64;
        let variance = if n > 1 {
            values.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / (n - 1) as f64
        } else {
            0.0
        };
        let sd = variance.sqrt();
        let min = values.iter().fold(f64::INFINITY, |a, &b| a.min(b));
        let max = values.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));

        Stats {
            mean,
            sd,
            min,
            max,
            n,
        }
    }
}

// Quality issue tracking
#[derive(Debug)]
struct QualityIssue {
    participant_id: String,
    issue_type: String,
    details: String,
}

impl QualityIssue {
    fn new(
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

fn validate_config(config: &SurveyConfig, headers: &[String]) -> Result<()> {
    // Check scale definitions
    if config.scales.is_empty() {
        return Err(anyhow!("No scales defined in config"));
    }

    // Check that all scale items exist in CSV headers
    for (scale_name, scale_def) in &config.scales {
        if scale_def.items.is_empty() {
            return Err(anyhow!("Scale '{}' has no items defined", scale_name));
        }

        for item in &scale_def.items {
            if !headers.contains(item) {
                return Err(anyhow!(
                    "Item '{}' from scale '{}' not found in CSV headers",
                    item,
                    scale_name
                ));
            }
        }

        // Check reverse-scored items are subset of items
        if let Some(reversed) = &scale_def.reverse_scored {
            for rev_item in reversed {
                if !scale_def.items.contains(rev_item) {
                    return Err(anyhow!(
                        "Reverse-scored item '{}' in scale '{}' not in items list",
                        rev_item,
                        scale_name
                    ));
                }
            }
        }
    }

    // Validate score ranges
    if config.survey.min_score >= config.survey.max_score {
        return Err(anyhow!(
            "min_score ({}) must be less than max_score ({})",
            config.survey.min_score,
            config.survey.max_score
        ));
    }

    Ok(())
}

fn main() -> Result<()> {
    let args = Cli::parse();

    // 1. Load Configuration
    let config_content = fs::read_to_string(&args.config).context(format!(
        "Could not read config file '{}'. Check if the file exists and has .toml extension",
        args.config
    ))?;
    let config: SurveyConfig = toml::from_str(&config_content).context(format!(
        "Could not parse TOML config from '{}'. Check for syntax errors",
        args.config
    ))?;

    if args.verbose {
        println!("✓ Configuration loaded successfully");
    }
    println!(
        "\n{} Processing Survey: {}",
        if args.dry_run { "[DRY RUN]" } else { "▸" },
        config.survey.name
    );

    // 2. Setup CSV Reader and Writer
    let mut reader = csv::Reader::from_path(&args.input).context(format!(
        "Could not open input CSV '{}'. Check if the file exists",
        args.input
    ))?;

    if args.verbose {
        println!("✓ Input CSV opened successfully");
    }

    let mut writer = if args.dry_run {
        csv::Writer::from_writer(vec![])
    } else {
        csv::Writer::from_path(&args.output).context(format!(
            "Could not create output CSV '{}'. Check write permissions",
            args.output
        ))?
    };

    // Get headers to map column names to indices
    let headers = reader.headers()?.clone();
    let header_vec: Vec<String> = headers.iter().map(|h| h.to_string()).collect();
    let header_map: HashMap<String, usize> = header_vec
        .iter()
        .enumerate()
        .map(|(i, name)| (name.clone(), i))
        .collect();

    // Validate config against CSV headers
    validate_config(&config, &header_vec)
        .context("Config validation failed. Check that scale items match CSV column names")?;

    if args.verbose {
        println!("✓ Configuration validated against CSV headers");
        println!("  - Found {} columns", header_vec.len());
        println!("  - Configured {} scales", config.scales.len());
    }

    // Prepare Output Headers (Original + New Scales + Flags)
    let mut out_headers = headers.iter().map(|h| h.to_string()).collect::<Vec<_>>();
    for scale_name in config.scales.keys() {
        out_headers.push(format!("{}_total", scale_name));
        out_headers.push(format!("{}_mean", scale_name));
    }
    out_headers.push("quality_flag".to_string());
    writer.write_record(&out_headers)?;

    // Determine output paths
    let stats_path = if args.all_outputs {
        Some(
            args.stats_output
                .unwrap_or_else(|| DEFAULT_STATS_FILE.to_string()),
        )
    } else {
        args.stats_output
    };

    let quality_path = if args.all_outputs {
        Some(
            args.quality_report
                .unwrap_or_else(|| DEFAULT_QUALITY_FILE.to_string()),
        )
    } else {
        args.quality_report
    };

    if args.dry_run {
        println!("\n[DRY RUN] Would process with:");
        println!("  Input:   {}", args.input);
        println!("  Output:  {}", args.output);
        if let Some(ref path) = stats_path {
            println!("  Stats:   {}", path);
        }
        if let Some(ref path) = quality_path {
            println!("  Quality: {}", path);
        }
        println!();
    }

    // 3. Process Each Participant
    let mut processed_count = 0;
    let mut flagged_count = 0;
    let mut scale_scores: HashMap<String, Vec<f64>> = HashMap::new();
    let mut quality_issues: Vec<QualityIssue> = Vec::new();

    // Initialize storage for each scale
    for scale_name in config.scales.keys() {
        scale_scores.insert(scale_name.clone(), Vec::new());
    }

    for result in reader.records() {
        let record = result?;
        let mut out_record: Vec<String> = record.iter().map(|s| s.to_string()).collect();
        let mut quality_flags = Vec::new();
        let participant_id = record.get(0).unwrap_or("Unknown").to_string();

        if args.verbose && processed_count < 3 {
            println!("  Processing participant: {}", participant_id);
        }

        // Process each scale defined in TOML
        for (scale_name, scale_def) in &config.scales {
            let (scale_result, missing_count) =
                process_scale(scale_def, &record, &header_map, &config)?;

            // Check missing data percentage
            check_missing_data(
                scale_name,
                missing_count,
                scale_def.items.len(),
                &participant_id,
                &config,
                &mut quality_flags,
                &mut quality_issues,
            );

            // Record results
            if scale_result.valid_items > 0 {
                out_record.push(format!("{:.2}", scale_result.total));
                out_record.push(format!("{:.2}", scale_result.mean));
                scale_scores
                    .get_mut(scale_name)
                    .unwrap()
                    .push(scale_result.mean);

                // Check straightlining
                check_straightlining(
                    scale_name,
                    &scale_result.item_values,
                    &participant_id,
                    &config,
                    &mut quality_flags,
                    &mut quality_issues,
                );

                // Check out-of-range (already done in process_scale, but record here)
            } else {
                out_record.push("NA".to_string());
                out_record.push("NA".to_string());
                let issue = format!("Missing: {}", scale_name);
                quality_flags.push(issue.clone());
                quality_issues.push(QualityIssue::new(
                    &participant_id,
                    "CompletelyMissing",
                    issue,
                ));
            }
        }

        let flag_str = if quality_flags.is_empty() {
            QUALITY_FLAG_OK.to_string()
        } else {
            flagged_count += 1;
            quality_flags.join(QUALITY_FLAG_SEPARATOR)
        };
        out_record.push(flag_str);

        writer.write_record(&out_record)?;
        processed_count += 1;

        // Progress indicator
        if !args.verbose && processed_count % PROGRESS_INTERVAL == 0 {
            println!("  Processed {} participants...", processed_count);
        }
    }

    writer.flush()?;

    let clean_count = processed_count - flagged_count;

    if args.dry_run {
        println!(
            "\n[DRY RUN] Would have processed {} participants",
            processed_count
        );
        return Ok(());
    }

    println!("\n{}", "═".repeat(50));
    println!("✓ Processing Complete");
    println!("{}", "═".repeat(50));
    println!("Total Participants:  {}", processed_count);
    println!(
        "Clean Records:       {} ({:.1}%)",
        clean_count,
        (clean_count as f64 / processed_count as f64) * 100.0
    );
    println!(
        "Flagged Records:     {} ({:.1}%)",
        flagged_count,
        (flagged_count as f64 / processed_count as f64) * 100.0
    );
    println!("Total Issues:        {}", quality_issues.len());
    println!("{}", "═".repeat(50));
    println!("Output saved to: {}", args.output);

    // 4. Generate Summary Statistics File (if requested)
    if let Some(stats_path) = &stats_path {
        generate_summary_stats(
            &config,
            &scale_scores,
            processed_count,
            stats_path,
            &quality_issues,
        )?;
        println!("Summary statistics saved to: {}", stats_path);
    }

    // 5. Generate Quality Report File (if requested)
    if let Some(quality_path) = &quality_path {
        generate_quality_report(&quality_issues, processed_count, quality_path)?;
        println!("Quality report saved to: {}", quality_path);
    }

    if args.verbose {
        println!("\n✓ All operations completed successfully");
    }

    Ok(())
}

/// Process a single scale for a participant
fn process_scale(
    scale_def: &config::ScaleDefinition,
    record: &csv::StringRecord,
    header_map: &HashMap<String, usize>,
    config: &SurveyConfig,
) -> Result<(ScaleResult, usize)> {
    let mut total_score = 0.0;
    let mut valid_items = 0;
    let mut item_values = Vec::new();
    let mut missing_count = 0;

    let min_score = config.survey.min_score as f64;
    let max_score = config.survey.max_score as f64;
    let score_range = max_score + min_score;

    for item_name in &scale_def.items {
        let idx = header_map
            .get(item_name)
            .ok_or_else(|| anyhow!("Item '{}' not found in CSV", item_name))?;

        let val_str = &record[*idx];

        if let Ok(val) = val_str.parse::<f64>() {
            // Skip out-of-range values (but don't fail)
            if val < min_score || val > max_score {
                missing_count += 1;
                continue;
            }

            // Reverse scoring if needed
            let final_val = if scale_def
                .reverse_scored
                .as_ref()
                .map_or(false, |rev| rev.contains(item_name))
            {
                score_range - val
            } else {
                val
            };

            total_score += final_val;
            valid_items += 1;
            item_values.push(final_val);
        } else {
            missing_count += 1;
        }
    }

    let mean = if valid_items > 0 {
        total_score / valid_items as f64
    } else {
        0.0
    };

    Ok((
        ScaleResult {
            total: total_score,
            mean,
            valid_items,
            item_values,
        },
        missing_count,
    ))
}

/// Check for missing data issues
fn check_missing_data(
    scale_name: &str,
    missing_count: usize,
    total_items: usize,
    participant_id: &str,
    config: &SurveyConfig,
    quality_flags: &mut Vec<String>,
    quality_issues: &mut Vec<QualityIssue>,
) {
    let missing_percent = missing_count as f64 / total_items as f64;
    if let Some(quality_settings) = &config.quality {
        if missing_percent > quality_settings.max_missing_percent {
            let issue = format!(
                "High missing data: {} ({:.1}% missing)",
                scale_name,
                missing_percent * 100.0
            );
            quality_flags.push(issue.clone());
            quality_issues.push(QualityIssue::new(participant_id, "MissingData", issue));
        }
    }
}

/// Check for straightlining
fn check_straightlining(
    scale_name: &str,
    item_values: &[f64],
    participant_id: &str,
    config: &SurveyConfig,
    quality_flags: &mut Vec<String>,
    quality_issues: &mut Vec<QualityIssue>,
) {
    if !config
        .quality
        .as_ref()
        .map_or(true, |q| q.flag_straightlining)
    {
        return;
    }

    if item_values.len() > 1 {
        let first = item_values[0];
        if item_values
            .iter()
            .all(|&x| (x - first).abs() < FLOAT_EPSILON)
        {
            let issue = format!("Straightlining: {}", scale_name);
            quality_flags.push(issue.clone());
            quality_issues.push(QualityIssue::new(participant_id, "Straightlining", issue));
        }
    }
}

fn generate_summary_stats(
    config: &SurveyConfig,
    scale_scores: &HashMap<String, Vec<f64>>,
    total_participants: usize,
    output_path: &str,
    quality_issues: &[QualityIssue],
) -> Result<()> {
    use std::io::Write;
    let mut file = fs::File::create(output_path)?;

    writeln!(
        file,
        "{} - Summary Statistics",
        config.survey.name.to_uppercase()
    )?;
    writeln!(
        file,
        "Generated: {}",
        chrono::Local::now().format("%Y-%m-%d %H:%M:%S")
    )?;
    writeln!(file)?;
    writeln!(file, "Total Participants: {}", total_participants)?;
    writeln!(
        file,
        "Complete Responses: {} ({:.1}%)",
        total_participants, 100.0
    )?;
    writeln!(file)?;

    for (scale_name, scores) in scale_scores {
        if let Some(scale_def) = config.scales.get(scale_name) {
            writeln!(file, "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━")?;
            writeln!(file)?;
            writeln!(
                file,
                "SCALE: {} ({} items)",
                scale_name,
                scale_def.items.len()
            )?;

            // Show items with reverse-scored markers
            write!(file, "Items: ")?;
            for (i, item) in scale_def.items.iter().enumerate() {
                if i > 0 {
                    write!(file, ", ")?;
                }
                if scale_def
                    .reverse_scored
                    .as_ref()
                    .map_or(false, |rev| rev.contains(item))
                {
                    write!(file, "{}*", item)?;
                } else {
                    write!(file, "{}", item)?;
                }
            }
            if scale_def
                .reverse_scored
                .as_ref()
                .map_or(false, |rev| !rev.is_empty())
            {
                writeln!(file, "  (* = reverse scored)")?;
            } else {
                writeln!(file)?;
            }
            writeln!(file)?;

            let stats = Stats::calculate(scores);
            writeln!(file, "  Mean (M)              = {:.2}", stats.mean)?;
            writeln!(file, "  Standard Deviation    = {:.2}", stats.sd)?;
            writeln!(
                file,
                "  Range                 = [{:.2}, {:.2}]",
                stats.min, stats.max
            )?;
            writeln!(file, "  N                     = {}", stats.n)?;
            writeln!(file)?;
        }
    }

    writeln!(file, "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━")?;
    writeln!(file)?;

    if quality_issues.is_empty() {
        writeln!(file, "DATA QUALITY: No issues detected")?;
    } else {
        writeln!(
            file,
            "DATA QUALITY: {} issues detected (see quality report for details)",
            quality_issues.len()
        )?;
    }

    Ok(())
}

fn generate_quality_report(
    quality_issues: &[QualityIssue],
    total_participants: usize,
    output_path: &str,
) -> Result<()> {
    use std::io::Write;
    let mut file = fs::File::create(output_path)?;

    writeln!(file, "DATA QUALITY REPORT")?;
    writeln!(
        file,
        "Generated: {}",
        chrono::Local::now().format("%Y-%m-%d %H:%M:%S")
    )?;
    writeln!(file)?;
    writeln!(file, "Total Participants: {}", total_participants)?;
    writeln!(file, "Flagged Issues: {}", quality_issues.len())?;
    writeln!(file)?;

    if quality_issues.is_empty() {
        writeln!(file, "✓ No quality issues detected. Data appears clean.")?;
    } else {
        writeln!(file, "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━")?;
        writeln!(file)?;

        // Group by issue type
        let mut by_type: HashMap<String, Vec<&QualityIssue>> = HashMap::new();
        for issue in quality_issues {
            by_type
                .entry(issue.issue_type.clone())
                .or_insert_with(Vec::new)
                .push(issue);
        }

        for (issue_type, issues) in &by_type {
            writeln!(file, "{} ({} occurrences):", issue_type, issues.len())?;
            writeln!(file)?;
            for issue in issues {
                writeln!(
                    file,
                    "  • Participant {}: {}",
                    issue.participant_id, issue.details
                )?;
            }
            writeln!(file)?;
        }

        writeln!(file, "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━")?;
        writeln!(file)?;
        writeln!(file, "RECOMMENDATIONS:")?;
        writeln!(file, "• Review flagged participants manually")?;
        writeln!(file, "• Consider excluding straightliners from analysis")?;
        writeln!(file, "• Check out-of-range values for data entry errors")?;
        writeln!(
            file,
            "• Assess whether missing data is random or systematic"
        )?;
    }

    Ok(())
}
