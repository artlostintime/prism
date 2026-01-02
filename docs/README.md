# Prism Documentation Wiki

<div align="center">

**Psychology Research Data Processing Pipeline**

📊 Automated survey data processing | 🔍 Quality checks | 📈 Statistical reports

[Quick Start](#quick-start) • [User Guide](HOW_TO_USE.md) • [API Reference](API_REFERENCE.md) • [Examples](../examples/)

</div>

---

## 🚀 Quick Start

**1. Install & Build:**

```bash
cargo build --release
```

**2. Process Your Data:**

```bash
./target/release/prism -i data.csv -c config.toml -o clean.csv
```

**3. Review Results:**

- `clean.csv` - Processed data with scale scores
- `summary_stats.txt` - Aggregate statistics
- `quality_report.txt` - Quality issues

**That's it!** See the [complete guide](HOW_TO_USE.md) for details.

---

## 📚 Documentation

### Getting Started

- **[Installation Guide](INSTALLATION.md)** - Setup and requirements
- **[How to Use](HOW_TO_USE.md)** - Complete usage guide with examples
- **[Quick Reference](QUICK_REFERENCE.md)** - Command cheat sheet
- **[Tutorial](TUTORIAL.md)** - Step-by-step walkthrough

### User Guides

- **[Configuration Guide](CONFIGURATION_GUIDE.md)** - Config file reference
- **[Quality Checks](QUALITY_CHECKS.md)** - Understanding quality reports
- **[Workflow Examples](WORKFLOW_EXAMPLE.md)** - Real-world use cases
- **[Best Practices](BEST_PRACTICES.md)** - Tips and recommendations

### Technical Documentation

- **[Architecture](ARCHITECTURE.md)** - System design and structure
- **[API Reference](API_REFERENCE.md)** - Function and module documentation
- **[Implementation Status](IMPLEMENTATION_STATUS.md)** - Feature completion
- **[Refactoring Notes](REFACTORING_NOTES.md)** - Code quality improvements

### For Contributors

- **[Contributing Guide](CONTRIBUTING.md)** - How to contribute
- **[Development Setup](DEVELOPMENT.md)** - Local development guide
- **[Testing Guide](TESTING.md)** - Writing and running tests
- **[Code Style](CODE_STYLE.md)** - Rust coding standards

### Reference

- **[FAQ](FAQ.md)** - Frequently asked questions
- **[Troubleshooting](TROUBLESHOOTING.md)** - Common issues and solutions
- **[Glossary](GLOSSARY.md)** - Terminology and definitions
- **[Changelog](CHANGELOG.md)** - Version history

---

## 🎯 Use Cases

### Research Workflows

- [Processing Burnout Studies](WORKFLOW_EXAMPLE.md#burnout-study)
- [Longitudinal Data Analysis](WORKFLOW_EXAMPLE.md#longitudinal-data)
- [Multi-Scale Surveys](WORKFLOW_EXAMPLE.md#multi-scale)

### Integration

- [Import to R](HOW_TO_USE.md#import-to-rpython)
- [Import to SPSS](HOW_TO_USE.md#import-to-rpython)
- [Import to Python/Pandas](HOW_TO_USE.md#import-to-rpython)

### Automation

- [Batch Processing](WORKFLOW_EXAMPLE.md#batch-processing)
- [CI/CD Integration](WORKFLOW_EXAMPLE.md#automation)
- [Scheduled Processing](WORKFLOW_EXAMPLE.md#automated-weekly-processing)

---

## 🔧 Features

### Core Processing

- ✅ Automatic reverse scoring
- ✅ Scale total and mean computation
- ✅ Configurable survey definitions
- ✅ CSV input/output

### Quality Assurance

- ✅ Straightlining detection
- ✅ Missing data analysis
- ✅ Out-of-range detection
- ✅ Participant-level flagging

### Statistical Reporting

- ✅ Aggregate statistics (M, SD, min, max, N)
- ✅ Summary statistics reports
- ✅ Quality issue reports
- ✅ APA-ready output

### Interfaces

- ✅ Command-line interface
- ✅ Desktop GUI (Tauri)
- ✅ Batch processing support

See [Implementation Status](IMPLEMENTATION_STATUS.md) for complete feature list.

---

## 📖 Documentation by Role

### **I'm a Researcher** 👨‍🔬

Start here:

1. [Installation Guide](INSTALLATION.md)
2. [How to Use](HOW_TO_USE.md)
3. [Configuration Guide](CONFIGURATION_GUIDE.md)
4. [Workflow Examples](WORKFLOW_EXAMPLE.md)

### **I'm a Student/RA** 👨‍🎓

Start here:

1. [Quick Reference](QUICK_REFERENCE.md)
2. [Tutorial](TUTORIAL.md)
3. [FAQ](FAQ.md)
4. [Troubleshooting](TROUBLESHOOTING.md)

### **I'm a Developer** 👨‍💻

Start here:

1. [Architecture](ARCHITECTURE.md)
2. [Development Setup](DEVELOPMENT.md)
3. [API Reference](API_REFERENCE.md)
4. [Contributing Guide](CONTRIBUTING.md)

### **I'm a Lab Manager** 👨‍💼

Start here:

1. [Workflow Examples](WORKFLOW_EXAMPLE.md)
2. [Best Practices](BEST_PRACTICES.md)
3. [Quality Checks](QUALITY_CHECKS.md)

---

## 🆘 Getting Help

1. Check the [FAQ](FAQ.md)
2. Review [Troubleshooting](TROUBLESHOOTING.md)
3. Search [existing documentation](#documentation)
4. Open an issue on GitHub

---

## 📊 Project Stats

- **Language:** Rust
- **Version:** 0.1.0
- **License:** MIT
- **Status:** Production-ready
- **Lines of Code:** ~500 (core)
- **Test Coverage:** In progress

---

## 🗺️ Wiki Navigation

**Current Page:** Home

**Quick Links:**

- [← Back to Repository](../README.md)
- [Getting Started →](INSTALLATION.md)
- [Examples →](../examples/README.md)
- [Tests →](../tests/README.md)

---

<div align="center">

**[⬆ Back to Top](#prism-documentation-wiki)**

Made with ❤️ for psychology researchers | [MIT License](../LICENSE)

</div>
