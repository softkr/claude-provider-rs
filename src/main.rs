use clap::{Parser, Subcommand};
use anyhow::Result;
use colored::*;

mod config;
mod provider;
mod utils;

use config::{ConfigManager, Provider};
use provider::{AnthropicSwitcher, ZAISwitcher, StatusDisplay};
use utils::{TokenManager, Installer};

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Parser)]
#[command(name = "claude-switch")]
#[command(about = "Claude Code API Switcher - Switch between different API providers for Claude Code")]
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
    /// Switch to Z.AI API (use API key)
    #[command(alias = "z")]
    ZAI,
    /// Show current configuration
    #[command(alias = "s")]
    Status,
    /// Remove saved Z_AI API token
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
    println!("  -z, --z_ai       Switch to Z.AI API (use API key)");
    println!("  -s, --status     Show current configuration");
    println!("  --clear-token    Remove saved Z_AI API token");
    println!("  --install        Install aliases to shell");
    println!("  -v, --version    Show version");
    println!("  -h, --help       Show this help message");
    println!();
    println!("{}", "Authentication:".cyan());
    println!("  Anthropic  Uses default configuration (automatically backed up)");
    println!("  Z.AI       Uses API key (prompted or from Z_AI_AUTH_TOKEN env)");
    println!();
    println!("{}", "Environment Variables:".cyan());
    println!("  Z_AI_AUTH_TOKEN  Z.AI API key (optional)");
    println!();
    println!("{}", "Examples:".cyan());
    println!("  claude-switch --z_ai       # Backup Anthropic config, switch to Z.AI");
    println!("  claude-switch --anthropic  # Restore Anthropic config from backup");
    println!("  claude-switch --status     # Check current provider");
    println!();
    println!("{}", "Note: Switching to Z.AI automatically backs up your Anthropic".yellow());
    println!("      {}", "configuration. Use --anthropic to restore it later.".yellow());
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
        Some(Commands::ZAI) => {
            let switcher = ZAISwitcher::new(config_manager);
            if let Err(e) = switcher.switch_to_zai() {
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