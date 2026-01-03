use prism::config::SurveyConfig;
use std::fs;
use std::io::Write;
use std::time::Instant;

fn generate_test_data(
    rows: usize,
    cols: usize,
    filepath: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = fs::File::create(filepath)?;

    // Write header
    write!(file, "ID")?;
    for i in 1..=cols {
        write!(file, ",Q{}", i)?;
    }
    writeln!(file)?;

    // Write data rows
    for participant_id in 1..=rows {
        write!(file, "P{:04}", participant_id)?;
        for _ in 1..=cols {
            // Random scores between 1-7
            let score = (participant_id % 7) + 1;
            write!(file, ",{}", score)?;
        }
        writeln!(file)?;
    }

    Ok(())
}

fn create_test_config(cols: usize, filepath: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut config = String::new();
    config.push_str(
        r#"[survey]
name = "Performance Benchmark Test"
min_score = 1
max_score = 7

[quality]
max_missing_percent = 0.10
flag_straightlining = true

"#,
    );

    // Create scales of 10 items each
    let num_scales = cols / 10;
    for scale_num in 0..num_scales {
        let start_item = scale_num * 10 + 1;
        config.push_str(&format!("[scales.scale_{}]\n", scale_num));
        config.push_str("items = [");
        for item in start_item..(start_item + 10) {
            if item > start_item {
                config.push_str(", ");
            }
            config.push_str(&format!("\"Q{}\"", item));
        }
        config.push_str("]\nreverse_scored = []\n\n");
    }

    fs::write(filepath, config)?;
    Ok(())
}

fn benchmark_scenario(
    name: &str,
    rows: usize,
    cols: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== Benchmark: {} ===", name);
    println!("Dimensions: {} rows × {} columns", rows, cols);

    let data_file = format!("target/bench_data_{}_{}.csv", rows, cols);
    let config_file = format!("target/bench_config_{}_{}.toml", rows, cols);

    // Setup
    print!("  Generating test data... ");
    let setup_start = Instant::now();
    generate_test_data(rows, cols, &data_file)?;
    create_test_config(cols, &config_file)?;
    println!("✓ ({:.2}s)", setup_start.elapsed().as_secs_f64());

    // Load config
    print!("  Loading configuration... ");
    let load_start = Instant::now();
    let config_str = fs::read_to_string(&config_file)?;
    let _config: SurveyConfig = toml::from_str(&config_str)?;
    println!("✓ ({:.2}s)", load_start.elapsed().as_secs_f64());

    // Process data (using CLI approach)
    print!("  Processing survey data... ");
    // Note: This benchmark is outdated and needs to be updated to use the current CLI API
    // For now, we just measure config loading time
    println!("⚠ (Benchmark needs API update)");

    println!("  Results:");
    println!("    • Dataset size: {} rows × {} columns", rows, cols);
    println!("    • Config loaded: ✓");
    println!("    • Note: Full benchmark requires API update");

    // Cleanup
    let _ = fs::remove_file(&data_file);
    let _ = fs::remove_file(&config_file);

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("╔════════════════════════════════════════════════════╗");
    println!("║         PRISM PERFORMANCE BENCHMARK SUITE          ║");
    println!("╚════════════════════════════════════════════════════╝");

    let scenarios = vec![
        ("Small Dataset", 100, 50),
        ("Medium Dataset", 1_000, 100),
        ("Large Dataset", 10_000, 100),
        ("Very Large Dataset", 50_000, 100),
        ("Wide Dataset", 1_000, 500),
    ];

    let total_start = Instant::now();

    for (name, rows, cols) in scenarios {
        match benchmark_scenario(name, rows, cols) {
            Ok(_) => println!("  Status: ✓ PASSED"),
            Err(e) => println!("  Status: ✗ FAILED - {}", e),
        }
    }

    let total_time = total_start.elapsed();

    println!("\n╔════════════════════════════════════════════════════╗");
    println!("║               BENCHMARK COMPLETE                   ║");
    println!("╚════════════════════════════════════════════════════╝");
    println!("Total time: {:.2}s", total_time.as_secs_f64());

    Ok(())
}
