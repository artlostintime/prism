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

# Add your scales below
[scales.my_scale]
items = ["Q1", "Q2", "Q3", "Q4", "Q5"]
reverse_scored = []

# Example with reverse scoring
[scales.another_scale]
items = ["Q10", "Q11", "Q12"]
reverse_scored = ["Q12"]
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
            get_csv_info,
            open_folder
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
