// src/main.rs
use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use indicatif::{ProgressBar, ProgressStyle};
use log::{debug, info};
use prism::{
    config::SurveyConfig,
    output::*,
    processor::{get_participant_id, process_quality_checks, process_scale, QualityCheckParams},
    scales,
    stats::Stats,
    types::{OutputFormat, QualityIssue},
    utils,
    validation::{generate_config_template, validate_batch_file, validate_config},
    visualization, DEFAULT_QUALITY_FILE, DEFAULT_STATS_FILE, QUALITY_FLAG_OK,
    QUALITY_FLAG_SEPARATOR,
};
use std::collections::HashMap;
use std::fs;
use std::time::Instant;

/// Prism - Psychology Survey Data Pipeline
#[derive(Parser)]
#[command(
    author,
    version,
    about = "Psychology survey data processing with automated scoring and quality control",
    long_about = "Prism transforms raw survey data into analysis-ready datasets with automated \nreverse-scoring, scale computation, quality checks, and statistical reporting. \nDesigned for psychology researchers who need accurate results fast.",
    after_help = "EXAMPLES:\n    prism process -i data.csv -c config.toml -o clean.csv\n    prism process -i data.csv -c config.toml --all-outputs\n    prism validate -c config.toml -i data.csv\n    prism generate --template > config.toml\n    prism process --batch files.txt -c config.toml"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Enable verbose output
    #[arg(short, long, global = true)]
    verbose: bool,

    /// Quiet mode (minimal output)
    #[arg(short, long, global = true)]
    quiet: bool,

    /// Disable colored output
    #[arg(long, global = true)]
    no_color: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Process survey data (main command)
    Process {
        /// Path to the raw CSV file
        #[arg(short, long)]
        input: Option<String>,

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

        /// Dry run - validate config and show preview without writing output
        #[arg(long)]
        dry_run: bool,

        /// Generate all output files (stats and quality report)
        #[arg(long)]
        all_outputs: bool,

        /// Export to all available formats (CSV, Excel, SPSS, R, JSON)
        #[arg(long)]
        export_all: bool,

        /// Output format
        #[arg(long, value_enum, default_value = "csv")]
        format: OutputFormat,

        /// Path to JSON output (if --format json)
        #[arg(long)]
        json_output: Option<String>,

        /// Process multiple files from batch list
        #[arg(long)]
        batch: Option<String>,

        /// Run benchmark mode
        #[arg(long)]
        benchmark: bool,
    },

    /// Validate configuration and CSV without processing
    Validate {
        /// Path to the TOML configuration file
        #[arg(short, long)]
        config: String,

        /// Path to the CSV file to validate against
        #[arg(short, long)]
        input: String,
    },

    /// Generate configuration template or examples
    Generate {
        /// Generate a sample configuration template
        #[arg(long)]
        template: bool,

        /// Generate config for a pre-built scale (PHQ-9, GAD-7, PSS-10, PSS-14, PANAS, BDI-II, BAI, SWLS)
        #[arg(long)]
        scale: Option<String>,

        /// List all available pre-built scales
        #[arg(long)]
        list_scales: bool,

        /// Show detailed information about a scale
        #[arg(long)]
        scale_info: Option<String>,
    },

    /// Merge multiple waves of longitudinal data
    Merge {
        /// Paths to wave files in format: wave1:file1.csv,wave2:file2.csv
        #[arg(short, long, required = true, value_delimiter = ',')]
        waves: Vec<String>,

        /// ID column name for matching participants across waves
        #[arg(short, long, default_value = "ParticipantID")]
        id_column: String,

        /// Output file path
        #[arg(short, long)]
        output: String,

        /// Use inner join (only include participants in all waves)
        #[arg(long)]
        inner_join: bool,
    },

    /// Reshape data between wide and long formats
    Reshape {
        /// Input file path
        #[arg(short, long)]
        input: String,

        /// Output file path
        #[arg(short, long)]
        output: String,

        /// Target format (wide or long)
        #[arg(short, long)]
        format: String,

        /// ID column name
        #[arg(long, default_value = "ParticipantID")]
        id_column: String,

        /// Time/wave column name (required for long format)
        #[arg(long, default_value = "Wave")]
        time_column: String,

        /// Wave names (comma-separated, e.g., T1,T2,T3)
        #[arg(short, long, value_delimiter = ',')]
        waves: Vec<String>,

        /// Variable names to reshape (comma-separated, optional)
        #[arg(long, value_delimiter = ',')]
        variables: Vec<String>,
    },

    /// Calculate Reliable Change Index (RCI) between two time points
    Rci {
        /// Baseline (T1) data file
        #[arg(short, long)]
        baseline: String,

        /// Follow-up (T2) data file
        #[arg(short, long)]
        followup: String,

        /// Scale/variable name to analyze
        #[arg(short, long)]
        scale: String,

        /// ID column name
        #[arg(long, default_value = "ParticipantID")]
        id_column: String,

        /// Test-retest reliability coefficient (0-1)
        #[arg(short, long)]
        reliability: f64,

        /// Baseline standard deviation (optional, will be calculated if not provided)
        #[arg(long)]
        baseline_sd: Option<f64>,

        /// Output file path
        #[arg(short, long)]
        output: String,
    },

    /// Statistical power analysis for study planning
    Power {
        /// Test type (independent-t, paired-t, one-sample-t, correlation)
        #[arg(short, long)]
        test: String,

        /// Effect size (Cohen's d for t-tests, r for correlation)
        #[arg(short, long)]
        effect_size: f64,

        /// Significance level (alpha)
        #[arg(short, long, default_value = "0.05")]
        alpha: f64,

        /// Desired statistical power (for a priori, default 0.80)
        #[arg(short = 'p', long)]
        power: Option<f64>,

        /// Sample size (for post-hoc power calculation)
        #[arg(short = 'n', long)]
        sample_size: Option<usize>,

        /// Number of tails (1 or 2)
        #[arg(long, default_value = "2")]
        tails: u8,

        /// Output file path (optional, prints to console if not specified)
        #[arg(short, long)]
        output: Option<String>,
    },

    /// Generate data dictionary documenting dataset structure
    Dictionary {
        /// Path to the TOML configuration file
        #[arg(short, long)]
        config: String,

        /// Output file path
        #[arg(short, long)]
        output: String,

        /// Output format (csv or json)
        #[arg(short, long, default_value = "csv")]
        format: String,
    },
}

fn main() -> Result<()> {
    // Detect if running without arguments (double-clicked)
    if std::env::args().len() == 1 {
        show_interactive_help_and_install();
        return Ok(());
    }

    let args = Cli::parse();

    // Initialize logger
    let log_level = if args.verbose {
        "debug"
    } else if args.quiet {
        "error"
    } else {
        "info"
    };

    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(log_level))
        .format_timestamp(None)
        .init();

    match args.command {
        Commands::Process {
            input,
            config,
            output,
            stats_output,
            quality_report,
            dry_run,
            all_outputs,
            export_all,
            format,
            json_output,
            batch,
            benchmark,
        } => {
            if benchmark {
                run_benchmark(&input, &config)?;
            } else if let Some(batch_path) = batch {
                process_batch(&batch_path, &config, &output, stats_output, quality_report)?;
            } else {
                let input = input.ok_or_else(|| anyhow::anyhow!("--input is required"))?;
                process_file(ProcessingOptions {
                    input,
                    config_path: config,
                    output,
                    stats_output,
                    quality_report,
                    dry_run,
                    all_outputs,
                    export_all,
                    format,
                    json_output,
                    quiet: args.quiet,
                })?;
            }
        }
        Commands::Validate { config, input } => {
            validate_command(&config, &input)?;
        }
        Commands::Generate {
            template,
            scale,
            list_scales,
            scale_info,
        } => {
            if list_scales {
                println!("📚 Available Pre-built Scales:\n");
                for scale_name in scales::list_available_scales() {
                    if let Ok(metadata) = scales::get_scale_metadata(&scale_name) {
                        println!("  • {} - {}", scale_name, metadata.full_name);
                        println!(
                            "    {} items, Citation: {}",
                            metadata.num_items,
                            metadata
                                .citation
                                .split('.')
                                .next()
                                .unwrap_or(&metadata.citation)
                        );
                        println!();
                    }
                }
                println!("\nUsage:");
                println!("  prism generate --scale PHQ-9 > phq9_config.toml");
                println!("  prism generate --scale-info GAD-7");
            } else if let Some(scale_id) = scale_info {
                match scales::get_scale_metadata(&scale_id) {
                    Ok(metadata) => {
                        println!("\n{} ({})", metadata.name, metadata.full_name);
                        println!("{}", "=".repeat(60));
                        println!("\n📖 Description:");
                        println!("  {}", metadata.description);
                        println!("\n📝 Citation:");
                        println!("  {}", metadata.citation);
                        println!("\n🔢 Scale Details:");
                        println!("  • Number of items: {}", metadata.num_items);
                        println!(
                            "  • Score range: {}-{}",
                            metadata.min_score, metadata.max_score
                        );
                        println!("\n📊 Interpretation:");
                        println!("  {}", metadata.interpretation);

                        if let Some(norm_data) = metadata.normative_data {
                            println!("\n📈 Normative Data ({}):", norm_data.population);
                            println!("  • Mean: {:.2}, SD: {:.2}", norm_data.mean, norm_data.sd);
                            if let Some(cutoff) = norm_data.clinical_cutoff {
                                println!("  • Clinical cutoff: {:.2}", cutoff);
                            }
                            if !norm_data.severity_ranges.is_empty() {
                                println!("\n  Severity Ranges:");
                                for (label, min, max) in norm_data.severity_ranges {
                                    println!("    • {}: {:.0}-{:.0}", label, min, max);
                                }
                            }
                        }

                        println!("\n💡 To generate config:");
                        println!("  prism generate --scale {} > config.toml", metadata.name);
                    }
                    Err(e) => {
                        eprintln!("Error: {}", e);
                        eprintln!("\nRun 'prism generate --list-scales' to see available scales");
                        std::process::exit(1);
                    }
                }
            } else if let Some(scale_id) = scale {
                match scales::generate_scale_config(&scale_id) {
                    Ok(config) => {
                        println!("{}", config);
                    }
                    Err(e) => {
                        eprintln!("Error: {}", e);
                        eprintln!("\nRun 'prism generate --list-scales' to see available scales");
                        std::process::exit(1);
                    }
                }
            } else if template {
                println!("{}", generate_config_template());
            } else {
                eprintln!(
                    "Error: Please specify --template, --scale, --list-scales, or --scale-info"
                );
                eprintln!("\nExamples:");
                eprintln!("  prism generate --template");
                eprintln!("  prism generate --list-scales");
                eprintln!("  prism generate --scale PHQ-9");
                eprintln!("  prism generate --scale-info GAD-7");
                std::process::exit(1);
            }
        }
        Commands::Merge {
            waves,
            id_column,
            output,
            inner_join,
        } => {
            use prism::longitudinal::{merge_waves, MergeParams};

            // Parse wave specifications (format: "T1:file1.csv")
            let wave_files: Result<Vec<(String, String)>, _> = waves
                .iter()
                .map(|spec| {
                    // Split on first colon only to handle Windows paths like C:\path\file.csv
                    if let Some(colon_pos) = spec.find(':') {
                        let wave_name = &spec[..colon_pos];
                        let file_path = &spec[colon_pos + 1..];

                        if wave_name.is_empty() || file_path.is_empty() {
                            Err(anyhow::anyhow!(
                                "Invalid wave specification '{}'. Expected format: 'wave:file.csv'",
                                spec
                            ))
                        } else {
                            Ok((wave_name.to_string(), file_path.to_string()))
                        }
                    } else {
                        Err(anyhow::anyhow!(
                            "Invalid wave specification '{}'. Expected format: 'wave:file.csv'",
                            spec
                        ))
                    }
                })
                .collect();

            let wave_files = wave_files.context("Failed to parse wave specifications")?;

            if wave_files.is_empty() {
                eprintln!("Error: No wave files specified");
                eprintln!("\nExample:");
                eprintln!("  prism merge -w T1:data_t1.csv,T2:data_t2.csv -o merged.csv");
                std::process::exit(1);
            }

            info!("Merging {} waves into {}", wave_files.len(), output);

            let params = MergeParams {
                wave_files,
                id_column,
                output_path: output,
                inner_join,
            };

            match merge_waves(params) {
                Ok(n) => {
                    println!("✓ Successfully merged {} participants", n);
                }
                Err(e) => {
                    eprintln!("Error merging waves: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Commands::Reshape {
            input,
            output,
            format,
            id_column,
            time_column,
            waves,
            variables,
        } => {
            use prism::longitudinal::{reshape_data, DataFormat, ReshapeParams};

            let target_format = match format.to_lowercase().as_str() {
                "wide" => DataFormat::Wide,
                "long" => DataFormat::Long,
                _ => {
                    eprintln!("Error: Format must be 'wide' or 'long'");
                    std::process::exit(1);
                }
            };

            if waves.is_empty() {
                eprintln!("Error: Wave names must be specified (e.g., --waves T1,T2,T3)");
                std::process::exit(1);
            }

            info!("Reshaping {} to {} format", input, format);

            let params = ReshapeParams {
                input_path: input,
                output_path: output,
                target_format,
                id_column,
                time_column,
                variables,
                waves,
            };

            match reshape_data(params) {
                Ok(n) => {
                    println!("✓ Successfully reshaped data ({} rows)", n);
                }
                Err(e) => {
                    eprintln!("Error reshaping data: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Commands::Rci {
            baseline,
            followup,
            scale,
            id_column,
            reliability,
            baseline_sd,
            output,
        } => {
            use prism::longitudinal::{calculate_rci, RCIParams};

            if !(0.0..=1.0).contains(&reliability) {
                eprintln!("Error: Reliability must be between 0 and 1");
                std::process::exit(1);
            }

            info!(
                "Calculating RCI for {} (reliability = {:.2})",
                scale, reliability
            );

            let params = RCIParams {
                baseline_path: baseline,
                followup_path: followup,
                scale_name: scale,
                id_column,
                reliability,
                baseline_sd,
                output_path: output.clone(),
            };

            match calculate_rci(params) {
                Ok(results) => {
                    let reliable_count = results.iter().filter(|r| r.is_reliable).count();
                    let improved_count = results
                        .iter()
                        .filter(|r| r.direction == "decreased")
                        .count();
                    let worsened_count = results
                        .iter()
                        .filter(|r| r.direction == "increased")
                        .count();

                    println!("\n✓ RCI Analysis Complete");
                    println!("{}", "=".repeat(50));
                    println!("Total participants: {}", results.len());
                    let reliable_pct = utils::calculate_percentage(reliable_count, results.len());
                    println!("Reliable change: {} ({:.1}%)", reliable_count, reliable_pct);
                    println!("  • Decreased: {}", improved_count);
                    println!("  • Increased: {}", worsened_count);
                    println!("\nResults saved to: {}", output);
                }
                Err(e) => {
                    eprintln!("Error calculating RCI: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Commands::Power {
            test,
            effect_size,
            alpha,
            power,
            sample_size,
            tails,
            output,
        } => {
            use prism::power::{
                calculate_observed_power, calculate_sample_size, interpret_effect_size,
                APrioriParams, PostHocParams, TestType,
            };

            // Parse test type
            let test_type = match test.to_lowercase().as_str() {
                "independent-t" | "indep-t" | "independent" => TestType::IndependentT,
                "paired-t" | "paired" => TestType::PairedT,
                "one-sample-t" | "one-sample" => TestType::OneSampleT,
                "correlation" | "corr" | "r" => TestType::Correlation,
                _ => {
                    eprintln!(
                        "Error: Unknown test type '{}'. Supported: independent-t, paired-t, one-sample-t, correlation",
                        test
                    );
                    std::process::exit(1);
                }
            };

            // Determine if this is a priori or post-hoc
            let result = if let Some(n) = sample_size {
                // Post-hoc: Calculate observed power from sample size
                info!("Calculating observed power for {} with n={}", test, n);

                let params = PostHocParams {
                    test_type: test_type.clone(),
                    effect_size,
                    sample_size: n,
                    alpha,
                    tails,
                };

                calculate_observed_power(&params)
            } else if let Some(desired_power) = power {
                // A priori: Calculate required sample size
                info!(
                    "Calculating required sample size for {} with power={}",
                    test, desired_power
                );

                let params = APrioriParams {
                    test_type: test_type.clone(),
                    effect_size,
                    alpha,
                    power: desired_power,
                    tails,
                };

                calculate_sample_size(&params)
            } else {
                eprintln!("Error: Must specify either --power (for sample size calculation) or --sample-size (for power calculation)");
                std::process::exit(1);
            };

            match result {
                Ok(r) => {
                    let effect_interpretation = interpret_effect_size(&test_type, effect_size);

                    // Format output
                    let output_text = format!(
                        "\n{}\nPower Analysis Results\n{}\n\nTest Type:       {}\nEffect Size:     {:.3} ({})\nAlpha:           {:.3}\nTails:           {}\nSample Size:     {}\nPower:           {:.3} ({:.1}%)\n\nCritical Value:  {:.3}\n{}\n{}\n",
                        "=".repeat(60),
                        "=".repeat(60),
                        r.test_type,
                        r.effect_size,
                        effect_interpretation,
                        r.alpha,
                        tails,
                        r.sample_size,
                        r.power,
                        r.power * 100.0,
                        r.critical_value,
                        r.interpretation,
                        "=".repeat(60)
                    );

                    // Print to console
                    println!("{}", output_text);

                    // Save to file if specified
                    if let Some(output_path) = output {
                        use std::fs::File;
                        use std::io::Write;

                        let mut file = File::create(&output_path)
                            .context(format!("Could not create output file '{}'", output_path))?;
                        file.write_all(output_text.as_bytes())?;
                        println!("✓ Results saved to: {}", output_path);
                    }
                }
                Err(e) => {
                    eprintln!("Error in power analysis: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Commands::Dictionary {
            config,
            output,
            format,
        } => {
            use prism::output::{generate_data_dictionary_csv, generate_data_dictionary_json};

            // Load configuration
            let config_content = match fs::read_to_string(&config) {
                Ok(content) => content,
                Err(e) => {
                    eprintln!("Error reading config file '{}': {}", config, e);
                    std::process::exit(1);
                }
            };

            let survey_config: SurveyConfig = match toml::from_str(&config_content) {
                Ok(cfg) => cfg,
                Err(e) => {
                    eprintln!("Error parsing TOML config from '{}': {}", config, e);
                    std::process::exit(1);
                }
            };

            // Generate dictionary based on format
            let result = match format.to_lowercase().as_str() {
                "csv" => generate_data_dictionary_csv(&survey_config, &output),
                "json" => generate_data_dictionary_json(&survey_config, &output),
                _ => {
                    eprintln!("Error: Unknown format '{}'. Supported: csv, json", format);
                    std::process::exit(1);
                }
            };

            match result {
                Ok(_) => {
                    println!("✓ Data dictionary generated successfully: {}", output);
                }
                Err(e) => {
                    eprintln!("Error generating data dictionary: {}", e);
                    std::process::exit(1);
                }
            }
        }
    }

    Ok(())
}

/// Configuration options for processing a file
struct ProcessingOptions {
    input: String,
    config_path: String,
    output: String,
    stats_output: Option<String>,
    quality_report: Option<String>,
    dry_run: bool,
    all_outputs: bool,
    export_all: bool,
    format: OutputFormat,
    json_output: Option<String>,
    quiet: bool,
}

fn process_file(options: ProcessingOptions) -> Result<()> {
    let start_time = Instant::now();

    // 1. Load Configuration
    let config_content = fs::read_to_string(&options.config_path).context(format!(
        "Could not read config file '{}'. Check if the file exists and has .toml extension",
        options.config_path
    ))?;
    let config: SurveyConfig = toml::from_str(&config_content).context(format!(
        "Could not parse TOML config from '{}'. Check for syntax errors",
        options.config_path
    ))?;

    info!("✓ Configuration loaded successfully");
    if !options.quiet {
        println!(
            "\n{} Processing Survey: {}",
            if options.dry_run { "[DRY RUN]" } else { "▸" },
            config.survey.name
        );
    }

    // 2. Setup CSV Reader
    let mut reader = csv::Reader::from_path(&options.input).context(format!(
        "Could not open input CSV '{}'. Check if the file exists",
        options.input
    ))?;

    info!("✓ Input CSV opened successfully");

    let headers = reader.headers()?;
    let header_vec: Vec<String> = headers.iter().map(|h| h.to_string()).collect();
    let header_map: HashMap<String, usize> = header_vec
        .iter()
        .enumerate()
        .map(|(i, name)| (name.to_string(), i)) // Avoid double allocation
        .collect();
    let headers = headers.clone(); // Clone only if needed later

    // Validate config against CSV headers
    validate_config(&config, &header_vec)
        .context("Config validation failed. Check that scale items match CSV column names")?;

    info!("✓ Configuration validated against CSV headers");
    debug!("Found {} columns", header_vec.len());
    debug!("Configured {} scales", config.scales.len());

    // Preview in dry run mode
    if options.dry_run && !options.quiet {
        show_preview(&mut reader, &header_vec, &config)?;
        return Ok(());
    }

    // Prepare output
    let mut writer = csv::Writer::from_path(&options.output).context(format!(
        "Could not create output CSV '{}'. Check write permissions",
        options.output
    ))?;

    let mut out_headers = headers.iter().map(|h| h.to_string()).collect::<Vec<_>>();
    for scale_name in config.scales.keys() {
        out_headers.push(format!("{}_total", scale_name));
        out_headers.push(format!("{}_mean", scale_name));
    }
    out_headers.push("quality_flag".to_string());
    writer.write_record(&out_headers)?;

    // Determine output paths
    let stats_path = if options.all_outputs {
        Some(
            options
                .stats_output
                .unwrap_or_else(|| DEFAULT_STATS_FILE.to_string()),
        )
    } else {
        options.stats_output
    };

    let quality_path = if options.all_outputs {
        Some(
            options
                .quality_report
                .unwrap_or_else(|| DEFAULT_QUALITY_FILE.to_string()),
        )
    } else {
        options.quality_report
    };

    // 3. Process Each Participant
    let mut processed_count = 0;
    let mut flagged_count = 0;
    let mut scale_scores: HashMap<String, Vec<f64>> = HashMap::new();
    let mut scale_items_matrix: HashMap<String, Vec<Vec<f64>>> = HashMap::new();
    let mut quality_issues: Vec<QualityIssue> = Vec::new();
    let mut all_records: Vec<Vec<String>> = Vec::new();

    // Count total records for progress bar
    let total_records = reader.records().count();

    // Initialize storage with pre-allocated capacity
    for scale_name in config.scales.keys() {
        scale_scores.insert(scale_name.clone(), Vec::with_capacity(total_records));
        scale_items_matrix.insert(scale_name.clone(), Vec::with_capacity(total_records));
    }
    let mut reader = csv::Reader::from_path(&options.input)?; // Re-open after counting
    reader.headers()?; // Skip headers

    let pb = if !options.quiet {
        let pb = ProgressBar::new(total_records as u64);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} {msg}")
                .unwrap()
                .progress_chars("█▓▒░"),
        );
        Some(pb)
    } else {
        None
    };

    for (row_num, result) in reader.records().enumerate() {
        let record = result?;
        let mut out_record: Vec<String> = record.iter().map(|s| s.to_string()).collect();
        let mut quality_flags = Vec::new();
        let participant_id = get_participant_id(&record, &header_map, &config, row_num + 1);

        debug!("Processing participant: {}", participant_id);

        // Process each scale
        for (scale_name, scale_def) in &config.scales {
            let (scale_result, missing_count) =
                process_scale(scale_def, &record, &header_map, &config)?;

            // Quality checks
            process_quality_checks(QualityCheckParams {
                scale_name,
                scale_result: &scale_result,
                missing_count,
                total_items: scale_def.items.len(),
                participant_id: &participant_id,
                config: &config,
                quality_flags: &mut quality_flags,
                quality_issues: &mut quality_issues,
            });

            // Record results
            if scale_result.valid_items > 0 {
                let decimal_places = config
                    .output
                    .as_ref()
                    .map(|o| o.decimal_places)
                    .unwrap_or(2);

                out_record.push(format!(
                    "{:.prec$}",
                    scale_result.total,
                    prec = decimal_places
                ));
                out_record.push(format!(
                    "{:.prec$}",
                    scale_result.mean,
                    prec = decimal_places
                ));

                scale_scores
                    .get_mut(scale_name)
                    .unwrap()
                    .push(scale_result.mean);
                scale_items_matrix
                    .get_mut(scale_name)
                    .unwrap()
                    .push(scale_result.item_values.clone());
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
        all_records.push(out_record);
        processed_count += 1;

        if let Some(ref pb) = pb {
            pb.inc(1);
        }
    }

    if let Some(ref pb) = pb {
        pb.finish_with_message("✓ Complete");
    }

    writer.flush()?;

    let clean_count = processed_count - flagged_count;
    let elapsed = start_time.elapsed();

    // Console output
    if !options.quiet {
        println!("\n{}", "═".repeat(50));
        println!("✓ Processing Complete");
        println!("{}", "═".repeat(50));
        println!("Total Participants:  {}", processed_count);
        let clean_pct = utils::calculate_percentage(clean_count, processed_count);
        let flagged_pct = utils::calculate_percentage(flagged_count, processed_count);
        println!("Clean Records:       {} ({:.1}%)", clean_count, clean_pct);
        println!(
            "Flagged Records:     {} ({:.1}%)",
            flagged_count, flagged_pct
        );
        println!("Total Issues:        {}", quality_issues.len());
        println!("Processing Time:     {:.2}s", elapsed.as_secs_f64());
        println!(
            "Throughput:          {:.0} records/sec",
            processed_count as f64 / elapsed.as_secs_f64()
        );
        println!("{}", "═".repeat(50));

        // Show scale summaries
        println!("\n📊 SCALE SUMMARIES:");
        for (scale_name, scores) in &scale_scores {
            let stats = Stats::calculate(scores);
            println!("  {} (n={}):", scale_name, stats.n);
            println!(
                "    M={:.2}, SD={:.2}, Range=[{:.2}, {:.2}]",
                stats.mean, stats.sd, stats.min, stats.max
            );
        }
        println!();
    }

    info!("Output saved to: {}", options.output);

    // 4. Generate additional outputs
    if let Some(stats_path) = &stats_path {
        generate_summary_stats(
            &config,
            &scale_scores,
            &scale_items_matrix,
            processed_count,
            stats_path,
            &quality_issues,
        )?;
        info!("Summary statistics saved to: {}", stats_path);
    }

    if let Some(quality_path) = &quality_path {
        generate_quality_report(&quality_issues, processed_count, quality_path)?;
        info!("Quality report saved to: {}", quality_path);
    }

    // Export in different formats
    if options.export_all {
        // Export to all formats when --export-all flag is used
        info!("Exporting to all formats...");

        // Excel
        let excel_path = options.output.replace(".csv", ".xlsx");
        generate_excel_output(&all_records, &out_headers, &excel_path)?;
        info!("Excel output saved to: {}", excel_path);

        // JSON
        let json_path = options.output.replace(".csv", ".json");
        generate_json_output(
            &config,
            &scale_scores,
            &quality_issues,
            processed_count,
            &json_path,
        )?;
        info!("JSON output saved to: {}", json_path);

        // SPSS
        let spss_path = options.output.replace(".csv", ".sps");
        generate_spss_syntax(&options.output, &config, &spss_path)?;
        info!("SPSS syntax saved to: {}", spss_path);

        // R
        let r_path = options.output.replace(".csv", ".R");
        generate_r_script(&options.output, &config, &r_path)?;
        info!("R script saved to: {}", r_path);

        info!("✓ All formats exported successfully");
    } else {
        // Export in specified format only
        match options.format {
            OutputFormat::Excel => {
                let excel_path = options.output.replace(".csv", ".xlsx");
                generate_excel_output(&all_records, &out_headers, &excel_path)?;
                info!("Excel output saved to: {}", excel_path);
            }
            OutputFormat::Json => {
                if let Some(json_path) = options.json_output {
                    generate_json_output(
                        &config,
                        &scale_scores,
                        &quality_issues,
                        processed_count,
                        &json_path,
                    )?;
                    info!("JSON output saved to: {}", json_path);
                }
            }
            OutputFormat::Spss => {
                let spss_path = options.output.replace(".csv", ".sps");
                generate_spss_syntax(&options.output, &config, &spss_path)?;
                info!("SPSS syntax saved to: {}", spss_path);
            }
            OutputFormat::R => {
                let r_path = options.output.replace(".csv", ".R");
                generate_r_script(&options.output, &config, &r_path)?;
                info!("R script saved to: {}", r_path);
            }
            OutputFormat::Python => {
                let py_path = options.output.replace(".csv", ".py");
                generate_python_script(&options.output, &config, &py_path)?;
                info!("Python script saved to: {}", py_path);
            }
            OutputFormat::HtmlReport => {
                let html_path = options.output.replace(".csv", "_report.html");
                visualization::generate_html_report(
                    &config,
                    &scale_scores,
                    &quality_issues,
                    processed_count,
                    &html_path,
                )?;
                info!("HTML report saved to: {}", html_path);
            }
            OutputFormat::Csv => {
                // Already saved
            }
        }
    }

    info!("✓ All operations completed successfully");
    Ok(())
}

fn validate_command(config_path: &str, input: &str) -> Result<()> {
    println!("🔍 Validating configuration and data...\n");

    // Load config
    let config_content = fs::read_to_string(config_path)?;
    let config: SurveyConfig = toml::from_str(&config_content)?;
    println!("✓ Configuration file parsed successfully");

    // Load CSV headers
    let mut reader = csv::Reader::from_path(input)?;
    let headers: Vec<String> = reader.headers()?.iter().map(|h| h.to_string()).collect();
    println!("✓ CSV file opened successfully ({} columns)", headers.len());

    // Validate
    validate_config(&config, &headers)?;

    println!("\n✅ Validation passed! Configuration and CSV are compatible.");
    println!("\nConfiguration summary:");
    println!("  Survey: {}", config.survey.name);
    println!(
        "  Score range: {} to {}",
        config.survey.min_score, config.survey.max_score
    );
    println!("  Scales defined: {}", config.scales.len());
    for (name, def) in &config.scales {
        println!("    - {} ({} items)", name, def.items.len());
    }

    Ok(())
}

fn show_preview(
    reader: &mut csv::Reader<std::fs::File>,
    headers: &[String],
    config: &SurveyConfig,
) -> Result<()> {
    println!("\n[DRY RUN] CSV Preview (first 3 rows):");
    println!("{}", "─".repeat(80));
    println!("{}", headers.join(" | "));
    println!("{}", "─".repeat(80));

    for (i, result) in reader.records().enumerate() {
        if i >= 3 {
            break;
        }
        let record = result?;
        let values: Vec<&str> = record.iter().collect();
        println!("{}", values.join(" | "));
    }

    println!("{}", "─".repeat(80));
    println!("\nComputed columns that would be added:");
    for scale_name in config.scales.keys() {
        println!("  - {}_total", scale_name);
        println!("  - {}_mean", scale_name);
    }
    println!("  - quality_flag");

    Ok(())
}

fn process_batch(
    batch_path: &str,
    config_path: &str,
    output_base: &str,
    stats_output: Option<String>,
    quality_report: Option<String>,
) -> Result<()> {
    let files = validate_batch_file(batch_path)?;
    println!("📦 Batch processing {} files...\n", files.len());

    for (i, file) in files.iter().enumerate() {
        println!("[{}/{}] Processing: {}", i + 1, files.len(), file);

        let output = output_base.replace(".csv", &format!("_{}.csv", i + 1));
        let stats = stats_output
            .as_ref()
            .map(|s| s.replace(".txt", &format!("_{}.txt", i + 1)));
        let quality = quality_report
            .as_ref()
            .map(|q| q.replace(".txt", &format!("_{}.txt", i + 1)));

        process_file(ProcessingOptions {
            input: file.to_string(),
            config_path: config_path.to_string(),
            output,
            stats_output: stats,
            quality_report: quality,
            dry_run: false,
            all_outputs: true,
            export_all: false, // export_all not used in batch mode
            format: OutputFormat::Csv,
            json_output: None,
            quiet: true, // Quiet mode for batch
        })?;
    }

    println!("\n✅ Batch processing complete!");
    Ok(())
}

fn run_benchmark(input: &Option<String>, config_path: &str) -> Result<()> {
    let input = input
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("--input required for benchmark"))?;

    println!("⚡ Running benchmark...\n");

    let iterations = 5;
    let mut times = Vec::new();

    for i in 1..=iterations {
        println!("Iteration {}/{}...", i, iterations);
        let start = Instant::now();

        process_file(ProcessingOptions {
            input: input.to_string(),
            config_path: config_path.to_string(),
            output: "benchmark_output.csv".to_string(),
            stats_output: None,
            quality_report: None,
            dry_run: false,
            all_outputs: false,
            export_all: false, // export_all not used in benchmark
            format: OutputFormat::Csv,
            json_output: None,
            quiet: true,
        })?;

        let elapsed = start.elapsed();
        times.push(elapsed.as_secs_f64());
    }

    let avg_time = times.iter().sum::<f64>() / times.len() as f64;
    let min_time = times.iter().fold(f64::INFINITY, |a, &b| a.min(b));
    let max_time = times.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));

    // Count records
    let reader = csv::Reader::from_path(input)?;
    let record_count = reader.into_records().count();

    println!("\n📊 Benchmark Results:");
    println!("  Records processed: {}", record_count);
    println!("  Average time:      {:.3}s", avg_time);
    println!("  Min time:          {:.3}s", min_time);
    println!("  Max time:          {:.3}s", max_time);
    println!(
        "  Throughput:        {:.0} records/sec",
        record_count as f64 / avg_time
    );

    // Clean up benchmark file
    let _ = fs::remove_file("benchmark_output.csv");

    Ok(())
}

#[cfg(windows)]
fn show_interactive_help_and_install() {
    use std::io::{self, Write};
    use std::path::PathBuf;

    println!("\n");
    println!("    ██████╗ ██████╗ ██╗███████╗███╗   ███╗");
    println!("    ██╔══██╗██╔══██╗██║██╔════╝████╗ ████║");
    println!("    ██████╔╝██████╔╝██║███████╗██╔████╔██║");
    println!("    ██╔═══╝ ██╔══██╗██║╚════██║██║╚██╔╝██║");
    println!("    ██║     ██║  ██║██║███████║██║ ╚═╝ ██║");
    println!("    ╚═╝     ╚═╝  ╚═╝╚═╝╚══════╝╚═╝     ╚═╝");
    println!();
    println!("    ╔════════════════════════════════════════╗");
    println!("    ║   Survey Data Processor (CLI) v0.2.0  ║");
    println!("    ║   Psychology Research Made Simple     ║");
    println!("    ╚════════════════════════════════════════╝");
    println!();

    // Define installation directory
    let install_dir = std::env::var("LOCALAPPDATA")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from(std::env::var("USERPROFILE").unwrap_or_default()))
        .join("Programs")
        .join("Prism");

    let install_path = install_dir.join("prism.exe");

    // Check if already installed in the target location
    let current_exe = std::env::current_exe().ok();
    let is_installed = current_exe
        .as_ref()
        .map(|p| p == &install_path)
        .unwrap_or(false);

    // Check if install directory is in PATH
    let is_in_path = std::env::var("PATH")
        .unwrap_or_default()
        .split(';')
        .any(|p| p == install_dir.to_str().unwrap_or(""));

    if !is_installed || !is_in_path {
        println!("    ⚠️  Prism is not installed.");
        println!();
        println!("    📦 Installation will:");
        println!("       • Copy prism.exe to: {}", install_dir.display());
        println!("       • Add to PATH for global access");
        println!("       • Allow you to delete the downloaded file");
        println!();
        print!("    Would you like to install now? (Y/n): ");
        io::stdout().flush().unwrap();

        let mut response = String::new();
        io::stdin().read_line(&mut response).ok();
        let response = response.trim().to_lowercase();

        if response.is_empty() || response == "y" || response == "yes" {
            // Create installation directory
            match fs::create_dir_all(&install_dir) {
                Ok(_) => {
                    // Copy executable
                    if let Some(current) = current_exe {
                        match fs::copy(&current, &install_path) {
                            Ok(_) => {
                                println!();
                                println!("    ✅ Copied to: {}", install_path.display());

                                // Add to PATH
                                match add_to_path(install_dir.to_str().unwrap()) {
                                    Ok(_) => {
                                        println!("    ✅ Added to PATH!");
                                        println!(
                                            "    🔄 Please restart your terminal/command prompt."
                                        );
                                        println!("       Then you can use 'prism' from anywhere!");
                                        println!();
                                        println!(
                                            "    💡 You can now safely delete the downloaded file."
                                        );
                                        println!();
                                    }
                                    Err(e) => {
                                        println!("    ⚠️  Warning: Failed to add to PATH: {}", e);
                                        println!(
                                            "    You can still run: {}",
                                            install_path.display()
                                        );
                                        println!();
                                    }
                                }
                            }
                            Err(e) => {
                                println!();
                                println!("    ❌ Failed to copy file: {}", e);
                                println!("       Try running as administrator.");
                                println!();
                            }
                        }
                    }
                }
                Err(e) => {
                    println!();
                    println!("    ❌ Failed to create directory: {}", e);
                    println!("       Try running as administrator.");
                    println!();
                }
            }
        } else {
            println!();
            println!("    Installation skipped.");
            println!("    You can run prism using the full path:");
            if let Some(exe_path) = current_exe {
                println!("    {}", exe_path.display());
            }
            println!();
        }
    } else {
        println!("    ✅ Prism is already installed!");
        println!("    📍 Location: {}", install_path.display());
        println!();
    }

    println!("    ┌─────────────────────────────────────────────────────────┐");
    println!("    │  📖  COMMON COMMANDS                                    │");
    println!("    └─────────────────────────────────────────────────────────┘");
    println!();
    println!("    ▸ Process data:");
    println!("      prism process -i data.csv -c config.toml -o clean.csv");
    println!();
    println!("    ▸ Validate config:");
    println!("      prism validate -c config.toml -i data.csv");
    println!();
    println!("    ▸ Generate template:");
    println!("      prism generate --template > config.toml");
    println!();
    println!("    ▸ Show all options:");
    println!("      prism --help");
    println!();
    println!("    ┌─────────────────────────────────────────────────────────┐");
    println!("    │  💡 TIP: For a graphical interface, use the GUI app    │");
    println!("    └─────────────────────────────────────────────────────────┘");
    println!();

    print!("Press Enter to exit...");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok();
}

#[cfg(windows)]
fn add_to_path(dir: &str) -> Result<()> {
    use winreg::enums::*;
    use winreg::RegKey;

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let env = hkcu.open_subkey_with_flags("Environment", KEY_READ | KEY_WRITE)?;

    let current_path: String = env.get_value("Path").unwrap_or_default();

    // Check if already in PATH
    if current_path.split(';').any(|p| p == dir) {
        return Ok(());
    }

    let new_path = if current_path.is_empty() {
        dir.to_string()
    } else if current_path.ends_with(';') {
        format!("{}{}", current_path, dir)
    } else {
        format!("{};{}", current_path, dir)
    };

    env.set_value("Path", &new_path)?;

    // Broadcast WM_SETTINGCHANGE to notify system
    Ok(())
}

#[cfg(not(windows))]
fn show_interactive_help_and_install() {
    use std::io::{self, Write};

    println!("\n╔══════════════════════════════════════════════════════════════╗");
    println!("║                                                              ║");
    println!("║          🔍 PRISM - Survey Data Processor (CLI)             ║");
    println!("║                                                              ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    println!("📦 INSTALLATION:\n");
    println!("  To install globally, run:");
    println!("    cargo install --path .\n");
    println!("  Or use the install script:");
    println!("    ./install-cli.sh\n");

    println!("📖 COMMON COMMANDS:\n");
    println!("  Process data:");
    println!("    prism process -i data.csv -c config.toml -o clean.csv\n");
    println!("  Validate config:");
    println!("    prism validate -c config.toml -i data.csv\n");
    println!("  Generate template:");
    println!("    prism generate --template > config.toml\n");
    println!("  Show all options:");
    println!("    prism --help\n");
    println!("💡 TIP: For a graphical interface, use the GUI version instead.\n");

    print!("Press Enter to exit...");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok();
}
