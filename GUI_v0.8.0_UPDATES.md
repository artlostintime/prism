# Prism GUI v0.8.0 Updates

**Updated:** January 6, 2026  
**GUI Version:** 0.3.0  
**Prism Core:** v0.8.0

---

## 🎉 Major Features Added

### 1. **Data Dictionary Export** 📖

Generate comprehensive variable documentation for reproducibility and open science compliance.

**Features:**

- **Format Options**: CSV or JSON
- **Documentation Includes**:
  - Participant ID variable
  - All survey items with scale membership
  - Value ranges and reverse-scoring status
  - Computed variables (totals, means)
  - Quality check descriptions

**Use Cases:**

- Journal supplementary materials
- Data sharing repositories
- Team collaboration and onboarding
- Grant compliance (data management plans)

**GUI Location:** New section below "Process Data" button  
**Command:** `run_dictionary`

---

### 2. **CONSORT Flowchart Generation** 📊

Publication-ready participant flow diagrams following CONSORT guidelines.

**Features:**

- **Format Options**: Text or JSON
- **Tracks**:
  - Total participants screened
  - Exclusions by quality check type
  - Final analysis sample size
  - Retention and exclusion rates

**Exclusion Categories:**

- Missing data
- Straightlining
- Diagonal patterns
- Alternating patterns
- Block patterns
- Low variance
- Response time violations
- Semantic inconsistency

**GUI Location:** New section below "Process Data" button  
**Command:** `run_consort`

---

### 3. **Enhanced Quality Checks** 🔍

All 8 quality check categories now enabled in default configuration template.

**New Patterns Detected:**

- ✅ Diagonal patterns (1,2,3,4,5 or 5,4,3,2,1)
- ✅ Alternating patterns (1,5,1,5,1,5)
- ✅ Block patterns (1,1,1,5,5,5)
- ✅ Response time validation (too fast/slow)
- ✅ Semantic inconsistency detection

**Updated Default Config:**

```toml
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
```

---

## 🎨 UI/UX Improvements

### Header Update

- **New Title:** "Prism v0.8.0"
- **New Subtitle:** "Psychology Survey Data Processor with Advanced Quality Checks"

### New Features Section

Added dedicated section for v0.8.0 features with:

- Data Dictionary export controls
- CONSORT flowchart generation controls
- Format selection dropdowns (CSV/JSON for dictionary, Text/JSON for CONSORT)
- Informative tooltip descriptions
- Visual feature highlight banner

### Enhanced Success Message

After successful processing, users now see:

```
✅ Success! Processed X participants.

🎉 New in v0.8.0: Your data was checked for 8 quality categories
including pattern detection!

[📁 Open Folder] [📖 Data Dictionary] [📊 CONSORT Report]
```

### Updated Help Dialog

Comprehensive help text now includes:

- v0.8.0 feature highlights
- All 8 quality check categories
- Advanced feature descriptions
- Output file documentation

### Updated Example Templates

All example configurations now include:

- Pattern detection flags
- Response time validation
- Semantic inconsistency checks (in complex example)

---

## 🛠️ Technical Implementation

### New Tauri Commands

#### `run_dictionary`

```rust
#[command]
fn run_dictionary(
    config_path: String,
    output_path: String,
    format: String,
) -> Result<String, String>
```

- Calls CLI `dictionary` command
- Supports CSV and JSON formats
- Returns success message with output path

#### `run_consort`

```rust
#[command]
fn run_consort(
    input_path: String,
    config_path: String,
    output_path: String,
    format: String,
) -> Result<String, String>
```

- Processes data first to generate quality report
- Parses quality issues by category
- Generates CONSORT text or JSON format
- Tracks exclusions without double-counting

### Helper Functions

#### `generate_consort_text`

Creates publication-ready text format:

```
CONSORT Participant Flow Report
================================

Participants Screened
  n = 100

  ↓

Excluded (Quality Issues)
  n = 15 (15.0%)

  Exclusion Breakdown:
    - Missing data: 5 issue(s)
    - Diagonal pattern: 3 issue(s)
    ...
```

#### `generate_consort_json`

Creates structured JSON format:

```json
{
  "total_screened": 100,
  "excluded": 15,
  "excluded_percent": 15.0,
  "exclusion_reasons": [...],
  "final_sample": 85,
  "retention_rate": 85.0
}
```

#### `parse_quality_issues`

Intelligently parses quality report text to categorize issues:

- Detects flagged participants
- Categorizes by issue type
- Counts occurrences
- Returns exclusion summary

---

## 📦 Files Modified

### Frontend

- **ui/index.html**
  - Updated header with v0.8.0 branding
  - Added new features section (Data Dictionary & CONSORT)
  - Enhanced success message with action buttons
  - Updated help dialog
  - Added JavaScript functions: `generateDictionary()`, `generateConsort()`
  - Updated example configurations

### Backend

- **src-tauri/src/lib.rs**

  - Added `run_dictionary` command (~40 lines)
  - Added `run_consort` command (~120 lines)
  - Added `generate_consort_text` helper (~30 lines)
  - Added `generate_consort_json` helper (~30 lines)
  - Added `parse_quality_issues` helper (~40 lines)
  - Updated `generate_config_template` with v0.8.0 flags
  - Total: ~260 new lines of code

- **src-tauri/Cargo.toml**
  - Added `toml = "0.8"` dependency for config parsing

---

## 🚀 How to Use New Features

### Generate Data Dictionary

1. Load your CSV and configuration
2. In the "v0.8.0 Advanced Features" section:
   - Select format (CSV or JSON)
   - Click "📥 Export Dictionary"
3. Dictionary saved to output folder as `data_dictionary.csv` or `.json`

### Generate CONSORT Flowchart

1. Load your CSV and configuration
2. In the "v0.8.0 Advanced Features" section:
   - Select format (Text or JSON)
   - Click "📋 Generate CONSORT"
3. Report saved to output folder as `consort_report.txt` or `.json`

### Quick Workflow

1. Click "▶ Process Data"
2. After processing completes, click quick action buttons:
   - **📁 Open Folder** - View all outputs
   - **📖 Data Dictionary** - Generate documentation
   - **📊 CONSORT Report** - Create flowchart

---

## ✅ Testing & Validation

### Build Status

- ✅ Cargo check: No errors
- ✅ Release build: Successful
- ✅ All dependencies resolved
- ✅ Tauri commands registered

### Functionality Tested

- ✅ Data dictionary CSV export
- ✅ Data dictionary JSON export
- ✅ CONSORT text generation
- ✅ CONSORT JSON generation
- ✅ Quality issue parsing
- ✅ Button interactions
- ✅ Format selection
- ✅ Success message display

---

## 🔮 Future Enhancements

### Potential Additions (v0.9.0+)

- **Live Preview**: Show CONSORT diagram in GUI
- **Interactive Settings**: Adjust quality thresholds in GUI
- **Batch Processing**: Process multiple files
- **Visualization Dashboard**: Real-time quality metrics
- **Export Options**: PDF generation for reports

### User Feedback Requests

- Preferred CONSORT diagram visualization style
- Additional export formats needed
- Quality check threshold customization priority

---

## 📚 Documentation Updates

### Updated Files

- ✅ GUI_v0.8.0_UPDATES.md (this file)
- ✅ ui/index.html (embedded help text)
- ✅ RELEASE_v0.8.0.md (mentions GUI features)

### Next Steps

- [ ] Update main README.md with GUI screenshots
- [ ] Create video tutorial demonstrating new features
- [ ] Add GUI section to TUTORIAL.md
- [ ] Update FAQ with GUI-specific questions

---

## 🎯 Key Achievements

### Developer Experience

- **Clean Architecture**: Modular command structure
- **Error Handling**: Comprehensive error messages
- **Maintainability**: Well-documented code
- **Extensibility**: Easy to add new commands

### User Experience

- **Intuitive Interface**: Clear section organization
- **Helpful Tooltips**: Context-sensitive guidance
- **Quick Actions**: One-click access to new features
- **Visual Feedback**: Updated branding and success messages

### Research Impact

- **Reproducibility**: Data dictionaries for open science
- **Transparency**: CONSORT flowcharts for publications
- **Quality Assurance**: 8 comprehensive quality checks
- **Efficiency**: Automated report generation

---

## 📞 Support & Resources

- **Documentation**: `docs/GUI_USAGE.md`
- **Issues**: https://github.com/artlostintime/prism/issues
- **Discussions**: https://github.com/artlostintime/prism/discussions
- **CLI Documentation**: `docs/HOW_TO_USE.md`

---

**🎉 Prism v0.8.0 GUI is production-ready!**

All new features fully implemented, tested, and documented. The GUI now provides comprehensive access to advanced quality checking capabilities with publication-ready outputs.
