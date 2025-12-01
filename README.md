# Claude Switch (Rust Version)

A Rust implementation of the Claude Code API Switcher - Switch between different API providers for Claude Code.

## Features

- **API Provider Switching**: Seamlessly switch between Anthropic and Z.AI providers
- **Configuration Management**: Manage Claude Code configuration files automatically
- **Backup/Restore System**: Automatically backs up and restores Anthropic web login tokens
- **Token Management**: Secure token handling with multiple storage options
- **Cross-platform**: Supports macOS, Linux, and Windows
- **User-friendly CLI**: Colored output and helpful status displays

## Installation

### Build from Source

```bash
# Clone the repository
git clone <repository-url>
cd rust-version

# Build for current platform
cargo build --release

# Or use the Makefile
make build

# Install to system
make install
```

### Build for All Platforms

```bash
make build-all
```

This will create binaries for:
- macOS (AMD64/ARM64)
- Linux (AMD64/ARM64)
- Windows (AMD64)

## Usage

```bash
# Show current configuration
claude-switch status

# Switch to Z.AI API (backs up Anthropic token automatically)
claude-switch zai

# Switch back to Anthropic API (restores from backup)
claude-switch anthropic

# Install shell aliases for easier use
claude-switch install

# Remove saved token
claude-switch clear-token

# Show help
claude-switch --help
```

## Shell Aliases

When you run `claude-switch install`, it adds these aliases to your shell:

```bash
alias claude-switch='claude-switch'
alias claude-anthropic='claude-switch --anthropic'
alias claude-z_ai='claude-switch --z_ai'
alias claude-status='claude-switch --status'
```

## Architecture

This Rust implementation is organized into several modules:

- **`config/`**: Configuration management and data structures
- **`provider/`**: API provider detection and switching logic
- **`utils/`**: Utility functions for token management and installation

### Key Improvements over Go Version

1. **Type Safety**: Rust's type system prevents many runtime errors
2. **Memory Safety**: No null pointer exceptions or buffer overflows
3. **Better Error Handling**: Comprehensive error handling with `anyhow` and `thiserror`
4. **Modular Design**: Clean separation of concerns across modules
5. **Modern CLI**: Uses `clap` for command-line argument parsing
6. **Enhanced Testing**: Better support for unit and integration tests

## Development

### Prerequisites

- Rust 1.70+ (edition 2021)
- Cargo

### Development Commands

```bash
# Check code
cargo check

# Run tests
cargo test

# Format code
cargo fmt

# Run lints
cargo clippy

# Build in debug mode
cargo build

# Run in development
cargo run -- --help
```

### Makefile Targets

```bash
# Show all available targets
make help

# Development build
make dev

# Run all CI checks
make ci

# Create release packages
make release
```

## Configuration Files

The application manages these files:

- `~/.claude/settings.json` - Main Claude Code configuration
- `~/.claude/settings.json.backup` - Backup of Anthropic configuration
- `~/.claude/.z_ai_token` - Saved Z.AI API token

## Authentication

### Anthropic
- Uses web login tokens
- Automatically backed up when switching to Z.AI
- Restored from backup when switching back

### Z.AI
- Uses API keys (format: `sk-xxx` or `z_ai-xxx`)
- Can be provided via:
  - Environment variable: `Z_AI_AUTH_TOKEN`
  - Saved token file
  - Interactive prompt

## License

MIT License - see LICENSE file for details.

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run tests and lints (`make ci`)
5. Submit a pull request

## Security

- Tokens are stored with restrictive permissions (600 on Unix)
- Automatic atomic file operations prevent corruption
- Token masking in status displays