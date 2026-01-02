# Contributing to Prism

**[📚 Wiki Home](README.md)** | **[💻 Development](DEVELOPMENT.md)** | **[🧪 Testing](TESTING.md)** | **[🏗️ Architecture](ARCHITECTURE.md)**

---

## Welcome! 🎉

Thank you for your interest in contributing to Prism! This guide will help you get started.

---

## Table of Contents

1. [Code of Conduct](#code-of-conduct)
2. [How to Contribute](#how-to-contribute)
3. [Development Setup](#development-setup)
4. [Coding Standards](#coding-standards)
5. [Testing Guidelines](#testing-guidelines)
6. [Pull Request Process](#pull-request-process)
7. [Issue Guidelines](#issue-guidelines)

---

## Code of Conduct

### Our Pledge

We are committed to providing a welcoming and inclusive environment for all contributors.

### Expected Behavior

- Be respectful and considerate
- Welcome newcomers
- Focus on constructive feedback
- Respect differing viewpoints

### Unacceptable Behavior

- Harassment, discrimination, or offensive language
- Personal attacks or trolling
- Publishing others' private information
- Other unprofessional conduct

---

## How to Contribute

### Ways to Contribute

**Code:**

- 🐛 Fix bugs
- ✨ Add new features
- ⚡ Improve performance
- 🎨 Enhance UI/UX

**Documentation:**

- 📖 Improve guides and tutorials
- ✏️ Fix typos or unclear explanations
- 📝 Add examples
- 🌍 Translate documentation

**Quality:**

- 🧪 Write tests
- 🔍 Review pull requests
- 🐛 Report bugs
- 💡 Suggest improvements

**Community:**

- ❓ Answer questions
- 📢 Share Prism with others
- 🎓 Create tutorials or blog posts

---

## Development Setup

### 1. Fork and Clone

```bash
# Fork the repository on GitHub, then:
git clone https://github.com/YOUR_USERNAME/prism.git
cd prism
```

### 2. Install Dependencies

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build the project
cargo build
```

### 3. Create a Branch

```bash
git checkout -b feature/your-feature-name
# or
git checkout -b fix/bug-description
```

### 4. Make Changes

See [Development Guide](DEVELOPMENT.md) for detailed instructions.

### 5. Test Your Changes

```bash
# Run tests
cargo test

# Test manually
cargo run -- -i examples/sample_data.csv -c examples/study_config.toml -o test_output.csv
```

### 6. Commit and Push

```bash
git add .
git commit -m "Add feature: your description"
git push origin feature/your-feature-name
```

### 7. Create Pull Request

Go to GitHub and create a pull request from your branch.

---

## Coding Standards

### Rust Style

Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/):

**✅ Good:**

```rust
// Use descriptive names
fn calculate_scale_statistics(values: &[f64]) -> Stats {
    // Implementation
}

// Document public APIs
/// Calculates statistics for a scale
///
/// # Arguments
/// * `values` - Slice of scale values
///
/// # Returns
/// `Stats` struct with mean, SD, min, max, N
pub fn calculate_stats(values: &[f64]) -> Stats {
    // Implementation
}
```

**❌ Bad:**

```rust
// Vague names
fn calc(v: &[f64]) -> S {
    // Implementation
}

// No documentation
pub fn xyz(a: &[f64]) -> Stats {
    // Implementation
}
```

### Code Organization

**Use helper functions:**

```rust
// ✅ Good
fn main() -> Result<()> {
    let config = load_config()?;
    validate_config(&config)?;
    let data = process_data(&config)?;
    save_output(&data)?;
    Ok(())
}

// ❌ Bad
fn main() -> Result<()> {
    // 200 lines of code here...
}
```

### Error Handling

**Use `Result` and `?` operator:**

```rust
// ✅ Good
fn process_file(path: &str) -> Result<Data> {
    let file = File::open(path)?;
    let data = parse_file(file)?;
    Ok(data)
}

// ❌ Bad
fn process_file(path: &str) -> Data {
    let file = File::open(path).unwrap();  // Avoid unwrap()!
    let data = parse_file(file).unwrap();
    data
}
```

### Comments

**Write clear, helpful comments:**

```rust
// ✅ Good
// Reverse scoring: (max + min) - original_value
// Example: On 1-5 scale, 5 becomes 1, 1 becomes 5
let reversed = (max_val + min_val) - original_val;

// ❌ Bad
// Do the thing
let x = (a + b) - c;
```

---

## Testing Guidelines

### Unit Tests

**Write tests for new functions:**

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_mean() {
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let stats = Stats::calculate(&values);
        assert!((stats.mean - 3.0).abs() < 1e-6);
    }

    #[test]
    fn test_reverse_scoring() {
        let reversed = reverse_score(5.0, 1.0, 5.0);
        assert!((reversed - 1.0).abs() < 1e-6);
    }
}
```

### Integration Tests

**Create tests in `tests/` directory:**

```rust
// tests/integration_test.rs
use prism::config::SurveyConfig;

#[test]
fn test_full_pipeline() {
    let config = SurveyConfig::from_file("tests/fixtures/test_config.toml").unwrap();
    // Test full processing pipeline
}
```

### Test Data

**Use fixtures:**

```
tests/
├── fixtures/
│   ├── test_data.csv
│   ├── test_config.toml
│   └── expected_output.csv
└── integration_test.rs
```

---

## Pull Request Process

### Before Submitting

**Checklist:**

- ✅ Code compiles without warnings
- ✅ All tests pass
- ✅ New tests added for new features
- ✅ Documentation updated
- ✅ Code formatted (`cargo fmt`)
- ✅ Linter passes (`cargo clippy`)
- ✅ Commit messages are clear

### PR Template

```markdown
## Description

Brief description of changes

## Type of Change

- [ ] Bug fix
- [ ] New feature
- [ ] Documentation update
- [ ] Performance improvement

## Testing

- How have you tested this?
- [ ] Unit tests added
- [ ] Integration tests added
- [ ] Manual testing performed

## Checklist

- [ ] Code compiles
- [ ] Tests pass
- [ ] Documentation updated
- [ ] cargo fmt run
- [ ] cargo clippy passes
```

### Review Process

1. **Automated checks:** CI/CD runs tests
2. **Code review:** Maintainer reviews code
3. **Feedback:** Address any comments
4. **Approval:** Once approved, PR is merged

### After Merge

- Your contribution will be in the next release!
- You'll be added to contributors list
- Thank you! 🎉

---

## Issue Guidelines

### Reporting Bugs

**Use this template:**

````markdown
**Describe the bug**
Clear description of the issue

**To Reproduce**
Steps to reproduce:

1. Run command '...'
2. With config '...'
3. See error

**Expected behavior**
What you expected to happen

**Actual behavior**
What actually happened

**Environment**

- Prism version: 0.1.0
- OS: Windows 11
- Rust version: 1.70

**Sample Data**
(If possible, include minimal example)

**Config File**

```toml
[survey]
name = "Test"
# Minimal config showing issue
```
````

````

### Requesting Features

**Use this template:**

```markdown
**Feature Description**
Clear description of the feature

**Use Case**
Why is this feature needed?
What problem does it solve?

**Proposed Solution**
How might this work?

**Alternatives Considered**
Other approaches you've thought about

**Examples**
Example usage or similar features in other tools
````

### Asking Questions

**Before asking:**

1. Check [FAQ](FAQ.md)
2. Search existing issues
3. Review [documentation](README.md)

**If still needed:**

- Be specific and clear
- Include relevant context
- Share what you've tried

---

## Development Resources

### Documentation

- [Development Guide](DEVELOPMENT.md) - Detailed development setup
- [Architecture](ARCHITECTURE.md) - System design
- [API Reference](API_REFERENCE.md) - Function documentation
- [Testing Guide](TESTING.md) - Testing best practices

### Tools

- [Rust Book](https://doc.rust-lang.org/book/) - Learn Rust
- [Cargo Book](https://doc.rust-lang.org/cargo/) - Cargo guide
- [Tauri Docs](https://tauri.app/) - GUI framework
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/) - Style guide

### Community

- GitHub Issues - Bug reports and features
- GitHub Discussions - Questions and ideas
- Pull Requests - Code contributions

---

## Recognition

### Contributors

All contributors are recognized in:

- README.md contributors section
- GitHub contributors page
- Release notes

### Types of Contributions

We value all contributions equally:

- 💻 Code
- 📖 Documentation
- 🐛 Bug reports
- 💡 Feature ideas
- ❓ Helping others
- 🎨 Design
- 🧪 Testing

---

## Questions?

- 📖 Read the [FAQ](FAQ.md)
- 💬 Open a GitHub Discussion
- 📧 Contact maintainers
- 🐛 Report issues on GitHub

---

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

---

## Wiki Navigation

- [📚 Back to Wiki Home](README.md)
- [💻 Development Guide](DEVELOPMENT.md)
- [🧪 Testing Guide](TESTING.md)
- [🏗️ Architecture](ARCHITECTURE.md)
- [📖 Code Style](CODE_STYLE.md)

---

**Thank you for contributing to Prism! 🚀**

---

[⬆ Back to Top](#contributing-to-prism) | [📚 Wiki Home](README.md)
