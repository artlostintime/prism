# Homebrew Installation Guide

Prism can be installed on macOS via Homebrew for easy package management.

## Installation

### Option 1: Install from GitHub (Recommended)

```bash
# Install latest release
brew install artlostintime/tap/prism

# Or install from this repository directly
brew install --HEAD https://raw.githubusercontent.com/artlostintime/prism/main/prism.rb
```

### Option 2: Build from Source

```bash
# Clone the repository
git clone https://github.com/artlostintime/prism.git
cd prism

# Install using the formula
brew install --build-from-source prism.rb
```

### Option 3: Development Version

```bash
# Install the latest development version
brew install --HEAD artlostintime/tap/prism
```

## Verification

After installation, verify Prism is working:

```bash
# Check version
prism --version

# List available scales
prism generate --list-scales

# View help
prism --help
```

## Usage

### Generate Configuration

```bash
# Create a config for PHQ-9 depression scale
prism generate --scale PHQ-9 > phq9_config.toml
```

### Process Data

```bash
# Process survey data
prism process \
  --input raw_data.csv \
  --config phq9_config.toml \
  --output clean_data.csv \
  --stats-report statistics.txt \
  --quality-report quality_checks.txt
```

### View Examples

Example configurations are installed with Prism:

```bash
# macOS (Homebrew)
ls $(brew --prefix prism)/share/prism/examples/

# Copy example to current directory
cp $(brew --prefix prism)/share/prism/examples/study_config.toml .
```

## Updating

Keep Prism up to date:

```bash
# Update Homebrew
brew update

# Upgrade Prism to latest version
brew upgrade prism

# Or upgrade all packages
brew upgrade
```

## Uninstallation

Remove Prism:

```bash
brew uninstall prism

# Also remove cached downloads (optional)
brew cleanup prism
```

## Troubleshooting

### Command Not Found

If `prism` command is not found after installation:

```bash
# Check if Homebrew bin is in PATH
echo $PATH | grep -q "$(brew --prefix)/bin" || echo "Homebrew not in PATH"

# Add Homebrew to PATH (if needed)
echo 'export PATH="/opt/homebrew/bin:$PATH"' >> ~/.zshrc  # Apple Silicon
echo 'export PATH="/usr/local/bin:$PATH"' >> ~/.zshrc      # Intel Mac
source ~/.zshrc
```

### Build Errors

If build fails:

```bash
# Update Rust
rustup update

# Clear Homebrew cache
brew cleanup --prune=all

# Try reinstalling
brew reinstall prism
```

### Permission Issues

If you encounter permission errors:

```bash
# Fix Homebrew permissions
sudo chown -R $(whoami) $(brew --prefix)/*
```

## Alternative Installation Methods

If Homebrew doesn't work for your setup:

### Direct Binary Download

Download pre-built binaries from [GitHub Releases](https://github.com/artlostintime/prism/releases):

```bash
# Download for macOS (Intel)
curl -L https://github.com/artlostintime/prism/releases/latest/download/prism-v0.8.6-macos-x64 -o prism
chmod +x prism
sudo mv prism /usr/local/bin/

# Download for macOS (Apple Silicon)
curl -L https://github.com/artlostintime/prism/releases/latest/download/prism-v0.8.6-macos-arm64 -o prism
chmod +x prism
sudo mv prism /usr/local/bin/
```

### Build from Source

```bash
# Clone repository
git clone https://github.com/artlostintime/prism.git
cd prism

# Build release binary
cargo build --release

# Copy to system path
sudo cp target/release/prism /usr/local/bin/
```

## Shell Completions

Prism includes shell completions for enhanced command-line experience:

### Zsh (default on modern macOS)

```bash
# Generate completions
prism completion zsh > ~/.zsh/completions/_prism

# Or install via Homebrew (automatic)
# Completions are installed to $(brew --prefix)/share/zsh/site-functions
```

### Bash

```bash
# Generate completions
prism completion bash > ~/.bash_completions/prism

# Add to .bashrc
echo 'source ~/.bash_completions/prism' >> ~/.bashrc
source ~/.bashrc
```

### Fish

```bash
# Generate completions
prism completion fish > ~/.config/fish/completions/prism.fish
```

## Integration with Other Tools

### Use with R

```bash
# Generate R script for analysis
prism process \
  --input data.csv \
  --config config.toml \
  --format r

# Output: analysis_script.R
Rscript analysis_script.R
```

### Use with Python

```bash
# Generate Python script
prism process \
  --input data.csv \
  --config config.toml \
  --format python

# Output: analysis_script.py
python analysis_script.py
```

### Use with SPSS

```bash
# Generate SPSS syntax
prism process \
  --input data.csv \
  --config config.toml \
  --format spss

# Import clean_data.csv and run analysis_script.sps in SPSS
```

## Getting Help

- **Documentation**: https://github.com/artlostintime/prism/tree/main/docs
- **Examples**: `$(brew --prefix prism)/share/prism/examples/`
- **Issues**: https://github.com/artlostintime/prism/issues
- **Discussions**: https://github.com/artlostintime/prism/discussions

## See Also

- [Installation Guide](INSTALLATION.md) - All installation methods
- [Quick Start Guide](HOW_TO_USE.md) - Getting started tutorial
- [Configuration Guide](CONFIGURATION_GUIDE.md) - Config file reference
