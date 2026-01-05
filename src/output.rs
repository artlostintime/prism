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

/// Generate comprehensive SPSS syntax file with full transformations
pub fn generate_spss_syntax(
    csv_path: &str,
    config: &SurveyConfig,
    output_path: &str,
) -> Result<()> {
    let mut file = fs::File::create(output_path)?;

    // Header and metadata
    writeln!(
        file,
        "* ======================================================================"
    )?;
    writeln!(file, "* SPSS Syntax for: {}", config.survey.name)?;
    writeln!(
        file,
        "* Generated by Prism v{} on {}",
        env!("CARGO_PKG_VERSION"),
        chrono::Local::now().format("%Y-%m-%d %H:%M:%S")
    )?;
    writeln!(
        file,
        "* ======================================================================"
    )?;
    writeln!(file)?;
    writeln!(file, "* This syntax file includes:")?;
    writeln!(file, "*   - Data import from CSV")?;
    writeln!(file, "*   - Variable labels and descriptions")?;
    writeln!(file, "*   - Value labels for response scales")?;
    writeln!(file, "*   - Reverse scoring transformations")?;
    writeln!(file, "*   - Scale computation (total and mean scores)")?;
    writeln!(file, "*   - Missing value declarations")?;
    writeln!(file)?;
    writeln!(
        file,
        "* ======================================================================"
    )?;
    writeln!(file, "* SECTION 1: DATA IMPORT")?;
    writeln!(
        file,
        "* ======================================================================"
    )?;
    writeln!(file)?;

    // Get DATA command
    writeln!(file, "GET DATA")?;
    writeln!(file, "  /TYPE=TXT")?;
    writeln!(file, "  /FILE='{}'", csv_path)?;
    writeln!(file, "  /ENCODING='UTF8'")?;
    writeln!(file, "  /DELIMITERS=\",\"")?;
    writeln!(file, "  /QUALIFIER='\"'")?;
    writeln!(file, "  /FIRSTCASE=2")?;
    writeln!(file, "  /IMPORTCASE=ALL")?;
    writeln!(file, "  /VARIABLES=")?;

    // Participant ID if specified
    if let Some(id_col) = &config.survey.participant_id_column {
        writeln!(file, "    {} A255", id_col)?;
    }

    // List all raw item variables
    for (scale_name, scale_def) in &config.scales {
        writeln!(file, "    * Items for scale: {}", scale_name)?;
        for item in &scale_def.items {
            writeln!(file, "    {} F8.0", item)?;
        }
    }

    // List all computed scale variables
    for scale_name in config.scales.keys() {
        writeln!(file, "    {}_total F8.2", scale_name)?;
        writeln!(file, "    {}_mean F8.2", scale_name)?;
    }

    // Quality flag and reason if applicable
    if config.quality.is_some() {
        writeln!(file, "    quality_flag A50")?;
        writeln!(file, "    quality_reason A255")?;
    }

    writeln!(file, "    .")?;
    writeln!(file, "EXECUTE.")?;
    writeln!(file)?;

    // SECTION 2: Variable Labels
    writeln!(
        file,
        "* ======================================================================"
    )?;
    writeln!(file, "* SECTION 2: VARIABLE LABELS")?;
    writeln!(
        file,
        "* ======================================================================"
    )?;
    writeln!(file)?;
    writeln!(file, "VARIABLE LABELS")?;

    if let Some(id_col) = &config.survey.participant_id_column {
        writeln!(file, "  {} 'Participant ID'", id_col)?;
    }

    for (scale_name, scale_def) in &config.scales {
        // Labels for raw items
        for (idx, item) in scale_def.items.iter().enumerate() {
            let reverse_marker = if scale_def
                .reverse_scored
                .as_ref()
                .map_or(false, |rev| rev.contains(item))
            {
                " (reverse scored)"
            } else {
                ""
            };
            writeln!(
                file,
                "  {} '{} - Item {}{}'",
                item,
                scale_name,
                idx + 1,
                reverse_marker
            )?;
        }

        // Labels for computed scales
        writeln!(
            file,
            "  {}_total '{} Total Score ({}-{})'",
            scale_name,
            scale_name,
            config.survey.min_score * scale_def.items.len() as u32,
            config.survey.max_score * scale_def.items.len() as u32
        )?;
        writeln!(
            file,
            "  {}_mean '{} Mean Score ({}-{})'",
            scale_name, scale_name, config.survey.min_score, config.survey.max_score
        )?;
    }

    if config.quality.is_some() {
        writeln!(file, "  quality_flag 'Data Quality Flag'")?;
        writeln!(file, "  quality_reason 'Quality Issue Description'")?;
    }

    writeln!(file, "  .")?;
    writeln!(file, "EXECUTE.")?;
    writeln!(file)?;

    // SECTION 3: Value Labels
    writeln!(
        file,
        "* ======================================================================"
    )?;
    writeln!(file, "* SECTION 3: VALUE LABELS FOR RESPONSE SCALES")?;
    writeln!(
        file,
        "* ======================================================================"
    )?;
    writeln!(file)?;

    // Collect all item variables for value labeling
    let mut all_items: Vec<String> = Vec::new();
    for scale_def in config.scales.values() {
        all_items.extend(scale_def.items.clone());
    }

    if !all_items.is_empty() {
        writeln!(file, "VALUE LABELS")?;
        write!(file, "  ")?;
        for (idx, item) in all_items.iter().enumerate() {
            if idx > 0 {
                write!(file, " ")?;
            }
            write!(file, "{}", item)?;
        }
        writeln!(file)?;

        // Generate value labels based on scale range
        for val in config.survey.min_score..=config.survey.max_score {
            let label =
                generate_likert_label(val, config.survey.min_score, config.survey.max_score);
            writeln!(file, "  {} '{}'", val, label)?;
        }
        writeln!(file, "  .")?;
        writeln!(file, "EXECUTE.")?;
        writeln!(file)?;
    }

    if config.quality.is_some() {
        writeln!(file, "VALUE LABELS quality_flag")?;
        writeln!(file, "  'OK' 'Passed all quality checks'")?;
        writeln!(file, "  'FLAGGED' 'Quality issues detected'")?;
        writeln!(file, "  .")?;
        writeln!(file, "EXECUTE.")?;
        writeln!(file)?;
    }

    // SECTION 4: Missing Values
    writeln!(
        file,
        "* ======================================================================"
    )?;
    writeln!(file, "* SECTION 4: MISSING VALUE DECLARATIONS")?;
    writeln!(
        file,
        "* ======================================================================"
    )?;
    writeln!(file)?;
    writeln!(file, "* Declare system missing for out-of-range values")?;

    for scale_def in config.scales.values() {
        for item in &scale_def.items {
            writeln!(
                file,
                "IF ({} < {} OR {} > {}) {} = $SYSMIS.",
                item, config.survey.min_score, item, config.survey.max_score, item
            )?;
        }
    }
    writeln!(file, "EXECUTE.")?;
    writeln!(file)?;

    // SECTION 5: Reverse Scoring
    writeln!(
        file,
        "* ======================================================================"
    )?;
    writeln!(file, "* SECTION 5: REVERSE SCORING TRANSFORMATIONS")?;
    writeln!(
        file,
        "* ======================================================================"
    )?;
    writeln!(file)?;

    let mut has_reverse_items = false;
    for (scale_name, scale_def) in &config.scales {
        if let Some(reverse_items) = &scale_def.reverse_scored {
            if !reverse_items.is_empty() {
                has_reverse_items = true;
                writeln!(file, "* Reverse scoring for {} scale", scale_name)?;
                for item in reverse_items {
                    writeln!(
                        file,
                        "RECODE {} ({} = {}) ({} = {}).",
                        item,
                        config.survey.min_score,
                        config.survey.max_score,
                        config.survey.max_score,
                        config.survey.min_score
                    )?;

                    // Add intermediate values if scale has more than 2 points
                    if config.survey.max_score - config.survey.min_score > 1 {
                        write!(file, "  ")?;
                        for val in (config.survey.min_score + 1)..config.survey.max_score {
                            let reversed_val =
                                config.survey.min_score + config.survey.max_score - val;
                            write!(file, "({} = {}) ", val, reversed_val)?;
                        }
                        writeln!(file)?;
                    }
                }
                writeln!(file, "EXECUTE.")?;
                writeln!(file)?;
            }
        }
    }

    if !has_reverse_items {
        writeln!(file, "* No reverse-scored items in this survey.")?;
        writeln!(file)?;
    }

    // SECTION 6: Scale Computation
    writeln!(
        file,
        "* ======================================================================"
    )?;
    writeln!(file, "* SECTION 6: SCALE SCORE COMPUTATION")?;
    writeln!(
        file,
        "* ======================================================================"
    )?;
    writeln!(file)?;

    for (scale_name, scale_def) in &config.scales {
        writeln!(
            file,
            "* Compute {} scale ({} items)",
            scale_name,
            scale_def.items.len()
        )?;

        // Total score
        write!(file, "COMPUTE {}_total = ", scale_name)?;
        for (idx, item) in scale_def.items.iter().enumerate() {
            if idx > 0 {
                write!(file, " + ")?;
            }
            write!(file, "{}", item)?;
        }
        writeln!(file, ".")?;

        // Mean score
        write!(file, "COMPUTE {}_mean = MEAN(", scale_name)?;
        for (idx, item) in scale_def.items.iter().enumerate() {
            if idx > 0 {
                write!(file, ", ")?;
            }
            write!(file, "{}", item)?;
        }
        writeln!(file, ").")?;
        writeln!(file, "EXECUTE.")?;
        writeln!(file)?;
    }

    // Footer
    writeln!(
        file,
        "* ======================================================================"
    )?;
    writeln!(file, "* END OF SYNTAX")?;
    writeln!(
        file,
        "* ======================================================================"
    )?;
    writeln!(file)?;
    writeln!(file, "* To verify data:")?;
    writeln!(file, "DESCRIPTIVES VARIABLES=")?;
    write!(file, "  ")?;
    for (idx, scale_name) in config.scales.keys().enumerate() {
        if idx > 0 {
            write!(file, " ")?;
        }
        write!(file, "{}_mean", scale_name)?;
    }
    writeln!(file)?;
    writeln!(file, "  /STATISTICS=MEAN STDDEV MIN MAX.")?;
    writeln!(file)?;
    writeln!(file, "* Reliability analysis example:")?;
    if let Some((scale_name, scale_def)) = config.scales.iter().next() {
        writeln!(file, "RELIABILITY")?;
        writeln!(file, "  /VARIABLES=")?;
        write!(file, "    ")?;
        for (idx, item) in scale_def.items.iter().enumerate() {
            if idx > 0 {
                write!(file, " ")?;
            }
            write!(file, "{}", item)?;
        }
        writeln!(file)?;
        writeln!(file, "  /SCALE('{}') ALL", scale_name)?;
        writeln!(file, "  /MODEL=ALPHA.")?;
    }

    Ok(())
}

/// Generate appropriate Likert scale labels based on scale range
fn generate_likert_label(value: u32, min: u32, max: u32) -> String {
    if min == 1 && max == 5 {
        match value {
            1 => "Strongly Disagree".to_string(),
            2 => "Disagree".to_string(),
            3 => "Neutral".to_string(),
            4 => "Agree".to_string(),
            5 => "Strongly Agree".to_string(),
            _ => format!("Response {}", value),
        }
    } else if min == 1 && max == 7 {
        match value {
            1 => "Strongly Disagree".to_string(),
            2 => "Disagree".to_string(),
            3 => "Somewhat Disagree".to_string(),
            4 => "Neutral".to_string(),
            5 => "Somewhat Agree".to_string(),
            6 => "Agree".to_string(),
            7 => "Strongly Agree".to_string(),
            _ => format!("Response {}", value),
        }
    } else if min == 0 && max == 10 {
        match value {
            0 => "Not at all".to_string(),
            5 => "Moderately".to_string(),
            10 => "Extremely".to_string(),
            _ => format!("Response {}", value),
        }
    } else {
        // Generic labels
        if value == min {
            format!("Minimum ({})", value)
        } else if value == max {
            format!("Maximum ({})", value)
        } else if value == (min + max) / 2 {
            format!("Midpoint ({})", value)
        } else {
            format!("Response {}", value)
        }
    }
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
