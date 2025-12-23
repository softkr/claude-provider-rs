use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::*;

mod config;
mod provider;
mod utils;

use config::ConfigManager;
use provider::{AnthropicSwitcher, GLMSwitcher, StatusDisplay};
use utils::{Installer, TokenManager};

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Parser)]
#[command(name = "claude-switch")]
#[command(
    about = "Claude Code API Switcher - Switch between different API providers for Claude Code"
)]
#[command(version = VERSION)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Switch to Anthropic API (restore configuration)
    #[command(alias = "a")]
    Anthropic,
    /// Switch to GLM API (use API key)
    #[command(alias = "g")]
    GLM,
    /// Show current configuration
    #[command(alias = "s")]
    Status,
    /// Remove saved GLM API token
    ClearToken,
    /// Install aliases to shell
    Install,
}

fn print_header() {
    println!("{}{}", "🤖 Claude Code API Switcher v".cyan(), VERSION);
    println!();
}

fn print_usage() {
    print_header();
    println!("{}", "Usage:".cyan());
    println!();
    println!("  claude-switch [command]");
    println!();
    println!("{}", "Commands:".cyan());
    println!("  -a, --anthropic  Switch to Anthropic API (restore configuration)");
    println!("  -g, --glm        Switch to GLM API (use API key)");
    println!("  -s, --status     Show current configuration");
    println!("  --clear-token    Remove saved GLM API token");
    println!("  --install        Install aliases to shell");
    println!("  -v, --version    Show version");
    println!("  -h, --help       Show this help message");
    println!();
    println!("{}", "Authentication:".cyan());
    println!("  Anthropic  Uses default configuration (automatically backed up)");
    println!("  GLM        Uses API key (prompted or from GLM_AUTH_TOKEN env)");
    println!();
    println!("{}", "Environment Variables:".cyan());
    println!("  GLM_AUTH_TOKEN  GLM API key (optional)");
    println!();
    println!("{}", "Examples:".cyan());
    println!("  claude-switch --glm        # Backup Anthropic config, switch to GLM");
    println!("  claude-switch --anthropic  # Restore Anthropic config from backup");
    println!("  claude-switch --status     # Check current provider");
    println!();
    println!(
        "{}",
        "Note: Switching to GLM automatically backs up your Anthropic".yellow()
    );
    println!(
        "      {}",
        "configuration. Use --anthropic to restore it later.".yellow()
    );
    println!();
}

fn main() -> Result<()> {
    // Parse command line arguments using clap for better compatibility
    let cli = Cli::parse();

    // Initialize config manager
    let config_manager = match ConfigManager::new() {
        Ok(cm) => cm,
        Err(e) => {
            eprintln!("{}{}", "Error: ".red(), e);
            std::process::exit(1);
        }
    };

    // Handle the command
    match cli.command {
        Some(Commands::Anthropic) => {
            let switcher = AnthropicSwitcher::new(config_manager);
            if let Err(e) = switcher.switch_to_anthropic() {
                eprintln!("{}{}", "Error: ".red(), e);
                std::process::exit(1);
            }
        }
        Some(Commands::GLM) => {
            let switcher = GLMSwitcher::new(config_manager);
            if let Err(e) = switcher.switch_to_glm() {
                eprintln!("{}{}", "Error: ".red(), e);
                std::process::exit(1);
            }
        }
        Some(Commands::Status) => {
            let display = StatusDisplay::new(config_manager);
            if let Err(e) = display.show_status() {
                eprintln!("{}{}", "Error: ".red(), e);
                std::process::exit(1);
            }
        }
        Some(Commands::ClearToken) => {
            if let Err(e) = TokenManager::clear_saved_token(&config_manager) {
                eprintln!("{}{}", "Error: ".red(), e);
                std::process::exit(1);
            }
        }
        Some(Commands::Install) => {
            let installer = Installer::new()?;
            if let Err(e) = installer.install() {
                eprintln!("{}{}", "Error: ".red(), e);
                std::process::exit(1);
            }
        }
        None => {
            // No command provided, show usage
            print_usage();
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_constant() {
        assert_eq!(VERSION, env!("CARGO_PKG_VERSION"));
    }
}
