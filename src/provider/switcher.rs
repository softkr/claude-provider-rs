use crate::config::{Config, Provider};
use crate::provider::detector::ProviderDetector;
use crate::config::manager::ConfigManager;
use anyhow::Result;
use colored::*;

pub struct StatusDisplay {
    config_manager: ConfigManager,
}

impl StatusDisplay {
    pub fn new(config_manager: ConfigManager) -> Self {
        Self { config_manager }
    }

    pub fn show_status(&self) -> Result<()> {
        println!("{}", "📊 Current Configuration Status".cyan());
        println!();

        let config = self.config_manager.load_current_config()?;

        if config.env.is_empty() {
            println!("{}", "⚠️  No configuration found (empty or missing)".yellow());
            return Ok(());
        }

        let base_url = config.env.get("ANTHROPIC_BASE_URL").cloned().unwrap_or_default();
        let provider = ProviderDetector::detect_provider(&config);

        match provider {
            Provider::ZAI => self.show_zai_status(&config, &base_url),
            Provider::Anthropic => self.show_anthropic_status(&config),
            Provider::Custom => self.show_custom_status(&config, &base_url),
            Provider::Unknown => self.show_unknown_status(),
        }

        println!();

        // Show other environment variables
        self.show_other_env_vars(&config);

        // Show backup status
        self.show_backup_status()?;

        // Show saved token status
        self.show_saved_token_status()?;

        Ok(())
    }

    fn show_zai_status(&self, config: &Config, base_url: &str) {
        println!("{}", "┌─────────────────────────────────────┐".green());
        println!("{}", "│  🔗 Provider: Z.AI (GLM Models)     │".green());
        println!("{}", "└─────────────────────────────────────┘".green());
        println!();
        println!("  {}{}", "Base URL: ".cyan(), base_url);

        if let Some(model) = config.env.get("ANTHROPIC_DEFAULT_SONNET_MODEL") {
            println!("  {}{}", "Sonnet Model: ".cyan(), model);
        }
        if let Some(model) = config.env.get("ANTHROPIC_DEFAULT_OPUS_MODEL") {
            println!("  {}{}", "Opus Model: ".cyan(), model);
        }
        if let Some(model) = config.env.get("ANTHROPIC_DEFAULT_HAIKU_MODEL") {
            println!("  {}{}", "Haiku Model: ".cyan(), model);
        }
        if let Some(timeout) = config.env.get("API_TIMEOUT_MS") {
            println!("  {}{} {}", "Timeout: ".cyan(), timeout, "ms".cyan());
        }

        // Show masked token with type detection
        if let Some(token) = config.env.get("ANTHROPIC_AUTH_TOKEN") {
            let masked_token = ProviderDetector::mask_token(token);
            let token_type = ProviderDetector::detect_token_type(token);
            let token_type_str = match token_type {
                crate::config::TokenType::ZAI => " (API key)",
                crate::config::TokenType::Anthropic => " (web token - unexpected for Z.AI)",
                crate::config::TokenType::Unknown => "",
            };
            println!("  {}{}{}", "Auth Token: ".cyan(), masked_token, token_type_str);
        }
    }

    fn show_anthropic_status(&self, config: &Config) {
        println!("{}", "┌─────────────────────────────────────┐".green());
        println!("{}", "│  🔗 Provider: Anthropic (Default)   │".green());
        println!("{}", "└─────────────────────────────────────┘".green());
        println!();
        println!("{}", "  Base URL: api.anthropic.com (default)".cyan());
    }

    fn show_custom_status(&self, config: &Config, base_url: &str) {
        println!("{}", "┌─────────────────────────────────────┐".green());
        println!("{}", "│  🔗 Provider: Custom                │".green());
        println!("{}", "└─────────────────────────────────────┘".green());
        println!();
        println!("  {}{}", "Base URL: ".cyan(), base_url);
    }

    fn show_unknown_status(&self) {
        println!("{}", "⚠️  Unknown provider configuration".yellow());
    }

    fn show_other_env_vars(&self, config: &Config) {
        let other_env_count = config.env
            .keys()
            .filter(|key| !ProviderDetector::is_zai_key(key) && *key != "ANTHROPIC_BASE_URL")
            .count();

        if other_env_count > 0 {
            println!("  {}{}", "Other env vars: ".cyan(), other_env_count);
        }
    }

    fn show_backup_status(&self) -> Result<()> {
        let (has_backup, backup) = self.config_manager.has_valid_anthropic_backup()?;

        if has_backup && backup.is_some() {
            let backup = backup.unwrap();
            println!("  {}", "💾 Backup: Available (Anthropic)".cyan());
            if let Some(created_at) = backup.metadata.created_at {
                println!("     {}{}", "Created: ".cyan(), created_at.format("%Y-%m-%d %H:%M:%S UTC"));
            }
            // Show token type in backup
            if let Some(token) = backup.env.get("ANTHROPIC_AUTH_TOKEN") {
                let token_type = ProviderDetector::detect_token_type(token);
                match token_type {
                    crate::config::TokenType::Anthropic => {
                        println!("     {}", "Token: Web login token".cyan());
                    }
                    crate::config::TokenType::ZAI => {
                        println!("     {}", "Token: API key (unexpected)".yellow());
                    }
                    _ => {}
                }
            }
        } else if self.config_manager.backup_file().exists() {
            println!("  {}", "💾 Backup: Available (unknown format)".yellow());
        } else {
            println!("  {}", "💾 Backup: Not found".yellow());
        }

        Ok(())
    }

    fn show_saved_token_status(&self) -> Result<()> {
        if let Ok(Some(_)) = self.config_manager.load_saved_token() {
            println!("  {}", "🔑 Saved Token: Available".cyan());
        }
        Ok(())
    }
}