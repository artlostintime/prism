use rfd::FileDialog;
use std::path::Path;
use std::process::Command;
use tauri::command;

// COMMAND 1: Pick the File
#[command]
fn pick_file() -> String {
    let file = FileDialog::new().add_filter("CSV", &["csv"]).pick_file();

    match file {
        Some(path) => path.display().to_string(),
        None => "".to_string(),
    }
}

// COMMAND 2: Run the CLI (Simple Wrapper)
#[command]
fn run_analysis(input_path: String) -> String {
    // Find the config file in the parent directory
    let input_path_obj = Path::new(&input_path);
    let config_path = match input_path_obj.parent() {
        Some(parent) => parent.join("..").join("study_config.toml"),
        None => return "Error: Could not determine parent directory of input file".to_string(),
    };

    // Verify config exists
    if !config_path.exists() {
        return format!(
            "Error: Config file not found at {:?}\n\nPlease ensure study_config.toml is in the project root.",
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

    // Determine output path
    let output_path = match input_path_obj.parent() {
        Some(parent) => parent.join("clean_data.csv"),
        None => return "Error: Could not determine output path".to_string(),
    };

    // Run the CLI command with stats and quality reports
    let stats_path = match input_path_obj.parent() {
        Some(parent) => parent.join("summary_stats.txt"),
        None => return "Error: Could not determine output path".to_string(),
    };

    let quality_path = match input_path_obj.parent() {
        Some(parent) => parent.join("quality_report.txt"),
        None => return "Error: Could not determine output path".to_string(),
    };

    let output = match Command::new(&cli_path)
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
        return format!("❌ Processing Error:\n\n{}", stderr);
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
        "\n\n📊 Full statistics and quality report generated."
    } else {
        ""
    };

    format!(
        "✅ Success! Processed {} participants.{}\n\nOutput files:\n• {}\n• {}\n• {}",
        count,
        quality_summary,
        output_path.display(),
        stats_path.display(),
        quality_path.display()
    )
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // REGISTER BOTH COMMANDS HERE
        .invoke_handler(tauri::generate_handler![pick_file, run_analysis])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
