# 📝 Rust Todo CLI

A powerful command-line todo application built with Rust, featuring multiple time horizons and project contexts.

## ✨ Features

- **📅 Time Horizons**: Organize tasks by short-term (daily), mid-term (monthly), and long-term (yearly) goals
- **🗂️ Multiple Contexts**: Manage separate task lists for different projects (like git branches)
- **🎨 Beautiful CLI**: Colored output with clear visual distinction between task states
- **💾 Persistent Storage**: JSON-based storage with atomic file operations
- **📤 Import/Export**: Share task lists or create backups easily
- **🔍 Smart ID Matching**: Use partial IDs for quick task operations
- **⚡ Fast & Reliable**: Built with Rust for performance and safety

## 🚀 Installation

### From Source

```bash
# Clone the repository
git clone https://github.com/Nylio-prog/rust-todo.git
cd rust-todo

# Build the release binary
cargo build --release

# The binary will be at target/release/todo.exe (Windows) or target/release/todo (Unix)
```

### Add to PATH (Windows)

```powershell
# Add the release directory to your PATH
$env:Path += ";C:\path\to\rust-todo\target\release"
```

## 📖 Usage

### Basic Commands

```bash
# Add a task
todo add "Write documentation" --horizon short --priority high

# List all tasks
todo list

# List tasks from a specific time horizon
todo list --horizon short

# Complete a task (using partial ID)
todo complete abc123

# Edit a task
todo edit abc123 --description "Updated description" --priority medium

# Delete a task
todo delete abc123
```

### Context Management

```bash
# Create a new context
todo context new work

# Switch to a context
todo context switch work

# List all contexts
todo context list

# Delete a context
todo context delete work
```

### Import/Export

```bash
# Export tasks to a file
todo export backup.json

# Import tasks (replace existing)
todo import backup.json

# Import and merge with existing tasks
todo import backup.json --merge
```

## 🎯 Time Horizons

- **Short-term**: Daily tasks and immediate goals
- **Mid-term**: Monthly objectives and ongoing projects
- **Long-term**: Yearly goals and long-term aspirations

## 🎨 Priority Levels

- **High**: Urgent and important tasks
- **Medium**: Regular priority tasks (default)
- **Low**: Nice-to-have tasks

## 📂 Data Storage

Tasks are stored in a platform-specific location:
- **Windows**: `%APPDATA%\rust-todo\data.json`
- **Linux**: `~/.local/share/rust-todo/data.json`
- **macOS**: `~/Library/Application Support/rust-todo/data.json`

## 🛠️ Development

### Prerequisites

- Rust 1.70 or higher
- Cargo

### Building

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release

# Run tests
cargo test

# Run clippy (linter)
cargo clippy

# Format code
cargo fmt
```

### Project Structure

```
rust-todo/
├── src/
│   ├── main.rs       # Entry point and command handlers
│   ├── lib.rs        # Library exports
│   ├── cli.rs        # CLI argument parsing
│   ├── task.rs       # Task data structures
│   ├── context.rs    # Context management
│   ├── store.rs      # Storage and persistence
│   ├── display.rs    # Output formatting
│   └── error.rs      # Error types
├── Cargo.toml        # Dependencies and metadata
└── README.md         # This file
```

## 🧪 Testing

The project includes comprehensive unit tests and documentation tests:

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run only unit tests
cargo test --lib

# Run only doc tests
cargo test --doc
```

## 📚 Dependencies

- **clap**: Command-line argument parsing
- **serde**: Serialization framework
- **serde_json**: JSON support
- **colored**: Terminal colors
- **chrono**: Date and time handling
- **uuid**: Unique ID generation
- **directories**: Cross-platform paths
- **thiserror**: Error handling

## 🤝 Contributing

Contributions are welcome! Feel free to:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📝 License

This project is open source and available under the MIT License.

## 🙏 Acknowledgments

Built as a learning project to explore Rust's capabilities in building CLI applications with:
- Strong type safety
- Memory safety without garbage collection
- Excellent error handling
- Cross-platform compatibility
- Educational comments throughout the codebase

## 📧 Contact

Project Link: [https://github.com/Nylio-prog/rust-todo](https://github.com/Nylio-prog/rust-todo)

---

Made with ❤️ and Rust 🦀
