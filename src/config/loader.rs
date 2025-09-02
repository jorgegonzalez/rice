use super::{defaults::default_config_toml, Config};
use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;

pub fn load_config_from_path(custom_path: Option<String>) -> Result<Config> {
    let is_custom = custom_path.is_some();
    let config_path = if let Some(path) = custom_path {
        PathBuf::from(path)
    } else {
        get_config_path()?
    };

    if config_path.exists() {
        let content = fs::read_to_string(&config_path)
            .with_context(|| format!("Failed to read config file: {}", config_path.display()))?;

        toml::from_str(&content)
            .with_context(|| format!("Failed to parse config file: {}", config_path.display()))
    } else {
        if is_custom {
            return Err(anyhow::anyhow!(
                "Custom config file not found: {}",
                config_path.display()
            ));
        }
        // Create default config file
        create_default_config(&config_path)?;
        Ok(Config::default())
    }
}

pub fn get_config_path() -> Result<PathBuf> {
    // Use standard config directory paths:
    // Linux/macOS: ~/.config/rice/config.toml
    // Windows: %APPDATA%/rice/config.toml
    let config_dir = if cfg!(windows) {
        dirs::data_dir()
            .context("Could not determine data directory")?
            .join("rice")
    } else {
        dirs::home_dir()
            .context("Could not determine home directory")?
            .join(".config")
            .join("rice")
    };

    fs::create_dir_all(&config_dir).with_context(|| {
        format!(
            "Failed to create config directory: {}",
            config_dir.display()
        )
    })?;

    Ok(config_dir.join("config.toml"))
}

fn create_default_config(path: &PathBuf) -> Result<()> {
    fs::write(path, default_config_toml())
        .with_context(|| format!("Failed to create default config file: {}", path.display()))?;

    tracing::info!("Created default config file at: {}", path.display());
    Ok(())
}

pub fn _get_ascii_art_dir() -> Result<PathBuf> {
    let config_dir = if cfg!(windows) {
        dirs::data_dir()
            .context("Could not determine data directory")?
            .join("rice")
    } else {
        dirs::home_dir()
            .context("Could not determine home directory")?
            .join(".config")
            .join("rice")
    }
    .join("ascii_art");

    fs::create_dir_all(&config_dir).with_context(|| {
        format!(
            "Failed to create ASCII art directory: {}",
            config_dir.display()
        )
    })?;

    Ok(config_dir)
}
