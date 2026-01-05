use rfd::FileDialog;
use std::fs;
use std::io::BufRead;
use std::path::{Path, PathBuf};
use std::process::Command;
use tauri::command;

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
fn run_analysis(input_path: String, config_path: Option<String>) -> String {
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

    // Find the CLI binary
    let cli_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("target")
        .join("release")
        .join(cli_name);

    // Fallback to debug if release doesn't exist
    let cli_path = if cli_path.exists() {
        cli_path
    } else {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("target")
            .join("debug")
            .join(cli_name)
    };

    if !cli_path.exists() {
        return format!(
            "Error: CLI binary not found at {:?}\n\nPlease build the CLI first:\ncargo build --release",
            cli_path
        );
    }

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
    input_path: String,
    config_path: String,
    formats: Vec<String>, // "csv", "excel", "spss", "r", "python", "html"
) -> Result<String, String> {
    let input_path_obj = Path::new(&input_path);
    let cli_name = if cfg!(windows) { "prism.exe" } else { "prism" };

    let cli_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("target")
        .join("release")
        .join(cli_name);

    let cli_path = if cli_path.exists() {
        cli_path
    } else {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("target")
            .join("debug")
            .join(cli_name)
    };

    if !cli_path.exists() {
        return Err(format!(
            "CLI binary not found at {:?}\n\nPlease build first: cargo build --release",
            cli_path
        ));
    }

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
    config_path: String,
    output_path: String,
    format: String,
) -> Result<String, String> {
    let cli_name = if cfg!(windows) { "prism.exe" } else { "prism" };

    let cli_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("target")
        .join("release")
        .join(cli_name);

    let cli_path = if cli_path.exists() {
        cli_path
    } else {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("target")
            .join("debug")
            .join(cli_name)
    };

    if !cli_path.exists() {
        return Err(format!(
            "CLI binary not found at {:?}\n\nPlease build first: cargo build --release",
            cli_path
        ));
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
    input_path: String,
    config_path: String,
    output_path: String,
    format: String,
) -> Result<String, String> {
    let cli_name = if cfg!(windows) { "prism.exe" } else { "prism" };

    let cli_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("target")
        .join("release")
        .join(cli_name);

    let cli_path = if cli_path.exists() {
        cli_path
    } else {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("target")
            .join("debug")
            .join(cli_name)
    };

    if !cli_path.exists() {
        return Err(format!(
            "CLI binary not found at {:?}\n\nPlease build first: cargo build --release",
            cli_path
        ));
    }

    // First process the data
    let output_folder = Path::new(&output_path)
        .parent()
        .unwrap_or(Path::new(&output_path));
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
    let excluded_pct = (excluded as f64 / total as f64) * 100.0;
    let retained_pct = (retained as f64 / total as f64) * 100.0;

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
        (excluded as f64 / total as f64) * 100.0,
        issues_json.join(",\n    "),
        retained,
        (retained as f64 / total as f64) * 100.0
    )
}

// Helper: Parse quality report for issues
fn parse_quality_issues(quality_report: &str) -> (usize, Vec<(String, usize)>) {
    let mut excluded = 0;
    let mut issues: std::collections::HashMap<String, usize> = std::collections::HashMap::new();

    for line in quality_report.lines() {
        if line.contains("flagged") || line.contains("detected") {
            excluded += 1;

            if line.contains("missing") || line.contains("Missing") {
                *issues.entry("Missing data".to_string()).or_insert(0) += 1;
            } else if line.contains("straightlin") || line.contains("Straightlin") {
                *issues.entry("Straightlining".to_string()).or_insert(0) += 1;
            } else if line.contains("diagonal") || line.contains("Diagonal") {
                *issues.entry("Diagonal pattern".to_string()).or_insert(0) += 1;
            } else if line.contains("alternating") || line.contains("Alternating") {
                *issues.entry("Alternating pattern".to_string()).or_insert(0) += 1;
            } else if line.contains("block") || line.contains("Block") {
                *issues.entry("Block pattern".to_string()).or_insert(0) += 1;
            } else if line.contains("variance") || line.contains("Variance") {
                *issues.entry("Low variance".to_string()).or_insert(0) += 1;
            } else if line.contains("time") || line.contains("Time") {
                *issues.entry("Response time".to_string()).or_insert(0) += 1;
            } else if line.contains("semantic") || line.contains("Semantic") {
                *issues
                    .entry("Semantic inconsistency".to_string())
                    .or_insert(0) += 1;
            }
        }
    }

    let issues_vec: Vec<(String, usize)> = issues.into_iter().collect();
    (excluded, issues_vec)
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
            run_consort
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
