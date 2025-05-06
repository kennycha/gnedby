use crate::config::models::SyncConfig;
use anyhow::{Context, Result};
use directories::ProjectDirs;
use keyring::Entry;
use std::fs;
use std::path::PathBuf;

const SERVICE_NAME: &str = "gnedby";
const TOKEN_KEY: &str = "supabase_token";

fn get_config_path() -> Result<PathBuf> {
    let project_dirs = ProjectDirs::from("com", "gnedby", "gnedby")
        .ok_or_else(|| anyhow::anyhow!("Failed to determine config directory"))?;

    let config_dir = project_dirs.config_dir();
    fs::create_dir_all(config_dir)?;

    let config_path = config_dir.join("sync_config.json");
    Ok(config_path)
}

fn get_keyring_entry() -> Result<Entry> {
    Entry::new(SERVICE_NAME, TOKEN_KEY).context("Failed to create keyring entry")
}

pub fn save_token(token: &str) -> Result<()> {
    let entry = get_keyring_entry()?;
    entry
        .set_password(token)
        .context("Failed to save token to secure storage")
}

pub fn get_token() -> Result<Option<String>> {
    let entry = get_keyring_entry()?;
    match entry.get_password() {
        Ok(token) => Ok(Some(token)),
        Err(keyring::Error::NoEntry) => Ok(None),
        Err(err) => Err(anyhow::anyhow!("Failed to retrieve token: {}", err)),
    }
}

pub fn delete_token() -> Result<()> {
    let entry = get_keyring_entry()?;
    let _ = entry.delete_credential();
    Ok(())
}

pub fn load_config() -> Result<SyncConfig> {
    let config_path = get_config_path()?;

    if !config_path.exists() {
        return Ok(SyncConfig::default());
    }

    let config_str = fs::read_to_string(&config_path).context("Failed to read config file")?;

    let mut config: SyncConfig =
        serde_json::from_str(&config_str).context("Failed to parse config file")?;

    config.token = get_token()?;

    Ok(config)
}

pub fn save_config(config: &SyncConfig) -> Result<()> {
    if let Some(token) = &config.token {
        save_token(token)?;
    }

    let mut config_to_save = config.clone();
    config_to_save.token = None;

    let config_path = get_config_path()?;
    let config_str =
        serde_json::to_string_pretty(&config_to_save).context("Failed to serialize config")?;

    fs::write(&config_path, config_str).context("Failed to write config file")?;

    Ok(())
}
