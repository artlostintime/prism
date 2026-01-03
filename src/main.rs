// src/main.rs
use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use indicatif::{ProgressBar, ProgressStyle};
use log::{debug, info};
use prism::{
    config::SurveyConfig,
    output::*,
    processor::{get_participant_id, process_quality_checks, process_scale},
    stats::Stats,
    types::{OutputFormat, QualityIssue},
    validation::{generate_config_template, validate_batch_file, validate_config},
    DEFAULT_QUALITY_FILE, DEFAULT_STATS_FILE, QUALITY_FLAG_OK, QUALITY_FLAG_SEPARATOR,
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
                process_file(
                    &input,
                    &config,
                    &output,
                    stats_output,
                    quality_report,
                    dry_run,
                    all_outputs,
                    format,
                    json_output,
                    args.quiet,
                )?;
            }
        }
        Commands::Validate { config, input } => {
            validate_command(&config, &input)?;
        }
        Commands::Generate { template } => {
            if template {
                println!("{}", generate_config_template());
            }
        }
    }

    Ok(())
}

fn process_file(
    input: &str,
    config_path: &str,
    output: &str,
    stats_output: Option<String>,
    quality_report: Option<String>,
    dry_run: bool,
    all_outputs: bool,
    format: OutputFormat,
    json_output: Option<String>,
    quiet: bool,
) -> Result<()> {
    let start_time = Instant::now();

    // 1. Load Configuration
    let config_content = fs::read_to_string(config_path).context(format!(
        "Could not read config file '{}'. Check if the file exists and has .toml extension",
        config_path
    ))?;
    let config: SurveyConfig = toml::from_str(&config_content).context(format!(
        "Could not parse TOML config from '{}'. Check for syntax errors",
        config_path
    ))?;

    info!("✓ Configuration loaded successfully");
    if !quiet {
        println!(
            "\n{} Processing Survey: {}",
            if dry_run { "[DRY RUN]" } else { "▸" },
            config.survey.name
        );
    }

    // 2. Setup CSV Reader
    let mut reader = csv::Reader::from_path(input).context(format!(
        "Could not open input CSV '{}'. Check if the file exists",
        input
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
    if dry_run && !quiet {
        show_preview(&mut reader, &header_vec, &config)?;
        return Ok(());
    }

    // Prepare output
    let mut writer = csv::Writer::from_path(output).context(format!(
        "Could not create output CSV '{}'. Check write permissions",
        output
    ))?;

    let mut out_headers = headers.iter().map(|h| h.to_string()).collect::<Vec<_>>();
    for scale_name in config.scales.keys() {
        out_headers.push(format!("{}_total", scale_name));
        out_headers.push(format!("{}_mean", scale_name));
    }
    out_headers.push("quality_flag".to_string());
    writer.write_record(&out_headers)?;

    // Determine output paths
    let stats_path = if all_outputs {
        Some(stats_output.unwrap_or_else(|| DEFAULT_STATS_FILE.to_string()))
    } else {
        stats_output
    };

    let quality_path = if all_outputs {
        Some(quality_report.unwrap_or_else(|| DEFAULT_QUALITY_FILE.to_string()))
    } else {
        quality_report
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
    let mut reader = csv::Reader::from_path(input)?; // Re-open after counting
    reader.headers()?; // Skip headers

    let pb = if !quiet {
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

    for result in reader.records() {
        let record = result?;
        let mut out_record: Vec<String> = record.iter().map(|s| s.to_string()).collect();
        let mut quality_flags = Vec::new();
        let participant_id = get_participant_id(&record, &header_map, &config);

        debug!("Processing participant: {}", participant_id);

        // Process each scale
        for (scale_name, scale_def) in &config.scales {
            let (scale_result, missing_count) =
                process_scale(scale_def, &record, &header_map, &config)?;

            // Quality checks
            process_quality_checks(
                scale_name,
                &scale_result,
                missing_count,
                scale_def.items.len(),
                &participant_id,
                &config,
                &mut quality_flags,
                &mut quality_issues,
            );

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
    if !quiet {
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

    info!("Output saved to: {}", output);

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
    match format {
        OutputFormat::Excel => {
            let excel_path = output.replace(".csv", ".xlsx");
            generate_excel_output(&all_records, &out_headers, &excel_path)?;
            info!("Excel output saved to: {}", excel_path);
        }
        OutputFormat::Json => {
            if let Some(json_path) = json_output {
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
            let spss_path = output.replace(".csv", ".sps");
            generate_spss_syntax(output, &config, &spss_path)?;
            info!("SPSS syntax saved to: {}", spss_path);
        }
        OutputFormat::R => {
            let r_path = output.replace(".csv", ".R");
            generate_r_script(output, &config, &r_path)?;
            info!("R script saved to: {}", r_path);
        }
        OutputFormat::Csv => {
            // Already saved
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

        process_file(
            file,
            config_path,
            &output,
            stats,
            quality,
            false,
            true,
            OutputFormat::Csv,
            None,
            true, // Quiet mode for batch
        )?;
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

        process_file(
            input,
            config_path,
            "benchmark_output.csv",
            None,
            None,
            false,
            false,
            OutputFormat::Csv,
            None,
            true,
        )?;

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
    println!("");
    println!("    ╔════════════════════════════════════════╗");
    println!("    ║   Survey Data Processor (CLI) v0.2.0  ║");
    println!("    ║   Psychology Research Made Simple     ║");
    println!("    ╚════════════════════════════════════════╝");
    println!("");

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
        println!("");
        println!("    📦 Installation will:");
        println!("       • Copy prism.exe to: {}", install_dir.display());
        println!("       • Add to PATH for global access");
        println!("       • Allow you to delete the downloaded file");
        println!("");
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
                                println!("");
                                println!("    ✅ Copied to: {}", install_path.display());

                                // Add to PATH
                                match add_to_path(install_dir.to_str().unwrap()) {
                                    Ok(_) => {
                                        println!("    ✅ Added to PATH!");
                                        println!(
                                            "    🔄 Please restart your terminal/command prompt."
                                        );
                                        println!("       Then you can use 'prism' from anywhere!");
                                        println!("");
                                        println!(
                                            "    💡 You can now safely delete the downloaded file."
                                        );
                                        println!("");
                                    }
                                    Err(e) => {
                                        println!("    ⚠️  Warning: Failed to add to PATH: {}", e);
                                        println!(
                                            "    You can still run: {}",
                                            install_path.display()
                                        );
                                        println!("");
                                    }
                                }
                            }
                            Err(e) => {
                                println!("");
                                println!("    ❌ Failed to copy file: {}", e);
                                println!("       Try running as administrator.");
                                println!("");
                            }
                        }
                    }
                }
                Err(e) => {
                    println!("");
                    println!("    ❌ Failed to create directory: {}", e);
                    println!("       Try running as administrator.");
                    println!("");
                }
            }
        } else {
            println!("");
            println!("    Installation skipped.");
            println!("    You can run prism using the full path:");
            if let Some(exe_path) = current_exe {
                println!("    {}", exe_path.display());
            }
            println!("");
        }
    } else {
        println!("    ✅ Prism is already installed!");
        println!("    📍 Location: {}", install_path.display());
        println!("");
    }

    println!("    ┌─────────────────────────────────────────────────────────┐");
    println!("    │  📖  COMMON COMMANDS                                    │");
    println!("    └─────────────────────────────────────────────────────────┘");
    println!("");
    println!("    ▸ Process data:");
    println!("      prism process -i data.csv -c config.toml -o clean.csv");
    println!("");
    println!("    ▸ Validate config:");
    println!("      prism validate -c config.toml -i data.csv");
    println!("");
    println!("    ▸ Generate template:");
    println!("      prism generate --template > config.toml");
    println!("");
    println!("    ▸ Show all options:");
    println!("      prism --help");
    println!("");
    println!("    ┌─────────────────────────────────────────────────────────┐");
    println!("    │  💡 TIP: For a graphical interface, use the GUI app    │");
    println!("    └─────────────────────────────────────────────────────────┘");
    println!("");

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
