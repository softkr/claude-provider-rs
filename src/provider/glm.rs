use crate::config::manager::ConfigManager;
use crate::config::{Config, Provider};
use crate::provider::detector::ProviderDetector;
use crate::utils::token::TokenManager;
use anyhow::{Context, Result};
use colored::*;

pub struct GLMSwitcher {
    config_manager: ConfigManager,
    token_manager: TokenManager,
}

impl GLMSwitcher {
    pub fn new(config_manager: ConfigManager) -> Self {
        Self {
            config_manager,
            token_manager: TokenManager::new(),
        }
    }

    pub fn switch_to_glm(&self) -> Result<()> {
        println!("{}", "🔄 Switching to GLM API...".green());

        // Load current config
        let config = self
            .config_manager
            .load_current_config()
            .context("Failed to load current config")?;

        // Check if already using GLM
        if ProviderDetector::is_glm_config(&config) {
            println!("{}", "⚠️  Already using GLM configuration".yellow());
            println!("{}", "   Use --status to check current settings".cyan());
            return Ok(());
        }

        // Check current provider and backup if necessary
        let current_provider = ProviderDetector::detect_provider(&config);

        match current_provider {
            Provider::Anthropic => {
                self.backup_anthropic_config_if_needed(&config)?;
            }
            Provider::Unknown => {
                self.handle_unknown_provider()?;
            }
            Provider::Custom => {
                self.handle_custom_provider();
            }
            _ => {}
        }

        // Get GLM API token
        let token = self
            .token_manager
            .prompt_for_token(&self.config_manager)
            .context("Failed to get GLM API token")?;

        // Validate token format
        ProviderDetector::validate_token_for_provider(&token, &Provider::GLM);

        // Create new config for GLM
        let new_config = self.create_glm_config(&token);

        self.config_manager
            .save_current_config(&new_config)
            .context("Failed to save GLM configuration")?;

        println!("{}", "✅ GLM configuration applied successfully".green());
        println!();
        println!(
            "{}",
            "💡 To switch back to Anthropic: claude-switch --anthropic".cyan()
        );
        Ok(())
    }

    fn backup_anthropic_config_if_needed(&self, config: &Config) -> Result<()> {
        // Check if valid Anthropic backup already exists
        let (has_backup, existing_backup) = self
            .config_manager
            .has_valid_anthropic_backup()
            .context("Failed to check existing backup")?;

        if has_backup && existing_backup.is_some() {
            // Backup already exists - don't overwrite
            println!(
                "{}",
                "💾 Existing Anthropic backup found (preserving configuration)".cyan()
            );
            if let Some(backup) = existing_backup {
                if let Some(created_at) = backup.metadata.created_at {
                    println!(
                        "{}{}",
                        "   Backed up at: ".cyan(),
                        created_at.format("%Y-%m-%d %H:%M:%S UTC")
                    );
                }
            }
        } else {
            // Create new backup with metadata
            self.config_manager
                .create_backup_with_metadata(config, &Provider::Anthropic)
                .context("Failed to backup Anthropic configuration")?;
            println!("{}", "✅ Anthropic configuration backed up".green());
        }
        Ok(())
    }

    fn handle_unknown_provider(&self) -> Result<()> {
        let (has_backup, _) = self.config_manager.has_valid_anthropic_backup()?;
        if has_backup {
            println!("{}", "💾 Using existing Anthropic backup".cyan());
        } else {
            println!("{}", "⚠️  No Anthropic configuration to backup".yellow());
            println!(
                "{}",
                "   You may need to re-login when switching back".yellow()
            );
        }
        Ok(())
    }

    fn handle_custom_provider(&self) {
        println!(
            "{}",
            "⚠️  Current config is custom provider - not backing up".yellow()
        );
        println!(
            "{}",
            "   Anthropic backup will be preserved if it exists".yellow()
        );
    }

    fn create_glm_config(&self, token: &str) -> Config {
        let mut env = std::collections::HashMap::new();

        env.insert("ANTHROPIC_AUTH_TOKEN".to_string(), token.to_string());
        env.insert(
            "ANTHROPIC_BASE_URL".to_string(),
            "https://api.z.ai/api/anthropic".to_string(),
        );
        env.insert("API_TIMEOUT_MS".to_string(), "3000000".to_string());
        env.insert(
            "ANTHROPIC_DEFAULT_OPUS_MODEL".to_string(),
            "GLM-4.7".to_string(),
        );
        env.insert(
            "ANTHROPIC_DEFAULT_SONNET_MODEL".to_string(),
            "GLM-4.7".to_string(),
        );
        env.insert(
            "ANTHROPIC_DEFAULT_HAIKU_MODEL".to_string(),
            "GLM-4.5-Air".to_string(),
        );

        Config { env }
    }
}
