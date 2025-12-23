#!/bin/bash

# Claude Code API Switcher - Install Script
# This script downloads, compiles, and installs claude-switch

set -e

echo "🚀 Claude Code API Switcher - Installation"
echo "=========================================="
echo ""

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Rust/Cargo is not installed!"
    echo "Please install Rust first:"
    echo "  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi

echo "✅ Rust/Cargo found"
echo ""

# Clone or update repository
REPO_DIR="$HOME/claude-provider"
REPO_URL="https://github.com/softkr/claude-provider-rs.git"

if [ -d "$REPO_DIR" ]; then
    echo "📁 Repository exists, updating..."
    cd "$REPO_DIR"
    git pull
else
    echo "📁 Cloning repository..."
    git clone "$REPO_URL" "$REPO_DIR"
    cd "$REPO_DIR"
fi

echo ""
echo "🔨 Building release version..."
cargo build --release

echo ""
echo "📦 Installing to /usr/local/bin..."

# Check if sudo is needed
if [ -w /usr/local/bin ]; then
    cp target/release/claude-switch /usr/local/bin/claude-switch
    chmod +x /usr/local/bin/claude-switch
else
    sudo cp target/release/claude-switch /usr/local/bin/claude-switch
    sudo chmod +x /usr/local/bin/claude-switch
fi

echo ""
echo "🔧 Setting up shell aliases..."

# Detect shell and add aliases
add_aliases() {
    local shell_config="$1"
    local alias_block="
# Claude Code API Switcher
alias claude-switch='/usr/local/bin/claude-switch'
alias claude-anthropic='/usr/local/bin/claude-switch anthropic'
alias claude-glm='/usr/local/bin/claude-switch glm'
alias claude-status='/usr/local/bin/claude-switch status'
"

    # Check if aliases already exist
    if grep -q "Claude Code API Switcher" "$shell_config" 2>/dev/null; then
        echo "⚠️  Aliases already exist in $shell_config"
        echo "   Removing old aliases..."
        # Remove old alias block
        sed -i.tmp '/# Claude Code API Switcher/,/^$/d' "$shell_config" 2>/dev/null || true
        rm -f "${shell_config}.tmp"
    fi

    # Add new aliases
    echo "$alias_block" >> "$shell_config"
    echo "✅ Aliases added to $shell_config"
}

# Add aliases to shell configs
if [ -n "$ZSH_VERSION" ] || [ -f "$HOME/.zshrc" ]; then
    add_aliases "$HOME/.zshrc"
    SHELL_CONFIG="$HOME/.zshrc"
elif [ -n "$BASH_VERSION" ] || [ -f "$HOME/.bashrc" ]; then
    add_aliases "$HOME/.bashrc"
    SHELL_CONFIG="$HOME/.bashrc"
elif [ -f "$HOME/.config/fish/config.fish" ]; then
    # Fish shell uses different syntax
    FISH_ALIAS="
# Claude Code API Switcher
alias claude-switch '/usr/local/bin/claude-switch'
alias claude-anthropic '/usr/local/bin/claude-switch anthropic'
alias claude-glm '/usr/local/bin/claude-switch glm'
alias claude-status '/usr/local/bin/claude-switch status'
"
    if grep -q "Claude Code API Switcher" "$HOME/.config/fish/config.fish" 2>/dev/null; then
        echo "⚠️  Aliases already exist in fish config"
    else
        echo "$FISH_ALIAS" >> "$HOME/.config/fish/config.fish"
        echo "✅ Aliases added to fish config"
        SHELL_CONFIG="$HOME/.config/fish/config.fish"
    fi
fi

echo ""
echo "🎉 Installation complete!"
echo ""
echo "Available commands:"
echo "  claude-switch      - Show usage"
echo "  claude-anthropic   - Switch to Anthropic API"
echo "  claude-glm         - Switch to GLM API"
echo "  claude-status      - Check current configuration"
echo ""
echo "📝 Reload your shell to apply aliases:"
if [ -n "$SHELL_CONFIG" ]; then
    echo "  source $SHELL_CONFIG"
fi
echo ""
echo "Or start a new terminal session."
