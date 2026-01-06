# Building the Documentation Site

Prism uses [mdBook](https://rust-lang.github.io/mdBook/) for its documentation site.

## Prerequisites

Install mdBook:

```bash
cargo install mdbook
```

## Build the Docs

```bash
# Build the book
mdbook build

# Serve locally with live reload
mdbook serve

# Open in browser
mdbook serve --open
```

The documentation will be available at `http://localhost:3000`.

## Directory Structure

```
prism/
├── book.toml          # mdBook configuration
├── docs/              # Documentation source
│   ├── SUMMARY.md     # Table of contents
│   ├── README.md      # Introduction
│   ├── *.md           # All other docs
│   └── images/        # Images and diagrams
└── book/              # Generated output (gitignored)
    └── html/          # Static HTML site
```

## Publishing

### GitHub Pages

The documentation can be published to GitHub Pages:

```bash
# Build the book
mdbook build

# Deploy to gh-pages branch
git worktree add /tmp/book gh-pages
cp -r book/* /tmp/book/
cd /tmp/book
git add -A
git commit -m "Update documentation"
git push origin gh-pages
cd -
git worktree remove /tmp/book
```

### Automated with GitHub Actions

Add to `.github/workflows/docs.yml`:

```yaml
name: Deploy Documentation

on:
  push:
    branches: [main]
    paths:
      - "docs/**"
      - "book.toml"

jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Install mdBook
        run: |
          cargo install mdbook

      - name: Build book
        run: mdbook build

      - name: Deploy to GitHub Pages
        uses: peaceiris/actions-gh-pages@v3
        with:
          github_token: ${{ secrets.GITHUB_TOKEN }}
          publish_dir: ./book
```

## Customization

### Theme

Customize colors and styling in `book.toml`:

```toml
[output.html]
default-theme = "light"
preferred-dark-theme = "navy"
```

### Additional Plugins

Install useful mdBook plugins:

```bash
# Mermaid diagrams
cargo install mdbook-mermaid
mdbook-mermaid install

# Link checker
cargo install mdbook-linkcheck

# PDF output
cargo install mdbook-pdf
```

Add to `book.toml`:

```toml
[preprocessor.mermaid]

[output.linkcheck]

[output.pdf]
```

## Writing Documentation

### Internal Links

Use relative paths:

```markdown
See the [Configuration Guide](CONFIGURATION_GUIDE.md) for details.
```

### Code Examples

Use fenced code blocks with language:

```markdown
\`\`\`toml
[scales.PHQ9]
items = ["PHQ1", "PHQ2"]
\`\`\`
```

### Admonitions

Use quote blocks for notes:

```markdown
> **Note**: This requires version 0.8.0 or later.

> **Warning**: This will overwrite existing files.
```

### Diagrams

Use Mermaid for diagrams:

```markdown
\`\`\`mermaid
graph LR
A[Raw Data] --> B[Prism Process]
B --> C[Clean Data]
B --> D[Statistics]
B --> E[Quality Report]
\`\`\`
```

## Testing

Check for broken links:

```bash
mdbook test
```

## See Also

- [mdBook Documentation](https://rust-lang.github.io/mdBook/)
- [mdBook Guide](https://rust-lang.github.io/mdBook/guide/creating.html)
