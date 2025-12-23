use crate::config::manager::ConfigManager;
use anyhow::Result;
use colored::*;
use std::io::{self, Write};

pub struct TokenManager;

impl TokenManager {
    pub fn new() -> Self {
        Self
    }

    pub fn prompt_for_token(&self, config_manager: &ConfigManager) -> Result<String> {
        // Check environment variable first
        if let Ok(token) = std::env::var("Z_AI_AUTH_TOKEN") {
            if !token.is_empty() {
                println!("{}", "📌 Using token from Z_AI_AUTH_TOKEN environment variable".cyan());
                return Ok(token);
            }
        }

        // Check if token file exists
        if let Ok(Some(saved_token)) = config_manager.load_saved_token() {
            println!("{}", "📌 Using token from saved token file".cyan());
            return Ok(saved_token);
        }

        // Prompt user for token
        println!("{}", "⚠️  No API token found".yellow());
        println!();
        println!("{}", "Please enter your Z.AI API token:".cyan());
        print!("> ");
        io::stdout().flush()?;

        let mut token = String::new();
        io::stdin().read_line(&mut token)?;
        token = token.trim().to_string();

        if token.is_empty() {
            return Err(anyhow::anyhow!("Token cannot be empty"));
        }

        // Ask if user wants to save the token
        println!("{}", "\nSave token for future use? (y/n)".cyan());
        print!("> ");
        io::stdout().flush()?;

        let mut answer = String::new();
        io::stdin().read_line(&mut answer)?;
        answer = answer.trim().to_lowercase();

        if answer == "y" || answer == "yes" {
            match config_manager.save_token(&token) {
                Ok(_) => println!("{}", "✅ Token saved successfully".green()),
                Err(e) => println!("{}{}", "⚠️  Failed to save token: ".yellow(), e),
            }
        }

        Ok(token)
    }

    pub fn clear_saved_token(config_manager: &ConfigManager) -> Result<()> {
        match config_manager.load_saved_token() {
            Ok(Some(_)) => {
                config_manager.remove_saved_token()?;
                println!("{}", "✅ Saved token removed successfully".green());
            }
            Ok(None) => {
                println!("{}", "⚠️  No saved token found".yellow());
            }
            Err(e) => {
                return Err(anyhow::anyhow!("Failed to check for saved token: {}", e));
            }
        }
        Ok(())
    }
}