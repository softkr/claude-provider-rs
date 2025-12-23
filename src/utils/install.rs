use anyhow::{Context, Result};
use colored::*;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

pub struct Installer;

impl Installer {
    pub fn new() -> Result<Self> {
        Ok(Self)
    }

    pub fn install(&self) -> Result<()> {
        println!("{}", "🚀 Installing Claude Code API Switcher...".green());
        println!();

        // Get current executable path
        let exec_path = env::current_exe().context("Failed to get executable path")?;

        // Resolve symlinks
        let exec_path = exec_path
            .canonicalize()
            .context("Failed to resolve executable path")?;

        // Install binary to /usr/local/bin
        let install_path = PathBuf::from("/usr/local/bin/claude-switch");

        if exec_path != install_path {
            self.install_binary(&exec_path, &install_path)?;
        } else {
            println!(
                "{}",
                "📦 Binary already installed at /usr/local/bin/claude-switch".cyan()
            );
        }

        // Install shell aliases
        self.install_shell_aliases(&install_path)?;

        println!();
        println!("{}", "🎉 Installation complete!".green());
        println!();
        self.show_post_install_message();

        Ok(())
    }

    fn install_binary(&self, source_path: &Path, install_path: &Path) -> Result<()> {
        println!("{}", "📦 Installing binary to /usr/local/bin...".cyan());

        let source_data = fs::read(source_path).context("Failed to read source binary")?;

        // Try direct write first
        if let Err(_e) = fs::write(install_path, &source_data) {
            // Need sudo - use temp file approach
            println!(
                "{}",
                "⚠️  Need sudo permission to install to /usr/local/bin".yellow()
            );

            let temp_file = PathBuf::from("/tmp/claude-switch-install");
            fs::write(&temp_file, &source_data).context("Failed to write temp file")?;

            let cmd = format!(
                "sudo cp {} {} && sudo chmod +x {}",
                temp_file.display(),
                install_path.display(),
                install_path.display()
            );

            println!("Running: {}", cmd);

            let output = Command::new("bash")
                .args(&["-c", &cmd])
                .output()
                .context("Failed to execute sudo command")?;

            fs::remove_file(&temp_file)?;

            if !output.status.success() {
                return Err(anyhow::anyhow!(
                    "Failed to install binary (try running with sudo): {}",
                    String::from_utf8_lossy(&output.stderr)
                ));
            }
        }

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(install_path)?.permissions();
            perms.set_mode(0o755);
            fs::set_permissions(install_path, perms)?;
        }

        println!(
            "{}",
            "✅ Binary installed to /usr/local/bin/claude-switch".green()
        );
        Ok(())
    }

    fn install_shell_aliases(&self, exec_path: &Path) -> Result<()> {
        let shell_configs = self.detect_shell_configs();
        if shell_configs.is_empty() {
            return Err(anyhow::anyhow!("No supported shell configuration found"));
        }

        let exec_path_str = exec_path.to_string_lossy();

        // Create alias block for bash/zsh
        let alias_block = format!(
            r#"
# Claude Code API Switcher
alias claude-switch='{}'
alias claude-anthropic='{} --anthropic'
alias claude-glm='{} --glm'
alias claude-status='{} --status'
"#,
            exec_path_str, exec_path_str, exec_path_str, exec_path_str
        );

        // Fish shell uses different syntax
        let fish_alias_block = format!(
            r#"
# Claude Code API Switcher
alias claude-switch '{}'
alias claude-anthropic '{} --anthropic'
alias claude-glm '{} --glm'
alias claude-status '{} --status'
"#,
            exec_path_str, exec_path_str, exec_path_str, exec_path_str
        );

        let mut installed_count = 0;

        for shell_rc in &shell_configs {
            let is_fish = shell_rc.to_string_lossy().contains("fish");
            let block = if is_fish {
                &fish_alias_block
            } else {
                &alias_block
            };

            // Read existing shell config
            let content = fs::read_to_string(shell_rc).unwrap_or_default();

            // Check if aliases already exist
            if content.contains("Claude Code API Switcher") {
                println!(
                    "{}{}",
                    "⚠️  Aliases already exist in ".yellow(),
                    shell_rc.display()
                );
                continue;
            }

            // Append aliases
            fs::write(shell_rc, format!("{}\n{}", content, block))
                .with_context(|| format!("Failed to write to {}", shell_rc.display()))?;

            println!("{}{}", "✅ Aliases added to ".green(), shell_rc.display());
            installed_count += 1;
        }

        if installed_count == 0 {
            println!("{}", "⚠️  No new aliases were installed".yellow());
        }

        Ok(())
    }

    fn detect_shell_configs(&self) -> Vec<PathBuf> {
        let home = match dirs::home_dir() {
            Some(h) => h,
            None => return Vec::new(),
        };

        let shell = env::var("SHELL").unwrap_or_default();

        // Common shell config files
        let candidates = vec![
            (home.join(".zshrc"), "zsh"),
            (home.join(".bashrc"), "bash"),
            (home.join(".bash_profile"), "bash"),
            (home.join(".config/fish/config.fish"), "fish"),
        ];

        let mut configs = Vec::new();

        // Add config for current shell first
        for (path, shell_name) in &candidates {
            if shell.contains(shell_name) && path.exists() {
                configs.push(path.clone());
                break;
            }
        }

        // If no config found for current shell, check all
        if configs.is_empty() {
            for (path, _) in &candidates {
                if path.exists() {
                    configs.push(path.clone());
                    break;
                }
            }
        }

        configs
    }

    fn show_post_install_message(&self) {
        println!("{}", "Available commands after reload:".cyan());
        println!("  claude-switch --anthropic  # Use Anthropic Claude");
        println!("  claude-switch --glm        # Use GLM");
        println!("  claude-switch --status     # Check current config");
        println!("  claude-anthropic           # Quick switch to Anthropic");
        println!("  claude-glm                 # Quick switch to GLM");
        println!("  claude-status              # Quick status check");
        println!();
        println!("{}", "Reload your shell:".cyan());

        let shell_configs = self.detect_shell_configs();
        for config in shell_configs {
            println!("  source {}", config.display());
        }
    }
}
