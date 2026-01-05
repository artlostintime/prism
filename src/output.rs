// src/output.rs
use crate::config::SurveyConfig;
use crate::errors::Result;
use crate::stats::{calculate_cronbachs_alpha, Stats};
use crate::types::QualityIssue;
use rust_xlsxwriter::{Format, Workbook};
use std::collections::HashMap;
use std::fs;
use std::io::Write;

/// Generate summary statistics file
pub fn generate_summary_stats(
    config: &SurveyConfig,
    scale_scores: &HashMap<String, Vec<f64>>,
    scale_items_matrix: &HashMap<String, Vec<Vec<f64>>>,
    total_participants: usize,
    output_path: &str,
    quality_issues: &[QualityIssue],
) -> Result<()> {
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
                    .is_some_and(|rev| rev.contains(item))
                {
                    write!(file, "{}*", item)?;
                } else {
                    write!(file, "{}", item)?;
                }
            }
            if scale_def
                .reverse_scored
                .as_ref()
                .is_some_and(|rev| !rev.is_empty())
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

            // Calculate and display Cronbach's alpha
            if let Some(items_matrix) = scale_items_matrix.get(scale_name) {
                if items_matrix.len() > 1 && !items_matrix.is_empty() && items_matrix[0].len() > 1 {
                    let alpha = calculate_cronbachs_alpha(items_matrix);
                    writeln!(file, "  Cronbach's Alpha (α)  = {:.3}", alpha)?;

                    // Interpretation guide
                    let interpretation = match alpha {
                        a if a >= 0.9 => "Excellent",
                        a if a >= 0.8 => "Good",
                        a if a >= 0.7 => "Acceptable",
                        a if a >= 0.6 => "Questionable",
                        a if a >= 0.5 => "Poor",
                        _ => "Unacceptable",
                    };
                    writeln!(file, "                          ({})", interpretation)?;
                }
            }
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

/// Generate quality report file
pub fn generate_quality_report(
    quality_issues: &[QualityIssue],
    total_participants: usize,
    output_path: &str,
) -> Result<()> {
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
                .or_default()
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

/// Generate JSON output
pub fn generate_json_output(
    config: &SurveyConfig,
    scale_scores: &HashMap<String, Vec<f64>>,
    quality_issues: &[QualityIssue],
    total_participants: usize,
    output_path: &str,
) -> Result<()> {
    use serde_json::json;

    let mut scale_stats = serde_json::Map::new();
    for (scale_name, scores) in scale_scores {
        let stats = Stats::calculate(scores);
        scale_stats.insert(
            scale_name.clone(),
            json!({
                "mean": stats.mean,
                "sd": stats.sd,
                "min": stats.min,
                "max": stats.max,
                "n": stats.n
            }),
        );
    }

    let output = json!({
        "survey_name": config.survey.name,
        "generated": chrono::Local::now().to_rfc3339(),
        "total_participants": total_participants,
        "clean_records": total_participants - quality_issues.len(),
        "flagged_records": quality_issues.len(),
        "scale_statistics": scale_stats,
        "quality_issues": quality_issues
    });

    let mut file = fs::File::create(output_path)?;
    writeln!(file, "{}", serde_json::to_string_pretty(&output)?)?;

    Ok(())
}

/// Generate Excel output
pub fn generate_excel_output(
    records: &[Vec<String>],
    headers: &[String],
    output_path: &str,
) -> Result<()> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();

    // Format for headers
    let header_format = Format::new()
        .set_bold()
        .set_background_color(rust_xlsxwriter::Color::RGB(0xD3D3D3));

    // Write headers
    for (col, header) in headers.iter().enumerate() {
        worksheet.write_string_with_format(0, col as u16, header, &header_format)?;
    }

    // Write data
    for (row, record) in records.iter().enumerate() {
        for (col, value) in record.iter().enumerate() {
            // Try to parse as number, otherwise write as string
            if let Ok(num) = value.parse::<f64>() {
                worksheet.write_number((row + 1) as u32, col as u16, num)?;
            } else {
                worksheet.write_string((row + 1) as u32, col as u16, value)?;
            }
        }
    }

    workbook.save(output_path)?;
    Ok(())
}

/// Generate SPSS syntax file
pub fn generate_spss_syntax(
    csv_path: &str,
    config: &SurveyConfig,
    output_path: &str,
) -> Result<()> {
    let mut file = fs::File::create(output_path)?;

    writeln!(file, "* SPSS Syntax for {}", config.survey.name)?;
    writeln!(
        file,
        "* Generated: {}",
        chrono::Local::now().format("%Y-%m-%d %H:%M:%S")
    )?;
    writeln!(file)?;
    writeln!(file, "GET DATA")?;
    writeln!(file, "  /TYPE=TXT")?;
    writeln!(file, "  /FILE='{}'", csv_path)?;
    writeln!(file, "  /DELIMITERS=\",\"")?;
    writeln!(file, "  /FIRSTCASE=2")?;
    writeln!(file, "  /VARIABLES=")?;

    // List all scale variables
    for scale_name in config.scales.keys() {
        writeln!(file, "    {}_total F8.2", scale_name)?;
        writeln!(file, "    {}_mean F8.2", scale_name)?;
    }

    writeln!(file, "    .")?;
    writeln!(file, "EXECUTE.")?;
    writeln!(file)?;
    writeln!(file, "* Add variable labels and value labels as needed")?;

    Ok(())
}

/// Generate R script for data import
pub fn generate_r_script(csv_path: &str, config: &SurveyConfig, output_path: &str) -> Result<()> {
    let mut file = fs::File::create(output_path)?;

    writeln!(file, "# R Script for {}", config.survey.name)?;
    writeln!(
        file,
        "# Generated: {}",
        chrono::Local::now().format("%Y-%m-%d %H:%M:%S")
    )?;
    writeln!(file)?;
    writeln!(file, "# Load required packages")?;
    writeln!(file, "library(tidyverse)")?;
    writeln!(file)?;
    writeln!(file, "# Import data")?;
    writeln!(file, "data <- read_csv(\"{}\")", csv_path)?;
    writeln!(file)?;
    writeln!(file, "# View structure")?;
    writeln!(file, "glimpse(data)")?;
    writeln!(file)?;
    writeln!(file, "# Summary statistics for scales")?;
    for scale_name in config.scales.keys() {
        writeln!(file, "data %>% select({}_mean) %>% summary()", scale_name)?;
    }
    writeln!(file)?;
    writeln!(file, "# Filter clean records only")?;
    writeln!(file, "clean_data <- data %>% filter(quality_flag == 'OK')")?;
    writeln!(
        file,
        "message(sprintf('Clean records: %d / %d (%.1f%%)', nrow(clean_data), nrow(data), 100 * nrow(clean_data) / nrow(data)))"
    )?;

    Ok(())
}
