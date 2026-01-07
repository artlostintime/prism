use rfd::FileDialog;
use std::fs;
use std::io::BufRead;
use std::path::{Path, PathBuf};
use std::process::Command;
use tauri::{command, Manager};

// COMMAND 1: Pick CSV File
#[command]
fn pick_file() -> String {
    let file = FileDialog::new().add_filter("CSV", &["csv"]).pick_file();

    match file {
        Some(path) => path.display().to_string(),
        None => "".to_string(),
    }
}

// COMMAND 2: Pick Config File
#[command]
fn pick_config() -> String {
    let file = FileDialog::new().add_filter("TOML", &["toml"]).pick_file();

    match file {
        Some(path) => path.display().to_string(),
        None => "".to_string(),
    }
}

// Helper: Determine smart output folder
fn determine_output_folder(csv_path: &Path) -> PathBuf {
    let csv_parent = match csv_path.parent() {
        Some(p) => p,
        None => return csv_path.to_path_buf(), // Fallback
    };

    let csv_parent_name = csv_parent
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("");

    // If CSV is in raw_data, raw, data, or input folder
    if ["raw_data", "raw", "data", "input"].contains(&csv_parent_name) {
        // Go up one level and create processed/
        if let Some(grandparent) = csv_parent.parent() {
            return grandparent.join("processed");
        }
    }

    // Otherwise, create processed/ in same directory as CSV
    csv_parent.join("processed")
}

// COMMAND 3: Generate default config template
#[command]
fn generate_config_template() -> String {
    r#"[survey]
name = "My Survey Study"
min_score = 1
max_score = 7

[quality]
max_missing_percent = 0.10
flag_straightlining = true
flag_low_variance = true
flag_diagonal_pattern = true
flag_alternating_pattern = true
flag_block_pattern = true
check_response_time = true
min_response_time = 30
max_response_time = 300

# Add your scales below
[scales.my_scale]
items = ["Q1", "Q2", "Q3", "Q4", "Q5"]
reverse_scored = []

# Example with reverse scoring
[scales.another_scale]
items = ["Q10", "Q11", "Q12"]
reverse_scored = ["Q12"]

# Optional: Semantic Inconsistency Checks
# [semantic_checks.stress_wellbeing]
# scale1 = "stress"
# scale2 = "wellbeing"
# expected_correlation = "negative"
# threshold = 0.7
"#
    .to_string()
}

// COMMAND 4: Save config text to temporary file
#[command]
fn save_config_text(config_text: String, csv_path: String) -> Result<String, String> {
    let csv_path_obj = Path::new(&csv_path);
    let csv_parent = csv_path_obj
        .parent()
        .ok_or("Could not determine parent directory")?;

    let temp_config_path = csv_parent.join("temp_config.toml");

    fs::write(&temp_config_path, config_text)
        .map_err(|e| format!("Failed to save config: {}", e))?;

    Ok(temp_config_path.display().to_string())
}

// COMMAND 5: Run the Analysis Pipeline
#[command]
fn run_analysis(app: tauri::AppHandle, input_path: String, config_path: Option<String>) -> String {
    let input_path_obj = Path::new(&input_path);

    // Determine config path
    let config_path = if let Some(cfg) = config_path {
        PathBuf::from(cfg)
    } else {
        // Fallback: Look for config in parent directory (old behavior)
        match input_path_obj.parent() {
            Some(parent) => parent.join("..").join("study_config.toml"),
            None => return "Error: Could not determine parent directory of input file".to_string(),
        }
    };

    // Verify config exists
    if !config_path.exists() {
        return format!(
            "Error: Config file not found at {:?}\n\nPlease select a valid config file or create one.",
            config_path
        );
    }

    // Determine CLI executable name based on OS
    let cli_name = if cfg!(windows) { "prism.exe" } else { "prism" };

    // Try to use bundled CLI binary first (production)
    let cli_path = app
        .path()
        .resource_dir()
        .ok()
        .and_then(|resource_dir| {
            let bundled = resource_dir.join(cli_name);
            if bundled.exists() {
                Some(bundled)
            } else {
                None
            }
        })
        .or_else(|| {
            // Fallback to development build location
            let release_path = Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("..")
                .join("target")
                .join("release")
                .join(cli_name);
            if release_path.exists() {
                Some(release_path)
            } else {
                let debug_path = Path::new(env!("CARGO_MANIFEST_DIR"))
                    .join("..")
                    .join("target")
                    .join("debug")
                    .join(cli_name);
                if debug_path.exists() {
                    Some(debug_path)
                } else {
                    None
                }
            }
        });

    let cli_path = match cli_path {
        Some(path) => path,
        None => {
            return format!(
                "Error: CLI binary not found.\n\nExpected bundled binary or development build at target/release/{}\n\nPlease build the CLI first: cargo build --release",
                cli_name
            )
        }
    };

    // Determine smart output folder
    let output_folder = determine_output_folder(input_path_obj);

    // Create output folder if it doesn't exist
    if let Err(e) = fs::create_dir_all(&output_folder) {
        return format!(
            "Error: Could not create output folder {:?}: {}",
            output_folder, e
        );
    }

    // Determine output paths
    let output_path = output_folder.join("clean_data.csv");
    let stats_path = output_folder.join("summary_stats.txt");
    let quality_path = output_folder.join("quality_report.txt");

    // Run the CLI command with stats and quality reports
    let output = match Command::new(&cli_path)
        .arg("process")
        .arg("--input")
        .arg(&input_path)
        .arg("--config")
        .arg(&config_path)
        .arg("--output")
        .arg(&output_path)
        .arg("--stats-output")
        .arg(&stats_path)
        .arg("--quality-report")
        .arg(&quality_path)
        .output()
    {
        Ok(o) => o,
        Err(e) => return format!("Failed to execute CLI: {}", e),
    };

    // Check exit status
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return format!("Processing Error:\n\n{}", stderr);
    }

    // Parse output for success message
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Extract participant count from CLI output
    let count = stdout
        .lines()
        .find(|line| line.contains("participant"))
        .and_then(|line| {
            line.split_whitespace()
                .find_map(|word| word.parse::<usize>().ok())
        })
        .unwrap_or(0);

    // Check for quality issues
    let quality_summary = if stdout.contains("quality report") {
        "\n\nFull statistics and quality report generated."
    } else {
        ""
    };

    format!(
        "Success! Processed {} participants.{}\n\nOutput folder:\n{}\n\nGenerated files:\n- clean_data.csv\n- summary_stats.txt\n- quality_report.txt",
        count,
        quality_summary,
        output_folder.display()
    )
}

// COMMAND 6: Get CSV file info (rows and columns)
#[command]
fn get_csv_info(path: String) -> Result<String, String> {
    let file = fs::File::open(&path).map_err(|e| format!("Cannot open file: {}", e))?;
    let reader = std::io::BufReader::new(file);
    let mut lines = reader.lines();

    // Get column count from header
    let columns = if let Some(Ok(header)) = lines.next() {
        header.split(',').count()
    } else {
        return Err("Empty file or cannot read header".to_string());
    };

    // Count rows (excluding header)
    let rows = lines.count();

    // Return as JSON string
    Ok(format!(r#"{{"rows": {}, "columns": {}}}"#, rows, columns))
}

// COMMAND 7: Open folder in file explorer
#[command]
fn open_folder(path: String) -> Result<(), String> {
    let path_obj = Path::new(&path);

    // Create folder if it doesn't exist
    if !path_obj.exists() {
        fs::create_dir_all(&path_obj).map_err(|e| format!("Cannot create folder: {}", e))?;
    }

    // Open in file explorer based on OS
    #[cfg(target_os = "windows")]
    {
        Command::new("explorer")
            .arg(&path)
            .spawn()
            .map_err(|e| format!("Cannot open folder: {}", e))?;
    }

    #[cfg(target_os = "macos")]
    {
        Command::new("open")
            .arg(&path)
            .spawn()
            .map_err(|e| format!("Cannot open folder: {}", e))?;
    }

    #[cfg(target_os = "linux")]
    {
        Command::new("xdg-open")
            .arg(&path)
            .spawn()
            .map_err(|e| format!("Cannot open folder: {}", e))?;
    }

    Ok(())
}

// COMMAND 8: Get list of available pre-built scales
#[command]
fn get_available_scales() -> Vec<String> {
    prism::scales::list_available_scales()
}

// COMMAND 9: Get scale metadata
#[command]
fn get_scale_info(scale_id: String) -> Result<String, String> {
    prism::scales::get_scale_metadata(&scale_id)
        .map(|metadata| {
            serde_json::json!({
                "name": metadata.name,
                "full_name": metadata.full_name,
                "citation": metadata.citation,
                "description": metadata.description,
                "num_items": metadata.num_items,
                "min_score": metadata.min_score,
                "max_score": metadata.max_score,
                "interpretation": metadata.interpretation,
                "normative_data": metadata.normative_data.map(|norm| {
                    serde_json::json!({
                        "population": norm.population,
                        "mean": norm.mean,
                        "sd": norm.sd,
                        "clinical_cutoff": norm.clinical_cutoff,
                        "severity_ranges": norm.severity_ranges
                    })
                })
            })
            .to_string()
        })
        .map_err(|e| e.to_string())
}

// COMMAND 10: Generate scale config
#[command]
fn generate_scale_config(scale_id: String) -> Result<String, String> {
    prism::scales::generate_scale_config(&scale_id).map_err(|e| e.to_string())
}

// COMMAND 11: Generate multiple output formats
#[command]
fn run_analysis_multi_format(
    app: tauri::AppHandle,
    input_path: String,
    config_path: String,
    formats: Vec<String>, // "csv", "excel", "spss", "r", "python", "html"
) -> Result<String, String> {
    let input_path_obj = Path::new(&input_path);
    let cli_name = if cfg!(windows) { "prism.exe" } else { "prism" };

    // Try to use bundled CLI binary first (production)
    let cli_path = app
        .path()
        .resource_dir()
        .ok()
        .and_then(|resource_dir| {
            let bundled = resource_dir.join(cli_name);
            if bundled.exists() {
                Some(bundled)
            } else {
                None
            }
        })
        .or_else(|| {
            // Fallback to development build location
            let release_path = Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("..")
                .join("target")
                .join("release")
                .join(cli_name);
            if release_path.exists() {
                Some(release_path)
            } else {
                let debug_path = Path::new(env!("CARGO_MANIFEST_DIR"))
                    .join("..")
                    .join("target")
                    .join("debug")
                    .join(cli_name);
                if debug_path.exists() {
                    Some(debug_path)
                } else {
                    None
                }
            }
        });

    let cli_path = match cli_path {
        Some(path) => path,
        None => {
            return Err(format!(
                "CLI binary not found.\n\nExpected bundled binary or development build at target/release/{}\n\nPlease build the CLI first: cargo build --release",
                cli_name
            ))
        }
    };

    let output_folder = determine_output_folder(input_path_obj);
    fs::create_dir_all(&output_folder)
        .map_err(|e| format!("Could not create output folder: {}", e))?;

    let mut generated_files = Vec::new();
    let mut errors = Vec::new();

    // Process each format
    for format in formats {
        match format.as_str() {
            "csv" => {
                let output_path = output_folder.join("clean_data.csv");
                let result = Command::new(&cli_path)
                    .arg("process")
                    .arg("--input")
                    .arg(&input_path)
                    .arg("--config")
                    .arg(&config_path)
                    .arg("--output")
                    .arg(&output_path)
                    .output();

                match result {
                    Ok(out) if out.status.success() => generated_files.push("clean_data.csv"),
                    _ => errors.push("CSV generation failed"),
                }
            }
            "excel" => {
                let output_path = output_folder.join("clean_data.xlsx");
                let result = Command::new(&cli_path)
                    .arg("process")
                    .arg("--input")
                    .arg(&input_path)
                    .arg("--config")
                    .arg(&config_path)
                    .arg("--output")
                    .arg(&output_path)
                    .arg("--format")
                    .arg("excel")
                    .output();

                match result {
                    Ok(out) if out.status.success() => generated_files.push("clean_data.xlsx"),
                    _ => errors.push("Excel generation failed"),
                }
            }
            "spss" => {
                let output_path = output_folder.join("clean_data.sps");
                let result = Command::new(&cli_path)
                    .arg("process")
                    .arg("--input")
                    .arg(&input_path)
                    .arg("--config")
                    .arg(&config_path)
                    .arg("--output")
                    .arg(&output_path)
                    .arg("--format")
                    .arg("spss")
                    .output();

                match result {
                    Ok(out) if out.status.success() => generated_files.push("clean_data.sps"),
                    _ => errors.push("SPSS generation failed"),
                }
            }
            "r" => {
                let output_path = output_folder.join("analysis_script.R");
                let result = Command::new(&cli_path)
                    .arg("process")
                    .arg("--input")
                    .arg(&input_path)
                    .arg("--config")
                    .arg(&config_path)
                    .arg("--output")
                    .arg(&output_path)
                    .arg("--format")
                    .arg("r")
                    .output();

                match result {
                    Ok(out) if out.status.success() => generated_files.push("analysis_script.R"),
                    _ => errors.push("R script generation failed"),
                }
            }
            "python" => {
                let output_path = output_folder.join("analysis_script.py");
                let result = Command::new(&cli_path)
                    .arg("process")
                    .arg("--input")
                    .arg(&input_path)
                    .arg("--config")
                    .arg(&config_path)
                    .arg("--output")
                    .arg(&output_path)
                    .arg("--format")
                    .arg("python")
                    .output();

                match result {
                    Ok(out) if out.status.success() => generated_files.push("analysis_script.py"),
                    _ => errors.push("Python script generation failed"),
                }
            }
            "html" => {
                let output_path = output_folder.join("report.html");
                let result = Command::new(&cli_path)
                    .arg("process")
                    .arg("--input")
                    .arg(&input_path)
                    .arg("--config")
                    .arg(&config_path)
                    .arg("--output")
                    .arg(&output_path)
                    .arg("--format")
                    .arg("html-report")
                    .output();

                match result {
                    Ok(out) if out.status.success() => generated_files.push("report.html"),
                    _ => errors.push("HTML report generation failed"),
                }
            }
            _ => {}
        }
    }

    // Always generate stats and quality reports
    let stats_path = output_folder.join("summary_stats.txt");
    let quality_path = output_folder.join("quality_report.txt");

    let _ = Command::new(&cli_path)
        .arg("process")
        .arg("--input")
        .arg(&input_path)
        .arg("--config")
        .arg(&config_path)
        .arg("--output")
        .arg(output_folder.join("temp.csv"))
        .arg("--stats-output")
        .arg(&stats_path)
        .arg("--quality-report")
        .arg(&quality_path)
        .output();

    if generated_files.is_empty() && !errors.is_empty() {
        return Err(format!("All formats failed:\n{}", errors.join("\n")));
    }

    let mut result = format!(
        "✅ Success! Generated {} file(s):\n\n",
        generated_files.len()
    );
    for file in &generated_files {
        result.push_str(&format!("  • {}\n", file));
    }
    result.push_str(&format!("\n📁 Output folder:\n{}", output_folder.display()));

    if !errors.is_empty() {
        result.push_str(&format!(
            "\n\n⚠️ Some formats failed:\n{}",
            errors.join("\n")
        ));
    }

    Ok(result)
}

// COMMAND 12: Generate Data Dictionary (v0.8.0)
#[command]
fn run_dictionary(
    app: tauri::AppHandle,
    config_path: String,
    output_path: String,
    format: String,
) -> Result<String, String> {
    let cli_name = if cfg!(windows) { "prism.exe" } else { "prism" };

    // Try to use bundled CLI binary first (production)
    let cli_path = app
        .path()
        .resource_dir()
        .ok()
        .and_then(|resource_dir| {
            let bundled = resource_dir.join(cli_name);
            if bundled.exists() {
                Some(bundled)
            } else {
                None
            }
        })
        .or_else(|| {
            // Fallback to development build location
            let release_path = Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("..")
                .join("target")
                .join("release")
                .join(cli_name);
            if release_path.exists() {
                Some(release_path)
            } else {
                let debug_path = Path::new(env!("CARGO_MANIFEST_DIR"))
                    .join("..")
                    .join("target")
                    .join("debug")
                    .join(cli_name);
                if debug_path.exists() {
                    Some(debug_path)
                } else {
                    None
                }
            }
        });

    let cli_path = match cli_path {
        Some(path) => path,
        None => {
            return Err(format!(
                "CLI binary not found.\n\nExpected bundled binary or development build at target/release/{}\n\nPlease build the CLI first: cargo build --release",
                cli_name
            ))
        }
    };

    // Create output directory if it doesn't exist
    if let Some(parent) = Path::new(&output_path).parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create output directory: {}", e))?;
    }

    let mut cmd = Command::new(&cli_path);
    cmd.arg("dictionary")
        .arg("--config")
        .arg(&config_path)
        .arg("--output")
        .arg(&output_path);

    if format == "json" {
        cmd.arg("--format").arg("json");
    }

    let output = cmd
        .output()
        .map_err(|e| format!("Failed to execute: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Dictionary generation failed:\n\n{}", stderr));
    }

    Ok(format!(
        "Success! Data dictionary saved to:\n{}",
        output_path
    ))
}

// COMMAND 12: Generate CONSORT Flowchart (v0.8.0)
#[command]
fn run_consort(
    app: tauri::AppHandle,
    input_path: String,
    config_path: String,
    output_path: String,
    format: String,
) -> Result<String, String> {
    let cli_name = if cfg!(windows) { "prism.exe" } else { "prism" };

    // Try to use bundled CLI binary first (production)
    let cli_path = app
        .path()
        .resource_dir()
        .ok()
        .and_then(|resource_dir| {
            let bundled = resource_dir.join(cli_name);
            if bundled.exists() {
                Some(bundled)
            } else {
                None
            }
        })
        .or_else(|| {
            // Fallback to development build location
            let release_path = Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("..")
                .join("target")
                .join("release")
                .join(cli_name);
            if release_path.exists() {
                Some(release_path)
            } else {
                let debug_path = Path::new(env!("CARGO_MANIFEST_DIR"))
                    .join("..")
                    .join("target")
                    .join("debug")
                    .join(cli_name);
                if debug_path.exists() {
                    Some(debug_path)
                } else {
                    None
                }
            }
        });

    let cli_path = match cli_path {
        Some(path) => path,
        None => {
            return Err(format!(
                "CLI binary not found.\n\nExpected bundled binary or development build at target/release/{}\n\nPlease build the CLI first: cargo build --release",
                cli_name
            ))
        }
    };

    // First process the data
    let output_folder = Path::new(&output_path)
        .parent()
        .unwrap_or(Path::new(&output_path));

    // Create output directory if it doesn't exist
    if let Some(parent) = Path::new(&output_path).parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create output directory: {}", e))?;
    }

    let processed_csv = output_folder.join("consort_processed.csv");
    let quality_report = output_folder.join("consort_quality.txt");

    // Run processing to generate quality report
    let process_output = Command::new(&cli_path)
        .arg("process")
        .arg("--input")
        .arg(&input_path)
        .arg("--config")
        .arg(&config_path)
        .arg("--output")
        .arg(&processed_csv)
        .arg("--quality-report")
        .arg(&quality_report)
        .output()
        .map_err(|e| format!("Failed to process data: {}", e))?;

    if !process_output.status.success() {
        let stderr = String::from_utf8_lossy(&process_output.stderr);
        return Err(format!("Data processing failed:\n\n{}", stderr));
    }

    // Read quality report and generate CONSORT
    let quality_content = fs::read_to_string(&quality_report)
        .map_err(|e| format!("Failed to read quality report: {}", e))?;

    // Count total participants from CSV
    let csv_file = fs::File::open(&input_path).map_err(|e| format!("Cannot open CSV: {}", e))?;
    let reader = std::io::BufReader::new(csv_file);
    let total_participants = reader.lines().count() - 1; // Exclude header

    // Generate CONSORT content
    let consort_content = if format == "json" {
        generate_consort_json(&quality_content, total_participants)
    } else {
        generate_consort_text(&quality_content, total_participants)
    };

    fs::write(&output_path, consort_content)
        .map_err(|e| format!("Failed to write CONSORT report: {}", e))?;

    Ok(format!(
        "Success! CONSORT flowchart saved to:\n{}",
        output_path
    ))
}

// Helper: Generate CONSORT text format
fn generate_consort_text(quality_report: &str, total: usize) -> String {
    let (excluded, issues) = parse_quality_issues(quality_report);
    let retained = total - excluded;

    // Calculate percentages safely (avoid division by zero)
    let excluded_pct = prism::utils::calculate_percentage(excluded, total);
    let retained_pct = prism::utils::calculate_percentage(retained, total);

    let mut output = String::new();
    output.push_str("CONSORT Participant Flow Report\n");
    output.push_str("================================\n\n");
    output.push_str(&format!("Participants Screened\n  n = {}\n\n", total));
    output.push_str("  ↓\n\n");
    output.push_str(&format!(
        "Excluded (Quality Issues)\n  n = {} ({:.1}%)\n\n",
        excluded, excluded_pct
    ));

    if !issues.is_empty() {
        output.push_str("  Exclusion Breakdown:\n");
        for (reason, count) in issues {
            output.push_str(&format!("    - {}: {} issue(s)\n", reason, count));
        }
        output.push_str("\n");
    }

    output.push_str("  ↓\n\n");
    output.push_str(&format!(
        "Final Analysis Sample\n  n = {} ({:.1}%)\n",
        retained, retained_pct
    ));

    output
}

// Helper: Generate CONSORT JSON format
fn generate_consort_json(quality_report: &str, total: usize) -> String {
    let (excluded, issues) = parse_quality_issues(quality_report);
    let retained = total - excluded;

    // Calculate percentages safely (avoid division by zero)
    let excluded_pct = prism::utils::calculate_percentage(excluded, total);
    let retained_pct = prism::utils::calculate_percentage(retained, total);

    let issues_json: Vec<String> = issues
        .iter()
        .map(|(reason, count)| format!(r#"{{"reason": "{}", "count": {}}}"#, reason, count))
        .collect();

    format!(
        r#"{{
  "total_screened": {},
  "excluded": {},
  "excluded_percent": {:.1},
  "exclusion_reasons": [
    {}
  ],
  "final_sample": {},
  "retention_rate": {:.1}
}}"#,
        total,
        excluded,
        excluded_pct,
        issues_json.join(",\n    "),
        retained,
        retained_pct
    )
}

// Helper: Map quality issue keywords to user-friendly descriptions
fn map_issue_type(line: &str) -> Option<&'static str> {
    if line.contains("missing") || line.contains("Missing") {
        Some("Missing data")
    } else if line.contains("straightlin") || line.contains("Straightlin") {
        Some("Straightlining")
    } else if line.contains("diagonal") || line.contains("Diagonal") {
        Some("Diagonal pattern")
    } else if line.contains("alternating") || line.contains("Alternating") {
        Some("Alternating pattern")
    } else if line.contains("block") || line.contains("Block") {
        Some("Block pattern")
    } else if line.contains("variance") || line.contains("Variance") {
        Some("Low variance")
    } else if line.contains("time") || line.contains("Time") {
        Some("Response time")
    } else if line.contains("semantic") || line.contains("Semantic") {
        Some("Semantic inconsistency")
    } else {
        None
    }
}

// Helper: Parse quality report for issues
fn parse_quality_issues(quality_report: &str) -> (usize, Vec<(String, usize)>) {
    let mut excluded = 0;
    let mut issues: std::collections::HashMap<String, usize> = std::collections::HashMap::new();

    for line in quality_report.lines() {
        if line.contains("flagged") || line.contains("detected") {
            excluded += 1;

            if let Some(issue_type) = map_issue_type(line) {
                *issues.entry(issue_type.to_string()).or_insert(0) += 1;
            }
        }
    }

    let issues_vec: Vec<(String, usize)> = issues.into_iter().collect();
    (excluded, issues_vec)
}

// COMMAND: Preview CSV data (first 10 rows)
#[command]
fn preview_csv_data(csv_path: String) -> Result<String, String> {
    let path = Path::new(&csv_path);
    let file = fs::File::open(path).map_err(|e| format!("Failed to open CSV: {}", e))?;
    let reader = std::io::BufReader::new(file);

    let mut lines: Vec<String> = Vec::new();
    let mut count = 0;

    for line in reader.lines() {
        if count >= 11 {
            break; // Get header + 10 rows
        }
        if let Ok(line_content) = line {
            lines.push(line_content);
            count += 1;
        }
    }

    Ok(lines.join("\n"))
}

// COMMAND: Check if file exists
#[command]
fn file_exists(path: String) -> bool {
    Path::new(&path).exists()
}

// COMMAND: Open HTML report in new window
#[command]
async fn open_html_report(html_path: String, _app_handle: tauri::AppHandle) -> Result<(), String> {
    // Use shell to open in default browser
    #[cfg(target_os = "windows")]
    {
        Command::new("cmd")
            .args(["/C", "start", "", &html_path])
            .spawn()
            .map_err(|e| format!("Failed to open HTML: {}", e))?;
    }

    #[cfg(target_os = "macos")]
    {
        Command::new("open")
            .arg(&html_path)
            .spawn()
            .map_err(|e| format!("Failed to open HTML: {}", e))?;
    }

    #[cfg(target_os = "linux")]
    {
        Command::new("xdg-open")
            .arg(&html_path)
            .spawn()
            .map_err(|e| format!("Failed to open HTML: {}", e))?;
    }

    Ok(())
}

// COMMAND: Read HTML content for inline display
#[command]
fn read_html_content(html_path: String) -> Result<String, String> {
    fs::read_to_string(&html_path).map_err(|e| format!("Failed to read HTML: {}", e))
}

// COMMAND: Power Analysis - A priori (calculate required sample size)
#[command]
fn run_power_analysis(
    analysis_type: String,
    test_type: String,
    effect_size: f64,
    alpha: f64,
    power: Option<f64>,
    n: Option<i32>,
) -> Result<String, String> {
    // Validate inputs
    if effect_size <= 0.0 {
        return Err("Effect size must be positive".to_string());
    }
    if alpha <= 0.0 || alpha >= 1.0 {
        return Err("Alpha must be between 0 and 1".to_string());
    }

    let result = match analysis_type.as_str() {
        "a-priori" => {
            let power = power.ok_or("Power is required for a-priori analysis")?;
            if power <= 0.0 || power >= 1.0 {
                return Err("Power must be between 0 and 1".to_string());
            }

            // Calculate required sample size based on test type
            match test_type.as_str() {
                "independent-t" => {
                    // Cohen's d for independent t-test
                    // n per group = 2 * (Z_alpha + Z_beta)^2 / d^2
                    let z_alpha = norm_inv(1.0 - alpha / 2.0);
                    let z_beta = norm_inv(power);
                    let n_per_group =
                        (2.0 * (z_alpha + z_beta).powi(2) / effect_size.powi(2)).ceil() as i32;
                    format!(
                        "📊 A Priori Power Analysis Results\n\n\
                        Test: Independent t-test\n\
                        Effect size (Cohen's d): {:.3}\n\
                        Alpha level: {:.3}\n\
                        Target power: {:.3}\n\n\
                        ✅ Required sample size:\n\
                        • {} per group\n\
                        • {} total participants\n\n\
                        💡 Interpretation:\n\
                        You need {} participants in each group (total N={}) to detect \
                        a {} effect size with {}% power at α={:.2}.",
                        effect_size,
                        alpha,
                        power,
                        n_per_group,
                        n_per_group * 2,
                        n_per_group,
                        n_per_group * 2,
                        interpret_effect_size(effect_size),
                        (power * 100.0) as i32,
                        alpha
                    )
                }
                "paired-t" => {
                    // Cohen's d for paired t-test
                    // n = (Z_alpha + Z_beta)^2 / d^2
                    let z_alpha = norm_inv(1.0 - alpha / 2.0);
                    let z_beta = norm_inv(power);
                    let n_total = ((z_alpha + z_beta).powi(2) / effect_size.powi(2)).ceil() as i32;
                    format!(
                        "📊 A Priori Power Analysis Results\n\n\
                        Test: Paired t-test\n\
                        Effect size (Cohen's d): {:.3}\n\
                        Alpha level: {:.3}\n\
                        Target power: {:.3}\n\n\
                        ✅ Required sample size: {} participants\n\n\
                        💡 Interpretation:\n\
                        You need {} participants (measured at two time points) to detect \
                        a {} effect size with {}% power at α={:.2}.",
                        effect_size,
                        alpha,
                        power,
                        n_total,
                        n_total,
                        interpret_effect_size(effect_size),
                        (power * 100.0) as i32,
                        alpha
                    )
                }
                "correlation" => {
                    // r for correlation
                    // n = [(Z_alpha + Z_beta) / 0.5 * ln((1+r)/(1-r))]^2 + 3
                    let z_alpha = norm_inv(1.0 - alpha / 2.0);
                    let z_beta = norm_inv(power);
                    let fisher_z = 0.5 * ((1.0 + effect_size) / (1.0 - effect_size)).ln();
                    let n_total = ((z_alpha + z_beta) / fisher_z).powi(2).ceil() as i32 + 3;
                    format!(
                        "📊 A Priori Power Analysis Results\n\n\
                        Test: Correlation (Pearson r)\n\
                        Expected correlation: {:.3}\n\
                        Alpha level: {:.3}\n\
                        Target power: {:.3}\n\n\
                        ✅ Required sample size: {} participants\n\n\
                        💡 Interpretation:\n\
                        You need {} participants to detect a correlation of r={:.2} \
                        with {}% power at α={:.2}.",
                        effect_size,
                        alpha,
                        power,
                        n_total,
                        n_total,
                        effect_size,
                        (power * 100.0) as i32,
                        alpha
                    )
                }
                _ => return Err(format!("Unknown test type: {}", test_type)),
            }
        }
        "post-hoc" => {
            let n = n.ok_or("Sample size (n) is required for post-hoc analysis")?;
            if n <= 0 {
                return Err("Sample size must be positive".to_string());
            }

            // Calculate achieved power based on test type
            match test_type.as_str() {
                "independent-t" => {
                    let z_alpha = norm_inv(1.0 - alpha / 2.0);
                    let n_per_group = n / 2;
                    let ncp = effect_size * (n_per_group as f64 / 2.0).sqrt();
                    let z_beta = ncp - z_alpha;
                    let power = norm_cdf(z_beta);
                    format!(
                        "📊 Post-Hoc Power Analysis Results\n\n\
                        Test: Independent t-test\n\
                        Effect size (Cohen's d): {:.3}\n\
                        Alpha level: {:.3}\n\
                        Sample size: {} per group ({} total)\n\n\
                        ✅ Achieved power: {:.3} ({}%)\n\n\
                        💡 Interpretation:\n\
                        With {} participants per group, your study has {}% power to detect \
                        a {} effect size at α={:.2}. {}",
                        effect_size,
                        alpha,
                        n_per_group,
                        n,
                        power,
                        (power * 100.0) as i32,
                        n_per_group,
                        (power * 100.0) as i32,
                        interpret_effect_size(effect_size),
                        alpha,
                        interpret_power(power)
                    )
                }
                "paired-t" => {
                    let z_alpha = norm_inv(1.0 - alpha / 2.0);
                    let ncp = effect_size * (n as f64).sqrt();
                    let z_beta = ncp - z_alpha;
                    let power = norm_cdf(z_beta);
                    format!(
                        "📊 Post-Hoc Power Analysis Results\n\n\
                        Test: Paired t-test\n\
                        Effect size (Cohen's d): {:.3}\n\
                        Alpha level: {:.3}\n\
                        Sample size: {} participants\n\n\
                        ✅ Achieved power: {:.3} ({}%)\n\n\
                        💡 Interpretation:\n\
                        With {} participants, your study has {}% power to detect \
                        a {} effect size at α={:.2}. {}",
                        effect_size,
                        alpha,
                        n,
                        power,
                        (power * 100.0) as i32,
                        n,
                        (power * 100.0) as i32,
                        interpret_effect_size(effect_size),
                        alpha,
                        interpret_power(power)
                    )
                }
                "correlation" => {
                    let z_alpha = norm_inv(1.0 - alpha / 2.0);
                    let fisher_z = 0.5 * ((1.0 + effect_size) / (1.0 - effect_size)).ln();
                    let se = 1.0 / ((n - 3) as f64).sqrt();
                    let z_beta = fisher_z / se - z_alpha;
                    let power = norm_cdf(z_beta);
                    format!(
                        "📊 Post-Hoc Power Analysis Results\n\n\
                        Test: Correlation (Pearson r)\n\
                        Expected correlation: {:.3}\n\
                        Alpha level: {:.3}\n\
                        Sample size: {} participants\n\n\
                        ✅ Achieved power: {:.3} ({}%)\n\n\
                        💡 Interpretation:\n\
                        With {} participants, your study has {}% power to detect \
                        a correlation of r={:.2} at α={:.2}. {}",
                        effect_size,
                        alpha,
                        n,
                        power,
                        (power * 100.0) as i32,
                        n,
                        (power * 100.0) as i32,
                        effect_size,
                        alpha,
                        interpret_power(power)
                    )
                }
                _ => return Err(format!("Unknown test type: {}", test_type)),
            }
        }
        _ => return Err(format!("Unknown analysis type: {}", analysis_type)),
    };

    Ok(result)
}

// Helper: Normal distribution inverse (approximate)
fn norm_inv(p: f64) -> f64 {
    // Beasley-Springer-Moro algorithm (approximate)
    let a = [
        2.50662823884,
        -18.61500062529,
        41.39119773534,
        -25.44106049637,
    ];
    let b = [
        -8.47351093090,
        23.08336743743,
        -21.06224101826,
        3.13082909833,
    ];
    let c = [
        0.3374754822726147,
        0.9761690190917186,
        0.1607979714918209,
        0.0276438810333863,
        0.0038405729373609,
        0.0003951896511919,
        0.0000321767881768,
        0.0000002888167364,
        0.0000003960315187,
    ];

    let y = p - 0.5;
    if y.abs() < 0.42 {
        let r = y * y;
        let x = y * (((a[3] * r + a[2]) * r + a[1]) * r + a[0])
            / ((((b[3] * r + b[2]) * r + b[1]) * r + b[0]) * r + 1.0);
        x
    } else {
        let r = if y > 0.0 { 1.0 - p } else { p };
        let s = r.ln().abs().sqrt();
        let t = s
            - ((c[8] * s + c[7]) * s + c[6])
                / ((((((c[5] * s + c[4]) * s + c[3]) * s + c[2]) * s + c[1]) * s + c[0]) * s + 1.0);
        if y < 0.0 {
            -t
        } else {
            t
        }
    }
}

// Helper: Normal distribution CDF (approximate)
fn norm_cdf(x: f64) -> f64 {
    0.5 * (1.0 + erf(x / 2.0_f64.sqrt()))
}

// Helper: Error function (approximate)
fn erf(x: f64) -> f64 {
    let a1 = 0.254829592;
    let a2 = -0.284496736;
    let a3 = 1.421413741;
    let a4 = -1.453152027;
    let a5 = 1.061405429;
    let p = 0.3275911;

    let sign = if x < 0.0 { -1.0 } else { 1.0 };
    let x = x.abs();

    let t = 1.0 / (1.0 + p * x);
    let y = 1.0 - (((((a5 * t + a4) * t) + a3) * t + a2) * t + a1) * t * (-x * x).exp();

    sign * y
}

// Helper: Interpret effect size
fn interpret_effect_size(d: f64) -> &'static str {
    if d.abs() < 0.2 {
        "very small"
    } else if d.abs() < 0.5 {
        "small"
    } else if d.abs() < 0.8 {
        "medium"
    } else {
        "large"
    }
}

// Helper: Interpret power
fn interpret_power(power: f64) -> &'static str {
    if power < 0.5 {
        "⚠️ Power is very low - high risk of Type II error (missing a real effect)."
    } else if power < 0.7 {
        "⚠️ Power is below conventional standards (0.80). Consider increasing sample size."
    } else if power < 0.8 {
        "✓ Power is approaching conventional standards."
    } else if power < 0.9 {
        "✅ Power meets conventional standards (≥0.80)."
    } else {
        "✅ Power is excellent (≥0.90)."
    }
}

// COMMAND: Longitudinal merge - Combine two time points
#[command]
fn run_longitudinal_merge(
    t1_path: String,
    t2_path: String,
    id_column: String,
    output_path: String,
) -> Result<String, String> {
    let app = tauri::Builder::default()
        .build(tauri::generate_context!())
        .unwrap();
    let cli_name = if cfg!(windows) { "prism.exe" } else { "prism" };

    let cli_path = app
        .path()
        .resource_dir()
        .ok()
        .and_then(|resource_dir| {
            let bundled = resource_dir.join(cli_name);
            if bundled.exists() {
                Some(bundled)
            } else {
                None
            }
        })
        .or_else(|| {
            let release_path = Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("..")
                .join("target")
                .join("release")
                .join(cli_name);
            if release_path.exists() {
                Some(release_path)
            } else {
                let debug_path = Path::new(env!("CARGO_MANIFEST_DIR"))
                    .join("..")
                    .join("target")
                    .join("debug")
                    .join(cli_name);
                if debug_path.exists() {
                    Some(debug_path)
                } else {
                    None
                }
            }
        })
        .ok_or("CLI binary not found")?;

    let output = Command::new(&cli_path)
        .arg("longitudinal")
        .arg("merge")
        .arg("--t1")
        .arg(&t1_path)
        .arg("--t2")
        .arg(&t2_path)
        .arg("--id-column")
        .arg(&id_column)
        .arg("--output")
        .arg(&output_path)
        .output()
        .map_err(|e| format!("Failed to execute: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Merge failed:\n\n{}", stderr));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    Ok(format!(
        "✅ Successfully merged longitudinal data!\n\n{}\n\nOutput saved to:\n{}",
        stdout, output_path
    ))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // REGISTER ALL COMMANDS HERE
        .invoke_handler(tauri::generate_handler![
            pick_file,
            pick_config,
            generate_config_template,
            save_config_text,
            run_analysis,
            run_analysis_multi_format,
            get_csv_info,
            open_folder,
            get_available_scales,
            get_scale_info,
            generate_scale_config,
            run_dictionary,
            run_consort,
            preview_csv_data,
            file_exists,
            open_html_report,
            read_html_content,
            run_power_analysis,
            run_longitudinal_merge
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
