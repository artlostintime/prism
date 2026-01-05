// src/visualization.rs
use crate::config::SurveyConfig;
use crate::errors::Result;
use crate::stats::Stats;
use crate::types::QualityIssue;
use std::collections::HashMap;
use std::fs;

/// Generate comprehensive HTML visualization report
pub fn generate_html_report(
    config: &SurveyConfig,
    scale_scores: &HashMap<String, Vec<f64>>,
    quality_issues: &[QualityIssue],
    total_participants: usize,
    output_path: &str,
) -> Result<()> {
    // Count unique participants with issues
    let unique_flagged: std::collections::HashSet<_> =
        quality_issues.iter().map(|i| &i.participant_id).collect();
    let flagged_count = unique_flagged.len();
    let clean_count = total_participants.saturating_sub(flagged_count);

    let mut html = String::new();

    // HTML header with Chart.js CDN
    html.push_str(&format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Prism Analysis Report - {}</title>
    <script src="https://cdn.jsdelivr.net/npm/chart.js@4.4.0/dist/chart.umd.min.js"></script>
    <style>
        :root {{
            --primary-color: #3498db;
            --secondary-color: #2ecc71;
            --danger-color: #e74c3c;
            --warning-color: #f39c12;
            --dark-color: #2c3e50;
            --light-bg: #ecf0f1;
            --border-color: #bdc3c7;
        }}
        
        * {{
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }}
        
        body {{
            font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
            background-color: var(--light-bg);
            color: var(--dark-color);
            line-height: 1.6;
            padding: 20px;
        }}
        
        .container {{
            max-width: 1400px;
            margin: 0 auto;
            background: white;
            border-radius: 10px;
            box-shadow: 0 2px 10px rgba(0,0,0,0.1);
            overflow: hidden;
        }}
        
        .header {{
            background: linear-gradient(135deg, var(--primary-color), #2980b9);
            color: white;
            padding: 40px;
            text-align: center;
        }}
        
        .header h1 {{
            font-size: 2.5em;
            margin-bottom: 10px;
            font-weight: 600;
        }}
        
        .header .subtitle {{
            font-size: 1.1em;
            opacity: 0.9;
        }}
        
        .content {{
            padding: 40px;
        }}
        
        .section {{
            margin-bottom: 50px;
        }}
        
        .section-title {{
            font-size: 1.8em;
            color: var(--dark-color);
            margin-bottom: 20px;
            padding-bottom: 10px;
            border-bottom: 3px solid var(--primary-color);
        }}
        
        .stats-grid {{
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
            gap: 20px;
            margin-bottom: 30px;
        }}
        
        .stat-card {{
            background: var(--light-bg);
            border-radius: 8px;
            padding: 25px;
            text-align: center;
            transition: transform 0.2s;
        }}
        
        .stat-card:hover {{
            transform: translateY(-5px);
            box-shadow: 0 5px 15px rgba(0,0,0,0.1);
        }}
        
        .stat-value {{
            font-size: 2.5em;
            font-weight: bold;
            color: var(--primary-color);
        }}
        
        .stat-label {{
            font-size: 1em;
            color: #7f8c8d;
            margin-top: 5px;
        }}
        
        .chart-container {{
            position: relative;
            height: 400px;
            margin-bottom: 40px;
            background: white;
            padding: 20px;
            border-radius: 8px;
            border: 1px solid var(--border-color);
        }}
        
        .quality-badge {{
            display: inline-block;
            padding: 8px 16px;
            border-radius: 20px;
            font-weight: 600;
            font-size: 0.9em;
        }}
        
        .quality-ok {{
            background-color: var(--secondary-color);
            color: white;
        }}
        
        .quality-flagged {{
            background-color: var(--danger-color);
            color: white;
        }}
        
        .issue-list {{
            background: var(--light-bg);
            border-radius: 8px;
            padding: 20px;
        }}
        
        .issue-item {{
            padding: 12px;
            margin-bottom: 10px;
            background: white;
            border-radius: 5px;
            border-left: 4px solid var(--warning-color);
        }}
        
        .footer {{
            background: var(--dark-color);
            color: white;
            padding: 30px;
            text-align: center;
        }}
        
        .footer a {{
            color: var(--primary-color);
            text-decoration: none;
        }}
        
        table {{
            width: 100%;
            border-collapse: collapse;
            margin: 20px 0;
            background: white;
            border-radius: 8px;
            overflow: hidden;
        }}
        
        th, td {{
            padding: 15px;
            text-align: left;
            border-bottom: 1px solid var(--border-color);
        }}
        
        th {{
            background: var(--primary-color);
            color: white;
            font-weight: 600;
        }}
        
        tr:hover {{
            background: var(--light-bg);
        }}
        
        @media print {{
            body {{
                background: white;
            }}
            .container {{
                box-shadow: none;
            }}
        }}
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>📊 Analysis Report</h1>
            <div class="subtitle">{}</div>
            <div class="subtitle">Generated by Prism v{}</div>
        </div>
        
        <div class="content">
"#,
        config.survey.name,
        config.survey.name,
        env!("CARGO_PKG_VERSION")
    ));

    // Overview Statistics
    html.push_str(
        r#"
            <div class="section">
                <h2 class="section-title">📈 Overview</h2>
                <div class="stats-grid">
"#,
    );

    html.push_str(&format!(
        r#"
                    <div class="stat-card">
                        <div class="stat-value">{}</div>
                        <div class="stat-label">Total Participants</div>
                    </div>
                    <div class="stat-card">
                        <div class="stat-value">{}</div>
                        <div class="stat-label">Clean Records</div>
                        <div class="quality-badge quality-ok">{:.1}%</div>
                    </div>
                    <div class="stat-card">
                        <div class="stat-value">{}</div>
                        <div class="stat-label">Flagged Records</div>
                        <div class="quality-badge quality-flagged">{:.1}%</div>
                    </div>
                    <div class="stat-card">
                        <div class="stat-value">{}</div>
                        <div class="stat-label">Scales</div>
                    </div>
"#,
        total_participants,
        clean_count,
        100.0 * clean_count as f64 / total_participants as f64,
        flagged_count,
        100.0 * flagged_count as f64 / total_participants as f64,
        scale_scores.len()
    ));

    html.push_str(
        r#"
                </div>
            </div>
"#,
    );

    // Scale Statistics Table
    html.push_str(
        r#"
            <div class="section">
                <h2 class="section-title">📊 Scale Statistics</h2>
                <table>
                    <thead>
                        <tr>
                            <th>Scale</th>
                            <th>N</th>
                            <th>Mean</th>
                            <th>SD</th>
                            <th>Min</th>
                            <th>Max</th>
                            <th>Range</th>
                        </tr>
                    </thead>
                    <tbody>
"#,
    );

    for (scale_name, scores) in scale_scores {
        let stats = Stats::calculate(scores);
        html.push_str(&format!(
            r#"
                        <tr>
                            <td><strong>{}</strong></td>
                            <td>{}</td>
                            <td>{:.2}</td>
                            <td>{:.2}</td>
                            <td>{:.2}</td>
                            <td>{:.2}</td>
                            <td>{:.2} - {:.2}</td>
                        </tr>
"#,
            scale_name, stats.n, stats.mean, stats.sd, stats.min, stats.max, stats.min, stats.max
        ));
    }

    html.push_str(
        r#"
                    </tbody>
                </table>
            </div>
"#,
    );

    // Distribution Charts
    html.push_str(
        r#"
            <div class="section">
                <h2 class="section-title">📉 Score Distributions</h2>
"#,
    );

    for (scale_name, scores) in scale_scores {
        let stats = Stats::calculate(scores);

        // Create histogram bins
        let num_bins = 15;
        let bin_width = (stats.max - stats.min) / num_bins as f64;
        let mut bins = vec![0; num_bins];

        for &score in scores {
            let bin_idx = ((score - stats.min) / bin_width).floor() as usize;
            let bin_idx = bin_idx.min(num_bins - 1);
            bins[bin_idx] += 1;
        }

        let bin_labels: Vec<String> = (0..num_bins)
            .map(|i| format!("{:.1}", stats.min + (i as f64 * bin_width)))
            .collect();

        html.push_str(&format!(
            r#"
                <div class="chart-container">
                    <canvas id="chart_{}"></canvas>
                </div>
                <script>
                new Chart(document.getElementById('chart_{}'), {{
                    type: 'bar',
                    data: {{
                        labels: {},
                        datasets: [{{
                            label: 'Frequency',
                            data: {},
                            backgroundColor: 'rgba(52, 152, 219, 0.7)',
                            borderColor: 'rgba(52, 152, 219, 1)',
                            borderWidth: 2
                        }}]
                    }},
                    options: {{
                        responsive: true,
                        maintainAspectRatio: false,
                        plugins: {{
                            title: {{
                                display: true,
                                text: '{} Distribution (M={:.2}, SD={:.2}, N={})',
                                font: {{
                                    size: 18,
                                    weight: 'bold'
                                }}
                            }},
                            legend: {{
                                display: false
                            }}
                        }},
                        scales: {{
                            y: {{
                                beginAtZero: true,
                                title: {{
                                    display: true,
                                    text: 'Frequency'
                                }}
                            }},
                            x: {{
                                title: {{
                                    display: true,
                                    text: 'Score Range'
                                }}
                            }}
                        }}
                    }}
                }});
                </script>
"#,
            scale_name,
            scale_name,
            serde_json::to_string(&bin_labels).unwrap(),
            serde_json::to_string(&bins).unwrap(),
            scale_name,
            stats.mean,
            stats.sd,
            stats.n
        ));
    }

    html.push_str(
        r#"
            </div>
"#,
    );

    // Quality Issues
    if !quality_issues.is_empty() {
        html.push_str(
            r#"
            <div class="section">
                <h2 class="section-title">⚠️ Quality Issues</h2>
                <div class="issue-list">
"#,
        );

        // Group issues by type
        let mut issue_counts: HashMap<String, usize> = HashMap::new();
        for issue in quality_issues {
            *issue_counts.entry(issue.issue_type.clone()).or_insert(0) += 1;
        }

        html.push_str(
            r#"
                    <h3>Issue Summary</h3>
                    <div class="stats-grid">
"#,
        );

        for (issue_type, count) in issue_counts.iter() {
            html.push_str(&format!(
                r#"
                        <div class="stat-card">
                            <div class="stat-value">{}</div>
                            <div class="stat-label">{}</div>
                        </div>
"#,
                count, issue_type
            ));
        }

        html.push_str(
            r#"
                    </div>
                    
                    <h3 style="margin-top: 30px;">Recent Issues (first 10)</h3>
"#,
        );

        for issue in quality_issues.iter().take(10) {
            html.push_str(&format!(
                r#"
                    <div class="issue-item">
                        <strong>{}</strong>: {}
                    </div>
"#,
                issue.participant_id, issue.details
            ));
        }

        html.push_str(
            r#"
                </div>
            </div>
"#,
        );
    }

    // Footer
    html.push_str(&format!(
        r#"
        </div>
        
        <div class="footer">
            <p>Generated by <strong>Prism v{}</strong></p>
            <p>Psychology Survey Data Processing Pipeline</p>
            <p><a href="https://github.com/artlostintime/prism" target="_blank">GitHub Repository</a></p>
        </div>
    </div>
</body>
</html>
"#,
        env!("CARGO_PKG_VERSION")
    ));

    fs::write(output_path, html)?;
    Ok(())
}
