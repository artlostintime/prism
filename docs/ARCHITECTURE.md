# Prism Architecture

**[📚 Wiki Home](README.md)** | **[💻 Development](DEVELOPMENT.md)** | **[🔍 API Reference](API_REFERENCE.md)** | **[🧪 Testing](TESTING.md)**

---

## Minimal GUI Wrapper Design

This project uses a **minimal wrapper architecture** to avoid code duplication:

```
┌─────────────────────────────────────────┐
│         Tauri GUI (src-tauri/)          │
│                                         │
│  ┌────────────────────────────────┐    │
│  │  Simple HTML/JS Interface      │    │
│  │  - File picker dialog          │    │
│  │  - Status display              │    │
│  └────────────────────────────────┘    │
│                 │                       │
│                 ▼                       │
│  ┌────────────────────────────────┐    │
│  │   Rust Wrapper (lib.rs)        │    │
│  │   - pick_file()                │    │
│  │   - run_analysis()             │    │
│  └────────────────────────────────┘    │
│                 │                       │
└─────────────────┼───────────────────────┘
                  │ Calls via Command::new()
                  ▼
┌─────────────────────────────────────────┐
│         CLI Binary (src/)               │
│                                         │
│  - Config parsing (config.rs)          │
│  - CSV processing                      │
│  - Reverse scoring logic               │
│  - Scale calculations                  │
│  - Quality checks                      │
│  - Output generation                   │
└─────────────────────────────────────────┘
```

## Key Benefits

### ✅ Zero Code Duplication

- **One implementation** of all business logic (in CLI)
- GUI just launches the CLI binary
- Changes automatically affect both interfaces

### ✅ Testability

- CLI can be tested independently
- GUI is just a thin UI layer
- Business logic isolated from UI

### ✅ Maintainability

- Update scoring logic once
- Fix bugs in one place
- Easier to extend features

### ✅ Performance

- GUI doesn't reimplement parsing/processing
- Native CLI performance
- Small Tauri wrapper (~3MB overhead)

## How It Works

### GUI Flow

1. User clicks "Select CSV File"
2. `pick_file()` shows native file dialog
3. Returns file path to UI
4. UI calls `run_analysis(file_path)`
5. Wrapper finds CLI binary (`prism.exe` or `prism`)
6. Executes: `prism --input <path> --config study_config.toml`
7. Captures stdout/stderr
8. Parses output for participant count
9. Returns formatted success/error message
10. UI displays result

### CLI Flow (Standalone)

```bash
prism --input data.csv --config survey.toml --output clean.csv
```

Direct execution, no wrapper involved.

## File Structure

```
prism/
├── src/                    # CLI implementation
│   ├── main.rs            # Entry point, CSV processing
│   └── config.rs          # TOML config structures
├── src-tauri/             # GUI wrapper
│   ├── src/
│   │   ├── lib.rs        # Command wrappers (80 lines)
│   │   └── main.rs       # Tauri entry point
│   └── Cargo.toml        # Minimal deps: tauri, rfd
├── ui/
│   └── index.html         # Simple interface (120 lines)
└── target/
    ├── release/
    │   └── prism.exe     # ← CLI binary the GUI calls
    └── debug/
```

## Dependencies

### CLI (`Cargo.toml`)

- `csv` - CSV reading/writing
- `serde` + `toml` - Config parsing
- `clap` - Command-line arguments
- `anyhow` - Error handling

### GUI (`src-tauri/Cargo.toml`)

- `tauri` - Desktop app framework
- `rfd` - Native file dialogs

**Total GUI-specific code:** ~200 lines

## Why Not Shared Library?

We could extract logic to a shared lib:

```
prism/
├── prism-core/     # Shared library
├── prism-cli/      # CLI wrapper
└── prism-gui/      # GUI wrapper
```

**But:**

- Adds complexity (3 crates instead of 2)
- CLI is already a perfect "library" with stdin/stdout
- Shell integration works better with binary
- Simplicity is valuable for a small project

## Building

### CLI

```bash
cargo build --release
```

Output: `target/release/prism.exe`

### GUI

```bash
cd src-tauri
cargo tauri build
```

Output: `target/release/bundle/...`

**Important:** Build CLI first! GUI needs the binary to exist.

## Adding Features

To add a new feature (e.g., pattern detection):

1. ✅ Implement in `src/main.rs` (CLI)
2. ✅ Test with: `prism --input test.csv ...`
3. ✅ GUI automatically gets it (no changes needed)

That's it! No dual implementation required.

## Trade-offs

### ✅ Pros

- Simple to understand
- Easy to maintain
- No code duplication
- Fast to extend

### ⚠️ Cons

- GUI can't show real-time progress (CLI runs as black box)
- Slightly slower startup (process spawn overhead)
- Can't customize GUI output format independently

For this project, the pros far outweigh the cons.
