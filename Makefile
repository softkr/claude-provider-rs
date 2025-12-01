# Rust Claude Switch Makefile
# Based on the original Go Makefile structure

.PHONY: build build-all clean install test release help

# Variables
BINARY_NAME = claude-switch
VERSION = $(shell grep '^version = ' Cargo.toml | cut -d '"' -f 2)
BUILD_DIR = build
RELEASE_DIR = $(BUILD_DIR)/release

# Default target
help: ## Show this help message
	@echo 'Rust Claude Switch - Build System'
	@echo ''
	@echo 'Usage: make [target]'
	@echo ''
	@echo 'Targets:'
	@awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z_-]+:.*?## / {printf "  %-15s %s\n", $$1, $$2}' $(MAKEFILE_LIST)

# Build for current platform
build: ## Build for current platform
	@echo "Building $(BINARY_NAME) v$(VERSION) for $(shell uname -s)-$(shell uname -m)..."
	cargo build --release
	@mkdir -p $(BUILD_DIR)
	@cp target/release/$(BINARY_NAME) $(BUILD_DIR)/
	@echo "✅ Build complete: $(BUILD_DIR)/$(BINARY_NAME)"

# Build for all platforms
build-all: ## Build for all platforms
	@echo "Building $(BINARY_NAME) v$(VERSION) for all platforms..."
	@mkdir -p $(RELEASE_DIR)

# macOS ARM64 (Apple Silicon)
	@echo "Building for macOS ARM64..."
	@cargo build --release --target aarch64-apple-darwin
	@cp target/aarch64-apple-darwin/release/$(BINARY_NAME) $(RELEASE_DIR)/$(BINARY_NAME)-darwin-arm64

# macOS AMD64 (Intel)
	@echo "Building for macOS AMD64..."
	@cargo build --release --target x86_64-apple-darwin
	@cp target/x86_64-apple-darwin/release/$(BINARY_NAME) $(RELEASE_DIR)/$(BINARY_NAME)-darwin-amd64

# Linux AMD64
	@echo "Building for Linux AMD64..."
	@cargo build --release --target x86_64-unknown-linux-gnu
	@cp target/x86_64-unknown-linux-gnu/release/$(BINARY_NAME) $(RELEASE_DIR)/$(BINARY_NAME)-linux-amd64

# Linux ARM64
	@echo "Building for Linux ARM64..."
	@cargo build --release --target aarch64-unknown-linux-gnu
	@cp target/aarch64-unknown-linux-gnu/release/$(BINARY_NAME) $(RELEASE_DIR)/$(BINARY_NAME)-linux-arm64

# Windows AMD64
	@echo "Building for Windows AMD64..."
	@cargo build --release --target x86_64-pc-windows-gnu
	@cp target/x86_64-pc-windows-gnu/release/$(BINARY_NAME).exe $(RELEASE_DIR)/$(BINARY_NAME)-windows-amd64.exe

	@echo "✅ All builds complete in $(RELEASE_DIR)/"
	@ls -la $(RELEASE_DIR)/

# Install for current platform
install: ## Install binary to /usr/local/bin
	@echo "Installing $(BINARY_NAME) v$(VERSION)..."
	@if [ ! -f target/release/$(BINARY_NAME) ]; then \
		echo "Binary not found. Running 'make build' first..."; \
		$(MAKE) build; \
	fi
	@sudo cp target/release/$(BINARY_NAME) /usr/local/bin/
	@sudo chmod +x /usr/local/bin/$(BINARY_NAME)
	@echo "✅ Installed to /usr/local/bin/$(BINARY_NAME)"

# Clean build artifacts
clean: ## Clean build artifacts
	@echo "Cleaning build artifacts..."
	@cargo clean
	@rm -rf $(BUILD_DIR)
	@echo "✅ Clean complete"

# Run tests
test: ## Run tests
	@echo "Running tests..."
	@cargo test
	@echo "✅ Tests complete"

# Run tests with coverage
test-coverage: ## Run tests with coverage
	@echo "Running tests with coverage..."
	@cargo tarpaulin --out Html --output-dir $(BUILD_DIR)/coverage
	@echo "✅ Coverage report generated in $(BUILD_DIR)/coverage/"

# Check code formatting
fmt: ## Check code formatting
	@echo "Checking code formatting..."
	@cargo fmt --check
	@echo "✅ Code formatting check complete"

# Format code
fmt-fix: ## Format code
	@echo "Formatting code..."
	@cargo fmt
	@echo "✅ Code formatted"

# Run lints
lint: ## Run clippy lints
	@echo "Running clippy lints..."
	@cargo clippy -- -D warnings
	@echo "✅ Linting complete"

# Security audit
audit: ## Run security audit
	@echo "Running security audit..."
	@cargo audit
	@echo "✅ Security audit complete"

# Check for outdated dependencies
outdated: ## Check for outdated dependencies
	@echo "Checking for outdated dependencies..."
	@cargo outdated
	@echo "✅ Dependency check complete"

# Update dependencies
update: ## Update dependencies
	@echo "Updating dependencies..."
	@cargo update
	@echo "✅ Dependencies updated"

# Development build (debug mode)
dev: ## Build in debug mode for development
	@echo "Building $(BINARY_NAME) v$(VERSION) in debug mode..."
	@cargo build
	@echo "✅ Debug build complete: target/debug/$(BINARY_NAME)"

# Run in development mode
run-dev: ## Run in development mode
	@echo "Running $(BINARY_NAME) in development mode..."
	@cargo run -- --help

# Create release packages
release: build-all ## Create release packages
	@echo "Creating release packages..."
	@mkdir -p $(RELEASE_DIR)/packages

# Create tar.gz packages for Unix-like systems
	@for target in darwin-arm64 darwin-amd64 linux-amd64 linux-arm64; do \
		cd $(RELEASE_DIR) && \
		tar -czf packages/$(BINARY_NAME)-$$target.tar.gz $(BINARY_NAME)-$$target && \
		cd -; \
		echo "✅ Created packages/$(BINARY_NAME)-$$target.tar.gz"; \
	done

# Create zip for Windows
	@cd $(RELEASE_DIR) && \
	zip -q packages/$(BINARY_NAME)-windows-amd64.zip $(BINARY_NAME)-windows-amd64.exe && \
	cd - && \
	echo "✅ Created packages/$(BINARY_NAME)-windows-amd64.zip"

	@echo "✅ All release packages created in $(RELEASE_DIR)/packages/"
	@ls -la $(RELEASE_DIR)/packages/

# Install development dependencies
setup-dev: ## Install development dependencies
	@echo "Installing development dependencies..."
	@rustup component add clippy rustfmt
	@cargo install cargo-audit cargo-outdated cargo-tarpaulin
	@echo "✅ Development dependencies installed"

# Show version
version: ## Show version information
	@echo "Version: $(VERSION)"
	@echo "Rust version: $(shell rustc --version)"
	@echo "Cargo version: $(shell cargo --version)"

# Generate documentation
docs: ## Generate documentation
	@echo "Generating documentation..."
	@cargo doc --no-deps --open
	@echo "✅ Documentation generated"

# Continuous integration target
ci: fmt lint test audit ## Run all CI checks
	@echo "✅ All CI checks passed"