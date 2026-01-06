// tests/visualization_test.rs
use assert_cmd::cargo;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_html_report_generation() {
    let temp_dir = tempdir().unwrap();
    let output_path = temp_dir.path().join("output.csv");
    let output_str = output_path.to_str().unwrap();

    cargo::cargo_bin_cmd!("prism")
        .args([
            "process",
            "--config",
            "examples/study_config.toml",
            "--input",
            "examples/sample_data.csv",
            "--output",
            output_str,
            "--format",
            "html-report",
        ])
        .assert()
        .success();

    // Check that HTML report was created
    let html_path = output_str.replace(".csv", "_report.html");
    assert!(
        std::path::Path::new(&html_path).exists(),
        "HTML report should be generated"
    );

    // Read and validate HTML content
    let html_content = fs::read_to_string(&html_path).unwrap();

    // Check for essential HTML structure
    assert!(
        html_content.contains("<!DOCTYPE html>"),
        "Should have HTML doctype"
    );
    assert!(html_content.contains("<html"), "Should have html tag");
    assert!(html_content.contains("chart.js"), "Should include Chart.js");
}

#[test]
fn test_html_report_contains_overview_stats() {
    let temp_dir = tempdir().unwrap();
    let output_path = temp_dir.path().join("output.csv");
    let output_str = output_path.to_str().unwrap();

    cargo::cargo_bin_cmd!("prism")
        .args([
            "process",
            "--config",
            "examples/study_config.toml",
            "--input",
            "examples/sample_data.csv",
            "--output",
            output_str,
            "--format",
            "html-report",
        ])
        .assert()
        .success();

    let html_path = output_str.replace(".csv", "_report.html");
    let html_content = fs::read_to_string(&html_path).unwrap();

    // Check for overview section
    assert!(
        html_content.contains("Overview"),
        "Should have overview section"
    );
    assert!(
        html_content.contains("Total Participants"),
        "Should show total participants"
    );
    assert!(
        html_content.contains("Clean Records"),
        "Should show clean records"
    );
    assert!(
        html_content.contains("Flagged Records"),
        "Should show flagged records"
    );
}

#[test]
fn test_html_report_contains_scale_statistics() {
    let temp_dir = tempdir().unwrap();
    let output_path = temp_dir.path().join("output.csv");
    let output_str = output_path.to_str().unwrap();

    cargo::cargo_bin_cmd!("prism")
        .args([
            "process",
            "--config",
            "examples/study_config.toml",
            "--input",
            "examples/sample_data.csv",
            "--output",
            output_str,
            "--format",
            "html-report",
        ])
        .assert()
        .success();

    let html_path = output_str.replace(".csv", "_report.html");
    let html_content = fs::read_to_string(&html_path).unwrap();

    // Check for scale statistics table
    assert!(
        html_content.contains("Scale Statistics"),
        "Should have scale statistics section"
    );
    assert!(
        html_content.contains("<table>"),
        "Should have statistics table"
    );
    assert!(
        html_content.contains("<th>Scale</th>"),
        "Should have scale column"
    );
    assert!(
        html_content.contains("<th>Mean</th>"),
        "Should have mean column"
    );
    assert!(
        html_content.contains("<th>SD</th>"),
        "Should have SD column"
    );

    // Check that scales from config are present
    assert!(
        html_content.contains("alliance_total")
            || html_content.contains("emotional_exhaustion")
            || html_content.contains("supervision_rapport"),
        "Should contain at least one scale name"
    );
}

#[test]
fn test_html_report_contains_distribution_charts() {
    let temp_dir = tempdir().unwrap();
    let output_path = temp_dir.path().join("output.csv");
    let output_str = output_path.to_str().unwrap();

    cargo::cargo_bin_cmd!("prism")
        .args([
            "process",
            "--config",
            "examples/study_config.toml",
            "--input",
            "examples/sample_data.csv",
            "--output",
            output_str,
            "--format",
            "html-report",
        ])
        .assert()
        .success();

    let html_path = output_str.replace(".csv", "_report.html");
    let html_content = fs::read_to_string(&html_path).unwrap();

    // Check for distribution charts section
    assert!(
        html_content.contains("Score Distributions"),
        "Should have distributions section"
    );
    assert!(
        html_content.contains("<canvas"),
        "Should have canvas elements for charts"
    );
    assert!(
        html_content.contains("new Chart("),
        "Should have Chart.js initialization"
    );
    assert!(
        html_content.contains("'bar'"),
        "Should use bar chart type for histograms"
    );
    assert!(
        html_content.contains("Distribution"),
        "Should label as distribution"
    );
}

#[test]
fn test_html_report_contains_quality_issues() {
    let temp_dir = tempdir().unwrap();
    let output_path = temp_dir.path().join("output.csv");
    let output_str = output_path.to_str().unwrap();

    cargo::cargo_bin_cmd!("prism")
        .args([
            "process",
            "--config",
            "examples/study_config.toml",
            "--input",
            "examples/sample_data.csv",
            "--output",
            output_str,
            "--format",
            "html-report",
        ])
        .assert()
        .success();

    let html_path = output_str.replace(".csv", "_report.html");
    let html_content = fs::read_to_string(&html_path).unwrap();

    // Since sample data has quality issues, check for quality section
    assert!(
        html_content.contains("Quality Issues"),
        "Should have quality issues section"
    );
    assert!(
        html_content.contains("Issue Summary"),
        "Should have issue summary"
    );
}

#[test]
fn test_html_report_has_styling() {
    let temp_dir = tempdir().unwrap();
    let output_path = temp_dir.path().join("output.csv");
    let output_str = output_path.to_str().unwrap();

    cargo::cargo_bin_cmd!("prism")
        .args([
            "process",
            "--config",
            "examples/study_config.toml",
            "--input",
            "examples/sample_data.csv",
            "--output",
            output_str,
            "--format",
            "html-report",
        ])
        .assert()
        .success();

    let html_path = output_str.replace(".csv", "_report.html");
    let html_content = fs::read_to_string(&html_path).unwrap();

    // Check for CSS styling
    assert!(html_content.contains("<style>"), "Should have embedded CSS");
    assert!(
        html_content.contains("--primary-color"),
        "Should use CSS variables"
    );
    assert!(
        html_content.contains("font-family"),
        "Should have font styling"
    );
    assert!(
        html_content.contains(".container"),
        "Should have container class"
    );
}

#[test]
fn test_html_report_has_header_and_footer() {
    let temp_dir = tempdir().unwrap();
    let output_path = temp_dir.path().join("output.csv");
    let output_str = output_path.to_str().unwrap();

    cargo::cargo_bin_cmd!("prism")
        .args([
            "process",
            "--config",
            "examples/study_config.toml",
            "--input",
            "examples/sample_data.csv",
            "--output",
            output_str,
            "--format",
            "html-report",
        ])
        .assert()
        .success();

    let html_path = output_str.replace(".csv", "_report.html");
    let html_content = fs::read_to_string(&html_path).unwrap();

    // Check for header
    assert!(
        html_content.contains("Analysis Report"),
        "Should have report title"
    );
    assert!(
        html_content.contains("Generated by Prism"),
        "Should credit Prism"
    );

    // Check for footer
    assert!(
        html_content.contains("<div class=\"footer\">"),
        "Should have footer"
    );
    assert!(html_content.contains("GitHub"), "Should link to GitHub");
}

#[test]
fn test_html_report_includes_survey_name() {
    let temp_dir = tempdir().unwrap();
    let output_path = temp_dir.path().join("output.csv");
    let output_str = output_path.to_str().unwrap();

    cargo::cargo_bin_cmd!("prism")
        .args([
            "process",
            "--config",
            "examples/study_config.toml",
            "--input",
            "examples/sample_data.csv",
            "--output",
            output_str,
            "--format",
            "html-report",
        ])
        .assert()
        .success();

    let html_path = output_str.replace(".csv", "_report.html");
    let html_content = fs::read_to_string(&html_path).unwrap();

    // Survey name from study_config.toml is "Clinical Interactions & Trainee Well-Being"
    assert!(
        html_content.contains("Clinical Interactions &")
            || html_content.contains("Clinical Interactions &amp;"),
        "Should include survey name from config (with properly escaped &)"
    );
}

#[test]
fn test_html_report_chart_data_format() {
    let temp_dir = tempdir().unwrap();
    let output_path = temp_dir.path().join("output.csv");
    let output_str = output_path.to_str().unwrap();

    cargo::cargo_bin_cmd!("prism")
        .args([
            "process",
            "--config",
            "examples/study_config.toml",
            "--input",
            "examples/sample_data.csv",
            "--output",
            output_str,
            "--format",
            "html-report",
        ])
        .assert()
        .success();

    let html_path = output_str.replace(".csv", "_report.html");
    let html_content = fs::read_to_string(&html_path).unwrap();

    // Check that Chart.js is properly configured
    assert!(html_content.contains("labels:"), "Should have chart labels");
    assert!(
        html_content.contains("datasets:"),
        "Should have chart datasets"
    );
    assert!(
        html_content.contains("backgroundColor:"),
        "Should have chart colors"
    );
    assert!(
        html_content.contains("responsive: true"),
        "Should be responsive"
    );
}
