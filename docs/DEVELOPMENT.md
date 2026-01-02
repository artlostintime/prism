# Development Guide

**[📚 Wiki Home](README.md)** | **[🤝 Contributing](CONTRIBUTING.md)** | **[🧪 Testing](TESTING.md)** | **[🏗️ Architecture](ARCHITECTURE.md)**

---

## Overview

This guide provides detailed instructions for setting up a local development environment and working on Prism.

---

## Table of Contents

1. [Prerequisites](#prerequisites)
2. [Environment Setup](#environment-setup)
3. [Project Structure](#project-structure)
4. [Development Workflow](#development-workflow)
5. [Building](#building)
6. [Testing](#testing)
7. [Debugging](#debugging)
8. [Common Tasks](#common-tasks)

---

## Prerequisites

### Required Software

**Rust Toolchain:**

```bash
# Install rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Verify installation
rustc --version
cargo --version
```

**Git:**

```bash
# Verify installation
git --version
```

### Optional Software

**Code Editor:**

- VS Code with rust-analyzer extension (recommended)
- IntelliJ IDEA with Rust plugin
- Any text editor

**GUI Development (optional):**

- See [Installation Guide](INSTALLATION.md) for Tauri prerequisites

---

## Environment Setup

### 1. Clone Repository

```bash
git clone https://github.com/artlostintime/prism.git
cd prism
```

### 2. Build Project

```bash
# Debug build (faster compilation, slower runtime)
cargo build

# Release build (slower compilation, faster runtime)
cargo build --release
```

### 3. Verify Setup

```bash
# Run CLI
cargo run -- -i examples/sample_data.csv -c examples/study_config.toml -o test_output.csv

# Run tests
cargo test

# Check for errors
cargo clippy

# Format code
cargo fmt
```

---

## Project Structure

```
prism/
├── Cargo.toml           # Project manifest
├── src/
│   ├── main.rs          # CLI entry point + core logic
│   └── config.rs        # Config structures
├── src-tauri/           # GUI (Tauri)
│   ├── Cargo.toml
│   ├── src/
│   │   ├── lib.rs       # GUI commands
│   │   └── main.rs      # GUI entry point
│   └── tauri.conf.json  # Tauri config
├── examples/            # Example data and configs
├── tests/               # Integration tests
├── docs/                # Documentation wiki
└── target/              # Build output (git-ignored)
```

---

## Development Workflow

### Daily Workflow

**1. Create a branch:**

```bash
git checkout -b feature/my-feature
```

**2. Make changes:**

```bash
# Edit files
code src/main.rs
```

**3. Test changes:**

```bash
# Run specific test
cargo test test_name

# Run all tests
cargo test

# Manual testing
cargo run -- -i examples/sample_data.csv -c examples/study_config.toml -o test.csv
```

**4. Check code quality:**

```bash
# Format code
cargo fmt

# Check for issues
cargo clippy

# Build without errors
cargo build
```

**5. Commit:**

```bash
git add .
git commit -m "Add feature: description"
```

**6. Push and create PR:**

```bash
git push origin feature/my-feature
# Then create PR on GitHub
```

---

## Building

### Debug Build

**Fast compilation, slower runtime:**

```bash
cargo build

# Output: target/debug/prism (or prism.exe)
```

**Use for:**

- Development
- Testing
- Debugging

### Release Build

**Slow compilation, fast runtime:**

```bash
cargo build --release

# Output: target/release/prism (or prism.exe)
```

**Use for:**

- Production
- Performance testing
- Distribution

### GUI Build

```bash
cd src-tauri
cargo build --release

# Output: src-tauri/target/release/prism-gui (or .exe)
```

---

## Testing

### Unit Tests

**Run all tests:**

```bash
cargo test
```

**Run specific test:**

```bash
cargo test test_name
```

**Run with output:**

```bash
cargo test -- --nocapture
```

### Integration Tests

**Located in `tests/` directory:**

```bash
cargo test --test integration_test
```

### Manual Testing

**Test CLI:**

```bash
# Basic run
cargo run -- -i examples/sample_data.csv -c examples/study_config.toml -o test.csv

# With all options
cargo run -- -i examples/sample_data.csv -c examples/study_config.toml -o test.csv \
  --stats-output stats.txt --quality-report quality.txt
```

**Test error handling:**

```bash
# Test with invalid config
cargo run -- -i examples/sample_data.csv -c invalid.toml -o test.csv

# Test with missing file
cargo run -- -i nonexistent.csv -c examples/study_config.toml -o test.csv
```

---

## Debugging

### Print Debugging

```rust
// Add debug prints
println!("Debug: value = {:?}", value);
dbg!(variable);
```

### Rust Debugger

**VS Code:**

1. Install "CodeLLDB" extension
2. Add breakpoint (click left of line number)
3. Press F5 to start debugging

**Command line (lldb/gdb):**

```bash
# Build with debug symbols
cargo build

# Run with debugger
rust-lldb target/debug/prism
# or
rust-gdb target/debug/prism
```

### Logging

**Add env_logger (in Cargo.toml):**

```toml
[dependencies]
env_logger = "0.10"
log = "0.4"
```

**Use in code:**

```rust
use log::{info, warn, error};

fn main() {
    env_logger::init();
    info!("Starting processing");
    warn!("Warning message");
    error!("Error message");
}
```

**Run with logging:**

```bash
RUST_LOG=debug cargo run -- -i data.csv -c config.toml -o output.csv
```

---

## Common Tasks

### Adding a New Feature

**1. Create function:**

```rust
// src/main.rs
fn my_new_feature(data: &[f64]) -> f64 {
    // Implementation
    data.iter().sum()
}
```

**2. Add test:**

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_new_feature() {
        let data = vec![1.0, 2.0, 3.0];
        let result = my_new_feature(&data);
        assert_eq!(result, 6.0);
    }
}
```

**3. Integrate:**

```rust
fn main() -> Result<()> {
    // ... existing code ...
    let result = my_new_feature(&data);
    println!("Result: {}", result);
    Ok(())
}
```

### Adding a CLI Flag

**1. Add to Cli struct:**

```rust
#[derive(Parser)]
struct Cli {
    // ... existing fields ...

    /// New flag description
    #[arg(long)]
    my_flag: bool,
}
```

**2. Use in main:**

```rust
fn main() -> Result<()> {
    let cli = Cli::parse();

    if cli.my_flag {
        // Do something
    }

    Ok(())
}
```

### Updating Documentation

**1. Edit markdown files in `docs/`:**

```bash
code docs/HOW_TO_USE.md
```

**2. Test links:**

```bash
# Verify all links work
# Check formatting renders correctly
```

**3. Update wiki home if needed:**

```bash
code docs/README.md
```

---

## Performance Profiling

### Benchmarking

**Add criterion (in Cargo.toml):**

```toml
[dev-dependencies]
criterion = "0.5"

[[bench]]
name = "my_benchmark"
harness = false
```

**Create benchmark:**

```rust
// benches/my_benchmark.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_function(c: &mut Criterion) {
    c.bench_function("my_function", |b| {
        b.iter(|| {
            // Code to benchmark
            my_function(black_box(100))
        });
    });
}

criterion_group!(benches, benchmark_function);
criterion_main!(benches);
```

**Run benchmark:**

```bash
cargo bench
```

### Memory Profiling

**Using heaptrack (Linux):**

```bash
heaptrack target/release/prism -i data.csv -c config.toml -o output.csv
heaptrack_gui heaptrack.prism.*.gz
```

---

## Continuous Integration

### GitHub Actions

**Workflow runs on:**

- Push to main
- Pull requests
- Release tags

**Checks:**

- ✅ cargo build
- ✅ cargo test
- ✅ cargo clippy
- ✅ cargo fmt --check

---

## Release Process

### Creating a Release

**1. Update version:**

```toml
# Cargo.toml
[package]
version = "0.2.0"  # Increment version
```

**2. Update CHANGELOG:**

```bash
code docs/CHANGELOG.md
# Add new version section
```

**3. Commit and tag:**

```bash
git add .
git commit -m "Release v0.2.0"
git tag -a v0.2.0 -m "Version 0.2.0"
git push origin main --tags
```

**4. Build release binaries:**

```bash
cargo build --release
cd src-tauri
cargo build --release
```

**5. Create GitHub release:**

- Go to GitHub releases
- Create new release from tag
- Upload binaries
- Publish release

---

## Troubleshooting Development Issues

### "Cargo command not found"

```bash
# Add to PATH
source $HOME/.cargo/env
```

### "Cannot find crate"

```bash
# Update dependencies
cargo update
```

### "Compilation failed"

```bash
# Clean and rebuild
cargo clean
cargo build
```

### "Tests failing"

```bash
# Run specific test to see output
cargo test test_name -- --nocapture
```

---

## Best Practices

### Code Quality

✅ **DO:**

- Write tests for new features
- Run `cargo clippy` before committing
- Use `cargo fmt` for consistent formatting
- Add documentation comments
- Handle errors with `Result` types

❌ **DON'T:**

- Use `unwrap()` (use `?` instead)
- Commit without running tests
- Ignore clippy warnings
- Write 200+ line functions
- Skip error handling

### Git Workflow

✅ **DO:**

- Create feature branches
- Write descriptive commit messages
- Keep commits focused and atomic
- Pull latest main before branching
- Squash commits before merging

❌ **DON'T:**

- Commit directly to main
- Use vague commit messages ("fix bug")
- Mix unrelated changes
- Include build artifacts
- Force push to shared branches

---

## Resources

### Documentation

- [Rust Book](https://doc.rust-lang.org/book/)
- [Cargo Book](https://doc.rust-lang.org/cargo/)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [Tauri Documentation](https://tauri.app/)

### Tools

- [rust-analyzer](https://rust-analyzer.github.io/) - IDE support
- [clippy](https://github.com/rust-lang/rust-clippy) - Linter
- [rustfmt](https://github.com/rust-lang/rustfmt) - Code formatter

---

## Wiki Navigation

- [📚 Back to Wiki Home](README.md)
- [🤝 Contributing Guide](CONTRIBUTING.md)
- [🧪 Testing Guide](TESTING.md)
- [🏗️ Architecture](ARCHITECTURE.md)
- [🔍 API Reference](API_REFERENCE.md)

---

[⬆ Back to Top](#development-guide) | [📚 Wiki Home](README.md)
