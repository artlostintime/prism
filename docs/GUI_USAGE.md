# Prism GUI Usage Guide

## Overview

The Prism GUI provides an intuitive interface for processing psychology survey data without using the command line.

## New Features (v0.2.0)

### 1. **Smart Output Locations**

The GUI automatically determines where to save processed data based on your CSV location:

#### Scenario 1: Organized Structure

```
MyStudy/
├── raw_data/
│   └── survey.csv          ← Your CSV here
└── processed/              ← Outputs automatically go here
    ├── clean_data.csv
    ├── summary_stats.txt
    └── quality_report.txt
```

#### Scenario 2: Simple Structure

```
MyStudy/
├── survey.csv              ← Your CSV here
└── processed/              ← Outputs automatically go here
    ├── clean_data.csv
    ├── summary_stats.txt
    └── quality_report.txt
```

The GUI automatically detects if your CSV is in a `raw_data`, `raw`, `data`, or `input` folder and creates the `processed` folder at the appropriate level.

### 2. **Flexible Configuration**

You have three options for providing configuration:

#### Option A: Use Existing Config File

1. Click **"📂 Use Existing"**
2. Browse to your `.toml` config file
3. The file path will be displayed

#### Option B: Edit Config in GUI

1. Click **"✏️ Edit Config"**
2. A text editor appears with a template
3. Modify the template for your survey:
   - Update scale names
   - Add/remove items
   - Specify reverse-scored items
4. The config is automatically saved when you process

#### Option C: Drag & Drop Config

_(Coming soon)_ Drag a `.toml` file directly onto the config area.

### 3. **Modern Interface**

- **Visual feedback**: Color-coded status messages (blue=processing, green=success, red=error)
- **File picker**: Click the CSV drop zone to browse for files
- **Output preview**: See where your processed files will be saved before processing
- **Progress indicator**: Animated spinner shows processing in progress

## Step-by-Step Workflow

### Quick Start (3 Steps)

1. **Select CSV File**

   - Click the CSV drop zone
   - Browse to your survey data file
   - File path and output location are displayed

2. **Choose Configuration**

   - Click "Use Existing" to browse for a config file
   - OR click "Edit Config" to create/modify config in the GUI

3. **Process Data**
   - Click "▶ Process Data"
   - Wait for processing to complete
   - Check the output folder for results

### Example Config Template

When you click "Edit Config", you'll see this template:

```toml
[survey]
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
```

**Customize this for your survey:**

- Change `"my_scale"` to your actual scale name (e.g., `"emotional_exhaustion"`)
- Update `items` to match your CSV column names
- Add items that need reverse scoring to `reverse_scored`

## Output Files

After processing, you'll find three files in the `processed/` folder:

1. **clean_data.csv** - Your data with computed scale scores
2. **summary_stats.txt** - Descriptive statistics and Cronbach's alpha
3. **quality_report.txt** - Data quality issues (missing data, straightlining, etc.)

## Troubleshooting

### "Config file not found"

- Make sure you've selected a config file or enabled the config editor
- Check that your config file has a `.toml` extension

### "CLI binary not found"

- Build the CLI first: `cargo build --release`
- The GUI needs the CLI executable to process data

### "Processing Error"

- Check that your CSV column names match those in your config
- Verify your config file syntax is correct
- Check the error message for specific details

## Building the GUI

### Development Mode (for testing)

```bash
cd src-tauri
cargo tauri dev
```

### Release Build (for distribution)

```bash
cd src-tauri
cargo tauri build
```

Find the installer in:

- **Windows:** `src-tauri/target/release/bundle/msi/` or `src-tauri/target/release/bundle/nsis/`
- **Mac:** `src-tauri/target/release/bundle/dmg/`
- **Linux:** `src-tauri/target/release/bundle/appimage/`

## Tips for Best Organization

We recommend this folder structure:

```
YourStudy/
├── study_config.toml       ← Your configuration
├── raw_data/               ← Put your raw CSV files here
│   └── survey_data.csv
└── processed/              ← Auto-created by Prism
    ├── clean_data.csv
    ├── summary_stats.txt
    └── quality_report.txt
```

This keeps everything organized and prevents mixing raw and processed data.
