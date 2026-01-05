# Prism GUI - Advanced Features & QoL Guide

**Version:** 0.3.0+  
**Last Updated:** January 6, 2026  
**Prism Core:** v0.8.0

---

## 🎉 Overview

The Prism GUI has been significantly enhanced with advanced features and quality-of-life improvements to make data processing more powerful, flexible, and user-friendly.

---

## 🌓 Dark Mode

### Features

- **One-Click Toggle**: Switch between light and dark themes instantly
- **Persistent Settings**: Your theme preference is saved automatically
- **Smooth Transitions**: All elements animate smoothly during theme changes
- **Optimized Colors**: Carefully selected color palette for both modes

### How to Use

1. Click the **🌙** button in the top-right header
2. Theme switches to dark mode (button changes to **☀️**)
3. Your preference is saved automatically
4. Theme persists across browser sessions

### Theme Details

**Light Mode:**

- Bright, clean interface
- High contrast for readability
- Gradient background (purple to blue)
- White cards

**Dark Mode:**

- Easy on the eyes for long sessions
- Reduced eye strain in low-light environments
- Dark gradient background
- Charcoal cards with lighter text

---

## ⚙️ Output Format Selector

### Supported Formats

#### 📊 CSV File

- Standard comma-separated values
- Universal compatibility
- Default format for all analyses

#### 📗 Excel File (.xlsx)

- Native Excel workbook format
- Formatted with headers
- Ready for Excel analysis
- Includes all computed scales

#### 📘 SPSS Syntax (.sps)

- SPSS command syntax file
- Variable labels and value labels
- Compute statements for scales
- Ready to run in SPSS

#### 📙 R Script (.R)

- Complete R analysis script
- Data loading and preparation
- Cronbach's alpha calculations
- Summary statistics
- Ready to execute in RStudio

#### 🐍 Python Script (.py)

- Python analysis script
- Pandas-based data processing
- Statistical calculations
- Comments and documentation
- Ready to run with pandas

#### 🌐 HTML Report

- Interactive web report
- Visualizations (charts and graphs)
- Quality check summaries
- Statistical tables
- Pattern detection alerts
- Professional formatting

### How to Use

1. **Select Formats**: Check the boxes for desired output formats
2. **Multiple Selection**: Choose any combination of formats
3. **Process Data**: Click "▶ Process Data"
4. **View Results**: All selected formats are generated simultaneously

### Multi-Format Processing

**Benefits:**

- Generate multiple formats in one click
- Save time with parallel processing
- Ensure consistency across all outputs
- Perfect for collaborative teams using different tools

**Example Use Case:**

```
✓ CSV File      (for data storage)
✓ Excel File    (for quick viewing)
✓ R Script      (for statistical analysis)
✓ HTML Report   (for presentations)
```

**Output:**

```
✅ Success! Generated 4 file(s):
  • clean_data.csv
  • clean_data.xlsx
  • analysis_script.R
  • report.html

📁 Output folder: D:\projects\study\processed\
```

---

## 📚 Scale Library Browser

### Overview

Browse detailed information about all pre-built psychology scales with comprehensive metadata.

### Features

- **Detailed Metadata**: Full scale information including:

  - Full scale name
  - Description and purpose
  - Number of items
  - Score range (min/max)
  - Citation information
  - Interpretation guidelines (if available)
  - Normative data (if available)

- **One-Click Loading**: Load any scale configuration instantly
- **Professional Layout**: Card-based design with clear organization
- **Search Ready**: Easy to scan and find the scale you need

### How to Use

1. **Open Examples Modal**: Click "📝 Examples" button
2. **Browse All**: Click "📚 Browse All" button next to Pre-built Scales
3. **View Details**: See all available scales with full information
4. **Load Config**: Click "📥 Load Configuration" on any scale card
5. **Auto-Fill**: Configuration is loaded into the editor automatically

### Scale Card Information

Each scale card shows:

```
┌─────────────────────────────────────┐
│ Beck Depression Inventory (BDI-II)  │
│                                     │
│ Assesses severity of depression... │
│                                     │
│ Items: 21    Range: 0-63           │
│                                     │
│ Citation: Beck, A. T., et al....   │
│                                     │
│ [📥 Load Configuration]             │
└─────────────────────────────────────┘
```

### Available Scales

The browser displays all scales from the built-in library:

- **Depression Scales**: BDI-II, PHQ-9
- **Anxiety Scales**: GAD-7, BAI
- **Burnout Scales**: MBI-HSS
- **Relationship Scales**: Working Alliance Inventory
- And many more...

---

## 🎯 Collapsible Sections

### Purpose

Reduce visual clutter and improve focus by collapsing sections you're not currently using.

### Features

- **Click to Toggle**: Click any section header to expand/collapse
- **Visual Indicator**: Arrow icon (▼) rotates when collapsed
- **Smooth Animations**: Content fades and slides smoothly
- **State Preservation**: Each section maintains its state independently

### Collapsible Sections

#### ⚙️ Output Options

- Output format checkboxes
- Collapse when you've made your selection
- Expand to change formats

#### 📖 Data Dictionary (v0.8.0)

- Format selector and export button
- Collapse after generation

#### 📊 CONSORT Flowchart (v0.8.0)

- Format options and generation
- Collapse when not needed

### How to Use

1. **Collapse**: Click the section header (e.g., "⚙️ Output Options")
2. **Visual Feedback**:
   - Arrow rotates from ▼ to ▶
   - Content smoothly fades out
3. **Expand**: Click the header again to reveal content
4. **Focus**: Keep open only the sections you're actively using

### Keyboard-Friendly

- Use Tab to navigate to section headers
- Press Enter or Space to toggle

---

## 💡 Additional Quality-of-Life Improvements

### Enhanced Layout

- **Wider Interface**: Increased from 700px to 800px max-width
- **Better Spacing**: Improved padding and margins throughout
- **Grid Layouts**: Organized checkbox groups in clean grids

### Improved Tooltips

- **More Detailed**: Comprehensive descriptions for all features
- **Better Positioning**: Optimized tooltip placement
- **Hover Animations**: Smooth fade-in effects

### Better Visual Hierarchy

- **Clear Sections**: Distinct section boundaries
- **Color Coding**: Consistent use of theme colors
- **Icon Usage**: Meaningful icons for quick recognition

### Enhanced Buttons

- **Consistent Styling**: Uniform button appearance
- **Clear States**: Hover, active, and disabled states
- **Action Indicators**: Icons show button purpose

### Smart Defaults

- **CSV Always Selected**: Most common format pre-checked
- **Quality Flags Enabled**: All v0.8.0 checks active by default
- **Sensible Thresholds**: Pre-configured quality check thresholds

---

## 🚀 Advanced Workflows

### Workflow 1: Multi-Format Academic Paper

**Goal**: Generate all formats for publication submission

```
1. Load your data CSV
2. Configure or load a pre-built scale
3. Select outputs:
   ✓ CSV File (supplementary material)
   ✓ Excel File (for collaborators)
   ✓ SPSS Syntax (for reproducibility)
   ✓ HTML Report (for peer review)
4. Process data
5. Generate Data Dictionary (CSV)
6. Generate CONSORT flowchart (Text)
7. Submit all files with your paper
```

### Workflow 2: Team Collaboration

**Goal**: Support multiple analysis platforms

```
1. Process data with all formats:
   ✓ CSV (data management)
   ✓ Excel (quick checks)
   ✓ R Script (statistician)
   ✓ Python Script (data scientist)
   ✓ SPSS Syntax (clinical researcher)
2. Share the output folder
3. Each team member uses their preferred tool
4. Everyone works from identical processed data
```

### Workflow 3: Presentation Ready

**Goal**: Create outputs for stakeholder presentation

```
1. Enable dark mode for screen presentation
2. Process with HTML report format
3. Generate CONSORT flowchart (Text or JSON)
4. Open HTML report in browser
5. Present interactive visualizations
6. Show quality check results
7. Display CONSORT diagram
```

### Workflow 4: Rapid Prototyping

**Goal**: Test different scales quickly

```
1. Load CSV file
2. Click "📝 Examples" → "📚 Browse All"
3. Scan scale library for appropriate measures
4. Load scale configuration
5. Process with HTML report
6. Review results immediately
7. Iterate with different scales
```

---

## 🎨 Customization Tips

### Theme Selection

- **Daytime Work**: Use light mode for bright environments
- **Evening/Night**: Use dark mode to reduce eye strain
- **Presentations**: Match theme to room lighting

### Format Selection

- **Data Sharing**: Always include CSV
- **Statistical Analysis**: Add R or Python scripts
- **Collaborators**: Include Excel for universal access
- **Publications**: Generate HTML reports for supplementary
- **Reproducibility**: Include all script formats

### Section Organization

- **Active Work**: Keep relevant sections expanded
- **Completed Steps**: Collapse to reduce clutter
- **Review**: Collapse all to see full workflow overview

---

## 🔧 Technical Details

### Browser Compatibility

- **Chrome/Edge**: Full support for all features
- **Firefox**: Full support
- **Safari**: Full support
- **Tauri WebView**: Optimized for native experience

### Performance

- **Format Generation**: Parallel processing where possible
- **Theme Switching**: Instant with CSS transitions
- **Section Animations**: Hardware-accelerated for smoothness
- **Large Files**: Efficient handling of multi-format output

### Storage

- **Theme Preference**: Saved to localStorage
- **Persistent**: Survives browser restarts
- **Privacy**: All data stays local (no cloud sync)

### Accessibility

- **Keyboard Navigation**: Full keyboard support
- **Screen Readers**: Semantic HTML for accessibility
- **High Contrast**: Works with system accessibility settings
- **Focus Indicators**: Clear visual focus states

---

## 📊 Feature Comparison

| Feature           | Basic GUI  | Enhanced GUI  |
| ----------------- | ---------- | ------------- |
| Output Formats    | CSV only   | 6 formats     |
| Theme Options     | Light only | Light + Dark  |
| Scale Browser     | List only  | Full metadata |
| Section Layout    | Static     | Collapsible   |
| Multi-Format      | No         | Yes           |
| HTML Reports      | No         | Yes           |
| Format Grid       | No         | Yes           |
| Theme Persistence | No         | Yes           |
| Max Width         | 700px      | 800px         |
| Quick Actions     | Basic      | Enhanced      |

---

## 🐛 Troubleshooting

### Issue: Theme Not Persisting

**Solution**: Check browser localStorage is enabled

### Issue: Multiple Formats Fail

**Solution**: Ensure CLI is built (cargo build --release)

### Issue: Scale Browser Empty

**Solution**: Verify scale library is accessible

### Issue: Sections Won't Collapse

**Solution**: Ensure JavaScript is enabled

---

## 🔮 Future Enhancements

### Planned Features

- **Recent Files List**: Quick access to recently processed files
- **Batch Processing**: Process multiple files at once
- **Custom Themes**: Create and save your own color schemes
- **Export Presets**: Save favorite format combinations
- **Scale Search**: Filter scales by category or keyword
- **Drag-and-Drop**: Direct file drag-and-drop support

### Community Requests

- **PDF Reports**: Export reports as PDF
- **Chart Customization**: Adjust chart styles in HTML reports
- **Config Snippets**: Save and reuse configuration fragments
- **Live Preview**: Preview data before processing

---

## 📚 Additional Resources

- **GUI Usage Guide**: `docs/GUI_USAGE.md`
- **v0.8.0 Updates**: `GUI_v0.8.0_UPDATES.md`
- **CLI Documentation**: `docs/HOW_TO_USE.md`
- **Configuration Guide**: `docs/CONFIGURATION_GUIDE.md`

---

## 🙏 Feedback

We'd love to hear about your experience with these new features!

- **Report Issues**: https://github.com/artlostintime/prism/issues
- **Feature Requests**: https://github.com/artlostintime/prism/discussions
- **Share Workflows**: Help others by sharing your favorite workflows

---

**🎉 Enjoy the enhanced Prism GUI experience!**

All features are production-ready and optimized for your research workflow.
