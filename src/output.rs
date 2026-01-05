// src/output.rs
use crate::config::SurveyConfig;
use crate::errors::Result;
use crate::stats::{calculate_cronbachs_alpha, Stats};
use crate::types::QualityIssue;
use rust_xlsxwriter::{Format, Workbook};
use serde_json::json;
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

        // Group by issue type with capacity hint
        let mut by_type: HashMap<String, Vec<&QualityIssue>> = HashMap::with_capacity(8);
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
                .is_some_and(|rev| rev.contains(item))
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

/// Generate comprehensive R analysis script
pub fn generate_r_script(csv_path: &str, config: &SurveyConfig, output_path: &str) -> Result<()> {
    let mut file = fs::File::create(output_path)?;

    // Header
    writeln!(
        file,
        "# ======================================================================"
    )?;
    writeln!(file, "# R Analysis Script for: {}", config.survey.name)?;
    writeln!(
        file,
        "# Generated by Prism v{} on {}",
        env!("CARGO_PKG_VERSION"),
        chrono::Local::now().format("%Y-%m-%d %H:%M:%S")
    )?;
    writeln!(
        file,
        "# ======================================================================"
    )?;
    writeln!(file)?;
    writeln!(
        file,
        "# This script provides a complete analysis pipeline including:"
    )?;
    writeln!(file, "#   - Data import and cleaning")?;
    writeln!(file, "#   - Descriptive statistics")?;
    writeln!(file, "#   - Reliability analysis (Cronbach's alpha)")?;
    writeln!(file, "#   - Data visualization")?;
    writeln!(file, "#   - Quality checks and filtering")?;
    writeln!(file)?;
    writeln!(
        file,
        "# ======================================================================"
    )?;
    writeln!(file, "# SETUP: Install Required Packages")?;
    writeln!(
        file,
        "# ======================================================================"
    )?;
    writeln!(file)?;
    writeln!(file, "# Uncomment to install packages (run once):")?;
    writeln!(
        file,
        "# install.packages(c('tidyverse', 'psych', 'ggplot2', 'patchwork'))"
    )?;
    writeln!(file)?;
    writeln!(file, "# Load libraries")?;
    writeln!(
        file,
        "library(tidyverse)  # Data manipulation and visualization"
    )?;
    writeln!(file, "library(psych)      # Reliability analysis")?;
    writeln!(file, "library(ggplot2)    # Advanced plotting")?;
    writeln!(file, "library(patchwork)  # Combine plots")?;
    writeln!(file)?;
    writeln!(
        file,
        "# ======================================================================"
    )?;
    writeln!(file, "# DATA IMPORT")?;
    writeln!(
        file,
        "# ======================================================================"
    )?;
    writeln!(file)?;
    writeln!(file, "# Import processed data")?;
    writeln!(file, "data <- read_csv('{}')", csv_path)?;
    writeln!(file, "cat('\\n=== Data Overview ===\\n')")?;
    writeln!(file, "glimpse(data)")?;
    writeln!(file)?;
    writeln!(
        file,
        "# ======================================================================"
    )?;
    writeln!(file, "# QUALITY FILTERING")?;
    writeln!(
        file,
        "# ======================================================================"
    )?;
    writeln!(file)?;

    if config.quality.is_some() {
        writeln!(file, "# Examine quality flags")?;
        writeln!(file, "cat('\\n=== Quality Check Summary ===\\n')")?;
        writeln!(file, "table(data$quality_flag)")?;
        writeln!(file)?;
        writeln!(file, "# Display flagged records")?;
        writeln!(
            file,
            "flagged <- data %>% filter(quality_flag == 'FLAGGED')"
        )?;
        writeln!(file, "cat(sprintf('Flagged records: %d (%.1f%%)\\n', ")?;
        writeln!(
            file,
            "    nrow(flagged), 100 * nrow(flagged) / nrow(data)))"
        )?;
        writeln!(file)?;
        writeln!(file, "if (nrow(flagged) > 0) {{")?;
        writeln!(file, "  cat('\\nReasons for flagging:\\n')")?;
        writeln!(file, "  print(table(flagged$quality_reason))")?;
        writeln!(file, "}}")?;
        writeln!(file)?;
        writeln!(file, "# Create clean dataset")?;
        writeln!(file, "clean_data <- data %>% filter(quality_flag == 'OK')")?;
        writeln!(file, "cat(sprintf('\\nClean records: %d (%.1f%%)\\n', ")?;
        writeln!(
            file,
            "    nrow(clean_data), 100 * nrow(clean_data) / nrow(data)))"
        )?;
    } else {
        writeln!(file, "# No quality checks configured")?;
        writeln!(file, "clean_data <- data")?;
    }
    writeln!(file)?;
    writeln!(
        file,
        "# ======================================================================"
    )?;
    writeln!(file, "# DESCRIPTIVE STATISTICS")?;
    writeln!(
        file,
        "# ======================================================================"
    )?;
    writeln!(file)?;

    for (scale_name, scale_def) in &config.scales {
        writeln!(file, "# {} ({} items)", scale_name, scale_def.items.len())?;
        writeln!(file, "cat('\\n=== {} Statistics ===\\n')", scale_name)?;
        writeln!(file, "clean_data %>%")?;
        writeln!(file, "  select({}_mean) %>%", scale_name)?;
        writeln!(file, "  summary() %>%")?;
        writeln!(file, "  print()")?;
        writeln!(file)?;
        writeln!(file, "# Standard deviation")?;
        writeln!(
            file,
            "sd_val <- sd(clean_data${}_mean, na.rm = TRUE)",
            scale_name
        )?;
        writeln!(file, "cat(sprintf('SD: %.2f\\n', sd_val))")?;
        writeln!(file)?;
    }

    writeln!(
        file,
        "# ======================================================================"
    )?;
    writeln!(file, "# RELIABILITY ANALYSIS (Cronbach's Alpha)")?;
    writeln!(
        file,
        "# ======================================================================"
    )?;
    writeln!(file)?;

    for (scale_name, scale_def) in &config.scales {
        if scale_def.items.len() > 1 {
            writeln!(file, "# Reliability for {}", scale_name)?;
            writeln!(file, "cat('\\n=== {} Reliability ===\\n')", scale_name)?;
            write!(file, "{}_items <- clean_data %>% select(", scale_name)?;
            for (idx, item) in scale_def.items.iter().enumerate() {
                if idx > 0 {
                    write!(file, ", ")?;
                }
                write!(file, "{}", item)?;
            }
            writeln!(file, ")")?;
            writeln!(file, "{}_alpha <- alpha({}_items)", scale_name, scale_name)?;
            writeln!(file, "print({}_alpha, digits = 3)", scale_name)?;
            writeln!(file)?;
        }
    }

    writeln!(
        file,
        "# ======================================================================"
    )?;
    writeln!(file, "# DATA VISUALIZATION")?;
    writeln!(
        file,
        "# ======================================================================"
    )?;
    writeln!(file)?;

    // Distribution plots
    writeln!(file, "# Distribution plots for scales")?;
    for (idx, scale_name) in config.scales.keys().enumerate() {
        let plot_var = format!("p{}", idx + 1);
        writeln!(
            file,
            "{} <- ggplot(clean_data, aes(x = {}_mean)) +",
            plot_var, scale_name
        )?;
        writeln!(
            file,
            "  geom_histogram(binwidth = 0.5, fill = 'steelblue', color = 'black', alpha = 0.7) +"
        )?;
        writeln!(
            file,
            "  geom_vline(aes(xintercept = mean({}_mean, na.rm = TRUE)), ",
            scale_name
        )?;
        writeln!(
            file,
            "             color = 'red', linetype = 'dashed', size = 1) +"
        )?;
        writeln!(
            file,
            "  labs(title = '{}', x = 'Mean Score', y = 'Frequency') +",
            scale_name
        )?;
        writeln!(file, "  theme_minimal()")?;
        writeln!(file)?;
    }

    if config.scales.len() > 1 {
        writeln!(file, "# Combine plots")?;
        write!(file, "combined_plot <- ")?;
        for (idx, _) in config.scales.keys().enumerate() {
            if idx > 0 {
                write!(file, " + ")?;
            }
            write!(file, "p{}", idx + 1)?;
        }
        writeln!(file)?;
        writeln!(file, "print(combined_plot)")?;
        writeln!(file)?;
        writeln!(file, "# Save plot")?;
        writeln!(
            file,
            "ggsave('scale_distributions.png', combined_plot, width = 12, height = 8, dpi = 300)"
        )?;
    } else {
        writeln!(file, "print(p1)")?;
        writeln!(
            file,
            "ggsave('scale_distribution.png', p1, width = 8, height = 6, dpi = 300)"
        )?;
    }
    writeln!(file)?;

    // Box plots
    if config.quality.is_some() {
        writeln!(file, "# Box plot: Compare flagged vs clean data")?;
        for scale_name in config.scales.keys() {
            writeln!(file, "p_box_{} <- ggplot(data, aes(x = quality_flag, y = {}_mean, fill = quality_flag)) +", scale_name, scale_name)?;
            writeln!(file, "  geom_boxplot(alpha = 0.7) +")?;
            writeln!(
                file,
                "  scale_fill_manual(values = c('OK' = 'lightgreen', 'FLAGGED' = 'salmon')) +"
            )?;
            writeln!(
                file,
                "  labs(title = '{} by Quality Flag', x = 'Quality Flag', y = 'Mean Score') +",
                scale_name
            )?;
            writeln!(file, "  theme_minimal() +")?;
            writeln!(file, "  theme(legend.position = 'none')")?;
            writeln!(file, "print(p_box_{})", scale_name)?;
            writeln!(file)?;
        }
    }

    writeln!(
        file,
        "# ======================================================================"
    )?;
    writeln!(file, "# CORRELATION MATRIX (if multiple scales)")?;
    writeln!(
        file,
        "# ======================================================================"
    )?;
    writeln!(file)?;

    if config.scales.len() > 1 {
        writeln!(file, "# Select scale means")?;
        write!(file, "scale_means <- clean_data %>% select(")?;
        for (idx, scale_name) in config.scales.keys().enumerate() {
            if idx > 0 {
                write!(file, ", ")?;
            }
            write!(file, "{}_mean", scale_name)?;
        }
        writeln!(file, ")")?;
        writeln!(file)?;
        writeln!(file, "# Correlation matrix")?;
        writeln!(file, "cat('\\n=== Correlation Matrix ===\\n')")?;
        writeln!(
            file,
            "cor_matrix <- cor(scale_means, use = 'pairwise.complete.obs')"
        )?;
        writeln!(file, "print(round(cor_matrix, 2))")?;
        writeln!(file)?;
        writeln!(file, "# Correlation plot")?;
        writeln!(file, "library(corrplot)")?;
        writeln!(
            file,
            "corrplot(cor_matrix, method = 'circle', type = 'upper', "
        )?;
        writeln!(
            file,
            "         addCoef.col = 'black', tl.col = 'black', tl.srt = 45)"
        )?;
    }
    writeln!(file)?;

    writeln!(
        file,
        "# ======================================================================"
    )?;
    writeln!(file, "# EXPORT RESULTS")?;
    writeln!(
        file,
        "# ======================================================================"
    )?;
    writeln!(file)?;
    writeln!(file, "# Save clean dataset")?;
    writeln!(file, "write_csv(clean_data, 'clean_data_r.csv')")?;
    writeln!(file)?;
    writeln!(file, "# Create summary table")?;
    write!(file, "summary_stats <- clean_data %>% summarise(")?;
    for (idx, scale_name) in config.scales.keys().enumerate() {
        if idx > 0 {
            write!(file, ", ")?;
        }
        writeln!(file)?;
        write!(
            file,
            "  '{}_M' = mean({}_mean, na.rm = TRUE),",
            scale_name, scale_name
        )?;
        writeln!(file)?;
        write!(
            file,
            "  '{}_SD' = sd({}_mean, na.rm = TRUE),",
            scale_name, scale_name
        )?;
        writeln!(file)?;
        write!(
            file,
            "  '{}_Min' = min({}_mean, na.rm = TRUE),",
            scale_name, scale_name
        )?;
        writeln!(file)?;
        write!(
            file,
            "  '{}_Max' = max({}_mean, na.rm = TRUE)",
            scale_name, scale_name
        )?;
    }
    writeln!(file)?;
    writeln!(file, ")")?;
    writeln!(file)?;
    writeln!(file, "cat('\\n=== Summary Statistics Table ===\\n')")?;
    writeln!(file, "print(summary_stats)")?;
    writeln!(file)?;
    writeln!(file, "# Save summary table")?;
    writeln!(file, "write_csv(summary_stats, 'summary_statistics.csv')")?;
    writeln!(file)?;
    writeln!(file, "cat('\\n✓ Analysis complete! Results saved.\\n')")?;

    Ok(())
}

/// Generate comprehensive Python analysis script
pub fn generate_python_script(
    csv_path: &str,
    config: &SurveyConfig,
    output_path: &str,
) -> Result<()> {
    let mut file = fs::File::create(output_path)?;

    // Header
    writeln!(
        file,
        "# ======================================================================"
    )?;
    writeln!(file, "# Python Analysis Script for: {}", config.survey.name)?;
    writeln!(
        file,
        "# Generated by Prism v{} on {}",
        env!("CARGO_PKG_VERSION"),
        chrono::Local::now().format("%Y-%m-%d %H:%M:%S")
    )?;
    writeln!(
        file,
        "# ======================================================================"
    )?;
    writeln!(file)?;
    writeln!(
        file,
        "# This script provides a complete analysis pipeline including:"
    )?;
    writeln!(file, "#   - Data import and cleaning")?;
    writeln!(file, "#   - Descriptive statistics")?;
    writeln!(file, "#   - Reliability analysis (Cronbach's alpha)")?;
    writeln!(file, "#   - Data visualization")?;
    writeln!(file, "#   - Quality checks and filtering")?;
    writeln!(file)?;
    writeln!(
        file,
        "# ======================================================================"
    )?;
    writeln!(file, "# SETUP: Install Required Packages")?;
    writeln!(
        file,
        "# ======================================================================"
    )?;
    writeln!(file)?;
    writeln!(file, "# Run once in terminal:")?;
    writeln!(
        file,
        "# pip install pandas numpy matplotlib seaborn pingouin scipy"
    )?;
    writeln!(file)?;
    writeln!(file, "import pandas as pd")?;
    writeln!(file, "import numpy as np")?;
    writeln!(file, "import matplotlib.pyplot as plt")?;
    writeln!(file, "import seaborn as sns")?;
    writeln!(file, "import pingouin as pg  # For Cronbach's alpha")?;
    writeln!(file, "from scipy import stats")?;
    writeln!(file)?;
    writeln!(file, "# Set plot style")?;
    writeln!(file, "sns.set_theme(style='whitegrid')")?;
    writeln!(file, "plt.rcParams['figure.figsize'] = (12, 8)")?;
    writeln!(file)?;
    writeln!(
        file,
        "# ======================================================================"
    )?;
    writeln!(file, "# DATA IMPORT")?;
    writeln!(
        file,
        "# ======================================================================"
    )?;
    writeln!(file)?;
    writeln!(file, "# Import processed data")?;
    writeln!(file, "data = pd.read_csv('{}')", csv_path)?;
    writeln!(file, "print('\\n=== Data Overview ===')")?;
    writeln!(file, "print(data.info())")?;
    writeln!(
        file,
        "print(f'\\nShape: {{data.shape[0]}} rows × {{data.shape[1]}} columns')"
    )?;
    writeln!(file)?;
    writeln!(
        file,
        "# ======================================================================"
    )?;
    writeln!(file, "# QUALITY FILTERING")?;
    writeln!(
        file,
        "# ======================================================================"
    )?;
    writeln!(file)?;

    if config.quality.is_some() {
        writeln!(file, "# Examine quality flags")?;
        writeln!(file, "print('\\n=== Quality Check Summary ===')")?;
        writeln!(file, "print(data['quality_flag'].value_counts())")?;
        writeln!(file)?;
        writeln!(file, "# Display flagged records")?;
        writeln!(file, "flagged = data[data['quality_flag'] == 'FLAGGED']")?;
        writeln!(
            file,
            "print(f'\\nFlagged records: {{len(flagged)}} ({{100 * len(flagged) / len(data):.1f}}%)')"
        )?;
        writeln!(file)?;
        writeln!(file, "if len(flagged) > 0:")?;
        writeln!(file, "    print('\\nReasons for flagging:')")?;
        writeln!(file, "    print(flagged['quality_reason'].value_counts())")?;
        writeln!(file)?;
        writeln!(file, "# Create clean dataset")?;
        writeln!(
            file,
            "clean_data = data[data['quality_flag'] == 'OK'].copy()"
        )?;
        writeln!(
            file,
            "print(f'\\nClean records: {{len(clean_data)}} ({{100 * len(clean_data) / len(data):.1f}}%)')"
        )?;
    } else {
        writeln!(file, "# No quality checks configured")?;
        writeln!(file, "clean_data = data.copy()")?;
    }
    writeln!(file)?;
    writeln!(
        file,
        "# ======================================================================"
    )?;
    writeln!(file, "# DESCRIPTIVE STATISTICS")?;
    writeln!(
        file,
        "# ======================================================================"
    )?;
    writeln!(file)?;

    for (scale_name, scale_def) in &config.scales {
        writeln!(file, "# {} ({} items)", scale_name, scale_def.items.len())?;
        writeln!(file, "print('\\n=== {} Statistics ===')", scale_name)?;
        writeln!(file, "print(clean_data['{}_mean'].describe())", scale_name)?;
        writeln!(file)?;
    }

    writeln!(
        file,
        "# ======================================================================"
    )?;
    writeln!(file, "# RELIABILITY ANALYSIS (Cronbach's Alpha)")?;
    writeln!(
        file,
        "# ======================================================================"
    )?;
    writeln!(file)?;

    for (scale_name, scale_def) in &config.scales {
        if scale_def.items.len() > 1 {
            writeln!(file, "# Reliability for {}", scale_name)?;
            writeln!(file, "print('\\n=== {} Reliability ===')", scale_name)?;
            write!(file, "{}_items = clean_data[[", scale_name)?;
            for (idx, item) in scale_def.items.iter().enumerate() {
                if idx > 0 {
                    write!(file, ", ")?;
                }
                write!(file, "'{}'", item)?;
            }
            writeln!(file, "]].dropna()")?;
            writeln!(
                file,
                "{}_alpha = pg.cronbach_alpha(data={}_items)",
                scale_name, scale_name
            )?;
            writeln!(
                file,
                "print(f'Cronbach\\'s Alpha: {{{}_alpha[0]:.3f}}')",
                scale_name
            )?;
            writeln!(
                file,
                "print(f'95% CI: [{{ {}_alpha[1][0]:.3f }}, {{ {}_alpha[1][1]:.3f }}]')",
                scale_name, scale_name
            )?;
            writeln!(file)?;
        }
    }

    writeln!(
        file,
        "# ======================================================================"
    )?;
    writeln!(file, "# DATA VISUALIZATION")?;
    writeln!(
        file,
        "# ======================================================================"
    )?;
    writeln!(file)?;

    // Distribution plots
    let num_scales = config.scales.len();
    let ncols = if num_scales > 2 { 2 } else { num_scales };
    let nrows = num_scales.div_ceil(ncols);

    writeln!(
        file,
        "# Distribution plots for scales ({} scales)",
        num_scales
    )?;
    writeln!(
        file,
        "fig, axes = plt.subplots({}, {}, figsize=(12, {}))",
        nrows,
        ncols,
        nrows * 4
    )?;
    if num_scales == 1 {
        writeln!(file, "axes = [axes]")?;
    } else {
        writeln!(file, "axes = axes.flatten()")?;
    }
    writeln!(file)?;

    for (idx, scale_name) in config.scales.keys().enumerate() {
        writeln!(file, "# Plot {}: {}", idx + 1, scale_name)?;
        writeln!(
            file,
            "axes[{}].hist(clean_data['{}_mean'].dropna(), bins=20, color='steelblue', edgecolor='black', alpha=0.7)",
            idx, scale_name
        )?;
        writeln!(
            file,
            "axes[{}].axvline(clean_data['{}_mean'].mean(), color='red', linestyle='dashed', linewidth=2, label='Mean')",
            idx, scale_name
        )?;
        writeln!(file, "axes[{}].set_title('{}')", idx, scale_name)?;
        writeln!(file, "axes[{}].set_xlabel('Mean Score')", idx)?;
        writeln!(file, "axes[{}].set_ylabel('Frequency')", idx)?;
        writeln!(file, "axes[{}].legend()", idx)?;
        writeln!(file)?;
    }

    writeln!(file, "plt.tight_layout()")?;
    writeln!(
        file,
        "plt.savefig('scale_distributions.png', dpi=300, bbox_inches='tight')"
    )?;
    writeln!(file, "plt.show()")?;
    writeln!(file)?;

    // Box plots
    if config.quality.is_some() {
        writeln!(file, "# Box plot: Compare flagged vs clean data")?;
        writeln!(
            file,
            "fig, axes = plt.subplots({}, {}, figsize=(12, {}))",
            nrows,
            ncols,
            nrows * 4
        )?;
        if num_scales == 1 {
            writeln!(file, "axes = [axes]")?;
        } else {
            writeln!(file, "axes = axes.flatten()")?;
        }
        writeln!(file)?;

        for (idx, scale_name) in config.scales.keys().enumerate() {
            writeln!(
                file,
                "sns.boxplot(data=data, x='quality_flag', y='{}_mean', ax=axes[{}], palette={{'OK': 'lightgreen', 'FLAGGED': 'salmon'}})",
                scale_name, idx
            )?;
            writeln!(
                file,
                "axes[{}].set_title('{} by Quality Flag')",
                idx, scale_name
            )?;
            writeln!(file, "axes[{}].set_xlabel('Quality Flag')", idx)?;
            writeln!(file, "axes[{}].set_ylabel('Mean Score')", idx)?;
            writeln!(file)?;
        }

        writeln!(file, "plt.tight_layout()")?;
        writeln!(
            file,
            "plt.savefig('quality_comparison.png', dpi=300, bbox_inches='tight')"
        )?;
        writeln!(file, "plt.show()")?;
        writeln!(file)?;
    }

    // Correlation matrix
    if config.scales.len() > 1 {
        writeln!(
            file,
            "# ======================================================================"
        )?;
        writeln!(file, "# CORRELATION MATRIX")?;
        writeln!(
            file,
            "# ======================================================================"
        )?;
        writeln!(file)?;
        write!(file, "scale_means = clean_data[[")?;
        for (idx, scale_name) in config.scales.keys().enumerate() {
            if idx > 0 {
                write!(file, ", ")?;
            }
            write!(file, "'{}_mean'", scale_name)?;
        }
        writeln!(file, "]]")?;
        writeln!(file)?;
        writeln!(file, "# Correlation matrix")?;
        writeln!(file, "print('\\n=== Correlation Matrix ===')")?;
        writeln!(file, "cor_matrix = scale_means.corr()")?;
        writeln!(file, "print(cor_matrix.round(2))")?;
        writeln!(file)?;
        writeln!(file, "# Correlation heatmap")?;
        writeln!(file, "plt.figure(figsize=(8, 6))")?;
        writeln!(
            file,
            "sns.heatmap(cor_matrix, annot=True, cmap='coolwarm', center=0, vmin=-1, vmax=1, square=True, linewidths=1)"
        )?;
        writeln!(file, "plt.title('Scale Correlation Matrix')")?;
        writeln!(
            file,
            "plt.savefig('correlation_matrix.png', dpi=300, bbox_inches='tight')"
        )?;
        writeln!(file, "plt.show()")?;
        writeln!(file)?;
    }

    writeln!(
        file,
        "# ======================================================================"
    )?;
    writeln!(file, "# EXPORT RESULTS")?;
    writeln!(
        file,
        "# ======================================================================"
    )?;
    writeln!(file)?;
    writeln!(file, "# Save clean dataset")?;
    writeln!(
        file,
        "clean_data.to_csv('clean_data_python.csv', index=False)"
    )?;
    writeln!(file)?;
    writeln!(file, "# Create summary table")?;
    writeln!(file, "summary_stats = pd.DataFrame({{")?;
    for (idx, scale_name) in config.scales.keys().enumerate() {
        if idx > 0 {
            writeln!(file, ",")?;
        }
        write!(file, "    '{}': [", scale_name)?;
        writeln!(file)?;
        writeln!(file, "        clean_data['{}_mean'].mean(),", scale_name)?;
        writeln!(file, "        clean_data['{}_mean'].std(),", scale_name)?;
        writeln!(file, "        clean_data['{}_mean'].min(),", scale_name)?;
        writeln!(file, "        clean_data['{}_mean'].max()", scale_name)?;
        write!(file, "    ]")?;
    }
    writeln!(file)?;
    writeln!(file, "}},")?;
    writeln!(file, "index=['Mean', 'SD', 'Min', 'Max'])")?;
    writeln!(file)?;
    writeln!(file, "print('\\n=== Summary Statistics Table ===')")?;
    writeln!(file, "print(summary_stats.round(2))")?;
    writeln!(file)?;
    writeln!(file, "# Save summary table")?;
    writeln!(file, "summary_stats.to_csv('summary_statistics.csv')")?;
    writeln!(file)?;
    writeln!(file, "print('\\n✓ Analysis complete! Results saved.')")?;

    Ok(())
}

/// Generate data dictionary in CSV format
pub fn generate_data_dictionary_csv(config: &SurveyConfig, output_path: &str) -> Result<()> {
    let mut file = fs::File::create(output_path)?;

    // Write header
    writeln!(
        file,
        "Variable,Description,Type,Scale_Membership,Value_Range,Reverse_Scored,Notes"
    )?;

    // Participant ID column
    if let Some(id_col) = &config.survey.participant_id_column {
        writeln!(
            file,
            "{},Participant identifier,ID,N/A,N/A,No,Unique identifier for each participant",
            id_col
        )?;
    } else {
        writeln!(
            file,
            "ID,Participant identifier,ID,N/A,N/A,No,Unique identifier for each participant"
        )?;
    }

    // Individual item columns
    for (scale_name, scale_def) in &config.scales {
        for item in &scale_def.items {
            let is_reverse = scale_def
                .reverse_scored
                .as_ref()
                .map(|rs| rs.contains(item))
                .unwrap_or(false);
            let reverse_str = if is_reverse { "Yes" } else { "No" };

            writeln!(
                file,
                "{},Survey item,Item,{},{}-{},{},Raw item response{}",
                item,
                scale_name,
                config.survey.min_score,
                config.survey.max_score,
                reverse_str,
                if is_reverse {
                    " (will be reverse-scored)"
                } else {
                    ""
                }
            )?;
        }
    }

    // Scale total columns
    for (scale_name, scale_def) in &config.scales {
        let reverse_count = scale_def
            .reverse_scored
            .as_ref()
            .map(|rs| rs.len())
            .unwrap_or(0);
        let reverse_note = if reverse_count > 0 {
            format!(" (after reverse-scoring {} items)", reverse_count)
        } else {
            String::new()
        };

        writeln!(
            file,
            "{}_total,Scale total score,Computed,{},Continuous,N/A,Sum of {} items{}",
            scale_name,
            scale_name,
            scale_def.items.len(),
            reverse_note
        )?;
    }

    // Scale mean columns
    for (scale_name, scale_def) in &config.scales {
        writeln!(
            file,
            "{}_mean,Scale mean score,Computed,{},{}-{},N/A,Mean of {} items (total / {})",
            scale_name,
            scale_name,
            config.survey.min_score,
            config.survey.max_score,
            scale_def.items.len(),
            scale_def.items.len()
        )?;
    }

    // Quality flag column
    writeln!(
        file,
        "quality_flag,Quality control flags,Flag,Quality,Varies,N/A,Automated quality checks (OK if no issues)"
    )?;

    Ok(())
}

/// Generate data dictionary in JSON format
pub fn generate_data_dictionary_json(config: &SurveyConfig, output_path: &str) -> Result<()> {
    use serde_json::json;

    let mut variables = Vec::new();

    // Participant ID
    let id_col = config
        .survey
        .participant_id_column
        .as_deref()
        .unwrap_or("ID");

    variables.push(json!({
        "variable": id_col,
        "description": "Participant identifier",
        "type": "ID",
        "scale_membership": null,
        "value_range": null,
        "reverse_scored": false,
        "notes": "Unique identifier for each participant"
    }));

    // Individual items
    for (scale_name, scale_def) in &config.scales {
        for item in &scale_def.items {
            let is_reverse = scale_def
                .reverse_scored
                .as_ref()
                .map(|rs| rs.contains(item))
                .unwrap_or(false);

            variables.push(json!({
                "variable": item,
                "description": "Survey item",
                "type": "Item",
                "scale_membership": scale_name,
                "value_range": format!("{}-{}", config.survey.min_score, config.survey.max_score),
                "reverse_scored": is_reverse,
                "notes": if is_reverse {
                    "Raw item response (will be reverse-scored)"
                } else {
                    "Raw item response"
                }
            }));
        }
    }

    // Scale totals
    for (scale_name, scale_def) in &config.scales {
        let reverse_count = scale_def
            .reverse_scored
            .as_ref()
            .map(|rs| rs.len())
            .unwrap_or(0);
        let notes = if reverse_count > 0 {
            format!(
                "Sum of {} items (after reverse-scoring {} items)",
                scale_def.items.len(),
                reverse_count
            )
        } else {
            format!("Sum of {} items", scale_def.items.len())
        };

        variables.push(json!({
            "variable": format!("{}_total", scale_name),
            "description": "Scale total score",
            "type": "Computed",
            "scale_membership": scale_name,
            "value_range": "Continuous",
            "reverse_scored": false,
            "notes": notes
        }));
    }

    // Scale means
    for (scale_name, scale_def) in &config.scales {
        variables.push(json!({
            "variable": format!("{}_mean", scale_name),
            "description": "Scale mean score",
            "type": "Computed",
            "scale_membership": scale_name,
            "value_range": format!("{}-{}", config.survey.min_score, config.survey.max_score),
            "reverse_scored": false,
            "notes": format!("Mean of {} items (total / {})", scale_def.items.len(), scale_def.items.len())
        }));
    }

    // Quality flag
    variables.push(json!({
        "variable": "quality_flag",
        "description": "Quality control flags",
        "type": "Flag",
        "scale_membership": "Quality",
        "value_range": "Varies",
        "reverse_scored": false,
        "notes": "Automated quality checks (OK if no issues)"
    }));

    // Create full data dictionary structure
    let dictionary = json!({
        "survey": {
            "name": config.survey.name,
            "min_score": config.survey.min_score,
            "max_score": config.survey.max_score
        },
        "variables": variables,
        "scales": config.scales.iter().map(|(name, def)| {
            json!({
                "name": name,
                "items": def.items,
                "reverse_scored": def.reverse_scored,
                "item_count": def.items.len()
            })
        }).collect::<Vec<_>>(),
        "quality_checks": if config.quality.is_some() {
            json!({
                "enabled": true,
                "checks": [
                    "Missing data detection",
                    "Straightlining detection",
                    "Low variance detection",
                    "Diagonal pattern detection",
                    "Alternating pattern detection",
                    "Block pattern detection",
                    "Response time validation"
                ]
            })
        } else {
            json!({ "enabled": false })
        },
        "generated": chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
    });

    let json_string = serde_json::to_string_pretty(&dictionary)?;
    fs::write(output_path, json_string)?;

    Ok(())
}

/// Generate CONSORT flowchart data for publication reporting
///
/// Creates a structured report of participant flow through the study,
/// including screening, exclusions by reason, and final analysis sample.
///
/// # Arguments
/// * `total_screened` - Total number of participants screened
/// * `quality_issues` - Vector of quality issues detected
/// * `output_path` - Path to save the CONSORT report
///
/// # Output Format
/// The report includes:
/// - Total screened
/// - Total excluded (with breakdown by reason)
/// - Total analyzed (clean participants)
/// - Exclusion reasons with counts
///
/// # Example Output
/// ```text
/// CONSORT Participant Flow Report
/// ================================
///
/// Screened (n = 100)
///   ↓
/// Excluded (n = 15)
///   - Missing data: 5
///   - Straightlining: 4
///   - Careless patterns: 6
///   ↓
/// Analyzed (n = 85)
/// ```
pub fn generate_consort_report(
    total_screened: usize,
    quality_issues: &[QualityIssue],
    output_path: &str,
) -> Result<()> {
    use std::collections::HashMap;

    // Group issues by participant to count unique exclusions
    let mut participants_with_issues: std::collections::HashSet<String> =
        std::collections::HashSet::with_capacity(quality_issues.len() / 2);
    let mut issue_counts: HashMap<String, usize> = HashMap::with_capacity(8);

    for issue in quality_issues {
        participants_with_issues.insert(issue.participant_id.clone());

        // Categorize issue types for reporting
        let category = match issue.issue_type.as_str() {
            "MissingData" => "Missing data",
            "Straightlining" => "Straightlining",
            "LowVariance" => "Low variance",
            "OutOfRange" => "Out of range responses",
            "DiagonalPattern" => "Diagonal pattern",
            "AlternatingPattern" => "Alternating pattern",
            "BlockPattern" => "Block pattern",
            "ResponseTimeFast" => "Response time too fast",
            "ResponseTimeSlow" => "Response time too slow",
            "SemanticInconsistency" => "Semantic inconsistency",
            _ => "Other quality issue",
        };

        *issue_counts.entry(category.to_string()).or_insert(0) += 1;
    }

    let total_excluded = participants_with_issues.len();
    let total_analyzed = total_screened.saturating_sub(total_excluded);

    // Calculate percentages safely (avoid division by zero)
    let excluded_pct = crate::utils::calculate_percentage(total_excluded, total_screened);
    let analyzed_pct = crate::utils::calculate_percentage(total_analyzed, total_screened);

    // Generate report
    let mut report = String::new();
    report.push_str("CONSORT Participant Flow Report\n");
    report.push_str("================================\n\n");

    report.push_str("Participants Screened\n");
    report.push_str(&format!("  n = {}\n\n", total_screened));

    report.push_str("  ↓\n\n");

    report.push_str("Excluded (Quality Issues)\n");
    report.push_str(&format!(
        "  n = {} ({:.1}%)\n\n",
        total_excluded, excluded_pct
    ));

    report.push_str("  Exclusion Breakdown:\n");
    let mut sorted_issues: Vec<_> = issue_counts.iter().collect();
    sorted_issues.sort_by(|a, b| b.1.cmp(a.1)); // Sort by count descending

    for (reason, count) in sorted_issues {
        report.push_str(&format!("    - {}: {} issue(s)\n", reason, count));
    }

    report.push_str("\n  ↓\n\n");

    report.push_str("Final Analysis Sample\n");
    report.push_str(&format!(
        "  n = {} ({:.1}%)\n\n",
        total_analyzed, analyzed_pct
    ));

    report.push_str("================================\n");
    report.push_str("Summary Statistics:\n");
    report.push_str(&format!("  Retention rate: {:.1}%\n", analyzed_pct));
    report.push_str(&format!("  Exclusion rate: {:.1}%\n", excluded_pct));
    report.push_str(&format!(
        "  Total quality issues detected: {}\n",
        quality_issues.len()
    ));
    report.push_str(&format!("  Participants with issues: {}\n", total_excluded));

    // Write to file
    fs::write(output_path, report)?;

    Ok(())
}

/// Generate CONSORT data in JSON format for programmatic use
///
/// # Arguments
/// * `total_screened` - Total number of participants screened
/// * `quality_issues` - Vector of quality issues detected
/// * `output_path` - Path to save the JSON file
pub fn generate_consort_json(
    total_screened: usize,
    quality_issues: &[QualityIssue],
    output_path: &str,
) -> Result<()> {
    use std::collections::HashMap;

    // Group issues by participant
    let mut participants_with_issues: std::collections::HashSet<String> =
        std::collections::HashSet::with_capacity(quality_issues.len() / 2);
    let mut issue_counts: HashMap<String, usize> = HashMap::with_capacity(8);
    let mut participant_issues: HashMap<String, Vec<String>> =
        HashMap::with_capacity(quality_issues.len() / 2);

    for issue in quality_issues {
        participants_with_issues.insert(issue.participant_id.clone());

        let category = match issue.issue_type.as_str() {
            "MissingData" => "Missing data",
            "Straightlining" => "Straightlining",
            "LowVariance" => "Low variance",
            "OutOfRange" => "Out of range responses",
            "DiagonalPattern" => "Diagonal pattern",
            "AlternatingPattern" => "Alternating pattern",
            "BlockPattern" => "Block pattern",
            "ResponseTimeFast" => "Response time too fast",
            "ResponseTimeSlow" => "Response time too slow",
            "SemanticInconsistency" => "Semantic inconsistency",
            _ => "Other quality issue",
        };

        *issue_counts.entry(category.to_string()).or_insert(0) += 1;
        participant_issues
            .entry(issue.participant_id.clone())
            .or_default()
            .push(category.to_string());
    }

    let total_excluded = participants_with_issues.len();
    let total_analyzed = total_screened.saturating_sub(total_excluded);

    // Calculate percentages safely (avoid division by zero)
    let excluded_pct = crate::utils::calculate_percentage(total_excluded, total_screened);
    let analyzed_pct = crate::utils::calculate_percentage(total_analyzed, total_screened);

    // Build JSON structure
    let consort_data = json!({
        "study_flow": {
            "screened": {
                "n": total_screened,
                "description": "Total participants screened"
            },
            "excluded": {
                "n": total_excluded,
                "percentage": format!("{:.1}", excluded_pct),
                "description": "Participants excluded due to quality issues",
                "breakdown": issue_counts.iter()
                    .map(|(k, v)| json!({
                        "reason": k,
                        "count": v
                    }))
                    .collect::<Vec<_>>()
            },
            "analyzed": {
                "n": total_analyzed,
                "percentage": format!("{:.1}", analyzed_pct),
                "description": "Final analysis sample (clean data)"
            }
        },
        "summary": {
            "retention_rate": format!("{:.1}", analyzed_pct),
            "exclusion_rate": format!("{:.1}", excluded_pct),
            "total_issues_detected": quality_issues.len(),
            "participants_with_issues": total_excluded
        },
        "excluded_participants": participants_with_issues.iter()
            .map(|p| json!({
                "participant_id": p,
                "issues": participant_issues.get(p).unwrap_or(&vec![])
            }))
            .collect::<Vec<_>>(),
        "generated": chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
    });

    let json_string = serde_json::to_string_pretty(&consort_data)?;
    fs::write(output_path, json_string)?;

    Ok(())
}
