use crate::config::{Config, Provider, TokenType};
use colored::Colorize;

pub struct ProviderDetector;

impl ProviderDetector {
    pub fn detect_provider(config: &Config) -> Provider {
        if config.env.is_empty() {
            return Provider::Unknown;
        }

        let empty_string = String::new();
        let base_url = config.env.get("ANTHROPIC_BASE_URL").unwrap_or(&empty_string);

        // Z.AI detection
        if base_url.contains("z.ai") {
            return Provider::ZAI;
        }

        // If no custom base URL, it's Anthropic (default)
        if base_url.is_empty() {
            return Provider::Anthropic;
        }

        // Custom provider
        Provider::Custom
    }

    pub fn is_anthropic_config(config: &Config) -> bool {
        Self::detect_provider(config) == Provider::Anthropic
    }

    pub fn is_zai_config(config: &Config) -> bool {
        Self::detect_provider(config) == Provider::ZAI
    }

    pub fn is_zai_key(key: &str) -> bool {
        matches!(key,
            "ANTHROPIC_BASE_URL" |
            "API_TIMEOUT_MS" |
            "ANTHROPIC_DEFAULT_OPUS_MODEL" |
            "ANTHROPIC_DEFAULT_SONNET_MODEL" |
            "ANTHROPIC_DEFAULT_HAIKU_MODEL"
        )
    }

    pub fn detect_token_type(token: &str) -> TokenType {
        if token.is_empty() {
            return TokenType::Unknown;
        }

        // Z.AI API keys typically start with specific prefixes
        if token.starts_with("sk-") || token.starts_with("z_ai-") {
            return TokenType::ZAI;
        }

        // Anthropic tokens are longer JWT-like tokens
        if token.matches('.').count() >= 2 && token.len() > 100 {
            return TokenType::Anthropic;
        }

        // If token is very long and doesn't look like an API key, assume web token
        if token.len() > 200 {
            return TokenType::Anthropic;
        }

        // Short tokens without special prefixes might be API keys
        if token.len() < 100 {
            return TokenType::ZAI;
        }

        TokenType::Unknown
    }

    pub fn validate_token_for_provider(token: &str, provider: &Provider) -> bool {
        let token_type = Self::detect_token_type(token);

        match provider {
            Provider::ZAI => {
                if token_type == TokenType::Anthropic {
                    eprintln!("{}", "⚠️  Warning: Token looks like an Anthropic token".yellow());
                    eprintln!("{}", "   Z.AI typically uses API keys (sk-xxx or z_ai-xxx format)".yellow());
                    return true; // Still allow, just warn
                }
            }
            Provider::Anthropic => {
                if token_type == TokenType::ZAI {
                    eprintln!("{}", "⚠️  Warning: Token looks like an API key".yellow());
                    eprintln!("{}", "   Anthropic uses longer JWT-style tokens".yellow());
                    return true; // Still allow, just warn
                }
            }
            _ => {}
        }

        true
    }

    pub fn mask_token(token: &str) -> String {
        if token.len() <= 8 {
            return "********".to_string();
        }
        format!("{}...{}", &token[..4], &token[token.len() - 4..])
    }
}