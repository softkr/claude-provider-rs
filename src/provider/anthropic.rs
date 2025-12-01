use crate::config::{Config, Provider};
use crate::provider::detector::ProviderDetector;
use crate::config::manager::ConfigManager;
use anyhow::{Context, Result};
use colored::*;

pub struct AnthropicSwitcher {
    config_manager: ConfigManager,
}

impl AnthropicSwitcher {
    pub fn new(config_manager: ConfigManager) -> Self {
        Self { config_manager }
    }

    pub fn switch_to_anthropic(&self) -> Result<()> {
        println!("{}", "🔄 Switching to Anthropic API...".green());

        // Load current config to check if already using Anthropic
        let current_config = self.config_manager.load_current_config()
            .context("Failed to load current config")?;

        if ProviderDetector::is_anthropic_config(&current_config) {
            println!("{}", "⚠️  Already using Anthropic configuration".yellow());
            println!("{}", "   Use --status to check current settings".cyan());
            return Ok(());
        }

        // Check if valid Anthropic backup exists
        let (has_backup, backup) = self.config_manager.has_valid_anthropic_backup()
            .context("Failed to check for backup")?;

        if !has_backup || backup.is_none() {
            println!("{}", "❌ No valid Anthropic backup found!".red());
            println!("{}", "⚠️  Cannot restore Anthropic configuration without backup.".yellow());
            println!("{}", "   You may need to reconfigure Claude Code.".yellow());
            println!();

            // Create empty config without Z.AI keys
            let config = Config::default();
            self.config_manager.save_current_config(&config)
                .context("Failed to save empty config")?;

            println!("{}", "⚠️  Created empty configuration (re-login required)".yellow());
            return Ok(());
        }

        let backup = backup.unwrap();

        // Show backup info
        if let Some(created_at) = backup.metadata.created_at {
            println!("{}{}",
                "💾 Restoring from backup created at: ".cyan(),
                created_at.format("%Y-%m-%d %H:%M:%S UTC")
            );
        }

        // Create config from backup
        let mut restored_config = Config { env: backup.env };

        // Remove any Z.AI specific keys that might be in backup
        let keys_to_remove: Vec<String> = restored_config.env
            .keys()
            .filter(|key| ProviderDetector::is_zai_key(key))
            .cloned()
            .collect();

        for key in keys_to_remove {
            restored_config.env.remove(&key);
        }

        self.config_manager.save_current_config(&restored_config)
            .context("Failed to restore config")?;

        println!("{}", "✅ Anthropic configuration restored from backup".green());
        Ok(())
    }
}