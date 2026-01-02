# Installation Guide

**[📚 Wiki Home](README.md)** | **[📖 How to Use](HOW_TO_USE.md)** | **[🎓 Tutorial](TUTORIAL.md)** | **[❓ FAQ](FAQ.md)**

---

## Requirements

### System Requirements

- **OS:** Windows 10/11, macOS 10.15+, Linux (Ubuntu 20.04+)
- **RAM:** 512 MB minimum (2 GB recommended for large datasets)
- **Disk:** 100 MB for application + space for data

### Software Requirements

- **Rust:** 1.70+ (for building from source)
- **Cargo:** Included with Rust
- Optional: Git (for cloning repository)

---

## Installation Methods

### Method 1: Pre-built Binary (Easiest)

**Windows:**

```powershell
# Download from releases page
# Extract prism.exe to desired location
# Add to PATH (optional)
```

**macOS/Linux:**

```bash
# Download from releases page
chmod +x prism
sudo mv prism /usr/local/bin/
```

### Method 2: Build from Source (Recommended)

**1. Install Rust:**

```bash
# Visit https://rustup.rs/
# Or use:
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

**2. Clone Repository:**

```bash
git clone https://github.com/artlostintime/prism.git
cd prism
```

**3. Build Release:**

```bash
cargo build --release
```

**4. Binary Location:**

- **Output:** `./target/release/prism` (or `prism.exe` on Windows)
- **Size:** ~2-5 MB

### Method 3: Install via Cargo

```bash
cargo install --path .
# Or from crates.io (when published):
# cargo install prism-survey
```

---

## Verification

**Check installation:**

```bash
prism --version
# Output: prism 0.1.0
```

**Run test:**

```bash
cd examples
prism -i sample_data.csv -c study_config.toml -o test_output.csv
# Should process successfully
```

---

## GUI Installation (Optional)

### Building the GUI

**1. Install Tauri Prerequisites:**

**Windows:**

```powershell
# Install Visual Studio 2022 Build Tools
# Or Visual Studio Community with C++ Desktop Development
```

**macOS:**

```bash
xcode-select --install
```

**Linux (Ubuntu/Debian):**

```bash
sudo apt update
sudo apt install libwebkit2gtk-4.0-dev \
    build-essential \
    curl \
    wget \
    file \
    libssl-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev
```

**2. Build GUI:**

```bash
cd src-tauri
cargo build --release
```

**3. Run GUI:**

```bash
# Binary location:
# Windows: src-tauri/target/release/prism-gui.exe
# macOS: src-tauri/target/release/prism-gui
# Linux: src-tauri/target/release/prism-gui
```

---

## Post-Installation Setup

### 1. Add to PATH (Optional)

**Windows:**

```powershell
# Add to User PATH:
$path = [Environment]::GetEnvironmentVariable("Path", "User")
$path += ";C:\path\to\prism"
[Environment]::SetEnvironmentVariable("Path", $path, "User")
```

**macOS/Linux:**

```bash
# Add to ~/.bashrc or ~/.zshrc:
export PATH="$PATH:/path/to/prism/target/release"
source ~/.bashrc  # or ~/.zshrc
```

### 2. Verify Examples

```bash
cd examples
ls
# Should see: sample_data.csv, study_config.toml, README.md
```

### 3. Create Your First Config

See [Configuration Guide](CONFIGURATION_GUIDE.md) to create your survey config.

---

## Troubleshooting Installation

### "cargo: command not found"

**Problem:** Rust/Cargo not installed or not in PATH

**Solution:**

```bash
# Install Rust:
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# Restart terminal
```

### Build Fails on Windows

**Problem:** Missing Visual Studio Build Tools

**Solution:**

- Download [Visual Studio Build Tools](https://visualstudio.microsoft.com/downloads/)
- Select "C++ build tools" during installation
- Restart and rebuild

### "linking with `cc` failed" on Linux

**Problem:** Missing build dependencies

**Solution:**

```bash
sudo apt update
sudo apt install build-essential pkg-config libssl-dev
```

### GUI Build Fails

**Problem:** Missing Tauri dependencies

**Solution:** Follow [GUI Installation](#gui-installation-optional) prerequisites for your OS

### Permission Denied (macOS/Linux)

**Problem:** Binary not executable

**Solution:**

```bash
chmod +x target/release/prism
```

---

## Updating Prism

### From Source:

```bash
cd prism
git pull
cargo build --release
```

### Via Cargo:

```bash
cargo install --path . --force
# Or: cargo install prism-survey --force
```

---

## Uninstallation

### Remove Binary:

```bash
# If installed via cargo:
cargo uninstall prism-survey

# If manual installation:
rm /usr/local/bin/prism  # macOS/Linux
# Or delete prism.exe     # Windows
```

### Remove Source:

```bash
rm -rf /path/to/prism
```

---

## Next Steps

✅ **Installation complete!** Choose your path:

- **New User?** → [Tutorial](TUTORIAL.md)
- **Quick Start?** → [How to Use](HOW_TO_USE.md)
- **Need Help?** → [FAQ](FAQ.md)
- **Advanced User?** → [Configuration Guide](CONFIGURATION_GUIDE.md)

---

## Wiki Navigation

- [📚 Back to Wiki Home](README.md)
- [🎓 Tutorial →](TUTORIAL.md)
- [📖 How to Use →](HOW_TO_USE.md)
- [❓ FAQ](FAQ.md)

---

[⬆ Back to Top](#installation-guide) | [📚 Wiki Home](README.md)
