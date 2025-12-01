use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub env: std::collections::HashMap<String, String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            env: std::collections::HashMap::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupMetadata {
    pub provider: String,
    #[serde(with = "chrono::serde::ts_seconds_option")]
    pub created_at: Option<DateTime<Utc>>,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupConfig {
    #[serde(rename = "_metadata")]
    pub metadata: BackupMetadata,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub env: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Provider {
    Anthropic,
    ZAI,
    Custom,
    Unknown,
}

impl Provider {
    pub fn as_str(&self) -> &'static str {
        match self {
            Provider::Anthropic => "anthropic",
            Provider::ZAI => "z_ai",
            Provider::Custom => "custom",
            Provider::Unknown => "unknown",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    ZAI,
    Anthropic,
    Unknown,
}