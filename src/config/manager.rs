use crate::config::{BackupConfig, BackupMetadata, Config, Provider};
use anyhow::{Context, Result};
use chrono::Utc;
use dirs::home_dir;
use std::fs;
use std::path::{Path, PathBuf};

pub struct ConfigManager {
    settings_file: PathBuf,
    backup_file: PathBuf,
    token_file: PathBuf,
}

impl ConfigManager {
    pub fn new() -> Result<Self> {
        let home = home_dir().ok_or_else(|| anyhow::anyhow!("Could not find home directory"))?;
        let config_dir = home.join(".claude");

        Ok(Self {
            settings_file: config_dir.join("settings.json"),
            backup_file: config_dir.join("settings.json.backup"),
            token_file: config_dir.join(".z_ai_token"),
        })
    }

    pub fn load_config(&self, path: &Path) -> Result<Config> {
        if !path.exists() {
            return Ok(Config::default());
        }

        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read config file: {}", path.display()))?;

        let config: Config =
            serde_json::from_str(&content).with_context(|| "Failed to parse config file")?;

        Ok(config)
    }

    pub fn load_current_config(&self) -> Result<Config> {
        self.load_config(&self.settings_file)
    }

    pub fn save_config_atomic(&self, path: &Path, config: &Config) -> Result<()> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create directory: {}", parent.display()))?;
        }

        let content =
            serde_json::to_string_pretty(config).with_context(|| "Failed to serialize config")?;

        let temp_path = path.with_extension("tmp");

        fs::write(&temp_path, content)
            .with_context(|| format!("Failed to write temp file: {}", temp_path.display()))?;

        fs::rename(&temp_path, path)
            .with_context(|| format!("Failed to rename temp file to: {}", path.display()))?;

        Ok(())
    }

    pub fn save_current_config(&self, config: &Config) -> Result<()> {
        self.save_config_atomic(&self.settings_file, config)
    }

    pub fn has_valid_anthropic_backup(&self) -> Result<(bool, Option<BackupConfig>)> {
        if !self.backup_file.exists() {
            return Ok((false, None));
        }

        let content =
            fs::read_to_string(&self.backup_file).with_context(|| "Failed to read backup file")?;

        // Try parsing as new format first
        if let Ok(backup) = serde_json::from_str::<BackupConfig>(&content) {
            let is_anthropic = backup.metadata.provider == Provider::Anthropic.as_str();
            Ok((is_anthropic, Some(backup)))
        } else {
            // Try parsing as old format (without metadata)
            if let Ok(old_config) = serde_json::from_str::<Config>(&content) {
                let backup = BackupConfig {
                    metadata: BackupMetadata {
                        provider: Provider::Anthropic.as_str().to_string(),
                        created_at: Some(Utc::now()),
                        version: "2.2.0".to_string(),
                    },
                    env: old_config.env,
                };
                Ok((true, Some(backup)))
            } else {
                Ok((false, None))
            }
        }
    }

    pub fn create_backup_with_metadata(&self, config: &Config, provider: &Provider) -> Result<()> {
        let backup = BackupConfig {
            metadata: BackupMetadata {
                provider: provider.as_str().to_string(),
                created_at: Some(Utc::now()),
                version: "2.2.0".to_string(),
            },
            env: config.env.clone(),
        };

        self.save_config_atomic(&self.backup_file, &Config { env: backup.env })?;

        // Also save metadata separately for easier access
        let metadata_path = self.backup_file.with_extension("meta");
        let metadata_content = serde_json::to_string_pretty(&backup.metadata)?;

        let temp_metadata = metadata_path.with_extension("tmp");
        fs::write(&temp_metadata, metadata_content)?;
        fs::rename(&temp_metadata, &metadata_path)?;

        Ok(())
    }

    pub fn save_token(&self, token: &str) -> Result<()> {
        if let Some(parent) = self.token_file.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::write(&self.token_file, token).context("Failed to save token")?;

        // Set restrictive permissions (600)
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&self.token_file)?.permissions();
            perms.set_mode(0o600);
            fs::set_permissions(&self.token_file, perms)?;
        }

        Ok(())
    }

    pub fn load_saved_token(&self) -> Result<Option<String>> {
        if !self.token_file.exists() {
            return Ok(None);
        }

        let token = fs::read_to_string(&self.token_file)
            .context("Failed to read saved token")?
            .trim()
            .to_string();

        if token.is_empty() {
            return Ok(None);
        }

        Ok(Some(token))
    }

    pub fn remove_saved_token(&self) -> Result<()> {
        if self.token_file.exists() {
            fs::remove_file(&self.token_file).context("Failed to remove saved token")?;
        }
        Ok(())
    }

    pub fn backup_file(&self) -> &Path {
        &self.backup_file
    }
}
